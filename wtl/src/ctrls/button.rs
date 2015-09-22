
#![allow(non_snake_case,dead_code,unused_variables)]
///////////////////////////////////////////////////////////////////////////////
// CButton - client side for a Windows BUTTON control
use atl::{CWindow,NULL_HWND};
use winapi::*;
use user32::*;

pub struct Button {
    cwin: CWindow,
}

impl Button {
    pub fn new()->Button{
        Button{
            cwin: CWindow::new(NULL_HWND),
        }
    }
}
/*
(1)
^\t(\w+)\s+(\w+)\((.*)\)
=>
\tpub fn \2 \(\3\)->\1

(2)
delete 
->void
const
(3)
self.assert_window();
=>
self.assert_window();

(4)
::SendMessage(m_hWnd,
=>
self.SendMessage(

(5) parameter define
pub fn (\w+)\s*\((\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\)

(6) coercion
\(LPARAM\)(\w+)
=>
\1 as LPARAM

(7) const define
#define (\w+)\s+(\w+)
=>
const \1: UINT = \2;
*/
impl Button {
	
	// CButtonT(HWND hWnd = NULL) : TBase(hWnd)
	// { }

	// CButtonT< TBase >& operator =(HWND hWnd)
	// {
	// 	m_hWnd = hWnd;
	// 	return *this;
	// }

	// HWND Create(HWND hWndParent, ATL::_U_RECT rect = NULL, LPCTSTR szWindowName = NULL,
	// 		DWORD dwStyle = 0, DWORD dwExStyle = 0,
	// 		ATL::_U_MENUorID MenuOrID = 0U, LPVOID lpCreateParam = NULL)
	// {
	// 	return TBase::Create(GetWndClassName(), hWndParent, rect.m_lpRect, szWindowName, dwStyle, dwExStyle, MenuOrID.m_hMenu, lpCreateParam);
	// }

// Attributes
	// static LPCTSTR GetWndClassName()
	// {
	// 	return _T("BUTTON");
	// }

	pub fn GetState(&self) ->UINT {
		//self.assert_window();
		//return (UINT)self.SendMessage( BM_GETSTATE, 0, 0);
		self.assert_window();
		self.SendMessage(BM_GETSTATE, 0, 0) as UINT
	}

	pub fn SetState(&self,bHighlight: BOOL) {
		self.assert_window();
		self.SendMessage( BM_SETSTATE, bHighlight as WPARAM, 0);
	}

	pub fn GetCheck(&self)->c_int {
		self.assert_window();
		self.SendMessage( BM_GETCHECK, 0, 0) as c_int
	}

	pub fn SetCheck(&self,nCheck: c_int) {
		self.assert_window();
		self.SendMessage( BM_SETCHECK, nCheck as WPARAM, 0);
	}

	pub fn GetButtonStyle(&self)->UINT {
		self.assert_window();
		(self.GetWindowLong(GWL_STYLE) & 0xFFFF) as UINT
	}

	pub fn SetButtonStyle(&self,nStyle:UINT, bRedraw:BOOL) {
		self.assert_window();
		self.SendMessage( BM_SETSTYLE, nStyle as WPARAM, bRedraw as LPARAM);
	}

//#ifndef _WIN32_WCE
	pub fn GetIcon(&self)->HICON {
		self.assert_window();
		self.SendMessage( BM_GETIMAGE, IMAGE_ICON as WPARAM, 0) as HICON
	}

	pub fn SetIcon(&self,hIcon: HICON)->HICON {
		self.assert_window();
		self.SendMessage( BM_SETIMAGE, IMAGE_ICON as WPARAM, hIcon as LPARAM) as HICON
	}

	// pub fn GetBitmap ()->CBitmapHandle 
	// {
	// 	self.assert_window();
	// 	return CBitmapHandle((HBITMAP)self.SendMessage( BM_GETIMAGE, IMAGE_BITMAP, 0));
	// }

	// pub fn SetBitmap(&self,hBitmap: HBITMAP)->CBitmapHandle
	// {
	// 	self.assert_window();
	// 	return CBitmapHandle((HBITMAP)self.SendMessage( BM_SETIMAGE, IMAGE_BITMAP, hBitmap as LPARAM));
	// }
//#endif // !_WIN32_WCE

//#if (_WIN32_WINNT >= 0x0501)
	pub fn GetIdealSize(&self,lpSize: LPSIZE)->BOOL {
		self.assert_window();
		self.SendMessage( BCM_GETIDEALSIZE, 0, lpSize as LPARAM) as BOOL
	}

	pub fn GetImageList(&self,pButtonImagelist: PBUTTON_IMAGELIST)->BOOL {
		self.assert_window();
		self.SendMessage( BCM_GETIMAGELIST, 0, pButtonImagelist as LPARAM) as BOOL
	}

	pub fn SetImageList(&self,pButtonImagelist: PBUTTON_IMAGELIST)->BOOL {
		self.assert_window();
		self.SendMessage( BCM_SETIMAGELIST, 0, pButtonImagelist as LPARAM) as BOOL
	}

	pub fn GetTextMargin(&self,lpRect: LPRECT)->BOOL {
		self.assert_window();
		self.SendMessage( BCM_GETTEXTMARGIN, 0, lpRect as LPARAM) as BOOL
	}

	pub fn SetTextMargin(&self,lpRect: LPRECT)->BOOL {
		self.assert_window();
		self.SendMessage( BCM_SETTEXTMARGIN, 0, lpRect as LPARAM) as BOOL
	}
//#endif // (_WIN32_WINNT >= 0x0501)

//#if (WINVER >= 0x0600)
	pub fn SetDontClick(&self,bDontClick: BOOL) {
		self.assert_window();
		self.SendMessage( BM_SETDONTCLICK, bDontClick as WPARAM, 0);
	}
//#endif // (WINVER >= 0x0600)

//#if (_WIN32_WINNT >= 0x0600)
	pub fn SetDropDownState(&self,bDropDown: BOOL)->BOOL {
		self.assert_window();
		debug_assert!((self.GetStyle() & (BS_SPLITBUTTON | BS_DEFSPLITBUTTON)) != 0);
		self.SendMessage( BCM_SETDROPDOWNSTATE, bDropDown as WPARAM, 0) as BOOL
	}

	pub fn GetSplitInfo(&self,pSplitInfo: PBUTTON_SPLITINFO)->BOOL {
		self.assert_window();
		debug_assert!((self.GetStyle() & (BS_SPLITBUTTON | BS_DEFSPLITBUTTON)) != 0);
		self.SendMessage( BCM_GETSPLITINFO, 0, pSplitInfo as LPARAM) as BOOL
	}

	pub fn SetSplitInfo(&self,pSplitInfo: PBUTTON_SPLITINFO)->BOOL {
		self.assert_window();
		debug_assert!((self.GetStyle() & (BS_SPLITBUTTON | BS_DEFSPLITBUTTON)) != 0);
		self.SendMessage( BCM_SETSPLITINFO, 0, pSplitInfo as LPARAM) as BOOL
	}

	pub fn GetNoteLength (&self)->c_int {
		self.assert_window();
		debug_assert!((self.GetStyle() & (BS_COMMANDLINK | BS_DEFCOMMANDLINK)) != 0);
		self.SendMessage( BCM_GETNOTELENGTH, 0, 0) as c_int
	}

	// pub fn GetNote (LPWSTR lpstrNoteText, int cchNoteText)->BOOL {
	// 	self.assert_window();
	// 	debug_assert!((self.GetStyle() & (BS_COMMANDLINK | BS_DEFCOMMANDLINK)) != 0);
	// 	self.SendMessage( BCM_GETNOTE, cchNoteText, lpstrNoteText as LPARAM) as BOOL
	// }

	pub fn SetNote(&self,lpstrNoteText: LPCWSTR)->BOOL {
		self.assert_window();
		debug_assert!((self.GetStyle() & (BS_COMMANDLINK | BS_DEFCOMMANDLINK)) != 0);
		self.SendMessage( BCM_SETNOTE, 0, lpstrNoteText as LPARAM) as BOOL
	}

	pub fn SetElevationRequiredState(&self,bSet: BOOL)->LRESULT {
		self.assert_window();
		self.SendMessage( BCM_SETSHIELD, 0, bSet as LPARAM)
	}
//#endif // (_WIN32_WINNT >= 0x0600)

// Operations
	pub fn Click (&self) {
		self.assert_window();
		self.SendMessage( BM_CLICK, 0, 0);
	}
}

//typedef CButtonT<ATL::CWindow>   CButton;

/*
 * Button Control Messages
 */
const BM_GETCHECK: UINT = 0x00F0;
const BM_SETCHECK: UINT = 0x00F1;
const BM_GETSTATE: UINT = 0x00F2;
const BM_SETSTATE: UINT = 0x00F3;
const BM_SETSTYLE: UINT = 0x00F4;
//#if(WINVER >= 0x0400)
const BM_CLICK: UINT = 0x00F5;
const BM_GETIMAGE: UINT = 0x00F6;
const BM_SETIMAGE: UINT = 0x00F7;
//#endif /* WINVER >= 0x0400 */
//#if(WINVER >= 0x0600)
const BM_SETDONTCLICK: UINT = 0x00F8;
//#endif /* WINVER >= 0x0600 */

//#if(WINVER >= 0x0400)
const BST_UNCHECKED: UINT = 0x0000;
const BST_CHECKED: UINT = 0x0001;
const BST_INDETERMINATE: UINT = 0x0002;
const BST_PUSHED: UINT = 0x0004;
const BST_FOCUS: UINT = 0x0008;
//#endif /* WINVER >= 0x0400 */

const IMAGE_BITMAP: UINT = 0;
const IMAGE_ICON: UINT = 1;
const IMAGE_CURSOR: UINT = 2;
//#if(WINVER >= 0x0400)
const IMAGE_ENHMETAFILE: UINT = 3;


/////////////////////////////////////////////////////////
// expose all cwin methods

// currently racer not support macros,so add all functions manually
impl Button {
    #[inline(always)]
    pub fn GetHwnd(&self) -> HWND {
        self.cwin.GetHwnd()
    }

    #[inline(always)]
    pub fn Detach(&mut self)-> HWND {
        self.cwin.Detach()
    }

    #[inline(always)]
    pub fn Attach (&mut self,hWndNew:HWND){
        self.cwin.Attach(hWndNew)
    }

    #[inline(always)]
    pub fn assert_window(&self) {
        self.cwin.assert_window()
    }

    #[inline(always)]
    pub fn GetParent2 (&self) -> CWindow {
        self.cwin.GetParent2()
    }

    #[inline(always)]
    pub fn SetParent2 (&self,hWndNewParent:HWND) -> CWindow {
        self.cwin.SetParent2(hWndNewParent)
    }

    #[inline(always)]
    pub fn GetDlgItem2 (&self,nID:c_int) -> CWindow {
        self.cwin.GetDlgItem2(nID)
    }

    #[inline(always)]
    pub fn GetTopWindow (&self) -> CWindow {
        self.cwin.GetTopWindow()
    }

    #[inline(always)]
    pub fn GetWindow (&self,nCmd:UINT) -> CWindow {
        self.cwin.GetWindow(nCmd)
    }

    #[inline(always)]
    pub fn GetLastActivePopup (&self) -> CWindow {
        self.cwin.GetLastActivePopup()
    }

    #[inline(always)]
    pub fn ChildWindowFromPoint (&self,point:POINT) -> CWindow {
        self.cwin.ChildWindowFromPoint(point)
    }

    #[inline(always)]
    pub fn ChildWindowFromPointEx (&self,point:POINT,uFlags:UINT) -> CWindow {
        self.cwin.ChildWindowFromPointEx(point,uFlags)
    }

    #[inline(always)]
    pub fn GetNextDlgGroupItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
        self.cwin.GetNextDlgGroupItem(hWndCtl,bPrevious)
    }

    #[inline(always)]
    pub fn GetNextDlgTabItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
        self.cwin.GetNextDlgTabItem(hWndCtl,bPrevious)
    }

    #[inline(always)]
    pub fn GetTopLevelParent (&self) -> CWindow {
        self.cwin.GetTopLevelParent()
    }

    #[inline(always)]
    pub fn GetTopLevelWindow (&self) -> CWindow {
        self.cwin.GetTopLevelWindow()
    }

    #[inline(always)]
    pub fn GetDescendantWindow (&self,nID:c_int) -> CWindow {
        self.cwin.GetDescendantWindow(nID)
    }

    // #[inline(always)]
    // pub fn Create(lpstrWndClass:LPCWSTR ,hWndParent:HWND ,rect:&RECT,szWindowName:LPCWSTR,dwStyle:DWORD,dwExStyle:DWORD,hMenu:HMENU,lpCreateParam:LPVOID) -> HWND {

    // }


    // #[inline(always)]
    // pub fn DestroyWindow (&mut self) -> bool {
    //     self.cwin.DestroyWindow()
    // }

    #[inline(always)]
    pub fn GetStyle (&self) -> DWORD {
        self.cwin.GetStyle()
    }

    #[inline(always)]
    pub fn GetExStyle (&self) -> DWORD {
        self.cwin.GetExStyle()
    }

    #[inline(always)]
    pub fn GetWindowLong (&self,nIndex:c_int) -> LONG {
        self.cwin.GetWindowLong(nIndex)
    }

    #[inline(always)]
    pub fn GetWindowLongPtr (&self,nIndex:c_int) -> LONG_PTR {
        self.cwin.GetWindowLongPtr(nIndex)
    }

    #[inline(always)]
    pub fn SetWindowLong (&self,nIndex:c_int,dwNewLong:LONG) -> LONG {
        self.cwin.SetWindowLong(nIndex,dwNewLong)
    }

    #[inline(always)]
    pub fn SetWindowLongPtr (&self,nIndex:c_int,dwNewLong:LONG_PTR) -> LONG_PTR {
        self.cwin.SetWindowLongPtr(nIndex,dwNewLong)
    }

    #[inline(always)]
    pub fn GetWindowWord (&self,nIndex:c_int) -> WORD {
        self.cwin.GetWindowWord(nIndex)
    }

    #[inline(always)]
    pub fn SetWindowWord (&self,nIndex:c_int,wNewWord:WORD) -> WORD {
        self.cwin.SetWindowWord(nIndex,wNewWord)
    }

    #[inline(always)]
    pub fn SendMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
        self.cwin.SendMessage(message,wParam,lParam)
    }

    #[inline(always)]
    pub fn PostMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
        self.cwin.PostMessage(message,wParam,lParam)
    }

    #[inline(always)]
    pub fn SendNotifyMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
        self.cwin.SendNotifyMessage(message,wParam,lParam)
    }

    #[inline(always)]
    pub fn GetWindowTextLength (&self) -> c_int {
        self.cwin.GetWindowTextLength()
    }

    #[inline(always)]
    pub fn SetFont (&self,hFont:HFONT,bRedraw:BOOL)  {
        self.cwin.SetFont(hFont,bRedraw)
    }

    #[inline(always)]
    pub fn GetFont (&self) -> HFONT {
        self.cwin.GetFont()
    }

    #[inline(always)]
    pub fn GetMenu (&self) -> HMENU {
        self.cwin.GetMenu()
    }

    #[inline(always)]
    pub fn SetMenu (&self,hMenu:HMENU) -> bool {
        self.cwin.SetMenu(hMenu)
    }

    #[inline(always)]
    pub fn DrawMenuBar (&self) -> bool {
        self.cwin.DrawMenuBar()
    }

    #[inline(always)]
    pub fn GetSystemMenu (&self,bRevert:BOOL) -> HMENU {
        self.cwin.GetSystemMenu(bRevert)
    }

    #[inline(always)]
    pub fn HiliteMenuItem (&self,hMenu:HMENU,uItemHilite:UINT,uHilite:UINT) -> bool {
        self.cwin.HiliteMenuItem(hMenu,uItemHilite,uHilite)
    }

    #[inline(always)]
    pub fn IsIconic (&self) -> bool {
        self.cwin.IsIconic()
    }

    #[inline(always)]
    pub fn IsZoomed (&self) -> bool {
        self.cwin.IsZoomed()
    }

    #[inline(always)]
    pub fn MoveWindow(&self,x:c_int,y:c_int,nWidth:c_int,nHeight:c_int,bRepaint:BOOL) -> bool{
        self.cwin.MoveWindow(x,y,nWidth,nHeight,bRepaint)
    }

    #[inline(always)]
    pub fn MoveWindow2 (&self,lpRect:&RECT,bRepaint:BOOL) -> bool {
        self.cwin.MoveWindow2(lpRect,bRepaint)
    }

    #[inline(always)]
    pub fn SetWindowPos(&self,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,nFlags:UINT) -> bool {
        self.cwin.SetWindowPos(hWndInsertAfter,x,y,cx,cy,nFlags)
    }

    #[inline(always)]
    pub fn SetWindowPos2 (&self,hWndInsertAfter:HWND,lpRect:&RECT,nFlags:UINT) -> bool {
        self.cwin.SetWindowPos2(hWndInsertAfter,lpRect,nFlags)
    }

    #[inline(always)]
    pub fn ArrangeIconicWindows (&self) -> UINT {
        self.cwin.ArrangeIconicWindows()
    }

    #[inline(always)]
    pub fn BringWindowToTop (&self) -> bool {
        self.cwin.BringWindowToTop()
    }

    #[inline(always)]
    pub fn GetWindowRect (&self,lpRect:LPRECT) -> bool {
        self.cwin.GetWindowRect(lpRect)
    }

    #[inline(always)]
    pub fn GetClientRect (&self,lpRect:&mut RECT) -> bool {
        self.cwin.GetClientRect(lpRect)
    }

    #[inline(always)]
    pub fn GetWindowPlacement(&self,lpwndpl:&mut WINDOWPLACEMENT) -> bool {
        self.cwin.GetWindowPlacement(lpwndpl)
    }

    #[inline(always)]
    pub fn SetWindowPlacement(&self,lpwndpl:&WINDOWPLACEMENT) -> bool {
        self.cwin.SetWindowPlacement(lpwndpl)
    }

    #[inline(always)]
    pub fn ClientToScreen (&self,lpPoint:LPPOINT) -> bool {
        self.cwin.ClientToScreen(lpPoint)
    }

    #[inline(always)]
    pub fn ClientToScreen2 (&self,lpRect:&mut RECT) -> bool {
        self.cwin.ClientToScreen2(lpRect)
    }

    #[inline(always)]
    pub fn ScreenToClient (&self,lpPoint:LPPOINT) -> bool {
        self.cwin.ScreenToClient(lpPoint)
    }

    #[inline(always)]
    pub fn ScreenToClient2 (&self,lpRect:&mut RECT) -> bool {
        self.cwin.ScreenToClient2(lpRect)
    }

    #[inline(always)]
    pub fn MapWindowPoints (&self,hWndTo:HWND,lpPoint:LPPOINT,nCount:UINT) -> c_int {
        self.cwin.MapWindowPoints(hWndTo,lpPoint,nCount)
    }

    #[inline(always)]
    pub fn MapWindowPoints2 (&self,hWndTo:HWND,lpRect:LPRECT) -> c_int {
        self.cwin.MapWindowPoints2(hWndTo,lpRect)
    }

    #[inline(always)]
    pub fn BeginPaint (&self,lpPaint:LPPAINTSTRUCT) -> HDC {
        self.cwin.BeginPaint(lpPaint)
    }

    #[inline(always)]
    pub fn EndPaint (&self,lpPaint:LPPAINTSTRUCT)  {
        self.cwin.EndPaint(lpPaint)
    }

    #[inline(always)]
    pub fn GetDC (&self) -> HDC {
        self.cwin.GetDC()
    }

    #[inline(always)]
    pub fn GetWindowDC (&self) -> HDC {
        self.cwin.GetWindowDC()
    }

    #[inline(always)]
    pub fn ReleaseDC (&self,hDC:HDC) -> c_int {
        self.cwin.ReleaseDC(hDC)
    }

    #[inline(always)]
    pub fn Print (&self,hDC:HDC,dwFlags:DWORD)  {
        self.cwin.Print(hDC,dwFlags)
    }

    #[inline(always)]
    pub fn PrintClient (&self,hDC:HDC,dwFlags:DWORD)  {
        self.cwin.PrintClient(hDC,dwFlags)
    }

    #[inline(always)]
    pub fn UpdateWindow (&self) -> bool {
        self.cwin.UpdateWindow()
    }

    #[inline(always)]
    pub fn SetRedraw (&self,bRedraw:BOOL)  {
        self.cwin.SetRedraw(bRedraw)
    }

    #[inline(always)]
    pub fn GetUpdateRect (&self,lpRect:LPRECT,bErase:BOOL) -> bool {
        self.cwin.GetUpdateRect(lpRect,bErase)
    }

    #[inline(always)]
    pub fn GetUpdateRgn (&self,hRgn:HRGN,bErase:BOOL) -> c_int {
        self.cwin.GetUpdateRgn(hRgn,bErase)
    }

    #[inline(always)]
    pub fn Invalidate (&self,bErase:BOOL) -> bool {
        self.cwin.Invalidate(bErase)
    }

    #[inline(always)]
    pub fn Invalidate2 (&self,lpRect:LPCRECT,bErase:BOOL) -> bool {
        self.cwin.Invalidate2(lpRect,bErase)
    }

    #[inline(always)]
    pub fn ValidateRect (&self,lpRect:LPCRECT) -> bool {
        self.cwin.ValidateRect(lpRect)
    }

    #[inline(always)]
    pub fn InvalidateRgn (&self,hRgn:HRGN,bErase:BOOL)  {
        self.cwin.InvalidateRgn(hRgn,bErase)
    }

    #[inline(always)]
    pub fn ValidateRgn (&self,hRgn:HRGN) -> bool {
        self.cwin.ValidateRgn(hRgn)
    }

    #[inline(always)]
    pub fn ShowWindow (&self,nCmdShow:c_int) -> bool {
        self.cwin.ShowWindow(nCmdShow)
    }

    #[inline(always)]
    pub fn IsWindowVisible (&self) -> bool {
        self.cwin.IsWindowVisible()
    }

    #[inline(always)]
    pub fn ShowOwnedPopups (&self,bShow:BOOL) -> bool {
        self.cwin.ShowOwnedPopups(bShow)
    }

    #[inline(always)]
    pub fn GetDCEx (&self,hRgnClip:HRGN,flags:DWORD) -> HDC {
        self.cwin.GetDCEx(hRgnClip,flags)
    }

    #[inline(always)]
    pub fn LockWindowUpdate (&self,bLock:bool) -> bool {
        self.cwin.LockWindowUpdate(bLock)
    }

    #[inline(always)]
    pub fn RedrawWindow2(&self) -> bool{
        self.cwin.RedrawWindow2()
    }

    #[inline(always)]
    pub fn RedrawWindow(&self,lpRectUpdate:LPCRECT,hRgnUpdate:HRGN,flags:UINT)->bool{
        self.cwin.RedrawWindow(lpRectUpdate,hRgnUpdate,flags)
    }

    #[inline(always)]
    pub fn SetTimer(&self,nIDEvent:UINT_PTR,nElapse:UINT) -> UINT_PTR {
        self.cwin.SetTimer(nIDEvent,nElapse)
    }

    #[inline(always)]
    pub fn SetTimer2(&self,nIDEvent:UINT_PTR,nElapse:UINT,lpfnTimer:TimerProc)->UINT_PTR{
        self.cwin.SetTimer2(nIDEvent,nElapse,lpfnTimer)
    }

    #[inline(always)]
    pub fn KillTimer (&self,nIDEvent:UINT_PTR) -> bool {
        self.cwin.KillTimer(nIDEvent)
    }

    #[inline(always)]
    pub fn IsWindowEnabled (&self) -> bool {
        self.cwin.IsWindowEnabled()
    }

    #[inline(always)]
    pub fn EnableWindow (&self,bEnable:BOOL) -> bool {
        self.cwin.EnableWindow(bEnable)
    }

    #[inline(always)]
    pub fn SetActiveWindow (&self) -> HWND {
        self.cwin.SetActiveWindow()
    }

    #[inline(always)]
    pub fn SetCapture (&self) -> HWND {
        self.cwin.SetCapture()
    }

    #[inline(always)]
    pub fn SetFocus (&self) -> HWND {
        self.cwin.SetFocus()
    }

    #[inline(always)]
    pub fn CheckDlgButton (&self,nIDButton:c_int,nCheck:UINT) -> bool {
        self.cwin.CheckDlgButton(nIDButton,nCheck)
    }

    #[inline(always)]
    pub fn CheckRadioButton (&self,nIDFirstButton:c_int,nIDLastButton:c_int,nIDCheckButton:c_int) -> bool {
        self.cwin.CheckRadioButton(nIDFirstButton,nIDLastButton,nIDCheckButton)
    }

    #[inline(always)]
    pub fn GetDlgItemInt(&self,nID:c_int) -> UINT {
        self.cwin.GetDlgItemInt(nID)
    }

    #[inline(always)]
    pub fn GetDlgItemInt2(&self,nID:c_int,lpTrans:&mut BOOL,bSigned:BOOL) -> UINT {
        self.cwin.GetDlgItemInt2(nID,lpTrans,bSigned)
    }

    #[inline(always)]
    pub fn IsDlgButtonChecked (&self,nIDButton:c_int) -> UINT {
        self.cwin.IsDlgButtonChecked(nIDButton)
    }

    #[inline(always)]
    pub fn SendDlgItemMessage (&self,nID:c_int,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
        self.cwin.SendDlgItemMessage(nID,message,wParam,lParam)
    }

    #[inline(always)]
    pub fn SetDlgItemInt (&self,nID:c_int,nValue:UINT,bSigned:BOOL) -> bool {
        self.cwin.SetDlgItemInt(nID,nValue,bSigned)
    }

    #[inline(always)]
    pub fn GetScrollPos (&self,nBar:c_int) -> c_int {
        self.cwin.GetScrollPos(nBar)
    }

    #[inline(always)]
    pub fn GetScrollRange (&self,nBar:c_int,lpMinPos:LPINT,lpMaxPos:LPINT) -> bool {
        self.cwin.GetScrollRange(nBar,lpMinPos,lpMaxPos)
    }

    #[inline(always)]
    pub fn ScrollWindow (&self,xAmount:c_int,yAmount:c_int,lpRect:LPCRECT,lpClipRect:LPCRECT) -> bool {
        self.cwin.ScrollWindow(xAmount,yAmount,lpRect,lpClipRect)
    }

    #[inline(always)]
    pub fn ScrollWindowEx(&self,dx:c_int,dy:c_int,lpRectScroll:LPCRECT ,lpRectClip:LPCRECT ,hRgnUpdate:HRGN ,lpRectUpdate:LPRECT ,uFlags:UINT ) -> c_int {
        self.cwin.ScrollWindowEx(dx,dy,lpRectScroll ,lpRectClip ,hRgnUpdate ,lpRectUpdate ,uFlags )
    }

    #[inline(always)]
    pub fn ScrollWindowExDefault(&self,dx:c_int,dy:c_int,uFlags:UINT)->c_int{
        self.cwin.ScrollWindowExDefault(dx,dy,uFlags)
    }

    #[inline(always)]
    pub fn SetScrollPos (&self,nBar:c_int,nPos:c_int,bRedraw:BOOL) -> c_int {
        self.cwin.SetScrollPos(nBar,nPos,bRedraw)
    }

    #[inline(always)]
    pub fn SetScrollRange (&self,nBar:c_int,nMinPos:c_int,nMaxPos:c_int,bRedraw:BOOL) -> bool {
        self.cwin.SetScrollRange(nBar,nMinPos,nMaxPos,bRedraw)
    }

    #[inline(always)]
    pub fn ShowScrollBar (&self,nBar:c_int,bShow:BOOL) -> bool {
        self.cwin.ShowScrollBar(nBar,bShow)
    }

    #[inline(always)]
    pub fn EnableScrollBar (&self,uSBFlags:UINT,uArrowFlags:UINT) -> bool {
        self.cwin.EnableScrollBar(uSBFlags,uArrowFlags)
    }

    #[inline(always)]
    pub fn IsChild (&self,hWnd:HWND) -> bool {
        self.cwin.IsChild(hWnd)
    }

    #[inline(always)]
    pub fn GetDlgCtrlID (&self) -> c_int {
        self.cwin.GetDlgCtrlID()
    }

    #[inline(always)]
    pub fn SetDlgCtrlID (&self,nID:c_int) -> c_int {
        self.cwin.SetDlgCtrlID(nID)
    }

    #[inline(always)]
    pub fn FlashWindow (&self,bInvert:BOOL) -> bool {
        self.cwin.FlashWindow(bInvert)
    }

    #[inline(always)]
    pub fn ChangeClipboardChain (&self,hWndNewNext:HWND) -> bool {
        self.cwin.ChangeClipboardChain(hWndNewNext)
    }

    #[inline(always)]
    pub fn SetClipboardViewer (&self) -> HWND {
        self.cwin.SetClipboardViewer()
    }

    #[inline(always)]
    pub fn OpenClipboard (&self) -> bool {
        self.cwin.OpenClipboard()
    }

    #[inline(always)]
    pub fn CreateCaret (&self,hBitmap:HBITMAP) -> bool {
        self.cwin.CreateCaret(hBitmap)
    }

    #[inline(always)]
    pub fn CreateSolidCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
        self.cwin.CreateSolidCaret(nWidth,nHeight)
    }

    #[inline(always)]
    pub fn CreateGrayCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
        self.cwin.CreateGrayCaret(nWidth,nHeight)
    }

    #[inline(always)]
    pub fn HideCaret (&self) -> bool {
        self.cwin.HideCaret()
    }

    #[inline(always)]
    pub fn ShowCaret (&self) -> bool {
        self.cwin.ShowCaret()
    }

    #[inline(always)]
    pub fn DragAcceptFiles (&self,bAccept:BOOL)  {
        self.cwin.DragAcceptFiles(bAccept)
    }

    #[inline(always)]
    pub fn SetIcon (&self,hIcon:HICON,bBigIcon:BOOL) -> HICON {
        self.cwin.SetIcon(hIcon,bBigIcon)
    }

    #[inline(always)]
    pub fn GetIcon (&self,bBigIcon:BOOL) -> HICON {
        self.cwin.GetIcon(bBigIcon)
    }

    #[inline(always)]
    pub fn SetWindowContextHelpId (&self,dwContextHelpId:DWORD) -> bool {
        self.cwin.SetWindowContextHelpId(dwContextHelpId)
    }

    #[inline(always)]
    pub fn GetWindowContextHelpId (&self) -> DWORD {
        self.cwin.GetWindowContextHelpId()
    }

    #[inline(always)]
    pub fn SetHotKey (&self,wVirtualKeyCode:WORD,wModifiers:WORD) -> c_int {
        self.cwin.SetHotKey(wVirtualKeyCode,wModifiers)
    }

    #[inline(always)]
    pub fn GetHotKey (&self) -> DWORD {
        self.cwin.GetHotKey()
    }

    #[inline(always)]
    pub fn GetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO) -> bool {
        self.cwin.GetScrollInfo(nBar,lpScrollInfo)
    }

    #[inline(always)]
    pub fn SetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO,bRedraw:BOOL) -> c_int {
        self.cwin.SetScrollInfo(nBar,lpScrollInfo,bRedraw)
    }

    #[inline(always)]
    pub fn IsDialogMessage (&self,lpMsg:LPMSG) -> bool {
        self.cwin.IsDialogMessage(lpMsg)
    }

    #[inline(always)]
    pub fn NextDlgCtrl (&self)  {
        self.cwin.NextDlgCtrl()
    }

    #[inline(always)]
    pub fn PrevDlgCtrl (&self)  {
        self.cwin.PrevDlgCtrl()
    }

    #[inline(always)]
    pub fn GotoDlgCtrl (&self,hWndCtrl:HWND)  {
        self.cwin.GotoDlgCtrl(hWndCtrl)
    }

    #[inline(always)]
    pub fn ResizeClient (&self,nWidth:c_int,nHeight:c_int,bRedraw:BOOL) -> bool {
        self.cwin.ResizeClient(nWidth,nHeight,bRedraw)
    }

    #[inline(always)]
    pub fn GetWindowRgn (&self,hRgn:HRGN) -> c_int {
        self.cwin.GetWindowRgn(hRgn)
    }

    #[inline(always)]
    pub fn SetWindowRgn (&self,hRgn:HRGN,bRedraw:BOOL) -> c_int {
        self.cwin.SetWindowRgn(hRgn,bRedraw)
    }

    #[inline(always)]
    pub fn DeferWindowPos(&self,hWinPosInfo:HDWP,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,uFlags:UINT) -> HDWP {
        self.cwin.DeferWindowPos(hWinPosInfo,hWndInsertAfter,x,y,cx,cy,uFlags)
    }

    #[inline(always)]
    pub fn GetWindowThreadID (&self) -> DWORD {
        self.cwin.GetWindowThreadID()
    }

    #[inline(always)]
    pub fn GetWindowProcessID (&self) -> DWORD {
        self.cwin.GetWindowProcessID()
    }

    #[inline(always)]
    pub fn IsWindow (&self) -> bool {
        self.cwin.IsWindow()
    }

    #[inline(always)]
    pub fn IsWindowUnicode (&self) -> bool {
        self.cwin.IsWindowUnicode()
    }

    #[inline(always)]
    pub fn ShowWindowAsync (&self,nCmdShow:c_int) -> bool {
        self.cwin.ShowWindowAsync(nCmdShow)
    }

    #[inline(always)]
    pub fn CenterWindow (&self,hCenter:HWND) -> BOOL {
        self.cwin.CenterWindow(hCenter)
    }

    #[inline(always)]
    pub fn ModifyStyle (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
        self.cwin.ModifyStyle(dwRemove,dwAdd,nFlags)
    }

    #[inline(always)]
    pub fn ModifyStyleEx (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
        self.cwin.ModifyStyleEx(dwRemove,dwAdd,nFlags)
    }
}
