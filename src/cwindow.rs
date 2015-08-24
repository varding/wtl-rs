

// use winapi::{
//     BOOL, CREATESTRUCTW, HBRUSH, HCURSOR, HICON, HMENU, HWND, INT, LPARAM, LRESULT, RECT, UINT,
//     WNDCLASSEXW, WPARAM, c_int,TRUE,FALSE,DWORD,GWL_STYLE,GWL_EXSTYLE,LONG,LONG_PTR,WORD,HFONT,
//     WM_SETFONT,WM_GETFONT,LPCRECT,LPRECT,WINDOWPLACEMENT,LPPOINT,LPPAINTSTRUCT,HDC,WM_PRINT,WM_PRINTCLIENT,
//     WM_SETREDRAW,HRGN,UINT_PTR,TimerProc
// };



use winapi::*;
use user32;

// pub struct CWindow {
//     self.get_hwnd(): HWND;
// }

//dialog or window impl this trait can proc messages

// pub trait Wnd {
// 	pub fn ProcessWindowMessage(&self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD)->BOOL;
// }

pub struct Dialog {
    m_hWnd: HWND
}

//every control must impl the two methods
// impl CWindow for Dialog {
// 	#[inline(always)]
// 	fn get_hwnd(&self)-> HWND {
// 		self.m_hWnd
// 	}

// 	fn set_hwnd(&mut self,h:HWND){
// 		self.m_hWnd = h;
// 	}
// }

impl CWindow for Dialog {
	fn get_hwnd(&self) -> HWND{
		self.m_hWnd
	}

	fn set_hwnd(&mut self,h:HWND){
		self.m_hWnd = h;
	}

	fn from_hwnd(h:HWND)->Dialog{
		Dialog{
			m_hWnd:h
		}
	}
}

// pub trait HWND_T{
// 	fn get_hwnd(&self) -> HWND;
// 	fn set_hwnd(&mut self,h:HWND);
// 	fn from_hwnd(h:HWND)->Self;
// }

// impl<T> CWindow for T where T:HWND_T{

// }

//place hold
fn GetModuleInstance() -> HINSTANCE {
	0 as HINSTANCE
}

//#[cfg(target_arch = "x86_64")]
pub trait CWindow {
	fn get_hwnd(&self) -> HWND;
	fn set_hwnd(&mut self,h:HWND);
	fn from_hwnd(h:HWND)->Self;

	fn HWND (&self) -> HWND{
		self.get_hwnd()
	}

	fn assert_window(&self){
		unsafe{
			assert!(user32::IsWindow(self.get_hwnd()) == TRUE);
		}
	}

	///////////////////////////////////
	fn Attach (&mut self,hWndNew:HWND) {
		assert!(self.get_hwnd() == (0 as HWND));
		assert!(hWndNew != (0 as HWND));
		unsafe{
			assert!(user32::IsWindow(hWndNew) == TRUE);
		}
		self.set_hwnd(hWndNew);
	}

	fn Detach (&mut self) -> HWND {
		let hWnd = self.get_hwnd();
		self.set_hwnd(hWnd);
		hWnd
	}

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
		//ATLASSUME(self.get_hwnd() == NULL);
		//assert!(self.get_hwnd() == (0 as HWND));
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
		if unsafe{user32::DestroyWindow(self.get_hwnd())} == FALSE{
			false
		}else{
			self.set_hwnd(0 as HWND);
			true
		}
	}

	fn GetStyle (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowLongW(self.get_hwnd(), GWL_STYLE) as DWORD}
	}

	fn GetExStyle (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowLongW(self.get_hwnd(), GWL_EXSTYLE) as DWORD}
	}

	fn GetWindowLong (&self,nIndex:c_int) -> LONG {
		self.assert_window();
		unsafe{user32::GetWindowLongW(self.get_hwnd(), nIndex)}
	}

	fn GetWindowLongPtr (&self,nIndex:c_int) -> LONG_PTR {
		self.assert_window();
		unsafe{user32::GetWindowLongPtrW(self.get_hwnd(), nIndex)}
	}

	fn SetWindowLong (&self,nIndex:c_int,dwNewLong:LONG) -> LONG {
		self.assert_window();
		unsafe{user32::SetWindowLongW(self.get_hwnd(), nIndex, dwNewLong)}
	}

	fn SetWindowLongPtr (&self,nIndex:c_int,dwNewLong:LONG_PTR) -> LONG_PTR {
		self.assert_window();
		unsafe{user32::SetWindowLongPtrW(self.get_hwnd(), nIndex, dwNewLong)}
	}

	fn GetWindowWord (&self,nIndex:c_int) -> WORD {
		self.assert_window();
		unsafe{user32::GetWindowWord(self.get_hwnd(), nIndex)}
	}

	fn SetWindowWord (&self,nIndex:c_int,wNewWord:WORD) -> WORD {
		self.assert_window();
		unsafe{user32::SetWindowWord(self.get_hwnd(), nIndex, wNewWord)}
	}

	fn SendMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(),message,wParam,lParam)}
	}

	fn PostMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
		self.assert_window();
		unsafe{user32::PostMessageW(self.get_hwnd(),message,wParam,lParam) == TRUE}
	}

	fn SendNotifyMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
		self.assert_window();
		unsafe{user32::SendNotifyMessageW(self.get_hwnd(), message, wParam, lParam) == TRUE}
	}

	//static function
	// fn SendMessage (hWnd:HWND,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
	// 	//ATLASSERT(::IsWindow(hWnd));
	// 	assert!(user32::IsWindow(hWnd) == TRUE);
	// 	user32::SendMessage(hWnd, message, wParam, lParam)
	// }

	// fn SetWindowText (&self,lpszString:LPCTSTR) -> bool {
	// 	self.assert_window();
	// 	user32::SetWindowText(self.get_hwnd(), lpszString)  == TRUE
	// }

	// fn GetWindowText (&self,lpszStringBuf:LPTSTR,nMaxCount:c_int) -> c_int {
	// 	self.assert_window();
	// 	user32::GetWindowText(self.get_hwnd(), lpszStringBuf, nMaxCount)
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
		unsafe{user32::GetWindowTextLengthW(self.get_hwnd())}
	}

	/*
	MAKELPARAM is a macro in user32.h

	#define MAKELPARAM(l, h)      (LPARAM)MAKELONG(l, h)

	MAKELONG is a macro in common.h:

	#define MAKELONG(low, high)   ((DWORD)(((WORD)(low)) | (((DWORD)((WORD)(high))) << 16))) 

	*/


	fn SetFont (&self,hFont:HFONT,bRedraw:BOOL)  {
		self.assert_window();
		//user32::SendMessage(self.get_hwnd(), WM_SETFONT, hFont as WPARAM, MAKELPARAM(bRedraw, 0));
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_SETFONT, hFont as WPARAM, (bRedraw & 0xFFFF) as LPARAM);}
	}

	fn GetFont (&self) -> HFONT {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_GETFONT, 0, 0)  as HFONT}
	}



	fn GetMenu (&self) -> HMENU {
		self.assert_window();
		unsafe{user32::GetMenu(self.get_hwnd()) as HMENU}
	}

	fn SetMenu (&self,hMenu:HMENU) -> bool {
		self.assert_window();
		unsafe{user32::SetMenu(self.get_hwnd(), hMenu) == TRUE}
	}

	fn DrawMenuBar (&self) -> bool {
		self.assert_window();
		unsafe{user32::DrawMenuBar(self.get_hwnd()) == TRUE}
	}

	// fn GetSystemMenu (&self,bRevert:BOOL) -> HMENU {
	// 	self.assert_window();
	// 	user32::GetSystemMenu(self.get_hwnd(), bRevert) as HMENU
	// }

	// fn HiliteMenuItem (&self,hMenu:HMENU,uItemHilite:UINT,uHilite:UINT) -> bool {
	// 	self.assert_window();
	// 	user32::HiliteMenuItem(self.get_hwnd(), hMenu, uItemHilite, uHilite) == TRUE
	// }



	fn IsIconic (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsIconic(self.get_hwnd()) == TRUE}
	}

	fn IsZoomed (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsZoomed(self.get_hwnd()) == TRUE}
	}

	fn MoveWindow(&self,x:c_int,y:c_int,nWidth:c_int,nHeight:c_int,bRepaint:BOOL) -> bool{
		self.assert_window();
		unsafe{user32::MoveWindow(self.get_hwnd(), x, y, nWidth, nHeight, bRepaint) == TRUE}
	}

	//fn MoveWindow2 (&self,lpRect:LPCRECT,bRepaint:BOOL) -> bool {
	fn MoveWindow2 (&self,lpRect:&RECT,bRepaint:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::MoveWindow(self.get_hwnd(), lpRect.left, lpRect.top, lpRect.right - lpRect.left, lpRect.bottom - lpRect.top, bRepaint) == TRUE}
	}

	fn SetWindowPos(&self,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,nFlags:UINT) -> bool {
		self.assert_window();
		unsafe{user32::SetWindowPos(self.get_hwnd(), hWndInsertAfter, x, y, cx, cy, nFlags) == TRUE}
	}

	fn SetWindowPos2 (&self,hWndInsertAfter:HWND,lpRect:&RECT,nFlags:UINT) -> bool {
		self.assert_window();
		unsafe{user32::SetWindowPos(self.get_hwnd(), hWndInsertAfter, lpRect.left, lpRect.top, lpRect.right - lpRect.left, lpRect.bottom - lpRect.top, nFlags) == TRUE}
	}

	fn ArrangeIconicWindows (&self) -> UINT {
		self.assert_window();
		unsafe{user32::ArrangeIconicWindows(self.get_hwnd())}
	}

	fn BringWindowToTop (&self) -> bool {
		self.assert_window();
		unsafe{user32::BringWindowToTop(self.get_hwnd()) == TRUE}
	}

	fn GetWindowRect (&self,lpRect:LPRECT) -> bool {
		self.assert_window();
		unsafe{user32::GetWindowRect(self.get_hwnd(), lpRect) == TRUE}
	}

	fn GetClientRect (&self,lpRect:&mut RECT) -> bool {
		self.assert_window();
		let p = lpRect as LPRECT;
		unsafe{user32::GetClientRect(self.get_hwnd(), p) == TRUE}
	}

	fn GetWindowPlacement(&self,lpwndpl:&mut WINDOWPLACEMENT) -> bool {
		self.assert_window();
		unsafe{user32::GetWindowPlacement(self.get_hwnd(), lpwndpl) == TRUE}
	}

	// fn SetWindowPlacement(&self,lpwndpl:&WINDOWPLACEMENT) -> bool {
	// 	self.assert_window();
	// 	unsafe{user32::SetWindowPlacement(self.get_hwnd(), lpwndpl) == TRUE}
	// }

	fn ClientToScreen (&self,lpPoint:LPPOINT) -> bool {
		self.assert_window();
		unsafe{user32::ClientToScreen(self.get_hwnd(), lpPoint) == TRUE}
	}

	fn ClientToScreen2 (&self,lpRect:&mut RECT) -> bool {
		self.assert_window();
		let p1 = lpRect as LPRECT;
		let p2 = p1 as LPPOINT;
		if unsafe{user32::ClientToScreen(self.get_hwnd(), p2)} == FALSE{
			return false;
		}
		unsafe{user32::ClientToScreen(self.get_hwnd(), p2.offset(1)) == TRUE}
		//user32::ClientToScreen(self.get_hwnd(), ((LPPOINT)lpRect)+1) == TRUE
	}

	fn ScreenToClient (&self,lpPoint:LPPOINT) -> bool {
		self.assert_window();
		unsafe{user32::ScreenToClient(self.get_hwnd(), lpPoint) == TRUE}
	}

	fn ScreenToClient2 (&self,lpRect:&mut RECT) -> bool {
		self.assert_window();
		let p1 = lpRect as LPRECT;
		let p2 = p1 as LPPOINT;

		if unsafe{user32::ScreenToClient(self.get_hwnd(), p2)} == FALSE{
			return false;
		}
		//user32::ScreenToClient(self.get_hwnd(), ((LPPOINT)lpRect)+1) == TRUE
		unsafe{user32::ScreenToClient(self.get_hwnd(), p2.offset(1)) == TRUE}
	}

	fn MapWindowPoints (&self,hWndTo:HWND,lpPoint:LPPOINT,nCount:UINT) -> c_int {
		self.assert_window();
		unsafe{user32::MapWindowPoints(self.get_hwnd(), hWndTo, lpPoint, nCount)}
	}

	fn MapWindowPoints2 (&self,hWndTo:HWND,lpRect:LPRECT) -> c_int {
		self.assert_window();
		//user32::MapWindowPoints(self.get_hwnd(), hWndTo, (LPPOINT)lpRect, 2)
		unsafe{user32::MapWindowPoints(self.get_hwnd(), hWndTo, lpRect as LPPOINT, 2)}
	}



	fn BeginPaint (&self,lpPaint:LPPAINTSTRUCT) -> HDC {
		self.assert_window();
		unsafe{user32::BeginPaint(self.get_hwnd(), lpPaint)}
	}

	fn EndPaint (&self,lpPaint:LPPAINTSTRUCT)  {
		self.assert_window();
		unsafe{user32::EndPaint(self.get_hwnd(), lpPaint);}
	}

	fn GetDC (&self) -> HDC {
		self.assert_window();
		unsafe{user32::GetDC(self.get_hwnd())}
	}

	fn GetWindowDC (&self) -> HDC {
		self.assert_window();
		unsafe{user32::GetWindowDC(self.get_hwnd())}
	}

	fn ReleaseDC (&self,hDC:HDC) -> c_int {
		self.assert_window();
		unsafe{user32::ReleaseDC(self.get_hwnd(), hDC)}
	}

	fn Print (&self,hDC:HDC,dwFlags:DWORD)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_PRINT, hDC as WPARAM, dwFlags as LPARAM);}
	}

	fn PrintClient (&self,hDC:HDC,dwFlags:DWORD)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_PRINTCLIENT, hDC as WPARAM, dwFlags as LPARAM);}
	}

	fn UpdateWindow (&self) -> bool {
		self.assert_window();
		unsafe{user32::UpdateWindow(self.get_hwnd()) == TRUE}
	}

	fn SetRedraw (&self,bRedraw:BOOL)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_SETREDRAW, bRedraw as WPARAM, 0 as LPARAM);}
	}

	fn GetUpdateRect (&self,lpRect:LPRECT,bErase:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::GetUpdateRect(self.get_hwnd(), lpRect, bErase) == TRUE}
	}

	fn GetUpdateRgn (&self,hRgn:HRGN,bErase:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::GetUpdateRgn(self.get_hwnd(), hRgn, bErase)}
	}

	fn Invalidate (&self,bErase:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::InvalidateRect(self.get_hwnd(), 0 as LPRECT, bErase) == TRUE}
	}

	fn Invalidate2 (&self,lpRect:LPCRECT,bErase:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::InvalidateRect(self.get_hwnd(), lpRect, bErase) == TRUE}
	}

	fn ValidateRect (&self,lpRect:LPCRECT) -> bool {
		self.assert_window();
		unsafe{user32::ValidateRect(self.get_hwnd(), lpRect) == TRUE}
	}

	fn InvalidateRgn (&self,hRgn:HRGN,bErase:BOOL)  {
		self.assert_window();
		unsafe{user32::InvalidateRgn(self.get_hwnd(), hRgn, bErase);}
	}

	fn ValidateRgn (&self,hRgn:HRGN) -> bool {
		self.assert_window();
		unsafe{user32::ValidateRgn(self.get_hwnd(), hRgn) == TRUE}
	}

	fn ShowWindow (&self,nCmdShow:c_int) -> bool {
		self.assert_window();
		unsafe{user32::ShowWindow(self.get_hwnd(), nCmdShow) == TRUE}
	}

	fn IsWindowVisible (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsWindowVisible(self.get_hwnd()) == TRUE}
	}

	fn ShowOwnedPopups (&self,bShow:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::ShowOwnedPopups(self.get_hwnd(), bShow) == TRUE}
	}

	fn GetDCEx (&self,hRgnClip:HRGN,flags:DWORD) -> HDC {
		self.assert_window();
		unsafe{user32::GetDCEx(self.get_hwnd(), hRgnClip, flags)}
	}

	fn LockWindowUpdate (&self,bLock:bool) -> bool {
		self.assert_window();
		if bLock{
			unsafe{user32::LockWindowUpdate(self.get_hwnd()) == TRUE}
		}else{
			unsafe{user32::LockWindowUpdate(0 as HWND) == TRUE}
		}
	}

	// fn RedrawWindowDefault(&self) -> bool{
	// 	self.RedrawWindow(0 as HWND,0 as HRGN,RDW_UPDATENOW | RDW_ERASE) == TRUE
	// }

	fn RedrawWindow(&self,lpRectUpdate:LPCRECT,hRgnUpdate:HRGN,flags:UINT)->bool{
		self.assert_window();
		unsafe{user32::RedrawWindow(self.get_hwnd(), lpRectUpdate, hRgnUpdate, flags) == TRUE}
	}

	fn SetTimer(&self,nIDEvent:UINT_PTR,nElapse:UINT) -> UINT_PTR {
		self.assert_window();
		unsafe{user32::SetTimer(self.get_hwnd(), nIDEvent, nElapse, None)}
	}

	fn SetTimer2(&self,nIDEvent:UINT_PTR,nElapse:UINT,lpfnTimer:TimerProc)->UINT_PTR{
		self.assert_window();
		unsafe{user32::SetTimer(self.get_hwnd(), nIDEvent, nElapse, lpfnTimer)}
	}

	fn KillTimer (&self,nIDEvent:UINT_PTR) -> bool {
		self.assert_window();
		unsafe{user32::KillTimer(self.get_hwnd(), nIDEvent) == TRUE}
	}

	fn IsWindowEnabled (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsWindowEnabled(self.get_hwnd()) == TRUE}
	}

	fn EnableWindow (&self,bEnable:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::EnableWindow(self.get_hwnd(), bEnable) == TRUE}
	}

	fn SetActiveWindow (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetActiveWindow(self.get_hwnd())}
	}

	fn SetCapture (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetCapture(self.get_hwnd())}
	}

	fn SetFocus (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetFocus(self.get_hwnd())}
	}



	fn CheckDlgButton (&self,nIDButton:c_int,nCheck:UINT) -> bool {
		self.assert_window();
		unsafe{user32::CheckDlgButton(self.get_hwnd(), nIDButton, nCheck) == TRUE}
	}

	fn CheckRadioButton (&self,nIDFirstButton:c_int,nIDLastButton:c_int,nIDCheckButton:c_int) -> bool {
		self.assert_window();
		unsafe{user32::CheckRadioButton(self.get_hwnd(), nIDFirstButton, nIDLastButton, nIDCheckButton) == TRUE}
	}

	// fn DlgDirList (&self,lpPathSpec:LPTSTR,nIDListBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
	// 	self.assert_window();
	// 	user32::DlgDirList(self.get_hwnd(), lpPathSpec, nIDListBox, nIDStaticPath, nFileType)
	// }

	// fn DlgDirListComboBox (&self,lpPathSpec:LPTSTR,nIDComboBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
	// 	self.assert_window();
	// 	user32::DlgDirListComboBox(self.get_hwnd(), lpPathSpec, nIDComboBox, nIDStaticPath, nFileType)
	// }

	// fn DlgDirSelect (lpString:LPTSTR,nCount:c_int,nIDListBox:c_int) -> bool {
	// 	self.assert_window();
	// 	user32::DlgDirSelectEx(self.get_hwnd(), lpString, nCount, nIDListBox) == TRUE
	// }

	// fn DlgDirSelectComboBox (lpString:LPTSTR,nCount:c_int,nIDComboBox:c_int) -> bool {
	// 	self.assert_window();
	// 	user32::DlgDirSelectComboBoxEx(self.get_hwnd(), lpString, nCount, nIDComboBox) == TRUE
	// }

	fn GetDlgItemInt(&self,nID:c_int) -> UINT {
		self.assert_window();
		unsafe{user32::GetDlgItemInt(self.get_hwnd(), nID, 0 as *mut BOOL, TRUE)}
	}

	fn GetDlgItemInt2(&self,nID:c_int,lpTrans:&mut BOOL,bSigned:BOOL) -> UINT {
		self.assert_window();
		unsafe{user32::GetDlgItemInt(self.get_hwnd(), nID, lpTrans as *mut BOOL, bSigned)}
	}

	// fn GetDlgItemText (&self,nID:c_int,lpStr:LPTSTR,nMaxCount:c_int) -> UINT {
	// 	self.assert_window();
	// 	user32::GetDlgItemText(self.get_hwnd(), nID, lpStr, nMaxCount)
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


	// fn GetNextDlgGroupItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
	// 	self.assert_window();
	// 	user32::GetNextDlgGroupItem(self.get_hwnd(), hWndCtl, bPrevious)
	// }

	// fn GetNextDlgTabItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
	// 	self.assert_window();
	// 	user32::GetNextDlgTabItem(self.get_hwnd(), hWndCtl, bPrevious)
	// }

	fn IsDlgButtonChecked (&self,nIDButton:c_int) -> UINT {
		self.assert_window();
		unsafe{user32::IsDlgButtonChecked(self.get_hwnd(), nIDButton)}
	}

	fn SendDlgItemMessage (&self,nID:c_int,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
		self.assert_window();
		unsafe{user32::SendDlgItemMessageW(self.get_hwnd(), nID, message, wParam, lParam)}
	}

	fn SetDlgItemInt (&self,nID:c_int,nValue:UINT,bSigned:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::SetDlgItemInt(self.get_hwnd(), nID, nValue, bSigned) == TRUE}
	}

	// fn SetDlgItemText (&self,nID:c_int,lpszString:LPCTSTR) -> bool {
	// 	self.assert_window();
	// 	user32::SetDlgItemText(self.get_hwnd(), nID, lpszString) == TRUE
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
		unsafe{user32::GetScrollPos(self.get_hwnd(), nBar)}
	}

	fn GetScrollRange (&self,nBar:c_int,lpMinPos:LPINT,lpMaxPos:LPINT) -> bool {
		self.assert_window();
		unsafe{user32::GetScrollRange(self.get_hwnd(), nBar, lpMinPos, lpMaxPos) == TRUE}
	}

	fn ScrollWindow (&self,xAmount:c_int,yAmount:c_int,lpRect:LPCRECT,lpClipRect:LPCRECT) -> bool {
		self.assert_window();
		unsafe{user32::ScrollWindow(self.get_hwnd(), xAmount, yAmount, lpRect, lpClipRect) == TRUE}
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
		unsafe{user32::ScrollWindowEx(self.get_hwnd(), dx, dy, lpRectScroll, lpRectClip, hRgnUpdate, lpRectUpdate, uFlags)}
	}

	fn ScrollWindowExDefault(&self,dx:c_int,dy:c_int,uFlags:UINT)->c_int{
		self.ScrollWindowEx(dx,dy,0 as LPCRECT,0 as LPCRECT,0 as HRGN,0 as LPRECT,uFlags)
	}

	fn SetScrollPos (&self,nBar:c_int,nPos:c_int,bRedraw:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::SetScrollPos(self.get_hwnd(), nBar, nPos, bRedraw)}
	}

	fn SetScrollRange (&self,nBar:c_int,nMinPos:c_int,nMaxPos:c_int,bRedraw:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::SetScrollRange(self.get_hwnd(), nBar, nMinPos, nMaxPos, bRedraw) == TRUE}
	}

	fn ShowScrollBar (&self,nBar:c_int,bShow:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::ShowScrollBar(self.get_hwnd(), nBar, bShow) == TRUE}
	}

	fn EnableScrollBar (&self,uSBFlags:UINT,uArrowFlags:UINT) -> bool {
		self.assert_window();
		unsafe{user32::EnableScrollBar(self.get_hwnd(), uSBFlags, uArrowFlags) == TRUE}
	}



	// fn ChildWindowFromPoint (&self,point:POINT) -> CWindow {
	// 	self.assert_window();
	// 	user32::ChildWindowFromPoint(self.get_hwnd(), point)
	// }

	// fn ChildWindowFromPointEx (&self,point:POINT,uFlags:UINT) -> CWindow {
	// 	self.assert_window();
	// 	user32::ChildWindowFromPointEx(self.get_hwnd(), point, uFlags)
	// }

	// fn GetTopWindow (&self) -> CWindow {
	// 	self.assert_window();
	// 	user32::GetTopWindow(self.get_hwnd())
	// }

	// fn GetWindow (&self,nCmd:UINT) -> CWindow {
	// 	self.assert_window();
	// 	user32::GetWindow(self.get_hwnd(), nCmd)
	// }

	// fn GetLastActivePopup (&self) -> CWindow {
	// 	self.assert_window();
	// 	user32::GetLastActivePopup(self.get_hwnd())
	// }

	fn IsChild (&self,hWnd:HWND) -> bool {
		self.assert_window();
		unsafe{user32::IsChild(self.get_hwnd(), hWnd) == TRUE}
	}

	// fn GetParent (&self) -> CWindow {
	// 	self.assert_window();
	// 	user32::GetParent(self.get_hwnd())
	// }

	// fn SetParent (&self,hWndNewParent:HWND) -> CWindow {
	// 	self.assert_window();
	// 	user32::SetParent(self.get_hwnd(), hWndNewParent)
	// }



	fn GetDlgCtrlID (&self) -> c_int {
		self.assert_window();
		unsafe{user32::GetDlgCtrlID(self.get_hwnd())}
	}

	fn SetDlgCtrlID (&self,nID:c_int) -> c_int {
		self.assert_window();
		unsafe{user32::SetWindowLongW(self.get_hwnd(), GWL_ID, nID)}
	}

	fn GetDlgItem<T:CWindow> (&self,nID:c_int) -> T {
		self.assert_window();
		T::from_hwnd(unsafe{user32::GetDlgItem(self.get_hwnd(), nID)})
	}



	fn FlashWindow (&self,bInvert:BOOL) -> bool {
		self.assert_window();
		unsafe{user32::FlashWindow(self.get_hwnd(), bInvert) == TRUE}
	}

	// fn MessageBox(&self,lpszText:LPCTSTR ,lpszCaption:LPCTSTR,nType:UINT) -> c_int {
	// 	self.assert_window();
	// 	user32::MessageBox(self.get_hwnd(), lpszText, lpszCaption, nType)
	// }

	fn ChangeClipboardChain (&self,hWndNewNext:HWND) -> bool {
		self.assert_window();
		unsafe{user32::ChangeClipboardChain(self.get_hwnd(), hWndNewNext) == TRUE}
	}

	fn SetClipboardViewer (&self) -> HWND {
		self.assert_window();
		unsafe{user32::SetClipboardViewer(self.get_hwnd())}
	}

	fn OpenClipboard (&self) -> bool {
		self.assert_window();
		unsafe{user32::OpenClipboard(self.get_hwnd()) == TRUE}
	}



	fn CreateCaret (&self,hBitmap:HBITMAP) -> bool {
		self.assert_window();
		unsafe{user32::CreateCaret(self.get_hwnd(), hBitmap, 0, 0) == TRUE}
	}

	fn CreateSolidCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
		self.assert_window();
		unsafe{user32::CreateCaret(self.get_hwnd(), 0 as HBITMAP, nWidth, nHeight) == TRUE}
	}

	fn CreateGrayCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
		self.assert_window();
		unsafe{user32::CreateCaret(self.get_hwnd(), 1 as HBITMAP, nWidth, nHeight) == TRUE}
	}

	fn HideCaret (&self) -> bool {
		self.assert_window();
		unsafe{user32::HideCaret(self.get_hwnd()) == TRUE}
	}

	fn ShowCaret (&self) -> bool {
		self.assert_window();
		unsafe{user32::ShowCaret(self.get_hwnd()) == TRUE}
	}

	// fn DragAcceptFiles (&self,bAccept:BOOL)  {
	// 	self.assert_window(); 
	// 	user32::DragAcceptFiles(self.get_hwnd(), bAccept);
	// }

	fn SetIcon (&self,hIcon:HICON,bBigIcon:BOOL) -> HICON {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_SETICON, bBigIcon as WPARAM, hIcon as LPARAM) as HICON}
	}

	fn GetIcon (&self,bBigIcon:BOOL) -> HICON {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_GETICON, bBigIcon as WPARAM, 0 as LPARAM) as HICON}
	}

	// fn WinHelp (&self,lpszHelp:LPCTSTR,nCmd:UINT,dwData:DWORD) -> bool {
	// 	self.assert_window();
	// 	user32::WinHelp(self.get_hwnd(), lpszHelp, nCmd, dwData) == TRUE
	// }

	fn SetWindowContextHelpId (&self,dwContextHelpId:DWORD) -> bool {
		self.assert_window();
		unsafe{user32::SetWindowContextHelpId(self.get_hwnd(), dwContextHelpId) == TRUE}
	}

	fn GetWindowContextHelpId (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowContextHelpId(self.get_hwnd())}
	}

	fn SetHotKey (&self,wVirtualKeyCode:WORD,wModifiers:WORD) -> c_int {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_SETHOTKEY, MAKEWORD(wVirtualKeyCode as u8, wModifiers as u8) as WPARAM, 0 ) as c_int}
	}

	fn GetHotKey (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_GETHOTKEY, 0, 0) as DWORD}
	}

	fn GetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO) -> bool {
		self.assert_window();
		unsafe{user32::GetScrollInfo(self.get_hwnd(), nBar, lpScrollInfo) == TRUE}
	}
	fn SetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO,bRedraw:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::SetScrollInfo(self.get_hwnd(), nBar, lpScrollInfo, bRedraw)}
	}
	fn IsDialogMessage (&self,lpMsg:LPMSG) -> bool {
		self.assert_window();
		unsafe{user32::IsDialogMessageW(self.get_hwnd(), lpMsg) == TRUE}
	}

	fn NextDlgCtrl (&self)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_NEXTDLGCTL, 0, 0);}
	}
	fn PrevDlgCtrl (&self)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_NEXTDLGCTL, 1, 0);}
	}
	fn GotoDlgCtrl (&self,hWndCtrl:HWND)  {
		self.assert_window();
		unsafe{user32::SendMessageW(self.get_hwnd(), WM_NEXTDLGCTL, hWndCtrl as WPARAM, 1);}
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

		self.SetWindowPos(0 as HWND, 0, 0, rcWnd.right - rcWnd.left, rcWnd.bottom - rcWnd.top, uFlags)
	}

	fn GetWindowRgn (&self,hRgn:HRGN) -> c_int {
		self.assert_window();
		unsafe{user32::GetWindowRgn(self.get_hwnd(), hRgn)}
	}
	fn SetWindowRgn (&self,hRgn:HRGN,bRedraw:BOOL) -> c_int {
		self.assert_window();
		unsafe{user32::SetWindowRgn(self.get_hwnd(), hRgn, bRedraw)}
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
		unsafe{user32::DeferWindowPos(hWinPosInfo, self.get_hwnd(), hWndInsertAfter, x, y, cx, cy, uFlags)}
	}

	fn GetWindowThreadID (&self) -> DWORD {
		self.assert_window();
		unsafe{user32::GetWindowThreadProcessId(self.get_hwnd(), 0 as LPDWORD)}
	}

	fn GetWindowProcessID (&self) -> DWORD {
		self.assert_window();
		let mut dwProcessID:DWORD = 0;
		unsafe{user32::GetWindowThreadProcessId(self.get_hwnd(), &mut dwProcessID);}
		dwProcessID
	}

	fn IsWindow (&self) -> bool {
		unsafe{user32::IsWindow(self.get_hwnd()) == TRUE}
	}
	fn IsWindowUnicode (&self) -> bool {
		self.assert_window();
		unsafe{user32::IsWindowUnicode(self.get_hwnd()) == TRUE}
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
		unsafe{user32::ShowWindowAsync(self.get_hwnd(), nCmdShow) == TRUE}
	}

	// fn GetDescendantWindow (&self,nID:c_int) -> CWindow {
	// 	self.assert_window();

	// 	HWND hWndChild, hWndTmp;
	// 	if((hWndChild = ::GetDlgItem(self.get_hwnd(), nID)) != NULL)
	// 	{
	// 		if(::GetTopWindow(hWndChild) != NULL)
	// 		{
				
	// 			CWindow wnd(hWndChild);
	// 			hWndTmp = wnd.GetDescendantWindow(nID);
	// 			if(hWndTmp != NULL)
	// 				return CWindow(hWndTmp);
	// 		}
	// 		return CWindow(hWndChild);
	// 	}

		
	// 	for(hWndChild = ::GetTopWindow(self.get_hwnd()); hWndChild != NULL;
	// 		hWndChild = ::GetNextWindow(hWndChild, GW_HWNDNEXT))
	// 	{
	// 		CWindow wnd(hWndChild);
	// 		hWndTmp = wnd.GetDescendantWindow(nID);
	// 		if(hWndTmp != NULL)
	// 			return CWindow(hWndTmp);
	// 	}

	// 	return CWindow(NULL);    
	// }

	// fn SendMessageToDescendants (&self,message:UINT,wParam:WPARAM,lParam:LPARAM,bDeep:BOOL)  {
	// 	for(HWND hWndChild = ::GetTopWindow(self.get_hwnd()); hWndChild != NULL;
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
// 				hWndCenter = ::GetParent(self.get_hwnd());
// 			else
// 				hWndCenter = ::GetWindow(self.get_hwnd(), GW_OWNER);
// 		}

		
// 		RECT rcDlg;
// 		::GetWindowRect(self.get_hwnd(), &rcDlg);
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
// 				hMonitor = ::MonitorFromWindow(self.get_hwnd(), MONITOR_DEFAULTTONEAREST);
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
			
// 			hWndParent = ::GetParent(self.get_hwnd());
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

		
// 		return ::SetWindowPos(self.get_hwnd(), NULL, xLeft, yTop, -1, -1,
// 			SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
// 	}

	fn ModifyStyle (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
		self.assert_window();

		let dwStyle:DWORD = unsafe{user32::GetWindowLongW(self.get_hwnd(), GWL_STYLE) as DWORD};
		let dwNewStyle  = (dwStyle & !dwRemove) | dwAdd;
		if dwStyle == dwNewStyle{
			return false;
		}

		unsafe{user32::SetWindowLongW(self.get_hwnd(), GWL_STYLE, dwNewStyle as LONG);}
		if nFlags != 0 {
			unsafe{user32::SetWindowPos(self.get_hwnd(), 0 as HWND, 0, 0, 0, 0,
				SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE | nFlags)};
		}

		true
	}

	fn ModifyStyleEx (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
		self.assert_window();

		let dwStyle:DWORD = unsafe{user32::GetWindowLongW(self.get_hwnd(), GWL_EXSTYLE) as DWORD};
		let dwNewStyle:DWORD = (dwStyle & !dwRemove) | dwAdd;
		if dwStyle == dwNewStyle {
			return false;
		}

		unsafe{user32::SetWindowLongW(self.get_hwnd(), GWL_EXSTYLE, dwNewStyle as LONG);}
		if nFlags != 0 {
			unsafe{user32::SetWindowPos(self.get_hwnd(), 0 as HWND, 0, 0, 0, 0,
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

	// 	c_int nLen = ::GetWindowTextLength(self.get_hwnd());

	// 	CTempBuffer<TCHAR> lpszText;
	// 	if(nLen>0)
	// 	{
	// 		ATLTRY(lpszText.Allocate(nLen+1));
	// 		if (lpszText == NULL)
	// 		{
	// 			return FALSE;
	// 		}

	// 		if(!::GetWindowText(self.get_hwnd(), lpszText, nLen+1))
	// 		{
	// 			return FALSE;
	// 		}
	// 	}

	// 	bstrText = ::SysAllocString(T2OLE_EX_DEF(lpszText));

	// 	return nLen==0 ? FALSE : ((bstrText != NULL) ? TRUE : FALSE);
	// }

	// fn GetTopLevelParent (&self) -> CWindow {
	// 	self.assert_window();

	// 	let hWndParent:HWND  = self.get_hwnd();
	// 	let mut hWndTmp:HWND ;
	// 	while((hWndTmp = user32::GetParent(hWndParent)) != NULL)
	// 		hWndParent = hWndTmp;

	// 	return CWindow(hWndParent);
	// }

	// fn GetTopLevelWindow (&self) -> CWindow {
	// 	self.assert_window();

	// 	let mut hWndParent:HWND ;
	// 	let mut hWndTmp:HWND  = self.get_hwnd();

	// 	do
	// 	{
	// 		hWndParent = hWndTmp;
	// 		hWndTmp = (user32::GetWindowLong(hWndParent, GWL_STYLE) & WS_CHILD) ? ::GetParent(hWndParent) : ::GetWindow(hWndParent, GW_OWNER);
	// 	}
	// 	while(hWndTmp != NULL);

	// 	return CWindow(hWndParent);
	// }
}