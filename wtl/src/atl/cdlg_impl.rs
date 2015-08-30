

/*
::DialogBoxParam(_AtlBaseModule.GetResourceInstance(), MAKEINTRESOURCE(static_cast<T*>(this)->IDD),
					hWndParent, T::StartDialogProc, dwInitParam);
*/

use std::ptr;
use winapi::*;
use user32;
use kernel32;
use super::cdlg_impl_base::CDialogImplBaseT;
use super::thunk::*;
use super::cwindow::{NULL_HWND};
//https://github.com/klutzy/rust-windows/blob/master/src/window.rs
//https://github.com/klutzy/rust-windows/blob/master/src/lib.rs
// pub unsafe extern "system" fn main_wnd_proc(hwnd: HWND,msg: UINT,w: WPARAM,l: LPARAM) -> LRESULT {
// 	user32::DefWindowProcW(hwnd, msg, w, l)
// }

fn MAKEINTRESOURCEW(id:WORD)->LPCWSTR {
	id as usize as LPCWSTR
}

//impl <T> CDialogImpl for T where T:CDialogImplBaseT {}
pub trait CDialogImpl:CDialogImplBaseT {
	fn DoModal2(){
		 //HWND hWndParent = ::GetActiveWindow(),
		 //LPARAM dwInitParam = NULL
	}
	
	fn DoModal(&mut self,hWndParent:HWND,dwInitParam:LPARAM) -> INT_PTR {
		//ATLASSUME(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail
		// gracefully.

		let result = self.NewThunk();
		//result = m_thunk.Init(NULL,NULL);
		if result == false {
			//SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}
		set_this(self as *mut Self as *mut c_void);

		//_AtlWinModule.AddCreateWndData(&m_thunk.cd, (CDialogImplBaseT< TBase >*)this);
// #ifdef _DEBUG
// 		m_bModal = true;
// #endif //_DEBUG

		unsafe{
			let hInst = kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE;
			let r = user32::DialogBoxParamW(hInst, MAKEINTRESOURCEW(self.IDD()),hWndParent, Self::StartDialogProc, dwInitParam);
			let e = kernel32::GetLastError();
			println!("err:{}", e);
			r
		} 
	}

	fn EndDialog(&self, nRetCode:c_int) -> BOOL {
		//ATLASSERT(::IsWindow(m_hWnd));
		self.assert_window();
// #ifdef _DEBUG
// 		ATLASSUME(m_bModal);	// must be a modal dialog
// #endif //_DEBUG
		unsafe{user32::EndDialog(self.GetHwnd(), nRetCode as INT_PTR)}
	}
	// modeless dialogs
	fn Create(&mut self,hWndParent:HWND,dwInitParam:LPARAM) -> HWND {
		//BOOL result;

		//ATLASSUME(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail
		// gracefully.

		//result = m_thunk.Init(NULL,NULL);
		let result = self.NewThunk();
		if result == false {
			//SetLastError(ERROR_OUTOFMEMORY);
			return NULL_HWND;
		}

		set_this(self as *mut Self as *mut c_void);
		//_AtlWinModule.AddCreateWndData(&m_thunk.cd, (CDialogImplBaseT< TBase >*)this);
// #ifdef _DEBUG
// 		m_bModal = false;
// #endif //_DEBUG
		//MAKEINTRESOURCE(static_cast<T*>(this)->IDD)
		unsafe{
			let hWnd = user32::CreateDialogParamW(0 as HINSTANCE,MAKEINTRESOURCEW(self.IDD()) ,
						hWndParent, Self::StartDialogProc, dwInitParam);
			//ATLASSUME(m_hWnd == hWnd);
			hWnd
		}
	}
	// for CComControl
	// fn Create(hWndParent:HWND, RECT&,dwInitParam:LPARAM) -> HWND {
	// 	return Create(hWndParent, dwInitParam);
	// }

	fn DestroyWindow(&mut self) -> BOOL {
		//ATLASSERT(::IsWindow(m_hWnd));
		self.assert_window();
// #ifdef _DEBUG
// 		ATLASSERT(!m_bModal);	// must not be a modal dialog
// #endif //_DEBUG
		unsafe{
			if user32::DestroyWindow(self.GetHwnd()) == FALSE {
				return FALSE;
			}
		}
		

		return TRUE;
	}
}