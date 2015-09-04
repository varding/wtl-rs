
use std::{self,ptr};
use winapi::*;
use user32;
use kernel32;

use super::super::thunk;
use super::super::cwindow::*;
use super::consts::*;

pub type ProcWinMsg = fn(*mut c_void,HWND,UINT,WPARAM,LPARAM,&mut LRESULT,DWORD ) -> BOOL;

pub struct CDialogImpl {
	cwin 	: CWindow,						//basic operations for objects that have HWND
    thk 	: &'static mut thunk::Thunk,	//thunk that convert static function call to method
    idd 	: WORD,							//resource id of the dlg
    state 	: DWORD,						//destroy or not
    modal 	: bool,							//is modal dialog
    proc_msg: ProcWinMsg,					//user function:ProcessWindowMessage
    pdlg 	:*mut c_void,					//user dlg self pointer
    //cur_msg : Option<MSG>
    dlg_proc:DLGPROC,						//store the real dlg proc for StartDlgProc,user can override it 
}

//expose all method of cwindow
expose_cwindow!(CDialogImpl);

fn MAKEINTRESOURCEW(id:WORD)->LPCWSTR {
	id as usize as LPCWSTR
}

//frequently used
impl CDialogImpl {
	fn InitThunk(&mut self,h:HWND,dlg_proc:DLGPROC) -> DLGPROC {
		let pself = self as *mut Self as *mut c_void;
		self.thk.init(dlg_proc as DWORD_PTR, pself);
		self.cwin.Attach(h);
		let p = self.thk.GetCodeAddress();
		unsafe { std::mem::transmute(p) }
	}

	//user can pass a dlg_proc to override the default DLGPROC of CDialogImpl,and take over every msg your self
	pub fn new(idd:WORD,proc_msg:ProcWinMsg,dlg_proc:Option<DLGPROC>)->CDialogImpl{
		let mut real_dlg_proc:DLGPROC = Self::DialogProc;
		if let Some(p) = dlg_proc {
			real_dlg_proc = p;
		}

		CDialogImpl{
			cwin 	: CWindow::new(NULL_HWND),
			thk 	: thunk::get_thunk(),
			idd 	: idd,
			state 	: 0,
			modal 	: false,
			pdlg 	: 0 as *mut c_void,
			proc_msg: proc_msg,
			dlg_proc: real_dlg_proc,
		}
	}
}

//CWindowImplRoot
impl CDialogImpl {
	fn ForwardNotifications(&self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL) -> LRESULT {
		let mut lResult:LRESULT = 0;
		match uMsg{
			WM_COMMAND|WM_NOTIFY|WM_PARENTNOTIFY|WM_DRAWITEM|WM_MEASUREITEM|WM_COMPAREITEM|WM_DELETEITEM|WM_VKEYTOITEM|
			WM_CHARTOITEM|WM_HSCROLL|WM_VSCROLL|WM_CTLCOLORBTN|WM_CTLCOLORDLG|WM_CTLCOLOREDIT|WM_CTLCOLORLISTBOX|
			WM_CTLCOLORMSGBOX|WM_CTLCOLORSCROLLBAR|WM_CTLCOLORSTATIC=>{
				lResult = self.cwin.GetParent2().SendMessage(uMsg, wParam, lParam);
			},
			_=>*bHandled = FALSE,
		}
		lResult
	}

	fn ReflectNotifications(&self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL) -> LRESULT {
		let mut hWndChild = NULL_HWND;
		unsafe{
			match uMsg{
				WM_COMMAND=>{
					if lParam != NULL_LPARAM {
						hWndChild = lParam as HWND;
					}
				},
				WM_NOTIFY=>{
					hWndChild = (*(lParam as LPNMHDR)).hwndFrom;
				},
				WM_PARENTNOTIFY=>{
					match LOWORD(wParam as DWORD) as DWORD {
						WM_CREATE|WM_DESTROY=>hWndChild = lParam as HWND,
						_=>hWndChild = self.cwin.GetDlgItem2(HIWORD(wParam as DWORD) as c_int).GetHwnd(),
					}
				},
				WM_DRAWITEM=>{
					if wParam != 0{
						hWndChild = (*(lParam as LPDRAWITEMSTRUCT)).hwndItem;
					}
				},
				WM_MEASUREITEM=>{
					if wParam != 0{
						let id = (*(lParam as LPMEASUREITEMSTRUCT)).CtlID;
						hWndChild = self.cwin.GetDlgItem2(id as c_int).GetHwnd();
					}
				},
				WM_COMPAREITEM=>{
					if wParam != 0{
						hWndChild = (*(lParam as LPCOMPAREITEMSTRUCT)).hwndItem;
					}
				},
				WM_DELETEITEM=>{
					if wParam != 0{
						hWndChild = (*(lParam as LPDELETEITEMSTRUCT)).hwndItem;
					}
				},
				WM_VKEYTOITEM|WM_CHARTOITEM|WM_HSCROLL|WM_VSCROLL=>hWndChild = lParam as HWND,
				WM_CTLCOLORBTN|WM_CTLCOLORDLG|WM_CTLCOLOREDIT|WM_CTLCOLORLISTBOX|
				WM_CTLCOLORMSGBOX|WM_CTLCOLORSCROLLBAR|WM_CTLCOLORSTATIC=>hWndChild = lParam as HWND,
				_=>(),
			}

			if hWndChild == NULL_HWND {
				*bHandled = FALSE;
				return 1;
			}

			//ATLASSERT(::IsWindow(hWndChild));
			assert!(user32::IsWindow(hWndChild) == TRUE);
			user32::SendMessageW(hWndChild, OCM__BASE + uMsg, wParam, lParam)
		}
	}

	//static function
	fn DefaultReflectionHandler(hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT) -> BOOL{
		match uMsg{
			OCM_COMMAND|OCM_NOTIFY|OCM_PARENTNOTIFY|OCM_DRAWITEM|OCM_MEASUREITEM|OCM_COMPAREITEM|
			OCM_DELETEITEM|OCM_VKEYTOITEM|OCM_CHARTOITEM|OCM_HSCROLL|OCM_VSCROLL|OCM_CTLCOLORBTN|OCM_CTLCOLORDLG|
			OCM_CTLCOLOREDIT|OCM_CTLCOLORLISTBOX|OCM_CTLCOLORMSGBOX|OCM_CTLCOLORSCROLLBAR|OCM_CTLCOLORSTATIC=>{
				unsafe {*lResult = user32::DefWindowProcW(hWnd, uMsg - OCM__BASE, wParam, lParam);}
				return TRUE;
			},
			_=>(),
		}
		FALSE
	}	

}

//CDialogImplBaseT
impl CDialogImpl {
	unsafe extern "system" fn StartDialogProc(hWnd:HWND ,uMsg:UINT ,wParam:WPARAM ,lParam:LPARAM ) -> INT_PTR {
		let p_this = thunk::get_this();
		//println!("4. get this:{:p}", p_this);
		let pself = p_this as *mut Self;
		//println!("5. start dialog proc,addr:0x{:x},DialogProc:0x{:x}",Self::StartDialogProc as usize,Self::DialogProc as usize);
		//println!("6. proc_msg before init thunk:0x{:x}", Self::DialogProc as usize);
		let dlg_proc_thunk = Self::InitThunk(&mut *pself,hWnd,Self::DialogProc);
		//println!("7. start proc,thunk addr:0x{:x}", proc_msg as usize);

		//DWLP_DLGPROC = sizeof(LRESULT) + DWLP_MSGRESULT
		user32::SetWindowLongPtrW(hWnd, (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int, dlg_proc_thunk as LONG_PTR);
		
		//it is actually the entry of the thunk
		dlg_proc_thunk(hWnd, uMsg, wParam, lParam)
	}

	//if bHandled return TRUE
	unsafe extern "system" fn DialogProc(hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM) -> INT_PTR {
		let p_self = hWnd as *mut Self;
		let mut_self = unsafe{&mut *p_self};
		let mut lRes:LRESULT = 0;
		let mut bRet = unsafe{(mut_self.proc_msg)(mut_self.pdlg,hWnd,uMsg,wParam,lParam,&mut lRes,0)};
		if bRet == TRUE {
			match uMsg{
				WM_COMPAREITEM|WM_VKEYTOITEM|WM_CHARTOITEM|WM_INITDIALOG|WM_QUERYDRAGICON|WM_CTLCOLORMSGBOX|WM_CTLCOLOREDIT|
				WM_CTLCOLORLISTBOX|WM_CTLCOLORBTN|WM_CTLCOLORDLG|WM_CTLCOLORSCROLLBAR|WM_CTLCOLORSTATIC => {
					bRet = lRes as BOOL;
				},
				// return in DWL_MSGRESULT
				//Make sure the window was not destroyed before setting attributes.
				_=>{
					if mut_self.state & WINSTATE_DESTROYED == 0 {
						unsafe{user32::SetWindowLongPtrW(mut_self.GetHwnd(), DWLP_MSGRESULT as c_int, lRes);}
					}
				},
			}
		}else if uMsg == WM_NCDESTROY{
			mut_self.state |= WINSTATE_DESTROYED;
		}
		
		if ( mut_self.state & WINSTATE_DESTROYED != 0 ) /*&& mut_self.cur_msg.is_none()*/ {
			let hWndThis = mut_self.Detach();
			mut_self.state &= !WINSTATE_DESTROYED;
			// clean up after dialog is destroyed
			//mut_self->OnFinalMessage(hWndThis);
		}
		bRet as INT_PTR
		//0
	}
}

//CDialogImpl
impl CDialogImpl {
	pub fn DoModal2(&mut self,pdlg:*mut c_void){
		 let hWndParent = unsafe{user32::GetActiveWindow()};
		 self.DoModal(pdlg,hWndParent, NULL_LPARAM);
	}
	
	pub fn DoModal(&mut self,pdlg 	:*mut c_void,hWndParent:HWND,dwInitParam:LPARAM) -> INT_PTR {
		//ATLASSUME(m_hWnd == NULL);
		self.modal = true;
		self.pdlg  = pdlg 	;
		thunk::set_this(self as *mut Self as *mut c_void);

		unsafe{
			let hInst = kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE;
			let r = user32::DialogBoxParamW(hInst, MAKEINTRESOURCEW(self.idd),hWndParent, Self::StartDialogProc, dwInitParam);
			//let e = kernel32::GetLastError();
			//println!("err:{}", e);
			r
		} 
	}

	pub fn EndDialog(&self, nRetCode:c_int) -> BOOL {
		self.cwin.assert_window();
		assert!(self.modal);
		unsafe{user32::EndDialog(self.cwin.GetHwnd(), nRetCode as INT_PTR)}
	}

	// modeless dialogs
	pub fn Create2(&mut self,pdlg: *mut c_void){
		self.Create(pdlg,NULL_HWND, NULL_LPARAM);
	}

	pub fn Create(&mut self,pdlg:*mut c_void,hWndParent:HWND,dwInitParam:LPARAM) -> HWND {
		//ATLASSUME(m_hWnd == NULL);
		self.pdlg = pdlg 	;
		thunk::set_this(self as *mut Self as *mut c_void);

		self.modal = false;

		unsafe{
			let hWnd = user32::CreateDialogParamW(0 as HINSTANCE,MAKEINTRESOURCEW(self.idd) ,
						hWndParent, Self::StartDialogProc, dwInitParam);
			// let e = kernel32::GetLastError();
			// println!("err:{}", e);
			//self.cwin.Attach(hWnd);
			user32::ShowWindow(hWnd,SW_SHOW);
			//self.cwin.ShowWindow(SW_SHOW);
			//ATLASSUME(m_hWnd == hWnd);
			hWnd
		}
	}

	pub fn DestroyWindow(&mut self) -> BOOL {
		self.cwin.assert_window();
		assert!(self.modal == false);
		unsafe{
			if user32::DestroyWindow(self.cwin.GetHwnd()) == FALSE {
				return FALSE;
			}
		}
		return TRUE;
	}
}
