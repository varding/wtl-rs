
use std::{self,ptr};
use winapi::*;
use user32;
use kernel32;

use super::super::thunk;
use super::super::cwindow::*;
use super::consts::*;

pub struct CDialogImpl {
	pub cwin  : CWindow,
    thk   : &'static mut thunk::Thunk,
    idd   : WORD,
    state : DWORD,
    m_bModal:bool,
}

fn MAKEINTRESOURCEW(id:WORD)->LPCWSTR {
	id as usize as LPCWSTR
}

//frequently used
impl CDialogImpl {
	fn ProcessWindowMessage(&mut self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL{
		if uMsg == WM_CLOSE{
			// if self.m_bModal{
			// 	self.EndDialog(0);
			// }else{
			// 	self.DestroyWindow();
			// }
			unsafe{user32::PostQuitMessage(0)};
		}
		0
	}

	fn InitThunk(&mut self,h:HWND,dlg_proc:DLGPROC) -> DLGPROC { //convert &mut self to *const T in this method
		println!("init thunk,thunk addr:{:p}",&dlg_proc);
		//self.m_hWnd = h;
		self.cwin.Attach(h);
		dlg_proc
	}

	pub fn new(idd:WORD)->CDialogImpl{
		CDialogImpl{
			cwin  : CWindow::new(NULL_HWND),
			thk   : thunk::get_thunk(),
			idd   : idd,
			state : 0,
			m_bModal:false,
			//msg_handler : MsgHandler::new(),
		}
	}
}

impl CDialogImpl {
	//root
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
					if lParam != 0 as LPARAM{
						hWndChild = lParam as HWND;
					}
				}
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

//dialog baseT
impl CDialogImpl {
	unsafe extern "system" fn StartDialogProc(hWnd:HWND ,uMsg:UINT ,wParam:WPARAM ,lParam:LPARAM ) -> INT_PTR {
		let p_this = thunk::get_this();
		let pself = p_this as *mut Self;
		println!("start dialog proc,addr:{:p},DialogProc:{:p}",&Self::StartDialogProc,&Self::DialogProc);

		let dlg_proc = Self::InitThunk(&mut *pself,hWnd,Self::DialogProc);
		//user32::SetWindowLongPtrW(hWnd, DWLP_DLGPROC, dlg_proc as LONG_PTR);
		user32::SetWindowLongPtrW(hWnd, (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int, dlg_proc as LONG_PTR);
		//T::Attach(&mut *pself,hWnd);		//UFCS			
		//T::ProcessMessage(&*pself,hWnd,uMsg,wParam,lParam);
		dlg_proc(hWnd, uMsg, wParam, lParam)
	}

	fn DialogProcLocal(&mut self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM)->INT_PTR{
		let mut lRes:LRESULT = 0;
		let mut bRet = self.ProcessWindowMessage(hWnd,uMsg,wParam,lParam,&mut lRes,0);
		if bRet == TRUE{
			match uMsg{
				WM_COMPAREITEM|WM_VKEYTOITEM|WM_CHARTOITEM|WM_INITDIALOG|WM_QUERYDRAGICON|WM_CTLCOLORMSGBOX|WM_CTLCOLOREDIT|
				WM_CTLCOLORLISTBOX|WM_CTLCOLORBTN|WM_CTLCOLORDLG|WM_CTLCOLORSCROLLBAR|WM_CTLCOLORSTATIC => {
					bRet = lRes as BOOL;
				},
				// return in DWL_MSGRESULT
				//Make sure the window was not destroyed before setting attributes.
				_=>{
					if self.state & WINSTATE_DESTROYED == 0{
						unsafe{user32::SetWindowLongPtrW(self.cwin.GetHwnd(), DWLP_MSGRESULT as c_int, lRes);}
					}
				},
			}
		}else if uMsg == WM_NCDESTROY{
			//pThis->m_dwState |= WINSTATE_DESTROYED;
			//Self::AddState(&*p_self,WINSTATE_DESTROYED);
			self.state |= WINSTATE_DESTROYED;
		}

		//I don't know this variable mean,it points to a stack value ,where set NULL?  pThis->m_pCurrentMsg == NULL
		// if (pThis->m_dwState & WINSTATE_DESTROYED) // && pThis->m_pCurrentMsg == NULL)

		// {
		// 	// clear out window handle
		// 	HWND hWndThis = pThis->m_hWnd;
		// 	pThis->m_hWnd = NULL;
		// 	pThis->m_dwState &= ~WINSTATE_DESTROYED;
		// 	// clean up after dialog is destroyed
		// 	pThis->OnFinalMessage(hWndThis);
		// }
		//return bRet;
		//let dlg_proc = T::InitThunk(&mut *pself,hWnd);
		//user32::SetWindowLongPtrW(hWnd, DWLP_DLGPROC, dlg_proc as LONG_PTR);
		//user32::SetWindowLongPtrW(hWnd, (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT) as c_int, dlg_proc as LONG_PTR);
		//T::Attach(&mut *pself,hWnd);		//UFCS			
		//T::ProcessMessage(&*pself,hWnd,uMsg,wParam,lParam);
		//dlg_proc(hWnd, uMsg, wParam, lParam)
		0
	}

	unsafe extern "system" fn DialogProc(hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM) -> INT_PTR {
		let p_self = hWnd as *mut Self;
		//println!("dialog proc");

		//Self::ProcessWindowMessage(&*p_self,hWnd,uMsg,wParam,lParam,&mut lRes,0);
		//most of the code move to DialogProcLocal,it will be conveient to visit all field of CDialogImpl
		Self::DialogProcLocal(&mut *p_self,hWnd,uMsg,wParam,lParam)
	}
}

//dialog impl
impl CDialogImpl {
	pub fn DoModal2(&mut self){
		 //HWND hWndParent = ::GetActiveWindow(),
		 //LPARAM dwInitParam = NULL
		 self.DoModal(NULL_HWND, 0 as LPARAM);
	}
	
	pub fn DoModal(&mut self,hWndParent:HWND,dwInitParam:LPARAM) -> INT_PTR {
		//ATLASSUME(m_hWnd == NULL);
		self.m_bModal = true;

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
		//ATLASSERT(::IsWindow(m_hWnd));
		self.cwin.assert_window();
		assert!(self.m_bModal);
		unsafe{user32::EndDialog(self.cwin.GetHwnd(), nRetCode as INT_PTR)}
	}

	// modeless dialogs
	pub fn Create2(&mut self){
		self.Create(NULL_HWND, 0 as LPARAM);
	}

	pub fn Create(&mut self,hWndParent:HWND,dwInitParam:LPARAM) -> HWND {
		//BOOL result;

		//ATLASSUME(m_hWnd == NULL);

		thunk::set_this(self as *mut Self as *mut c_void);

		self.m_bModal = false;

		unsafe{
			let hWnd = user32::CreateDialogParamW(0 as HINSTANCE,MAKEINTRESOURCEW(self.idd) ,
						hWndParent, Self::StartDialogProc, dwInitParam);
			let e = kernel32::GetLastError();
			println!("err:{}", e);
			user32::ShowWindow(hWnd,SW_SHOW);
			//ATLASSUME(m_hWnd == hWnd);
			hWnd
		}
	}

	pub fn DestroyWindow(&mut self) -> BOOL {
		self.cwin.assert_window();
		assert!(self.m_bModal == false);
		unsafe{
			if user32::DestroyWindow(self.cwin.GetHwnd()) == FALSE {
				return FALSE;
			}
		}
		return TRUE;
	}
}


				