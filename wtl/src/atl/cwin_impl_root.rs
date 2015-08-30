

//include base message operations like ForwardNotifications,ReflectNotifications,DefaultReflectionHandler and handle state

/*
							impl_root
						/				\
					win_impl_base		dlg_impl_base
					/					/	  |		\
				win_impl		ax_dlg_impl	dlg_imp	simple_dlg

*/
use winapi::*;
use user32;
//use super::thunk::Thunk;
use super::cwindow::*;
// all fields will add to Wnd
// pub struct CWindowImplRoot {
//     proc_thunk: Thunk,
//     m_dwState : DWORD,
// }

pub const WINSTATE_DESTROYED: DWORD = 0x00000001;
//winsdk 7.0A  windowx.h
//const WM_CTLCOLOR:			UINT = 0x0019;

//winsdk 7.0A  OleCtl.h
const OCM__BASE: 			UINT = WM_USER+0x1c00;
const OCM_COMMAND: 			UINT = OCM__BASE + WM_COMMAND;

const OCM_CTLCOLORBTN: 		UINT = OCM__BASE + WM_CTLCOLORBTN;
const OCM_CTLCOLOREDIT: 	UINT = OCM__BASE + WM_CTLCOLOREDIT;
const OCM_CTLCOLORDLG: 		UINT = OCM__BASE + WM_CTLCOLORDLG;
const OCM_CTLCOLORLISTBOX: 	UINT = OCM__BASE + WM_CTLCOLORLISTBOX;
const OCM_CTLCOLORMSGBOX: 	UINT = OCM__BASE + WM_CTLCOLORMSGBOX;
const OCM_CTLCOLORSCROLLBAR:UINT = OCM__BASE + WM_CTLCOLORSCROLLBAR;
const OCM_CTLCOLORSTATIC: 	UINT = OCM__BASE + WM_CTLCOLORSTATIC;
//const OCM_CTLCOLOR: 		UINT = OCM__BASE + WM_CTLCOLOR;

const OCM_DRAWITEM: 		UINT = OCM__BASE + WM_DRAWITEM;
const OCM_MEASUREITEM: 		UINT = OCM__BASE + WM_MEASUREITEM;
const OCM_DELETEITEM: 		UINT = OCM__BASE + WM_DELETEITEM;
const OCM_VKEYTOITEM: 		UINT = OCM__BASE + WM_VKEYTOITEM;
const OCM_CHARTOITEM: 		UINT = OCM__BASE + WM_CHARTOITEM;
const OCM_COMPAREITEM: 		UINT = OCM__BASE + WM_COMPAREITEM;
const OCM_HSCROLL: 			UINT = OCM__BASE + WM_HSCROLL;
const OCM_VSCROLL: 			UINT = OCM__BASE + WM_VSCROLL;
const OCM_PARENTNOTIFY: 	UINT = OCM__BASE + WM_PARENTNOTIFY;

const OCM_NOTIFY: 			UINT = OCM__BASE + WM_NOTIFY;


pub trait CWindowImplRoot : WindowTrait {
	fn ForwardNotifications(&self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL) -> LRESULT {
		let mut lResult:LRESULT = 0;
		match uMsg{
			WM_COMMAND|WM_NOTIFY|WM_PARENTNOTIFY|WM_DRAWITEM|WM_MEASUREITEM|WM_COMPAREITEM|WM_DELETEITEM|WM_VKEYTOITEM|
			WM_CHARTOITEM|WM_HSCROLL|WM_VSCROLL|WM_CTLCOLORBTN|WM_CTLCOLORDLG|WM_CTLCOLOREDIT|WM_CTLCOLORLISTBOX|
			WM_CTLCOLORMSGBOX|WM_CTLCOLORSCROLLBAR|WM_CTLCOLORSTATIC=>{
				lResult = self.GetParent2().SendMessage(uMsg, wParam, lParam);
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
						_=>hWndChild = self.GetDlgItem2(HIWORD(wParam as DWORD) as c_int).GetHwnd(),
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
						hWndChild = self.GetDlgItem2(id as c_int).GetHwnd();
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

//only marker,prevent all the WindowTrait to be CWindowImpRoot
//pub trait WinImplMarker{}



				