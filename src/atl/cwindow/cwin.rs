#![allow(dead_code,unused_assignments,non_snake_case)]
use std;
use winapi::*;
use user32;
use shell32;
use misc::ToCU16Str;

pub const NULL_HWND  : HWND  = 0 as HWND;
pub const NULL_LPARAM:LPARAM = 0 as LPARAM;

pub trait WindowHandler {
    fn FromHwnd(h: HWND) -> Self;
}

fn GetModuleInstance() -> HINSTANCE {
    0 as HINSTANCE
}

pub struct CWindow(HWND);


impl CWindow {
    pub fn new(h: HWND) -> CWindow {
        CWindow(h)
    }
}
///////////////////////////////////////


//this is different from CWinTrait,and it was introdced since wtl-rs
impl CWindow {
    pub fn GetHwnd(&self) -> HWND {
        self.0
    }

    pub fn Detach(&mut self) -> HWND { //only set m_hWnd = 0,this prevent most write ability from a hwndTrait
        let hWnd = self.0;
        self.0 = NULL_HWND;
        hWnd
    }

    pub fn Attach(&mut self, hWndNew: HWND) {
        debug_assert!(self.0 == NULL_HWND);
        debug_assert!(hWndNew != NULL_HWND);
        unsafe {
            debug_assert!(user32::IsWindow(hWndNew) == TRUE);
        }
        self.0 = hWndNew;
    }

    #[inline]
    pub fn assert_window(&self) {
        debug_assert!(self.IsWindow());
    }

	//all get functions
	//output type depends on the inference of compiler
	// pub fn GetParent<T:WindowHandler> (&self) -> T {
	// 	self.assert_window();
	// 	T::FromHwnd(unsafe{user32::GetParent(self.0)})
	// }

	// pub fn SetParent<T:WindowHandler> (&self,hWndNewParent:HWND) -> T {
	// 	self.assert_window();
	// 	T::FromHwnd(unsafe{user32::SetParent(self.0, hWndNewParent)})
	// }

	// pub fn GetDlgItem<T:WindowHandler> (&self,nID:c_int) -> T {
	// 	self.assert_window();
	// 	T::FromHwnd(unsafe{user32::GetDlgItem(self.0, nID)})
	// }
    pub fn GetParent(&self) -> HWND {
        self.assert_window();
        unsafe{user32::GetParent(self.0)}
    }

    pub fn SetParent(&self,hWndNewParent:HWND) -> HWND {
        self.assert_window();
        unsafe{user32::SetParent(self.0, hWndNewParent)}
    }

    pub fn GetDlgItem(&self,nID:c_int) -> HWND {
        self.assert_window();
        unsafe{user32::GetDlgItem(self.0, nID)}
    }

	//add rewritted functions of above that use cwindow as output,sometimes very convenient
    pub fn GetParent2(&self) -> CWindow {
        self.assert_window();
        CWindow::new(unsafe {
                user32::GetParent(self.0)
            })
    }

    pub fn SetParent2(&self, hWndNewParent: HWND) -> CWindow {
        self.assert_window();
        CWindow::new(unsafe {
                user32::SetParent(self.0, hWndNewParent)
            })
    }

    pub fn GetDlgItem2(&self, nID: c_int) -> CWindow {
        self.assert_window();
        CWindow::new(unsafe {
                user32::GetDlgItem(self.0, nID)
            })
    }

	//return cwindow
    pub fn GetTopWindow(&self) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(user32::GetTopWindow(self.0))
        }
    }

    pub fn GetWindow(&self, nCmd: UINT) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(user32::GetWindow(self.0, nCmd))
        }
    }

    pub fn GetLastActivePopup(&self) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(user32::GetLastActivePopup(self.0))
        }
    }

	//https://msdn.microsoft.com/en-us/library/windows/desktop/ms632676(v=vs.85).aspx
	//I don't know what will get,so the return must be cwindow
    pub fn ChildWindowFromPoint(&self, point: POINT) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(user32::ChildWindowFromPoint(self.0, point))
        }
    }

    pub fn ChildWindowFromPointEx(&self, point: POINT, uFlags: UINT) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(user32::ChildWindowFromPointEx(self.0, point, uFlags))
        }
    }

    pub fn GetNextDlgGroupItem(&self, hWndCtl: HWND, bPrevious: BOOL) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(user32::GetNextDlgGroupItem(self.0, hWndCtl, bPrevious))
        }
    }

    pub fn GetNextDlgTabItem(&self, hWndCtl: HWND, bPrevious: BOOL) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(user32::GetNextDlgTabItem(self.0, hWndCtl, bPrevious))
        }
    }

    pub fn GetTopLevelParent(&self) -> CWindow {
        self.assert_window();

        let mut hWndParent: HWND = self.0;
        unsafe {
            let mut hWndTmp: HWND;
            loop {
                hWndTmp = user32::GetParent(hWndParent);
                if hWndTmp == NULL_HWND {
                    break;
                }
                hWndParent = hWndTmp;
            }
            CWindow::new(hWndParent)
        }
    }

    pub fn GetTopLevelWindow(&self) -> CWindow {
        self.assert_window();

        let mut hWndParent: HWND;
        let mut hWndTmp: HWND = self.0;

        unsafe {
            loop {
                hWndParent = hWndTmp;
                hWndTmp = if (user32::GetWindowLongW(hWndParent, GWL_STYLE) as DWORD) &
                             WS_CHILD != 0 {
                    user32::GetParent(hWndParent)
                } else {
                    user32::GetWindow(hWndParent, GW_OWNER)
                };

                if hWndTmp == NULL_HWND {
                    break;
                }
            }
        }

        CWindow::new(hWndParent)
    }


    pub fn GetDescendantWindow(&self, nID: c_int) -> CWindow {
        self.assert_window();
        let mut hWndTmp: HWND;
        unsafe {
            let mut hWndChild = user32::GetDlgItem(self.0, nID);
            if hWndChild != NULL_HWND {
                if user32::GetTopWindow(hWndChild) != NULL_HWND {
                    let wnd = CWindow::new(hWndChild);
                    hWndTmp = wnd.GetDescendantWindow(nID).GetHwnd();
                    if hWndTmp != NULL_HWND {
                        return CWindow::new(hWndTmp);
                    }
                }
                return CWindow::new(hWndChild);
            }

            loop {
                hWndChild = user32::GetTopWindow(self.0);
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
    pub fn Create(lpstrWndClass: &str,
                  hWndParent: HWND,
                  // _U_RECT rect = NULL,
                  rect: &RECT,
                  szWindowName: &str,
                  dwStyle: DWORD,
                  dwExStyle: DWORD,
                  // _U_MENUorID MenuOrID = 0U,
                  hMenu: HMENU,
                  lpCreateParam: LPVOID)
                  -> HWND {
		//ATLASSUME(self.0 == NULL);
		//assert!(self.0 == (0 as HWND));
		//if(rect.m_lpRect == NULL)
		//	rect.m_lpRect = &rcDefault;
        let c = lpstrWndClass.to_c_u16();
        let n = szWindowName.to_c_u16();
        unsafe {
            user32::CreateWindowExW(dwExStyle,
                                    c.as_ptr(),
                                    n.as_ptr(),
                                    dwStyle,
                                    rect.left,
                                    rect.top,
                                    rect.right - rect.left,
                                    rect.bottom - rect.top,
                                    hWndParent,
                                    hMenu,
                                    // _AtlBaseModule.GetModuleInstance(), lpCreateParam);
                                    GetModuleInstance(),
                                    lpCreateParam)
        }
    }

	// //create dialog controls
	// pub fn Create2(lpstrWndClass:LPCTSTR,hWndParent:HWND,rect:&RECT) -> HWND {

	// }

    pub fn DestroyWindow(&mut self) -> BOOL {
        self.assert_window();
        unsafe{
            if user32::DestroyWindow(self.0) == TRUE {
                self.Detach();
                return TRUE;
            }
            FALSE
        }
    }

    pub fn GetStyle(&self) -> DWORD {
        self.assert_window();
        unsafe {
            user32::GetWindowLongW(self.0, GWL_STYLE) as DWORD
        }
    }

    pub fn GetExStyle(&self) -> DWORD {
        self.assert_window();
        unsafe {
            user32::GetWindowLongW(self.0, GWL_EXSTYLE) as DWORD
        }
    }

    pub fn GetWindowLong(&self, nIndex: c_int) -> LONG {
        self.assert_window();
        unsafe {
            user32::GetWindowLongW(self.0, nIndex)
        }
    }

    pub fn GetWindowLongPtr(&self, nIndex: c_int) -> LONG_PTR {
        self.assert_window();
        unsafe {
            user32::GetWindowLongPtrW(self.0, nIndex)
        }
    }

    pub fn SetWindowLong(&self, nIndex: c_int, dwNewLong: LONG) -> LONG {
        self.assert_window();
        unsafe {
            user32::SetWindowLongW(self.0, nIndex, dwNewLong)
        }
    }

    pub fn SetWindowLongPtr(&self, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR {
        self.assert_window();
        unsafe {
            user32::SetWindowLongPtrW(self.0, nIndex, dwNewLong)
        }
    }

    pub fn GetWindowWord(&self, nIndex: c_int) -> WORD {
        self.assert_window();
        unsafe {
            user32::GetWindowWord(self.0, nIndex)
        }
    }

    pub fn SetWindowWord(&self, nIndex: c_int, wNewWord: WORD) -> WORD {
        self.assert_window();
        unsafe {
            user32::SetWindowWord(self.0, nIndex, wNewWord)
        }
    }

    pub fn SendMessage(&self, message: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, message, wParam, lParam)
        }
    }

    pub fn PostMessage(&self, message: UINT, wParam: WPARAM, lParam: LPARAM) -> bool {
        self.assert_window();
        unsafe {
            user32::PostMessageW(self.0, message, wParam, lParam) == TRUE
        }
    }

    pub fn SendNotifyMessage(&self, message: UINT, wParam: WPARAM, lParam: LPARAM) -> bool {
        self.assert_window();
        unsafe {
            user32::SendNotifyMessageW(self.0, message, wParam, lParam) == TRUE
        }
    }

	//static function
	// pub fn SendMessage (hWnd:HWND,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
	// 	//ATLASSERT(::IsWindow(hWnd));
	// 	assert!(user32::IsWindow(hWnd) == TRUE);
	// 	user32::SendMessage(hWnd, message, wParam, lParam)
	// }

	pub fn SetWindowText (&self, lpszString: &str) -> bool {
		self.assert_window();
        let s = lpszString.to_c_u16();
		unsafe{user32::SetWindowTextW(self.0, s.as_ptr())  == TRUE}
	}

	// pub fn GetWindowText (&self,lpszStringBuf:LPTSTR,nMaxCount:c_int) -> c_int {
	// 	self.assert_window();
	// 	user32::GetWindowText(self.0, lpszStringBuf, nMaxCount)
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

    pub fn GetWindowTextLength(&self) -> c_int {
        self.assert_window();
        unsafe {
            user32::GetWindowTextLengthW(self.0)
        }
    }


	// MAKELPARAM is a macro in user32.h

	// #define MAKELPARAM(l, h)      (LPARAM)MAKELONG(l, h)

	// MAKELONG is a macro in common.h:

	// #define MAKELONG(low, high)   ((DWORD)(((WORD)(low)) | (((DWORD)((WORD)(high))) << 16)))




    pub fn SetFont(&self, hFont: HFONT, bRedraw: BOOL) {
        self.assert_window();
		//user32::SendMessage(self.0, WM_SETFONT, hFont as WPARAM, MAKELPARAM(bRedraw, 0));
        unsafe {
            user32::SendMessageW(self.0, WM_SETFONT, hFont as WPARAM, (bRedraw & 0xFFFF) as LPARAM);
        }
    }

    pub fn GetFont(&self) -> HFONT {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_GETFONT, 0, 0)  as HFONT
        }
    }



    pub fn GetMenu(&self) -> HMENU {
        self.assert_window();
        unsafe {
            user32::GetMenu(self.0) as HMENU
        }
    }

    pub fn SetMenu(&self, hMenu: HMENU) -> bool {
        self.assert_window();
        unsafe {
            user32::SetMenu(self.0, hMenu) == TRUE
        }
    }

    pub fn DrawMenuBar(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::DrawMenuBar(self.0) == TRUE
        }
    }

    pub fn GetSystemMenu(&self, bRevert: BOOL) -> HMENU {
        self.assert_window();
        unsafe {
            user32::GetSystemMenu(self.0, bRevert) as HMENU
        }
    }

    pub fn HiliteMenuItem(&self, hMenu: HMENU, uItemHilite: UINT, uHilite: UINT) -> bool {
        self.assert_window();
        unsafe {
            user32::HiliteMenuItem(self.0, hMenu, uItemHilite, uHilite) == TRUE
        }
    }

    pub fn IsIconic(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::IsIconic(self.0) == TRUE
        }
    }

    pub fn IsZoomed(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::IsZoomed(self.0) == TRUE
        }
    }

    pub fn MoveWindow(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,bRepaint: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::MoveWindow(self.0, x, y, nWidth, nHeight, bRepaint) == TRUE
        }
    }

	//pub fn MoveWindow2 (&self,lpRect:LPCRECT,bRepaint:BOOL) -> bool {
    pub fn MoveWindow2(&self, lpRect: &RECT, bRepaint: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::MoveWindow(self.0,
                               lpRect.left,
                               lpRect.top,
                               lpRect.right - lpRect.left,
                               lpRect.bottom - lpRect.top,
                               bRepaint) == TRUE
        }
    }

    pub fn SetWindowPos(&self,
                        hWndInsertAfter: HWND,
                        x: c_int,
                        y: c_int,
                        cx: c_int,
                        cy: c_int,
                        nFlags: UINT)
                        -> bool {
        self.assert_window();
        unsafe {
            user32::SetWindowPos(self.0, hWndInsertAfter, x, y, cx, cy, nFlags) == TRUE
        }
    }

    pub fn SetWindowPos2(&self, hWndInsertAfter: HWND, lpRect: &RECT, nFlags: UINT) -> bool {
        self.assert_window();
        unsafe {
            user32::SetWindowPos(self.0,
                                 hWndInsertAfter,
                                 lpRect.left,
                                 lpRect.top,
                                 lpRect.right - lpRect.left,
                                 lpRect.bottom - lpRect.top,
                                 nFlags) == TRUE
        }
    }

    pub fn ArrangeIconicWindows(&self) -> UINT {
        self.assert_window();
        unsafe {
            user32::ArrangeIconicWindows(self.0)
        }
    }

    pub fn BringWindowToTop(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::BringWindowToTop(self.0) == TRUE
        }
    }

    pub fn GetWindowRect(&self, lpRect: LPRECT) -> bool {
        self.assert_window();
        unsafe {
            user32::GetWindowRect(self.0, lpRect) == TRUE
        }
    }

    pub fn GetClientRect(&self, lpRect: &mut RECT) -> bool {
        self.assert_window();
        let p = lpRect as LPRECT;
        unsafe {
            user32::GetClientRect(self.0, p) == TRUE
        }
    }

    pub fn GetWindowPlacement(&self, lpwndpl: &mut WINDOWPLACEMENT) -> bool {
        self.assert_window();
        unsafe {
            user32::GetWindowPlacement(self.0, lpwndpl) == TRUE
        }
    }

    pub fn SetWindowPlacement(&self, lpwndpl: &WINDOWPLACEMENT) -> bool {
        self.assert_window();
        unsafe {
            user32::SetWindowPlacement(self.0, lpwndpl) == TRUE
        }
    }

    pub fn ClientToScreen(&self, lpPoint: LPPOINT) -> bool {
        self.assert_window();
        unsafe {
            user32::ClientToScreen(self.0, lpPoint) == TRUE
        }
    }

    pub fn ClientToScreen2(&self, lpRect: &mut RECT) -> bool {
        self.assert_window();
        let p1 = lpRect as LPRECT;
        let p2 = p1 as LPPOINT;
        if unsafe {
            user32::ClientToScreen(self.0, p2)
        } == FALSE {
            return false;
        }
        unsafe {
            user32::ClientToScreen(self.0, p2.offset(1)) == TRUE
        }
    }

    pub fn ScreenToClient(&self, lpPoint: LPPOINT) -> bool {
        self.assert_window();
        unsafe {
            user32::ScreenToClient(self.0, lpPoint) == TRUE
        }
    }

    pub fn ScreenToClient2(&self, lpRect: &mut RECT) -> bool {
        self.assert_window();
        let p1 = lpRect as LPRECT;
        let p2 = p1 as LPPOINT;

        if unsafe {
            user32::ScreenToClient(self.0, p2)
        } == FALSE {
            return false;
        }
		//user32::ScreenToClient(self.0, ((LPPOINT)lpRect)+1) == TRUE
        unsafe {
            user32::ScreenToClient(self.0, p2.offset(1)) == TRUE
        }
    }

    pub fn MapWindowPoints(&self, hWndTo: HWND, lpPoint: LPPOINT, nCount: UINT) -> c_int {
        self.assert_window();
        unsafe {
            user32::MapWindowPoints(self.0, hWndTo, lpPoint, nCount)
        }
    }

    pub fn MapWindowPoints2(&self, hWndTo: HWND, lpRect: LPRECT) -> c_int {
        self.assert_window();
		//user32::MapWindowPoints(self.0, hWndTo, (LPPOINT)lpRect, 2)
        unsafe {
            user32::MapWindowPoints(self.0, hWndTo, lpRect as LPPOINT, 2)
        }
    }

    pub fn BeginPaint(&self, lpPaint: LPPAINTSTRUCT) -> HDC {
        self.assert_window();
        unsafe {
            user32::BeginPaint(self.0, lpPaint)
        }
    }

    pub fn EndPaint(&self, lpPaint: LPPAINTSTRUCT) {
        self.assert_window();
        unsafe {
            user32::EndPaint(self.0, lpPaint);
        }
    }

    pub fn GetDC(&self) -> HDC {
        self.assert_window();
        unsafe {
            user32::GetDC(self.0)
        }
    }

    pub fn GetWindowDC(&self) -> HDC {
        self.assert_window();
        unsafe {
            user32::GetWindowDC(self.0)
        }
    }

    pub fn ReleaseDC(&self, hDC: HDC) -> c_int {
        self.assert_window();
        unsafe {
            user32::ReleaseDC(self.0, hDC)
        }
    }

    pub fn Print(&self, hDC: HDC, dwFlags: DWORD) {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_PRINT, hDC as WPARAM, dwFlags as LPARAM);
        }
    }

    pub fn PrintClient(&self, hDC: HDC, dwFlags: DWORD) {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_PRINTCLIENT, hDC as WPARAM, dwFlags as LPARAM);
        }
    }

    pub fn UpdateWindow(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::UpdateWindow(self.0) == TRUE
        }
    }

    pub fn SetRedraw(&self, bRedraw: BOOL) {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_SETREDRAW, bRedraw as WPARAM, NULL_LPARAM);
        }
    }

    pub fn GetUpdateRect(&self, lpRect: LPRECT, bErase: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::GetUpdateRect(self.0, lpRect, bErase) == TRUE
        }
    }

    pub fn GetUpdateRgn(&self, hRgn: HRGN, bErase: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            user32::GetUpdateRgn(self.0, hRgn, bErase)
        }
    }

    pub fn Invalidate(&self, bErase: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::InvalidateRect(self.0, 0 as LPRECT, bErase) == TRUE
        }
    }

    pub fn Invalidate2(&self, lpRect: LPCRECT, bErase: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::InvalidateRect(self.0, lpRect, bErase) == TRUE
        }
    }

    pub fn ValidateRect(&self, lpRect: LPCRECT) -> bool {
        self.assert_window();
        unsafe {
            user32::ValidateRect(self.0, lpRect) == TRUE
        }
    }

    pub fn InvalidateRgn(&self, hRgn: HRGN, bErase: BOOL) {
        self.assert_window();
        unsafe {
            user32::InvalidateRgn(self.0, hRgn, bErase);
        }
    }

    pub fn ValidateRgn(&self, hRgn: HRGN) -> bool {
        self.assert_window();
        unsafe {
            user32::ValidateRgn(self.0, hRgn) == TRUE
        }
    }

    pub fn ShowWindow(&self, nCmdShow: c_int) -> bool {
        self.assert_window();
        unsafe {
            user32::ShowWindow(self.0, nCmdShow) == TRUE
        }
    }

    pub fn IsWindowVisible(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::IsWindowVisible(self.0) == TRUE
        }
    }

    pub fn ShowOwnedPopups(&self, bShow: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::ShowOwnedPopups(self.0, bShow) == TRUE
        }
    }

    pub fn GetDCEx(&self, hRgnClip: HRGN, flags: DWORD) -> HDC {
        self.assert_window();
        unsafe {
            user32::GetDCEx(self.0, hRgnClip, flags)
        }
    }

    pub fn LockWindowUpdate(&self, bLock: bool) -> bool {
        self.assert_window();
        if bLock {
            unsafe {
                user32::LockWindowUpdate(self.0) == TRUE
            }
        } else {
            unsafe {
                user32::LockWindowUpdate(NULL_HWND) == TRUE
            }
        }
    }

    pub fn RedrawWindow2(&self) -> bool {
        self.RedrawWindow(0 as LPCRECT,0 as HRGN,RDW_INVALIDATE | RDW_UPDATENOW | RDW_ERASE)
    }

    pub fn RedrawWindow(&self, lpRectUpdate: LPCRECT, hRgnUpdate: HRGN, flags: UINT) -> bool {
        self.assert_window();
        unsafe {
            user32::RedrawWindow(self.0, lpRectUpdate, hRgnUpdate, flags) == TRUE
        }
    }

    pub fn SetTimer(&self, nIDEvent: UINT_PTR, nElapse: UINT) -> UINT_PTR {
        self.assert_window();
        unsafe {
            user32::SetTimer(self.0, nIDEvent, nElapse, None)
        }
    }

    pub fn SetTimer2(&self, nIDEvent: UINT_PTR, nElapse: UINT, lpfnTimer: TimerProc) -> UINT_PTR {
        self.assert_window();
        unsafe {
            user32::SetTimer(self.0, nIDEvent, nElapse, lpfnTimer)
        }
    }

    pub fn KillTimer(&self, nIDEvent: UINT_PTR) -> bool {
        self.assert_window();
        unsafe {
            user32::KillTimer(self.0, nIDEvent) == TRUE
        }
    }

    pub fn IsWindowEnabled(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::IsWindowEnabled(self.0) == TRUE
        }
    }

    pub fn EnableWindow(&self, bEnable: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::EnableWindow(self.0, bEnable) == TRUE
        }
    }

    pub fn SetActiveWindow(&self) -> HWND {
        self.assert_window();
        unsafe {
            user32::SetActiveWindow(self.0)
        }
    }

    pub fn SetCapture(&self) -> HWND {
        self.assert_window();
        unsafe {
            user32::SetCapture(self.0)
        }
    }

    pub fn SetFocus(&self) -> HWND {
        self.assert_window();
        unsafe {
            user32::SetFocus(self.0)
        }
    }



    pub fn CheckDlgButton(&self, nIDButton: c_int, nCheck: UINT) -> bool {
        self.assert_window();
        unsafe {
            user32::CheckDlgButton(self.0, nIDButton, nCheck) == TRUE
        }
    }

    pub fn CheckRadioButton(&self,
                            nIDFirstButton: c_int,
                            nIDLastButton: c_int,
                            nIDCheckButton: c_int)
                            -> bool {
        self.assert_window();
        unsafe {
            user32::CheckRadioButton(self.0, nIDFirstButton, nIDLastButton, nIDCheckButton) == TRUE
        }
    }
/*
	pub fn DlgDirList (&self,lpPathSpec: &str,nIDListBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
		self.assert_window();
        let p  = lpPathSpec.to_c_u16();
		user32::DlgDirList(self.0, p, nIDListBox, nIDStaticPath, nFileType)
	}

	pub fn DlgDirListComboBox (&self,lpPathSpec: &str,nIDComboBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
		self.assert_window();
        let p = lpPathSpec.to_c_u16();
		user32::DlgDirListComboBox(self.0, p, nIDComboBox, nIDStaticPath, nFileType)
	}

	pub fn DlgDirSelect (&self,lpString: &str,nCount:c_int,nIDListBox:c_int) -> bool {
		self.assert_window();
        let s = lpString.to_c_u16();
		user32::DlgDirSelectEx(self.0, s.as_ptr(), nCount, nIDListBox) == TRUE
	}

	pub fn DlgDirSelectComboBox (&self,lpString:&str,nCount:c_int,nIDComboBox:c_int) -> bool {
		self.assert_window();
        let s = lpString.to_c_u16();
		user32::DlgDirSelectComboBoxEx(self.0, s.as_ptr(), nCount, nIDComboBox) == TRUE
	}
*/
    pub fn GetDlgItemInt(&self, nID: c_int) -> UINT {
        self.assert_window();
        unsafe {
            user32::GetDlgItemInt(self.0, nID, 0 as *mut BOOL, TRUE)
        }
    }

    pub fn GetDlgItemInt2(&self, nID: c_int, lpTrans: &mut BOOL, bSigned: BOOL) -> UINT {
        self.assert_window();
        unsafe {
            user32::GetDlgItemInt(self.0, nID, lpTrans as *mut BOOL, bSigned)
        }
    }

	//pub fn GetDlgItemText (&self,nID:c_int,lpStr:LPTSTR,nMaxCount:c_int) -> UINT {
    pub fn GetDlgItemText (&self, nID: c_int) -> String {
		self.assert_window();
        let hItem = self.GetDlgItem(nID);
        if hItem != NULL_HWND {
            let nLength = unsafe{user32::GetWindowTextLengthW(hItem) + 1};
            //LPTSTR pszText;
            let mut pszText: Vec<u16> = Vec::with_capacity(nLength as usize);
            //nLength = 
            //pszText = strText.GetBuffer(nLength+1);
            let nRead = unsafe{user32::GetWindowTextW(hItem, pszText.as_mut_ptr(), nLength)};
            debug_assert!(nRead == nLength - 1);
            String::from_utf16_lossy(pszText.as_ref())
        }
        else
        {
            String::new()
        }
		//user32::GetDlgItemText(self.0, nID, lpStr, nMaxCount)
	}

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


    pub fn IsDlgButtonChecked(&self, nIDButton: c_int) -> UINT {
        self.assert_window();
        unsafe {
            user32::IsDlgButtonChecked(self.0, nIDButton)
        }
    }

    pub fn SendDlgItemMessage(&self,
                              nID: c_int,
                              message: UINT,
                              wParam: WPARAM,
                              lParam: LPARAM)
                              -> LRESULT {
        self.assert_window();
        unsafe {
            user32::SendDlgItemMessageW(self.0, nID, message, wParam, lParam)
        }
    }

    pub fn SetDlgItemInt(&self, nID: c_int, nValue: UINT, bSigned: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::SetDlgItemInt(self.0, nID, nValue, bSigned) == TRUE
        }
    }

	// pub fn SetDlgItemText (&self,nID:c_int,lpszString:LPCTSTR) -> bool {
	// 	self.assert_window();
	// 	user32::SetDlgItemText(self.0, nID, lpszString) == TRUE
	// }

// #ipub fndef _ATL_NO_HOSTING
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



    pub fn GetScrollPos(&self, nBar: c_int) -> c_int {
        self.assert_window();
        unsafe {
            user32::GetScrollPos(self.0, nBar)
        }
    }

    pub fn GetScrollRange(&self, nBar: c_int, lpMinPos: LPINT, lpMaxPos: LPINT) -> bool {
        self.assert_window();
        unsafe {
            user32::GetScrollRange(self.0, nBar, lpMinPos, lpMaxPos) == TRUE
        }
    }

    pub fn ScrollWindow(&self,
                        xAmount: c_int,
                        yAmount: c_int,
                        lpRect: LPCRECT,
                        lpClipRect: LPCRECT)
                        -> bool {
        self.assert_window();
        unsafe {
            user32::ScrollWindow(self.0, xAmount, yAmount, lpRect, lpClipRect) == TRUE
        }
    }

    pub fn ScrollWindowEx(&self,
                          dx: c_int,
                          dy: c_int,
                          lpRectScroll: LPCRECT,
                          lpRectClip: LPCRECT,
                          hRgnUpdate: HRGN,
                          lpRectUpdate: LPRECT,
                          uFlags: UINT)
                          -> c_int {
        self.assert_window();
        unsafe {
            user32::ScrollWindowEx(self.0,
                                   dx,
                                   dy,
                                   lpRectScroll,
                                   lpRectClip,
                                   hRgnUpdate,
                                   lpRectUpdate,
                                   uFlags)
        }
    }

    pub fn ScrollWindowExDefault(&self, dx: c_int, dy: c_int, uFlags: UINT) -> c_int {
        self.ScrollWindowEx(dx,dy,0 as LPCRECT,0 as LPCRECT,0 as HRGN,0 as LPRECT,uFlags)
    }

    pub fn SetScrollPos(&self, nBar: c_int, nPos: c_int, bRedraw: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            user32::SetScrollPos(self.0, nBar, nPos, bRedraw)
        }
    }

    pub fn SetScrollRange(&self,
                          nBar: c_int,
                          nMinPos: c_int,
                          nMaxPos: c_int,
                          bRedraw: BOOL)
                          -> bool {
        self.assert_window();
        unsafe {
            user32::SetScrollRange(self.0, nBar, nMinPos, nMaxPos, bRedraw) == TRUE
        }
    }

    pub fn ShowScrollBar(&self, nBar: c_int, bShow: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::ShowScrollBar(self.0, nBar, bShow) == TRUE
        }
    }

    pub fn EnableScrollBar(&self, uSBFlags: UINT, uArrowFlags: UINT) -> bool {
        self.assert_window();
        unsafe {
            user32::EnableScrollBar(self.0, uSBFlags, uArrowFlags) == TRUE
        }
    }

    pub fn IsChild(&self, hWnd: HWND) -> bool {
        self.assert_window();
        unsafe {
            user32::IsChild(self.0, hWnd) == TRUE
        }
    }

    pub fn GetDlgCtrlID(&self) -> c_int {
        self.assert_window();
        unsafe {
            user32::GetDlgCtrlID(self.0)
        }
    }

    pub fn SetDlgCtrlID(&self, nID: c_int) -> c_int {
        self.assert_window();
        unsafe {
            user32::SetWindowLongW(self.0, GWL_ID, nID)
        }
    }

    pub fn FlashWindow(&self, bInvert: BOOL) -> bool {
        self.assert_window();
        unsafe {
            user32::FlashWindow(self.0, bInvert) == TRUE
        }
    }

	// pub fn MessageBox(&self,lpszText:LPCTSTR ,lpszCaption:LPCTSTR,nType:UINT) -> c_int {
	// 	self.assert_window();
	// 	user32::MessageBox(self.0, lpszText, lpszCaption, nType)
	// }

    pub fn ChangeClipboardChain(&self, hWndNewNext: HWND) -> bool {
        self.assert_window();
        unsafe {
            user32::ChangeClipboardChain(self.0, hWndNewNext) == TRUE
        }
    }

    pub fn SetClipboardViewer(&self) -> HWND {
        self.assert_window();
        unsafe {
            user32::SetClipboardViewer(self.0)
        }
    }

    pub fn OpenClipboard(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::OpenClipboard(self.0) == TRUE
        }
    }



    pub fn CreateCaret(&self, hBitmap: HBITMAP) -> bool {
        self.assert_window();
        unsafe {
            user32::CreateCaret(self.0, hBitmap, 0, 0) == TRUE
        }
    }

    pub fn CreateSolidCaret(&self, nWidth: c_int, nHeight: c_int) -> bool {
        self.assert_window();
        unsafe {
            user32::CreateCaret(self.0, 0 as HBITMAP, nWidth, nHeight) == TRUE
        }
    }

    pub fn CreateGrayCaret(&self, nWidth: c_int, nHeight: c_int) -> bool {
        self.assert_window();
        unsafe {
            user32::CreateCaret(self.0, 1 as HBITMAP, nWidth, nHeight) == TRUE
        }
    }

    pub fn HideCaret(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::HideCaret(self.0) == TRUE
        }
    }

    pub fn ShowCaret(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::ShowCaret(self.0) == TRUE
        }
    }

    pub fn DragAcceptFiles(&self, bAccept: BOOL) {
        self.assert_window();
        unsafe {
            shell32::DragAcceptFiles(self.0, bAccept);
        }
    }

    pub fn SetIcon(&self, hIcon: HICON, bBigIcon: BOOL) -> HICON {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_SETICON, bBigIcon as WPARAM, hIcon as LPARAM) as HICON
        }
    }

    pub fn GetIcon(&self, bBigIcon: BOOL) -> HICON {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_GETICON, bBigIcon as WPARAM, NULL_LPARAM) as HICON
        }
    }

	// pub fn WinHelp (&self,lpszHelp:LPCTSTR,nCmd:UINT,dwData:DWORD) -> bool {
	// 	self.assert_window();
	// 	user32::WinHelp(self.0, lpszHelp, nCmd, dwData) == TRUE
	// }

    pub fn SetWindowContextHelpId(&self, dwContextHelpId: DWORD) -> bool {
        self.assert_window();
        unsafe {
            user32::SetWindowContextHelpId(self.0, dwContextHelpId) == TRUE
        }
    }

    pub fn GetWindowContextHelpId(&self) -> DWORD {
        self.assert_window();
        unsafe {
            user32::GetWindowContextHelpId(self.0)
        }
    }

    pub fn SetHotKey(&self, wVirtualKeyCode: WORD, wModifiers: WORD) -> c_int {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_SETHOTKEY, MAKEWORD(wVirtualKeyCode as u8, wModifiers as u8) as WPARAM, 0 ) as c_int
        }
    }

    pub fn GetHotKey(&self) -> DWORD {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_GETHOTKEY, 0, 0) as DWORD
        }
    }

    pub fn GetScrollInfo(&self, nBar: c_int, lpScrollInfo: LPSCROLLINFO) -> bool {
        self.assert_window();
        unsafe {
            user32::GetScrollInfo(self.0, nBar, lpScrollInfo) == TRUE
        }
    }
    pub fn SetScrollInfo(&self, nBar: c_int, lpScrollInfo: LPSCROLLINFO, bRedraw: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            user32::SetScrollInfo(self.0, nBar, lpScrollInfo, bRedraw)
        }
    }
    pub fn IsDialogMessage(&self, lpMsg: LPMSG) -> bool {
        self.assert_window();
        unsafe {
            user32::IsDialogMessageW(self.0, lpMsg) == TRUE
        }
    }

    pub fn NextDlgCtrl(&self) {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_NEXTDLGCTL, 0, 0);
        }
    }
    pub fn PrevDlgCtrl(&self) {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_NEXTDLGCTL, 1, 0);
        }
    }
    pub fn GotoDlgCtrl(&self, hWndCtrl: HWND) {
        self.assert_window();
        unsafe {
            user32::SendMessageW(self.0, WM_NEXTDLGCTL, hWndCtrl as WPARAM, 1);
        }
    }

    pub fn ResizeClient(&self, nWidth: c_int, nHeight: c_int, bRedraw: BOOL) -> bool {
        self.assert_window();

        let mut rcWnd = RECT { left: 0, right: 0, top: 0, bottom: 0 };
		//if(!GetClientRect(&rcWnd))
		//	return FALSE;
        if self.GetClientRect(&mut rcWnd) == false {
            return false;
        }

        if nWidth != -1 {
            rcWnd.right = nWidth;
        }

        if nHeight != -1 {
            rcWnd.bottom = nHeight;
        }

		//let b1 = !(self.GetStyle() & WS_CHILD) && (self.GetMenu() != 0 as HMENU);
        let b1 = ((self.GetStyle() & WS_CHILD) == 0) && (self.GetMenu() != (0 as HMENU));
        if unsafe {
            user32::AdjustWindowRectEx(&mut rcWnd, self.GetStyle(), b1 as BOOL, self.GetExStyle())
        } == TRUE {
            return false;
        }

        let mut uFlags: UINT = SWP_NOZORDER | SWP_NOMOVE | SWP_NOACTIVATE;
        if bRedraw == FALSE {
            uFlags |= SWP_NOREDRAW;
        }

        self.SetWindowPos(NULL_HWND, 0, 0, rcWnd.right - rcWnd.left, rcWnd.bottom - rcWnd.top, uFlags)
    }

    pub fn GetWindowRgn(&self, hRgn: HRGN) -> c_int {
        self.assert_window();
        unsafe {
            user32::GetWindowRgn(self.0, hRgn)
        }
    }
    pub fn SetWindowRgn(&self, hRgn: HRGN, bRedraw: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            user32::SetWindowRgn(self.0, hRgn, bRedraw)
        }
    }

    pub fn DeferWindowPos(&self,
                          hWinPosInfo: HDWP,
                          hWndInsertAfter: HWND,
                          x: c_int,
                          y: c_int,
                          cx: c_int,
                          cy: c_int,
                          uFlags: UINT)
                          -> HDWP {
        self.assert_window();
        unsafe {
            user32::DeferWindowPos(hWinPosInfo, self.0, hWndInsertAfter, x, y, cx, cy, uFlags)
        }
    }

    pub fn GetWindowThreadID(&self) -> DWORD {
        self.assert_window();
        unsafe {
            user32::GetWindowThreadProcessId(self.0, 0 as LPDWORD)
        }
    }

    pub fn GetWindowProcessID(&self) -> DWORD {
        self.assert_window();
        let mut dwProcessID: DWORD = 0;
        unsafe {
            user32::GetWindowThreadProcessId(self.0, &mut dwProcessID);
        }
        dwProcessID
    }

    pub fn IsWindow(&self) -> bool {
        unsafe {
            user32::IsWindow(self.0) == TRUE
        }
    }
    pub fn IsWindowUnicode(&self) -> bool {
        self.assert_window();
        unsafe {
            user32::IsWindowUnicode(self.0) == TRUE
        }
    }
	// pub fn IsParentDialog (&self) -> bool {
	// 	self.assert_window();
	// 	TCHAR szBuf[8];
	// 	if (GetClassName(GetParent(), szBuf, sizeof(szBuf)/sizeof(szBuf[0])) == 0)
	// 		return FALSE;
	// 	return lstrcmp(szBuf, _T("#32770")) == 0;
	// }
    pub fn ShowWindowAsync(&self, nCmdShow: c_int) -> bool {
        self.assert_window();
        unsafe {
            user32::ShowWindowAsync(self.0, nCmdShow) == TRUE
        }
    }


	// pub fn SendMessageToDescendants (&self,message:UINT,wParam:WPARAM,lParam:LPARAM,bDeep:BOOL)  {
	// 	for(HWND hWndChild = ::GetTopWindow(self.0); hWndChild != NULL;
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

    pub fn CenterWindow(&self, hCenter: HWND) -> BOOL {
        self.assert_window();
        let mut hWndCenter = hCenter;
        unsafe {
            let dwStyle = self.GetStyle();
            if hWndCenter == NULL_HWND {
                if dwStyle & WS_CHILD != 0 {
                    hWndCenter = self.GetParent2().GetHwnd();//::GetParent(self.0);
                } else {
                    hWndCenter = user32::GetWindow(self.0, GW_OWNER);
                }
            }


            let mut rcDlg: RECT = Default::default();
            user32::GetWindowRect(self.0, &mut rcDlg);
            let mut rcArea: RECT = Default::default();
            let mut rcCenter: RECT = Default::default();
            let hWndParent: HWND;
            if dwStyle & WS_CHILD == 0 {

                if hWndCenter != NULL_HWND {
                    let dwStyleCenter = user32::GetWindowLongW(hWndCenter, GWL_STYLE) as DWORD;
                    if (dwStyleCenter & WS_VISIBLE) == 0 || (dwStyleCenter & WS_MINIMIZE) != 0 {
                        hWndCenter = NULL_HWND;
                    }
                }

				//>=win2k
                let mut hMonitor: HMONITOR = 0 as HMONITOR;
                if hWndCenter != NULL_HWND {
                    hMonitor = user32::MonitorFromWindow(hWndCenter, MONITOR_DEFAULTTONEAREST);
                } else {
                    hMonitor = user32::MonitorFromWindow(self.0, MONITOR_DEFAULTTONEAREST);
                }
				//ATLENSURE_RETURN_VAL(hMonitor != NULL, FALSE);
                let mut minfo: MONITORINFO = Default::default();
                minfo.cbSize = std::mem::size_of::<MONITORINFO>() as DWORD;
                //let bResult: BOOL = user32::GetMonitorInfoW(hMonitor, &mut minfo);
                user32::GetMonitorInfoW(hMonitor, &mut minfo);
				//ATLENSURE_RETURN_VAL(bResult, FALSE);

                rcArea = minfo.rcWork;

                if hWndCenter == NULL_HWND {
                    rcCenter = rcArea;
                } else {
                    user32::GetWindowRect(hWndCenter, &mut rcCenter);
                }
            } else {

                hWndParent = user32::GetParent(self.0);
				//ATLASSERT(::IsWindow(hWndParent));

                user32::GetClientRect(hWndParent, &mut rcArea);
				//ATLASSERT(::IsWindow(hWndCenter));
                user32::GetClientRect(hWndCenter, &mut rcCenter);
                user32::MapWindowPoints(hWndCenter,
                                        hWndParent,
                                        &mut rcCenter as *mut RECT as *mut POINT,
                                        2);
            }

            let DlgWidth: c_int = rcDlg.right - rcDlg.left;
            let DlgHeight: c_int = rcDlg.bottom - rcDlg.top;


            let mut xLeft: c_int = (rcCenter.left + rcCenter.right) / 2 - DlgWidth / 2;
            let mut yTop: c_int = (rcCenter.top + rcCenter.bottom) / 2 - DlgHeight / 2;


            if xLeft + DlgWidth > rcArea.right {
                xLeft = rcArea.right - DlgWidth;
            }

            if xLeft < rcArea.left {
                xLeft = rcArea.left;
            }

            if yTop + DlgHeight > rcArea.bottom {
                yTop = rcArea.bottom - DlgHeight;
            }

            if yTop < rcArea.top {
                yTop = rcArea.top;
            }

            user32::SetWindowPos(self.0,
                                 NULL_HWND,
                                 xLeft,
                                 yTop,
                                 -1,
                                 -1,
                                 SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE)
        }
    }

    pub fn ModifyStyle(&self, dwRemove: DWORD, dwAdd: DWORD, nFlags: UINT) -> bool {
        self.assert_window();

        let dwStyle: DWORD = unsafe {
            user32::GetWindowLongW(self.0, GWL_STYLE) as DWORD
        };
        let dwNewStyle = (dwStyle & !dwRemove) | dwAdd;
        if dwStyle == dwNewStyle {
            return false;
        }

        unsafe {
            user32::SetWindowLongW(self.0, GWL_STYLE, dwNewStyle as LONG);
        }
        if nFlags != 0 {
            unsafe {
                user32::SetWindowPos(self.0,
                                     NULL_HWND,
                                     0,
                                     0,
                                     0,
                                     0,
                                     SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE |
                                     nFlags)
            };
        }

        true
    }

    pub fn ModifyStyleEx(&self, dwRemove: DWORD, dwAdd: DWORD, nFlags: UINT) -> bool {
        self.assert_window();

        let dwStyle: DWORD = unsafe {
            user32::GetWindowLongW(self.0, GWL_EXSTYLE) as DWORD
        };
        let dwNewStyle: DWORD = (dwStyle & !dwRemove) | dwAdd;
        if dwStyle == dwNewStyle {
            return false;
        }

        unsafe {
            user32::SetWindowLongW(self.0, GWL_EXSTYLE, dwNewStyle as LONG);
        }
        if nFlags != 0 {
            unsafe {
                user32::SetWindowPos(self.0,
                                     NULL_HWND,
                                     0,
                                     0,
                                     0,
                                     0,
                                     SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE |
                                     nFlags)
            };
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

	// 	c_int nLen = ::GetWindowTextLength(self.0);

	// 	CTempBuffer<TCHAR> lpszText;
	// 	if(nLen>0)
	// 	{
	// 		ATLTRY(lpszText.Allocate(nLen+1));
	// 		if (lpszText == NULL)
	// 		{
	// 			return FALSE;
	// 		}

	// 		if(!::GetWindowText(self.0, lpszText, nLen+1))
	// 		{
	// 			return FALSE;
	// 		}
	// 	}

	// 	bstrText = ::SysAllocString(T2OLE_EX_DEF(lpszText));

	// 	return nLen==0 ? FALSE : ((bstrText != NULL) ? TRUE : FALSE);
	// }
}


const MONITOR_DEFAULTTONULL: DWORD = 0x00000000;
const MONITOR_DEFAULTTOPRIMARY: DWORD = 0x00000001;
const MONITOR_DEFAULTTONEAREST: DWORD = 0x00000002;