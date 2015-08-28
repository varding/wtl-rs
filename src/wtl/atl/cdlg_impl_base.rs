

//it's more simple than win_impl_base,
// StartDialogProc passed to create to hold the first message and init thunk
// DialogProc is the real message proc

use std;
use winapi::*;
use user32;
use super::thunk;
use super::cwin_impl_root::{CWindowImplRoot,WINSTATE_DESTROYED};

use super::cwindow::{CWindow,WindowTrait,HwndTrait,NULL_HWND};

pub type WNDPROC2 = unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;
pub type DLGPROC2 = unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> INT_PTR;

//Dialog must impl this trait to init thunk,and return the dlg_proc that difined in Dialog
pub trait WinInitTrait {
	//give a default dlg_proc,but the Dialog can also use other function
	fn NewThunk(&mut self);
	fn InitThunk(&mut self,h:HWND,dlg_proc:DLGPROC2) -> DLGPROC2;		//convert &mut self to *const T in this method
	fn ProcessWindowMessage(&self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL;
	fn State(&self)->DWORD;
	fn AddState(&self,s:DWORD);
	fn GetHwnd(&self)->HWND;
	fn OnFinalMessage(&self);
}

const DWLP_MSGRESULT: LRESULT = 0;
//const DWLP_DLGPROC: UINT = DWLP_MSGRESULT + sizeof(LRESULT);   

pub trait CDialogImplBaseT<T:WinInitTrait>:CWindowImplRoot {
	fn StartDialogProc(hWnd:HWND ,uMsg:UINT ,wParam:WPARAM ,lParam:LPARAM ) -> INT_PTR {
		let p_this = thunk::get_this();
		let pself = p_this as *mut T;
		unsafe{
			let dlg_proc = T::InitThunk(&mut *pself,hWnd,Self::DialogProc);
			//user32::SetWindowLongPtrW(hWnd, DWLP_DLGPROC, dlg_proc as LONG_PTR);
			user32::SetWindowLongPtrW(hWnd, (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int, dlg_proc as LONG_PTR);
			//T::Attach(&mut *pself,hWnd);		//UFCS			
			//T::ProcessMessage(&*pself,hWnd,uMsg,wParam,lParam);
			dlg_proc(hWnd, uMsg, wParam, lParam)
		}
	}

	unsafe extern "system" fn DialogProc(hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM) -> INT_PTR {
		let p_self = hWnd as *mut T;
		let mut lRes:LRESULT = 0;

		let mut bRet = T::ProcessWindowMessage(&*p_self,hWnd,uMsg,wParam,lParam,&mut lRes,0);
		if bRet == TRUE{
			match uMsg{
				WM_COMPAREITEM|WM_VKEYTOITEM|WM_CHARTOITEM|WM_INITDIALOG|WM_QUERYDRAGICON|WM_CTLCOLORMSGBOX|WM_CTLCOLOREDIT|
				WM_CTLCOLORLISTBOX|WM_CTLCOLORBTN|WM_CTLCOLORDLG|WM_CTLCOLORSCROLLBAR|WM_CTLCOLORSTATIC => {
					bRet = lRes as BOOL;
				},
				// return in DWL_MSGRESULT
				//Make sure the window was not destroyed before setting attributes.
				_=>{
					if T::State(&*p_self) & WINSTATE_DESTROYED == 0 {
						user32::SetWindowLongPtrW(T::GetHwnd(&*p_self), DWLP_MSGRESULT as c_int, lRes);
					}
				},
			}
		}else if uMsg == WM_NCDESTROY{
			//pThis->m_dwState |= WINSTATE_DESTROYED;
			T::AddState(&*p_self,WINSTATE_DESTROYED);
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
}

// trait MyDlgTrait<T:WindowTrait>:CDialogImplBaseT<T> {
// 	// Add code here
// }

pub trait MessageLoopDlg {
	// Add code here
	fn ProcessMessage(&self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM);
}
	
//
//impl <T,> CDialogImplBaseT for T where T: CWindowImplRoot + MessageLoopDlg {}
// struct Foo {
// 	m_hWnd:HWND
// }

// impl HwndReader for Foo {
// 	fn GetHwnd(&self) -> HWND {
// 		self.m_hWnd
// 	}

// 	fn Detach (&mut self) -> HWND {
// 		let hWnd = self.m_hWnd;
// 		self.m_hWnd = NULL_HWND;
// 		hWnd
// 	}
// }

// impl CDialogImplBaseT<CWindow> for Foo {
	
// }