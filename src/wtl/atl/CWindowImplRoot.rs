

//include base message operations like ForwardNotifications,ReflectNotifications,DefaultReflectionHandler and handle state

/*
							impl_root
						/				\
					win_impl_base		dlg_impl_base
					/					/	  |		\
				win_impl		ax_dlg_impl	dlg_imp	simple_dlg

*/
use winapi::*;

use super::thunk::Thunk;
use super::CWindow::CWindow;
// all fields will add to Wnd
// pub struct CWindowImplRoot {
//     proc_thunk: Thunk,
//     m_dwState : DWORD,
// }

pub trait CWindowImplRoot : CWindow {
	fn ForwardNotifications(&self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL) -> LRESULT {
		let mut lResult:LRESULT = 0;
		match uMsg{
			WM_COMMAND|WM_NOTIFY|WM_PARENTNOTIFY|WM_DRAWITEM|WM_MEASUREITEM|WM_COMPAREITEM|WM_DELETEITEM|WM_VKEYTOITEM|
			WM_CHARTOITEM|WM_HSCROLL|WM_VSCROLL|WM_CTLCOLORBTN|WM_CTLCOLORDLG|WM_CTLCOLOREDIT|WM_CTLCOLORLISTBOX|
			WM_CTLCOLORMSGBOX|WM_CTLCOLORSCROLLBAR|WM_CTLCOLORSTATIC=>{
				lResult = self.GetParent().SendMessage(uMsg, wParam, lParam);
			},
			_=>bHandled = FALSE,
		}
		lResult
	}

	fn ReflectNotifications(&self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL) -> LRESULT {

	}

	fn DefaultReflectionHandler(hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT) -> BOOL{

	}
}