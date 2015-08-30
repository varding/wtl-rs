
use winapi::*;
use user32;
use shell32;
//dialog or window impl this trait can proc messages

// pub trait Wnd {
// 	pub fn ProcessWindowMessage(&self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD)->BOOL;
// }

pub const NULL_HWND: HWND = 0 as HWND;
//any type that wants to operate on hwnd must impl this,include buttons,statics,cwindow,dialogs,frames etc
//it is used to be part of windowtrait,now use as a seprate part in order to impl easily by user
pub trait HwndTrait {
	fn GetHwnd(&self) -> HWND;
	fn Detach(&mut self)-> HWND; //only set m_hWnd = 0,this prevent most write ability from a hwndTrait
	fn Attach (&mut self,hWndNew:HWND);
}

//types that only have a m_hWnd field,all method operate base on the m_hWnd
//CWindow,CButton,CStatic...can impl this trait,Dialog,Frames should NOT impl this
pub trait WindowHandler {
	fn FromHwnd(h:HWND)->Self; 
}

#[macro_export]
macro_rules! impl_hwnd_trait {
	($tp:ident) => {
		impl HwndTrait for $tp {
			fn GetHwnd(&self) -> HWND{
				self.m_hWnd
			}

			fn Detach (&mut self) -> HWND {
				let hWnd = self.m_hWnd;
				self.m_hWnd = NULL_HWND;
				hWnd
			}

			fn Attach (&mut self,hWndNew:HWND) {
				assert!(self.m_hWnd == NULL_HWND);
				assert!(hWndNew != NULL_HWND);
				unsafe{
					assert!(user32::IsWindow(hWndNew) == TRUE);
				}
				self.m_hWnd = hWndNew;
			}
		}
	};
}

#[macro_export]
macro_rules! impl_hwnd_handler {
	($tp:ident) => {
		impl WindowHandler for $tp {
			fn FromHwnd(h:HWND)->$tp{
				$tp{
					m_hWnd:h
				}
			}
		}
	};
}

pub struct Dialog {
    m_hWnd: HWND
}
impl_hwnd_trait!(Dialog);

pub struct CWindow {
    m_hWnd: HWND,
}

impl CWindow {
	fn new(h:HWND) -> CWindow {
		CWindow{
			m_hWnd:h,
		}
	}
}

impl_hwnd_trait!(CWindow);
impl_hwnd_handler!(CWindow);

//place hold
fn GetModuleInstance() -> HINSTANCE {
	0 as HINSTANCE
}

//any type that impled HwndTrait will auto impl  WindowTrait
impl <T> WindowTrait for T where T:HwndTrait{}

//winuser.h
//#define RDW_ERASE               0x0004
//#define RDW_UPDATENOW           0x0100
//#define RDW_INVALIDATE          0x0001
// const RDW_ERASE     : UINT = 0x0004;
// const RDW_UPDATENOW : UINT = 0x0100;
// const RDW_INVALIDATE: UINT = 0x0001;

//this is different from CWinTrait,and it was introdced since wtl-rs
//#[cfg(target_arch = "x86_64")]
pub trait WindowTrait : HwndTrait {
	
	#[inline]
	fn assert_window(&self) {
		assert!(self.IsWindow());
	}

	//all get functions
	//output type depends on the inference of compiler
	fn GetParent<T:WindowHandler> (&self) -> T {
		self.assert_window();
		T::FromHwnd(unsafe{user32::GetParent(self.GetHwnd())})
	}

	fn SetParent<T:WindowHandler> (&self,hWndNewParent:HWND) -> T {
		self.assert_window();
		T::FromHwnd(unsafe{user32::SetParent(self.GetHwnd(), hWndNewParent)})
	}

	fn GetDlgItem<T:WindowHandler> (&self,nID:c_int) -> T {
		self.assert_window();
		T::FromHwnd(unsafe{user32::GetDlgItem(self.GetHwnd(), nID)})
	}

	//add rewritted functions of above that use cwindow as output,sometimes very convenient
	fn GetParent2 (&self) -> CWindow {
		self.assert_window();
		CWindow::new(unsafe{user32::GetParent(self.GetHwnd())})
	}

	fn SetParent2 (&self,hWndNewParent:HWND) -> CWindow {
		self.assert_window();
		CWindow::new(unsafe{user32::SetParent(self.GetHwnd(), hWndNewParent)})
	}

	fn GetDlgItem2 (&self,nID:c_int) -> CWindow {
		self.assert_window();
		CWindow::new(unsafe{user32::GetDlgItem(self.GetHwnd(), nID)})
	}

	//return cwindow 
	fn GetTopWindow (&self) -> CWindow {
		self.assert_window();
		unsafe{
			CWindow::new(user32::GetTopWindow(self.GetHwnd()))
		}
	}

	fn GetWindow (&self,nCmd:UINT) -> CWindow {
		self.assert_window();
		unsafe{
			CWindow::new(user32::GetWindow(self.GetHwnd(), nCmd))
		}
	}

	fn GetLastActivePopup (&self) -> CWindow {
		self.assert_window();
		unsafe{
			CWindow::new(user32::GetLastActivePopup(self.GetHwnd()))
		}
	}

	//https://msdn.microsoft.com/en-us/library/windows/desktop/ms632676(v=vs.85).aspx
	//we don't know what will get,so the return must be cwindow
	fn ChildWindowFromPoint (&self,point:POINT) -> CWindow {
		self.assert_window();
		unsafe{
			CWindow::new(user32::ChildWindowFromPoint(self.GetHwnd(), point))
		}
	}

	fn ChildWindowFromPointEx (&self,point:POINT,uFlags:UINT) -> CWindow {
		self.assert_window();
		unsafe{
			CWindow::new(user32::ChildWindowFromPointEx(self.GetHwnd(), point, uFlags))
		}
	}

	fn GetNextDlgGroupItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
		self.assert_window();
		unsafe{
			CWindow::new(user32::GetNextDlgGroupItem(self.GetHwnd(), hWndCtl, bPrevious))
		}
	}

	fn GetNextDlgTabItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
		self.assert_window();
		unsafe{
			CWindow::new(user32::GetNextDlgTabItem(self.GetHwnd(), hWndCtl, bPrevious))
		}
	}

	fn GetTopLevelParent (&self) -> CWindow {
		self.assert_window();

		let mut hWndParent:HWND  = self.GetHwnd();
		unsafe{
			let mut hWndTmp:HWND ;
			loop{
				hWndTmp = user32::GetParent(hWndParent);
				if hWndTmp == NULL_HWND {
					break;
				}
				hWndParent = hWndTmp;
			}
			CWindow::new(hWndParent)
		}
	}

	fn GetTopLevelWindow (&self) -> CWindow {
		self.assert_window();

		let mut hWndParent:HWND ;
		let mut hWndTmp:HWND  = self.GetHwnd();

		unsafe{
			loop{
				hWndParent = hWndTmp;
				hWndTmp = if (user32::GetWindowLongW(hWndParent, GWL_STYLE) as DWORD) & WS_CHILD != 0 {
					user32::GetParent(hWndParent)
				}else{
					user32::GetWindow(hWndParent, GW_OWNER)
				};

				if hWndTmp == NULL_HWND {
					break;
				}
			}
		}
		
		CWindow::new(hWndParent)
	}


	fn GetDescendantWindow (&self,nID:c_int) -> CWindow {
		self.assert_window();
		let mut hWndTmp :HWND;
		unsafe{
			let mut hWndChild = user32::GetDlgItem(self.GetHwnd(), nID);
			if hWndChild != NULL_HWND {
				if user32::GetTopWindow(hWndChild) != NULL_HWND	{
					let wnd = CWindow::new(hWndChild);
					hWndTmp = wnd.GetDescendantWindow(nID).GetHwnd();
					if hWndTmp != NULL_HWND{
						return CWindow::new(hWndTmp);
					}
				}
				return CWindow::new(hWndChild);
			}

			loop {
				hWndChild = user32::GetTopWindow(self.GetHwnd());
				if hWndChild == NULL_HWND {
					break;
				}
				//#define GetNextWindow(hWnd, wCmd) GetWindow(hWnd, wCmd)
				hWndChild = user32::GetWindow(hWndChild, GW_HWNDNEXT);
				let wnd = CWindow::new(hWndChild);
				hWndTmp = wnd.GetDescendantWindow(nID).GetHwnd();
				if hWndTmp != NULL_HWND {
					return CWindow::new(hWndTmp);
				}
			}
			
			CWindow::new(NULL_HWND)
		}
	}


	///////////////////////////////////


	//in cpp,stack alloc obj can use as follow:
	//CButton btn;
	//btn.Create(...)
	//but rust must init first,so this function must be static
	//let b:CButton = CButton::Create(...);
	//a wrapper is needed, e.g CButton::New(),new will call create
	fn Create(
		 lpstrWndClass:LPCWSTR ,
		 hWndParent:HWND ,
		 //_U_RECT rect = NULL,
		 rect:&RECT,
		 szWindowName:LPCWSTR,
		 dwStyle:DWORD,
		 dwExStyle:DWORD,
		 //_U_MENUorID MenuOrID = 0U,
		 hMenu:HMENU,
		 lpCreateParam:LPVOID) -> HWND
	{
		//ATLASSUME(self.GetHwnd() == NULL);
		//assert!(self.GetHwnd() == (0 as HWND));
		//if(rect.m_lpRect == NULL)
		//	rect.m_lpRect = &rcDefault;
		unsafe{
			user32::CreateWindowExW(dwExStyle, lpstrWndClass, szWindowName,
			dwStyle, rect.left, rect.top, rect.right - rect.left,
			rect.bottom - rect.top, hWndParent, hMenu,
			//_AtlBaseModule.GetModuleInstance(), lpCreateParam);
			GetModuleInstance(), lpCreateParam)
		}
	}

	// //create dialog controls 
	// fn Create2(lpstrWndClass:LPCTSTR,hWndParent:HWND,rect:&RECT) -> HWND {

	// }

	fn DestroyWindow (&mut self) -> bool {
		self.assert_window();
		if unsafe{user32::DestroyWindow(self.GetHwnd())} == FALSE{
			false
		}else{
			//self.set_hwnd(0 as HWND);
			self.Detach();
			true
		}
	}

	fn GetStyle (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowLongW(self.GetHwnd(), GWL_STYLE) as DWORD}
	}

	fn GetExStyle (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowLongW(self.GetHwnd(), GWL_EXSTYLE) as DWORD}
	}

	fn GetWindowLong (&self,nIndex:c_int) -> LONG {
		self.assert_window();
		unsafe{user32::GetWindowLongW(self.GetHwnd(), nIndex)}
	}

	fn GetWindowLongPtr (&self,nIndex:c_int) -> LONG_PTR {
		self.assert_window();
		unsafe{user32::GetWindowLongPtrW(self.GetHwnd(), nIndex)}
	}

	fn SetWindowLong (&self,nIndex:c_int,dwNewLong:LONG) -> LONG {
		self.assert_window();
		unsafe{user32::SetWindowLongW(self.GetHwnd(), nIndex, dwNewLong)}
	}

	fn SetWindowLongPtr (&self,nIndex:c_int,dwNewLong:LONG_PTR) -> LONG_PTR {
		self.assert_window();
		unsafe{user32::SetWindowLongPtrW(self.GetHwnd(), nIndex, dwNewLong)}
	}

	fn GetWindowWord (&self,nIndex:c_int) -> WORD {
		self.assert_window();
		unsafe{user32::GetWindowWord(self.GetHwnd(), nIndex)}
	}

	fn SetWindowWord (&self,nIndex:c_int,wNewWord:WORD) -> WORD {
		self.assert_window();
		unsafe{user32::SetWindowWord(self.GetHwnd(), nIndex, wNewWord)}
	}

	fn SendMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(),message,wParam,lParam)}
	}

	fn PostMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
		self.assert_window();
		unsafe{user32::PostMessageW(self.GetHwnd(),message,wParam,lParam) == TRUE}
	}

	fn SendNotifyMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
		self.assert_window();
		unsafe{user32::SendNotifyMessageW(self.GetHwnd(), message, wParam, lParam) == TRUE}
	}

	//static function
	// fn SendMessage (hWnd:HWND,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
	// 	//ATLASSERT(::IsWindow(hWnd));
	// 	assert!(user32::IsWindow(hWnd) == TRUE);
	// 	user32::SendMessage(hWnd, message, wParam, lParam)
	// }

	// fn SetWindowText (&self,lpszString:LPCTSTR) -> bool {
	// 	self.assert_window();
	// 	user32::SetWindowText(self.GetHwnd(), lpszString)  == TRUE
	// }

	// fn GetWindowText (&self,lpszStringBuf:LPTSTR,nMaxCount:c_int) -> c_int {
	// 	self.assert_window();
	// 	user32::GetWindowText(self.GetHwnd(), lpszStringBuf, nMaxCount)
	// }


	// c_int GetWindowText( CSimpleString& strText) const
	// {
	// 	c_int nLength;
	// 	LPTSTR pszText;

	// 	nLength = GetWindowTextLength();
	// 	pszText = strText.GetBuffer(nLength+1);
	// 	nLength = GetWindowText(pszText, nLength+1);
	// 	strText.ReleaseBuffer(nLength);

	// 	return nLength;
	// }

	fn GetWindowTextLength (&self) -> c_int {
		self.assert_window();
		unsafe{user32::GetWindowTextLengthW(self.GetHwnd())}
	}

	
	// MAKELPARAM is a macro in user32.h

	// #define MAKELPARAM(l, h)      (LPARAM)MAKELONG(l, h)

	// MAKELONG is a macro in common.h:

	// #define MAKELONG(low, high)   ((DWORD)(((WORD)(low)) | (((DWORD)((WORD)(high))) << 16))) 

	


	fn SetFont (&self,hFont:HFONT,bRedraw:BOOL)  {
		self.assert_window();
		//user32::SendMessage(self.GetHwnd(), WM_SETFONT, hFont as WPARAM, MAKELPARAM(bRedraw, 0));
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_SETFONT, hFont as WPARAM, (bRedraw & 0xFFFF) as LPARAM);}
	}

	fn GetFont (&self) -> HFONT {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_GETFONT, 0, 0)  as HFONT}
	}



	fn GetMenu (&self) -> HMENU {
		self.assert_window();
		unsafe{user32::GetMenu(self.GetHwnd()) as HMENU}
	}

	fn SetMenu (&self,hMenu:HMENU) -> bool {
		self.assert_window();
		unsafe{user32::SetMenu(self.GetHwnd(), hMenu) == TRUE}
	}

	fn DrawMenuBar (&self) -> bool {
		self.assert_window();
		unsafe{user32::DrawMenuBar(self.GetHwnd()) == TRUE}
	}

	fn GetSystemMenu (&self,bRevert:BOOL) -> HMENU {
		self.assert_window();
		unsafe{
			user32::GetSystemMenu(self.GetHwnd(), bRevert) as HMENU
		}
	}

	fn HiliteMenuItem (&self,hMenu:HMENU,uItemHilite:UINT,uHilite:UINT) -> bool {
		self.assert_window();
		unsafe{
			user32::HiliteMenuItem(self.GetHwnd(), hMenu, uItemHilite, uHilite) == TRUE
		}
	}

	fn IsIconic (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsIconic(self.GetHwnd()) == TRUE}
	}

	fn IsZoomed (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsZoomed(self.GetHwnd()) == TRUE}
	}

	fn MoveWindow(&self,x:c_int,y:c_int,nWidth:c_int,nHeight:c_int,bRepaint:BOOL) -> bool{
		self.assert_window();
		unsafe{user32::MoveWindow(self.GetHwnd(), x, y, nWidth, nHeight, bRepaint) == TRUE}
	}

	//fn MoveWindow2 (&self,lpRect:LPCRECT,bRepaint:BOOL) -> bool {
	fn MoveWindow2 (&self,lpRect:&RECT,bRepaint:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::MoveWindow(self.GetHwnd(), lpRect.left, lpRect.top, lpRect.right - lpRect.left, lpRect.bottom - lpRect.top, bRepaint) == TRUE}
	}

	fn SetWindowPos(&self,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,nFlags:UINT) -> bool {
		self.assert_window();
		unsafe{user32::SetWindowPos(self.GetHwnd(), hWndInsertAfter, x, y, cx, cy, nFlags) == TRUE}
	}

	fn SetWindowPos2 (&self,hWndInsertAfter:HWND,lpRect:&RECT,nFlags:UINT) -> bool {
		self.assert_window();
		unsafe{user32::SetWindowPos(self.GetHwnd(), hWndInsertAfter, lpRect.left, lpRect.top, lpRect.right - lpRect.left, lpRect.bottom - lpRect.top, nFlags) == TRUE}
	}

	fn ArrangeIconicWindows (&self) -> UINT {
		self.assert_window();
		unsafe{user32::ArrangeIconicWindows(self.GetHwnd())}
	}

	fn BringWindowToTop (&self) -> bool {
		self.assert_window();
		unsafe{user32::BringWindowToTop(self.GetHwnd()) == TRUE}
	}

	fn GetWindowRect (&self,lpRect:LPRECT) -> bool {
		self.assert_window();
		unsafe{user32::GetWindowRect(self.GetHwnd(), lpRect) == TRUE}
	}

	fn GetClientRect (&self,lpRect:&mut RECT) -> bool {
		self.assert_window();
		let p = lpRect as LPRECT;
		unsafe{user32::GetClientRect(self.GetHwnd(), p) == TRUE}
	}

	fn GetWindowPlacement(&self,lpwndpl:&mut WINDOWPLACEMENT) -> bool {
		self.assert_window();
		unsafe{user32::GetWindowPlacement(self.GetHwnd(), lpwndpl) == TRUE}
	}

	fn SetWindowPlacement(&self,lpwndpl:&WINDOWPLACEMENT) -> bool {
		self.assert_window();
		unsafe{user32::SetWindowPlacement(self.GetHwnd(), lpwndpl) == TRUE}
	}

	fn ClientToScreen (&self,lpPoint:LPPOINT) -> bool {
		self.assert_window();
		unsafe{user32::ClientToScreen(self.GetHwnd(), lpPoint) == TRUE}
	}

	fn ClientToScreen2 (&self,lpRect:&mut RECT) -> bool {
		self.assert_window();
		let p1 = lpRect as LPRECT;
		let p2 = p1 as LPPOINT;
		if unsafe{user32::ClientToScreen(self.GetHwnd(), p2)} == FALSE{
			return false;
		}
		unsafe{user32::ClientToScreen(self.GetHwnd(), p2.offset(1)) == TRUE}
	}

	fn ScreenToClient (&self,lpPoint:LPPOINT) -> bool {
		self.assert_window();
		unsafe{user32::ScreenToClient(self.GetHwnd(), lpPoint) == TRUE}
	}

	fn ScreenToClient2 (&self,lpRect:&mut RECT) -> bool {
		self.assert_window();
		let p1 = lpRect as LPRECT;
		let p2 = p1 as LPPOINT;

		if unsafe{user32::ScreenToClient(self.GetHwnd(), p2)} == FALSE{
			return false;
		}
		//user32::ScreenToClient(self.GetHwnd(), ((LPPOINT)lpRect)+1) == TRUE
		unsafe{user32::ScreenToClient(self.GetHwnd(), p2.offset(1)) == TRUE}
	}

	fn MapWindowPoints (&self,hWndTo:HWND,lpPoint:LPPOINT,nCount:UINT) -> c_int {
		self.assert_window();
		unsafe{user32::MapWindowPoints(self.GetHwnd(), hWndTo, lpPoint, nCount)}
	}

	fn MapWindowPoints2 (&self,hWndTo:HWND,lpRect:LPRECT) -> c_int {
		self.assert_window();
		//user32::MapWindowPoints(self.GetHwnd(), hWndTo, (LPPOINT)lpRect, 2)
		unsafe{user32::MapWindowPoints(self.GetHwnd(), hWndTo, lpRect as LPPOINT, 2)}
	}

	fn BeginPaint (&self,lpPaint:LPPAINTSTRUCT) -> HDC {
		self.assert_window();
		unsafe{user32::BeginPaint(self.GetHwnd(), lpPaint)}
	}

	fn EndPaint (&self,lpPaint:LPPAINTSTRUCT)  {
		self.assert_window();
		unsafe{user32::EndPaint(self.GetHwnd(), lpPaint);}
	}

	fn GetDC (&self) -> HDC {
		self.assert_window();
		unsafe{user32::GetDC(self.GetHwnd())}
	}

	fn GetWindowDC (&self) -> HDC {
		self.assert_window();
		unsafe{user32::GetWindowDC(self.GetHwnd())}
	}

	fn ReleaseDC (&self,hDC:HDC) -> c_int {
		self.assert_window();
		unsafe{user32::ReleaseDC(self.GetHwnd(), hDC)}
	}

	fn Print (&self,hDC:HDC,dwFlags:DWORD)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_PRINT, hDC as WPARAM, dwFlags as LPARAM);}
	}

	fn PrintClient (&self,hDC:HDC,dwFlags:DWORD)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_PRINTCLIENT, hDC as WPARAM, dwFlags as LPARAM);}
	}

	fn UpdateWindow (&self) -> bool {
		self.assert_window();
		unsafe{user32::UpdateWindow(self.GetHwnd()) == TRUE}
	}

	fn SetRedraw (&self,bRedraw:BOOL)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_SETREDRAW, bRedraw as WPARAM, 0 as LPARAM);}
	}

	fn GetUpdateRect (&self,lpRect:LPRECT,bErase:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::GetUpdateRect(self.GetHwnd(), lpRect, bErase) == TRUE}
	}

	fn GetUpdateRgn (&self,hRgn:HRGN,bErase:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::GetUpdateRgn(self.GetHwnd(), hRgn, bErase)}
	}

	fn Invalidate (&self,bErase:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::InvalidateRect(self.GetHwnd(), 0 as LPRECT, bErase) == TRUE}
	}

	fn Invalidate2 (&self,lpRect:LPCRECT,bErase:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::InvalidateRect(self.GetHwnd(), lpRect, bErase) == TRUE}
	}

	fn ValidateRect (&self,lpRect:LPCRECT) -> bool {
		self.assert_window();
		unsafe{user32::ValidateRect(self.GetHwnd(), lpRect) == TRUE}
	}

	fn InvalidateRgn (&self,hRgn:HRGN,bErase:BOOL)  {
		self.assert_window();
		unsafe{user32::InvalidateRgn(self.GetHwnd(), hRgn, bErase);}
	}

	fn ValidateRgn (&self,hRgn:HRGN) -> bool {
		self.assert_window();
		unsafe{user32::ValidateRgn(self.GetHwnd(), hRgn) == TRUE}
	}

	fn ShowWindow (&self,nCmdShow:c_int) -> bool {
		self.assert_window();
		unsafe{user32::ShowWindow(self.GetHwnd(), nCmdShow) == TRUE}
	}

	fn IsWindowVisible (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsWindowVisible(self.GetHwnd()) == TRUE}
	}

	fn ShowOwnedPopups (&self,bShow:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::ShowOwnedPopups(self.GetHwnd(), bShow) == TRUE}
	}

	fn GetDCEx (&self,hRgnClip:HRGN,flags:DWORD) -> HDC {
		self.assert_window();
		unsafe{user32::GetDCEx(self.GetHwnd(), hRgnClip, flags)}
	}

	fn LockWindowUpdate (&self,bLock:bool) -> bool {
		self.assert_window();
		if bLock{
			unsafe{user32::LockWindowUpdate(self.GetHwnd()) == TRUE}
		}else{
			unsafe{user32::LockWindowUpdate(NULL_HWND) == TRUE}
		}
	}

	fn RedrawWindow2(&self) -> bool{
		self.RedrawWindow(0 as LPCRECT,0 as HRGN,RDW_INVALIDATE | RDW_UPDATENOW | RDW_ERASE)
	}

	fn RedrawWindow(&self,lpRectUpdate:LPCRECT,hRgnUpdate:HRGN,flags:UINT)->bool{
		self.assert_window();
		unsafe{user32::RedrawWindow(self.GetHwnd(), lpRectUpdate, hRgnUpdate, flags) == TRUE}
	}

	fn SetTimer(&self,nIDEvent:UINT_PTR,nElapse:UINT) -> UINT_PTR {
		self.assert_window();
		unsafe{user32::SetTimer(self.GetHwnd(), nIDEvent, nElapse, None)}
	}

	fn SetTimer2(&self,nIDEvent:UINT_PTR,nElapse:UINT,lpfnTimer:TimerProc)->UINT_PTR{
		self.assert_window();
		unsafe{user32::SetTimer(self.GetHwnd(), nIDEvent, nElapse, lpfnTimer)}
	}

	fn KillTimer (&self,nIDEvent:UINT_PTR) -> bool {
		self.assert_window();
		unsafe{user32::KillTimer(self.GetHwnd(), nIDEvent) == TRUE}
	}

	fn IsWindowEnabled (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsWindowEnabled(self.GetHwnd()) == TRUE}
	}

	fn EnableWindow (&self,bEnable:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::EnableWindow(self.GetHwnd(), bEnable) == TRUE}
	}

	fn SetActiveWindow (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetActiveWindow(self.GetHwnd())}
	}

	fn SetCapture (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetCapture(self.GetHwnd())}
	}

	fn SetFocus (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetFocus(self.GetHwnd())}
	}



	fn CheckDlgButton (&self,nIDButton:c_int,nCheck:UINT) -> bool {
		self.assert_window();
		unsafe{user32::CheckDlgButton(self.GetHwnd(), nIDButton, nCheck) == TRUE}
	}

	fn CheckRadioButton (&self,nIDFirstButton:c_int,nIDLastButton:c_int,nIDCheckButton:c_int) -> bool {
		self.assert_window();
		unsafe{user32::CheckRadioButton(self.GetHwnd(), nIDFirstButton, nIDLastButton, nIDCheckButton) == TRUE}
	}

	// fn DlgDirList (&self,lpPathSpec:LPTSTR,nIDListBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
	// 	self.assert_window();
	// 	user32::DlgDirList(self.GetHwnd(), lpPathSpec, nIDListBox, nIDStaticPath, nFileType)
	// }

	// fn DlgDirListComboBox (&self,lpPathSpec:LPTSTR,nIDComboBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
	// 	self.assert_window();
	// 	user32::DlgDirListComboBox(self.GetHwnd(), lpPathSpec, nIDComboBox, nIDStaticPath, nFileType)
	// }

	// fn DlgDirSelect (lpString:LPTSTR,nCount:c_int,nIDListBox:c_int) -> bool {
	// 	self.assert_window();
	// 	user32::DlgDirSelectEx(self.GetHwnd(), lpString, nCount, nIDListBox) == TRUE
	// }

	// fn DlgDirSelectComboBox (lpString:LPTSTR,nCount:c_int,nIDComboBox:c_int) -> bool {
	// 	self.assert_window();
	// 	user32::DlgDirSelectComboBoxEx(self.GetHwnd(), lpString, nCount, nIDComboBox) == TRUE
	// }

	fn GetDlgItemInt(&self,nID:c_int) -> UINT {
		self.assert_window();
		unsafe{user32::GetDlgItemInt(self.GetHwnd(), nID, 0 as *mut BOOL, TRUE)}
	}

	fn GetDlgItemInt2(&self,nID:c_int,lpTrans:&mut BOOL,bSigned:BOOL) -> UINT {
		self.assert_window();
		unsafe{user32::GetDlgItemInt(self.GetHwnd(), nID, lpTrans as *mut BOOL, bSigned)}
	}

	// fn GetDlgItemText (&self,nID:c_int,lpStr:LPTSTR,nMaxCount:c_int) -> UINT {
	// 	self.assert_window();
	// 	user32::GetDlgItemText(self.GetHwnd(), nID, lpStr, nMaxCount)
	// }

	// UINT GetDlgItemText(
	// 	 c_int nID,
	// 	 CSimpleString& strText) const
	// {
	// 	self.assert_window();

	// 	HWND hItem = GetDlgItem(nID);
	// 	if (hItem != NULL)
	// 	{
	// 		c_int nLength;
	// 		LPTSTR pszText;

	// 		nLength = ::GetWindowTextLength(hItem);
	// 		pszText = strText.GetBuffer(nLength+1);
	// 		nLength = ::GetWindowText(hItem, pszText, nLength+1);
	// 		strText.ReleaseBuffer(nLength);

	// 		return nLength;
	// 	}
	// 	else
	// 	{
	// 		strText.Empty();

	// 		return 0;
	// 	}
	// }

	//OLE
	// BOOL GetDlgItemText(
	// 	 c_int nID,
	// 	 _Deref_post_opt_z_ BSTR& bstrText) 
	// {
	// 	self.assert_window();

	// 	HWND hWndCtl = GetDlgItem(nID);
	// 	if(hWndCtl == NULL)
	// 		return FALSE;

	// 	return CWindow(hWndCtl).GetWindowText(bstrText);
	// }


	fn IsDlgButtonChecked (&self,nIDButton:c_int) -> UINT {
		self.assert_window();
		unsafe{user32::IsDlgButtonChecked(self.GetHwnd(), nIDButton)}
	}

	fn SendDlgItemMessage (&self,nID:c_int,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
		self.assert_window();
		unsafe{user32::SendDlgItemMessageW(self.GetHwnd(), nID, message, wParam, lParam)}
	}

	fn SetDlgItemInt (&self,nID:c_int,nValue:UINT,bSigned:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::SetDlgItemInt(self.GetHwnd(), nID, nValue, bSigned) == TRUE}
	}

	// fn SetDlgItemText (&self,nID:c_int,lpszString:LPCTSTR) -> bool {
	// 	self.assert_window();
	// 	user32::SetDlgItemText(self.GetHwnd(), nID, lpszString) == TRUE
	// }

// #ifndef _ATL_NO_HOSTING
// ATLPREFAST_SUPPRESS(6387)
// 	HRESULT GetDlgControl(
// 		 c_int nID,
// 		 REFIID iid,
// 		 void** ppCtrl) 
// 	{
// 		self.assert_window();
// 		ATLASSERT(ppCtrl != NULL);
// 		if (ppCtrl == NULL)
// 			return E_POINTER;
// 		*ppCtrl = NULL;
// 		HRESULT hr = HRESULT_FROM_WIN32(ERROR_CONTROL_ID_NOT_FOUND);
// 		HWND hWndCtrl = GetDlgItem(nID);
// 		if (hWndCtrl != NULL)
// 		{
// 			*ppCtrl = NULL;
// 			CComPtr<IUnknown> spUnk;
// 			hr = AtlAxGetControl(hWndCtrl, &spUnk);
// 			if (SUCCEEDED(hr))
// 				hr = spUnk->QueryInterface(iid, ppCtrl);
// 		}
// 		return hr;
// 	}
// ATLPREFAST_UNSUPPRESS()

// ATLPREFAST_SUPPRESS(6387)
// 	HRESULT GetDlgHost(
// 		 c_int nID,
// 		 REFIID iid,
// 		 void** ppHost) 
// 	{
// 		self.assert_window();
// 		ATLASSERT(ppHost != NULL);
// 		if (ppHost == NULL)
// 			return E_POINTER;
// 		*ppHost = NULL;
// 		HRESULT hr = HRESULT_FROM_WIN32(ERROR_CONTROL_ID_NOT_FOUND);
// 		HWND hWndCtrl = GetDlgItem(nID);
// 		if (hWndCtrl != NULL)
// 		{
// 			CComPtr<IUnknown> spUnk;
// 			hr = AtlAxGetHost(hWndCtrl, &spUnk);
// 			if (SUCCEEDED(hr))
// 				hr = spUnk->QueryInterface(iid, ppHost);
// 		}
// 		return hr;
// 	}
// ATLPREFAST_UNSUPPRESS()
	
// #endif 



	fn GetScrollPos (&self,nBar:c_int) -> c_int {
		self.assert_window();
		unsafe{user32::GetScrollPos(self.GetHwnd(), nBar)}
	}

	fn GetScrollRange (&self,nBar:c_int,lpMinPos:LPINT,lpMaxPos:LPINT) -> bool {
		self.assert_window();
		unsafe{user32::GetScrollRange(self.GetHwnd(), nBar, lpMinPos, lpMaxPos) == TRUE}
	}

	fn ScrollWindow (&self,xAmount:c_int,yAmount:c_int,lpRect:LPCRECT,lpClipRect:LPCRECT) -> bool {
		self.assert_window();
		unsafe{user32::ScrollWindow(self.GetHwnd(), xAmount, yAmount, lpRect, lpClipRect) == TRUE}
	}

	fn ScrollWindowEx(
		 &self,
		 dx:c_int,
		 dy:c_int,
		 lpRectScroll:LPCRECT ,
		 lpRectClip:LPCRECT ,
		 hRgnUpdate:HRGN ,
		 lpRectUpdate:LPRECT ,
		 uFlags:UINT ) -> c_int 
	{
		self.assert_window();
		unsafe{user32::ScrollWindowEx(self.GetHwnd(), dx, dy, lpRectScroll, lpRectClip, hRgnUpdate, lpRectUpdate, uFlags)}
	}

	fn ScrollWindowExDefault(&self,dx:c_int,dy:c_int,uFlags:UINT)->c_int{
		self.ScrollWindowEx(dx,dy,0 as LPCRECT,0 as LPCRECT,0 as HRGN,0 as LPRECT,uFlags)
	}

	fn SetScrollPos (&self,nBar:c_int,nPos:c_int,bRedraw:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::SetScrollPos(self.GetHwnd(), nBar, nPos, bRedraw)}
	}

	fn SetScrollRange (&self,nBar:c_int,nMinPos:c_int,nMaxPos:c_int,bRedraw:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::SetScrollRange(self.GetHwnd(), nBar, nMinPos, nMaxPos, bRedraw) == TRUE}
	}

	fn ShowScrollBar (&self,nBar:c_int,bShow:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::ShowScrollBar(self.GetHwnd(), nBar, bShow) == TRUE}
	}

	fn EnableScrollBar (&self,uSBFlags:UINT,uArrowFlags:UINT) -> bool {
		self.assert_window();
		unsafe{user32::EnableScrollBar(self.GetHwnd(), uSBFlags, uArrowFlags) == TRUE}
	}

	fn IsChild (&self,hWnd:HWND) -> bool {
		self.assert_window();
		unsafe{user32::IsChild(self.GetHwnd(), hWnd) == TRUE}
	}

	fn GetDlgCtrlID (&self) -> c_int {
		self.assert_window();
		unsafe{user32::GetDlgCtrlID(self.GetHwnd())}
	}

	fn SetDlgCtrlID (&self,nID:c_int) -> c_int {
		self.assert_window();
		unsafe{user32::SetWindowLongW(self.GetHwnd(), GWL_ID, nID)}
	}

	fn FlashWindow (&self,bInvert:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::FlashWindow(self.GetHwnd(), bInvert) == TRUE}
	}

	// fn MessageBox(&self,lpszText:LPCTSTR ,lpszCaption:LPCTSTR,nType:UINT) -> c_int {
	// 	self.assert_window();
	// 	user32::MessageBox(self.GetHwnd(), lpszText, lpszCaption, nType)
	// }

	fn ChangeClipboardChain (&self,hWndNewNext:HWND) -> bool {
		self.assert_window();
		unsafe{user32::ChangeClipboardChain(self.GetHwnd(), hWndNewNext) == TRUE}
	}

	fn SetClipboardViewer (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetClipboardViewer(self.GetHwnd())}
	}

	fn OpenClipboard (&self) -> bool {
		self.assert_window();
		unsafe{user32::OpenClipboard(self.GetHwnd()) == TRUE}
	}



	fn CreateCaret (&self,hBitmap:HBITMAP) -> bool {
		self.assert_window();
		unsafe{user32::CreateCaret(self.GetHwnd(), hBitmap, 0, 0) == TRUE}
	}

	fn CreateSolidCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
		self.assert_window();
		unsafe{user32::CreateCaret(self.GetHwnd(), 0 as HBITMAP, nWidth, nHeight) == TRUE}
	}

	fn CreateGrayCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
		self.assert_window();
		unsafe{user32::CreateCaret(self.GetHwnd(), 1 as HBITMAP, nWidth, nHeight) == TRUE}
	}

	fn HideCaret (&self) -> bool {
		self.assert_window();
		unsafe{user32::HideCaret(self.GetHwnd()) == TRUE}
	}

	fn ShowCaret (&self) -> bool {
		self.assert_window();
		unsafe{user32::ShowCaret(self.GetHwnd()) == TRUE}
	}

	fn DragAcceptFiles (&self,bAccept:BOOL)  {
		self.assert_window(); 
		unsafe{
			shell32::DragAcceptFiles(self.GetHwnd(), bAccept);	
		}
	}

	fn SetIcon (&self,hIcon:HICON,bBigIcon:BOOL) -> HICON {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_SETICON, bBigIcon as WPARAM, hIcon as LPARAM) as HICON}
	}

	fn GetIcon (&self,bBigIcon:BOOL) -> HICON {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_GETICON, bBigIcon as WPARAM, 0 as LPARAM) as HICON}
	}

	// fn WinHelp (&self,lpszHelp:LPCTSTR,nCmd:UINT,dwData:DWORD) -> bool {
	// 	self.assert_window();
	// 	user32::WinHelp(self.GetHwnd(), lpszHelp, nCmd, dwData) == TRUE
	// }

	fn SetWindowContextHelpId (&self,dwContextHelpId:DWORD) -> bool {
		self.assert_window();
		unsafe{user32::SetWindowContextHelpId(self.GetHwnd(), dwContextHelpId) == TRUE}
	}

	fn GetWindowContextHelpId (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowContextHelpId(self.GetHwnd())}
	}

	fn SetHotKey (&self,wVirtualKeyCode:WORD,wModifiers:WORD) -> c_int {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_SETHOTKEY, MAKEWORD(wVirtualKeyCode as u8, wModifiers as u8) as WPARAM, 0 ) as c_int}
	}

	fn GetHotKey (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_GETHOTKEY, 0, 0) as DWORD}
	}

	fn GetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO) -> bool {
		self.assert_window();
		unsafe{user32::GetScrollInfo(self.GetHwnd(), nBar, lpScrollInfo) == TRUE}
	}
	fn SetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO,bRedraw:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::SetScrollInfo(self.GetHwnd(), nBar, lpScrollInfo, bRedraw)}
	}
	fn IsDialogMessage (&self,lpMsg:LPMSG) -> bool {
		self.assert_window();
		unsafe{user32::IsDialogMessageW(self.GetHwnd(), lpMsg) == TRUE}
	}

	fn NextDlgCtrl (&self)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_NEXTDLGCTL, 0, 0);}
	}
	fn PrevDlgCtrl (&self)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_NEXTDLGCTL, 1, 0);}
	}
	fn GotoDlgCtrl (&self,hWndCtrl:HWND)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.GetHwnd(), WM_NEXTDLGCTL, hWndCtrl as WPARAM, 1);}
	}

	fn ResizeClient (&self,nWidth:c_int,nHeight:c_int,bRedraw:BOOL) -> bool {
		self.assert_window();

		let mut rcWnd = RECT{left:0,right:0,top:0,bottom:0};
		//if(!GetClientRect(&rcWnd))
		//	return FALSE;
		if self.GetClientRect(&mut rcWnd) == false{
			return false;
		}

		if nWidth != -1{
			rcWnd.right = nWidth;
		}

		if nHeight != -1{
			rcWnd.bottom = nHeight;
		}

		//let b1 = !(self.GetStyle() & WS_CHILD) && (self.GetMenu() != 0 as HMENU);
		let b1 = ((self.GetStyle() & WS_CHILD) == 0) && (self.GetMenu() != (0 as HMENU));
		if unsafe{user32::AdjustWindowRectEx(&mut rcWnd, self.GetStyle(), b1 as BOOL, self.GetExStyle())} == TRUE {
			return false;
		}

		let mut uFlags:UINT = SWP_NOZORDER | SWP_NOMOVE | SWP_NOACTIVATE;
		if bRedraw == FALSE{
			uFlags |= SWP_NOREDRAW;
		}

		self.SetWindowPos(NULL_HWND, 0, 0, rcWnd.right - rcWnd.left, rcWnd.bottom - rcWnd.top, uFlags)
	}

	fn GetWindowRgn (&self,hRgn:HRGN) -> c_int {
		self.assert_window();
		unsafe{user32::GetWindowRgn(self.GetHwnd(), hRgn)}
	}
	fn SetWindowRgn (&self,hRgn:HRGN,bRedraw:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::SetWindowRgn(self.GetHwnd(), hRgn, bRedraw)}
	}

	fn DeferWindowPos(
		 &self,
		 hWinPosInfo:HDWP,
		 hWndInsertAfter:HWND,
		 x:c_int,
		 y:c_int,
		 cx:c_int,
		 cy:c_int,
		 uFlags:UINT) -> HDWP
	{
		self.assert_window();
		unsafe{user32::DeferWindowPos(hWinPosInfo, self.GetHwnd(), hWndInsertAfter, x, y, cx, cy, uFlags)}
	}

	fn GetWindowThreadID (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowThreadProcessId(self.GetHwnd(), 0 as LPDWORD)}
	}

	fn GetWindowProcessID (&self) -> DWORD {
		self.assert_window();
		let mut dwProcessID:DWORD = 0;
		unsafe{user32::GetWindowThreadProcessId(self.GetHwnd(), &mut dwProcessID);}
		dwProcessID
	}

	fn IsWindow (&self) -> bool {
		unsafe{user32::IsWindow(self.GetHwnd()) == TRUE}
	}
	fn IsWindowUnicode (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsWindowUnicode(self.GetHwnd()) == TRUE}
	}
	// fn IsParentDialog (&self) -> bool {
	// 	self.assert_window();
	// 	TCHAR szBuf[8]; 
	// 	if (GetClassName(GetParent(), szBuf, sizeof(szBuf)/sizeof(szBuf[0])) == 0)
	// 		return FALSE;
	// 	return lstrcmp(szBuf, _T("#32770")) == 0;
	// }
	fn ShowWindowAsync (&self,nCmdShow:c_int) -> bool {
		self.assert_window();
		unsafe{user32::ShowWindowAsync(self.GetHwnd(), nCmdShow) == TRUE}
	}


	// fn SendMessageToDescendants (&self,message:UINT,wParam:WPARAM,lParam:LPARAM,bDeep:BOOL)  {
	// 	for(HWND hWndChild = ::GetTopWindow(self.GetHwnd()); hWndChild != NULL;
	// 		hWndChild = ::GetNextWindow(hWndChild, GW_HWNDNEXT))
	// 	{
	// 		::SendMessage(hWndChild, message, wParam, lParam);

	// 		if(bDeep && ::GetTopWindow(hWndChild) != NULL)
	// 		{
				
	// 			CWindow wnd(hWndChild);
	// 			wnd.SendMessageToDescendants(message, wParam, lParam, bDeep);
	// 		}
	// 	}
	// }

// 	fn CenterWindow (&self,hWndCenter:HWND) -> bool {
// 		self.assert_window();
		
// 		DWORD dwStyle = GetStyle();
// 		if(hWndCenter == NULL)
// 		{
// 			if(dwStyle & WS_CHILD)
// 				hWndCenter = ::GetParent(self.GetHwnd());
// 			else
// 				hWndCenter = ::GetWindow(self.GetHwnd(), GW_OWNER);
// 		}

		
// 		RECT rcDlg;
// 		::GetWindowRect(self.GetHwnd(), &rcDlg);
// 		RECT rcArea;
// 		RECT rcCenter;
// 		HWND hWndParent;
// 		if(!(dwStyle & WS_CHILD))
// 		{
			
// 			if(hWndCenter != NULL)
// 			{
// 				DWORD dwStyleCenter = ::GetWindowLong(hWndCenter, GWL_STYLE);
// 				if(!(dwStyleCenter & WS_VISIBLE) || (dwStyleCenter & WS_MINIMIZE))
// 					hWndCenter = NULL;
// 			}

			
// #if WINVER < 0x0500
// 			::SystemParametersInfo(SPI_GETWORKAREA, NULL, &rcArea, NULL);
// #else
// 			HMONITOR hMonitor = NULL;
// 			if(hWndCenter != NULL)
// 			{
// 				hMonitor = ::MonitorFromWindow(hWndCenter, MONITOR_DEFAULTTONEAREST);
// 			}
// 			else
// 			{
// 				hMonitor = ::MonitorFromWindow(self.GetHwnd(), MONITOR_DEFAULTTONEAREST);
// 			}
// 			ATLENSURE_RETURN_VAL(hMonitor != NULL, FALSE);

// 			MONITORINFO minfo;
// 			minfo.cbSize = sizeof(MONITORINFO);
// 			BOOL bResult = ::GetMonitorInfo(hMonitor, &minfo);
// 			ATLENSURE_RETURN_VAL(bResult, FALSE);

// 			rcArea = minfo.rcWork;
// #endif
// 			if(hWndCenter == NULL)
// 				rcCenter = rcArea;
// 			else
// 				::GetWindowRect(hWndCenter, &rcCenter);
// 		}
// 		else
// 		{
			
// 			hWndParent = ::GetParent(self.GetHwnd());
// 			ATLASSERT(::IsWindow(hWndParent));

// 			::GetClientRect(hWndParent, &rcArea);
// 			ATLASSERT(::IsWindow(hWndCenter));
// 			::GetClientRect(hWndCenter, &rcCenter);
// 			::MapWindowPoints(hWndCenter, hWndParent, (POINT*)&rcCenter, 2);
// 		}

// 		c_int DlgWidth = rcDlg.right - rcDlg.left;
// 		c_int DlgHeight = rcDlg.bottom - rcDlg.top;

		
// 		c_int xLeft = (rcCenter.left + rcCenter.right) / 2 - DlgWidth / 2;
// 		c_int yTop = (rcCenter.top + rcCenter.bottom) / 2 - DlgHeight / 2;

		
// 		if(xLeft + DlgWidth > rcArea.right)
// 			xLeft = rcArea.right - DlgWidth;
// 		if(xLeft < rcArea.left)
// 			xLeft = rcArea.left;

// 		if(yTop + DlgHeight > rcArea.bottom)
// 			yTop = rcArea.bottom - DlgHeight;
// 		if(yTop < rcArea.top)
// 			yTop = rcArea.top;

		
// 		return ::SetWindowPos(self.GetHwnd(), NULL, xLeft, yTop, -1, -1,
// 			SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
// 	}

	fn ModifyStyle (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
		self.assert_window();

		let dwStyle:DWORD = unsafe{user32::GetWindowLongW(self.GetHwnd(), GWL_STYLE) as DWORD};
		let dwNewStyle  = (dwStyle & !dwRemove) | dwAdd;
		if dwStyle == dwNewStyle{
			return false;
		}

		unsafe{user32::SetWindowLongW(self.GetHwnd(), GWL_STYLE, dwNewStyle as LONG);}
		if nFlags != 0 {
			unsafe{user32::SetWindowPos(self.GetHwnd(), NULL_HWND, 0, 0, 0, 0,
				SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE | nFlags)};
		}

		true
	}

	fn ModifyStyleEx (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
		self.assert_window();

		let dwStyle:DWORD = unsafe{user32::GetWindowLongW(self.GetHwnd(), GWL_EXSTYLE) as DWORD};
		let dwNewStyle:DWORD = (dwStyle & !dwRemove) | dwAdd;
		if dwStyle == dwNewStyle {
			return false;
		}

		unsafe{user32::SetWindowLongW(self.GetHwnd(), GWL_EXSTYLE, dwNewStyle as LONG);}
		if nFlags != 0 {
			unsafe{user32::SetWindowPos(self.GetHwnd(), NULL_HWND, 0, 0, 0, 0,
				SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE | nFlags)};
		}
		true
	}

	// BOOL GetWindowText( _Deref_post_opt_z_ BSTR* pbstrText) 
	// {
	// 	return GetWindowText(*pbstrText);
	// }
	// BOOL GetWindowText( BSTR& bstrText) 
	// {
	// 	USES_CONVERSION_EX;
	// 	self.assert_window();
	// 	::SysFreeString(bstrText);
	// 	bstrText = NULL;

	// 	c_int nLen = ::GetWindowTextLength(self.GetHwnd());

	// 	CTempBuffer<TCHAR> lpszText;
	// 	if(nLen>0)
	// 	{
	// 		ATLTRY(lpszText.Allocate(nLen+1));
	// 		if (lpszText == NULL)
	// 		{
	// 			return FALSE;
	// 		}

	// 		if(!::GetWindowText(self.GetHwnd(), lpszText, nLen+1))
	// 		{
	// 			return FALSE;
	// 		}
	// 	}

	// 	bstrText = ::SysAllocString(T2OLE_EX_DEF(lpszText));

	// 	return nLen==0 ? FALSE : ((bstrText != NULL) ? TRUE : FALSE);
	// }
}