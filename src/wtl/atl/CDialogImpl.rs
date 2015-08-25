

/*
::DialogBoxParam(_AtlBaseModule.GetResourceInstance(), MAKEINTRESOURCE(static_cast<T*>(this)->IDD),
					hWndParent, T::StartDialogProc, dwInitParam);
*/

use winapi::*;
use user32;
//https://github.com/klutzy/rust-windows/blob/master/src/window.rs
//https://github.com/klutzy/rust-windows/blob/master/src/lib.rs
pub unsafe extern "system" fn main_wnd_proc(hwnd: HWND,msg: UINT,w: WPARAM,l: LPARAM) -> LRESULT {
	user32::DefWindowProcW(hwnd, msg, w, l)
}



pub fn new(){
	
}