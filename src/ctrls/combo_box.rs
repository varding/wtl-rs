#![allow(non_snake_case,dead_code,unused_variables)]

use atl::{CWindow,NULL_HWND};
use winapi::*;
//use user32::*;
use super::consts::*;
use super::types::*;

pub struct CComboBox {
    cwin: CWindow,
}

impl CComboBox {
    pub fn new()->CComboBox{
        CComboBox{
            cwin: CWindow::new(NULL_HWND),
        }
    }
}

/*
(1)
delete const
) const
=>
)

convert fn format
^\t(\w+)\s+(\w+)\((.*)\)\s+\{
=>
\tpub fn \2 \(\3\)->\1 {

(2)
delete 
) const
->void

(3)
ATLASSERT(::IsWindow(m_hWnd));
=>
self.assert_window();

(4)
::SendMessage(m_hWnd,
=>
self.SendMessage(

(5) parameter define

// no parameter
pub fn (\w+)\s*\(\)
=>
pub fn \1\(&self)

//one parameter
pub fn (\w+)\s*\((\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\)
// two parameter
pub fn (\w+)\s*\((\w+) (\w+), (\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\, \5: \4)

(6) coercion
\(LPARAM\)(\w+)
=>
\1 as LPARAM

\(WPARAM\)(\w+)
=>
\1 as WPARAM

(7)  define
#define (\w+)\s+(\w+)
=>
\1: UINT = \2;

(8) return convert
return \((\w+)\)(.*);
=>
\2 as \1
*/



impl CComboBox{
// Constructors
// 	CComboBoxT(HWND hWnd = NULL) : TBase(hWnd)
// 	{ }

// 	CComboBoxT< TBase >& operator =(HWND hWnd)
// 	{
// 		m_hWnd = hWnd;
// 		return *this;
// 	}

// 	HWND Create(HWND hWndParent, ATL::_U_RECT rect = NULL, LPCTSTR szWindowName = NULL,
// 			DWORD dwStyle = 0, DWORD dwExStyle = 0,
// 			ATL::_U_MENUorID MenuOrID = 0U, LPVOID lpCreateParam = NULL)
// 	{
// 		return TBase::Create(GetWndClassName(), hWndParent, rect.m_lpRect, szWindowName, dwStyle, dwExStyle, MenuOrID.m_hMenu, lpCreateParam);
// 	}

// // Attributes
// 	static LPCTSTR GetWndClassName()
// 	{
// 		return _T("COMBOBOX");
// 	}

	// for entire combo box
	pub fn GetCount(&self)->c_int {
		self.assert_window();
		self.SendMessage( CB_GETCOUNT, 0, 0) as c_int
	}

	pub fn GetCurSel(&self)->c_int {
		self.assert_window();
		self.SendMessage( CB_GETCURSEL, 0, 0) as c_int
	}

	pub fn SetCurSel(&self,nSelect: c_int)->c_int {
		self.assert_window();
		self.SendMessage( CB_SETCURSEL, nSelect as WPARAM, 0) as c_int
	}

	pub fn GetLocale(&self)->LCID {
		self.assert_window();
		self.SendMessage( CB_GETLOCALE, 0, 0) as LCID
	}

	pub fn SetLocale(&self,nNewLocale: LCID)->LCID {
		self.assert_window();
		self.SendMessage( CB_SETLOCALE, nNewLocale as WPARAM, 0) as LCID
	}

	pub fn GetTopIndex(&self)->c_int {
		self.assert_window();
		self.SendMessage( CB_GETTOPINDEX, 0, 0) as c_int
	}

	pub fn SetTopIndex(&self,nIndex: c_int)->c_int {
		self.assert_window();
		self.SendMessage( CB_SETTOPINDEX, nIndex as WPARAM, 0) as c_int
	}

	pub fn GetHorizontalExtent(&self)->UINT {
		self.assert_window();
		self.SendMessage( CB_GETHORIZONTALEXTENT, 0, 0) as UINT
	}

	pub fn SetHorizontalExtent(&self,nExtent: UINT) {
		self.assert_window();
		self.SendMessage( CB_SETHORIZONTALEXTENT, nExtent as WPARAM, 0);
	}

	pub fn GetDroppedWidth(&self)->c_int {
		self.assert_window();
		self.SendMessage( CB_GETDROPPEDWIDTH, 0, 0) as c_int
	}

	pub fn SetDroppedWidth(&self,nWidth: UINT)->c_int {
		self.assert_window();
		self.SendMessage( CB_SETDROPPEDWIDTH, nWidth as WPARAM, 0) as c_int
	}

//#if ((WINVER >= 0x0500) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
	pub fn GetComboBoxInfo(&self,pComboBoxInfo: PCOMBOBOXINFO)->BOOL {
		self.assert_window();
//#if ((_WIN32_WINNT >= 0x0501) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
		self.SendMessage( CB_GETCOMBOBOXINFO, 0, pComboBoxInfo as LPARAM) as BOOL
//#else // !((_WIN32_WINNT >= 0x0501) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
//		return ::GetComboBoxInfo(m_hWnd, pComboBoxInfo);
//#endif // !((_WIN32_WINNT >= 0x0501) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
	}
//#endif // ((WINVER >= 0x0500) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))

	// for edit control
	pub fn GetEditSel(&self)->DWORD {
		self.assert_window();
		self.SendMessage( CB_GETEDITSEL, 0, 0) as DWORD
	}

	pub fn SetEditSel(&self,nStartChar: c_int, nEndChar: c_int)->BOOL {
		self.assert_window();
		self.SendMessage( CB_SETEDITSEL, 0, MAKELONG(nStartChar as WORD, nEndChar as WORD) as LPARAM) as BOOL
	}

	// for combobox item
	pub fn GetItemData(&self,nIndex: c_int)->DWORD_PTR {
		self.assert_window();
		self.SendMessage( CB_GETITEMDATA, nIndex as WPARAM, 0) as DWORD_PTR
	}

	pub fn SetItemData(&self,nIndex: c_int, dwItemData: DWORD_PTR)->c_int {
		self.assert_window();
		self.SendMessage( CB_SETITEMDATA, nIndex as WPARAM, dwItemData as LPARAM) as c_int
	}

	pub fn GetItemDataPtr(&self,nIndex: c_int)->*const c_void {
		self.assert_window();
		self.GetItemData(nIndex) as *const c_void
	}

	pub fn SetItemDataPtr (&self, nIndex: c_int, pData: *const c_void)->c_int {
		self.assert_window();
		self.SetItemData(nIndex, pData as DWORD_PTR)
	}

	// pub fn GetLBText(&self,nIndex: c_int, lpszText: LPTSTR)->c_int {
	// 	self.assert_window();
	// 	self.SendMessage( CB_GETLBTEXT, nIndex, lpszText as LPARAM) as c_int
	// }

// #ifndef _ATL_NO_COM
// 	pub fn GetLBTextBSTR (c_int nIndex, BSTR& bstrText)->BOOL {
// 		USES_CONVERSION;
// 		self.assert_window();
// 		ATLASSERT(bstrText == NULL);

// 		c_int nLen = GetLBTextLen(nIndex);
// 		if(nLen == CB_ERR)
// 			return FALSE;

// 		CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
// 		LPTSTR lpstrText = buff.Allocate(nLen + 1);
// 		if(lpstrText == NULL)
// 			return FALSE;

// 		if(GetLBText(nIndex, lpstrText) == CB_ERR)
// 			return FALSE;

// 		bstrText = ::SysAllocString(T2OLE(lpstrText));
// 		return (bstrText != NULL) ? TRUE : FALSE;
// 	}
// #endif // !_ATL_NO_COM

// #if defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)
// 	pub fn GetLBText (c_int nIndex, _CSTRING_NS::CString& strText)->c_int {
// 		self.assert_window();
// 		c_int cchLen = GetLBTextLen(nIndex);
// 		if(cchLen == CB_ERR)
// 			return CB_ERR;
// 		c_int nRet = CB_ERR;
// 		LPTSTR lpstr = strText.GetBufferSetLength(cchLen);
// 		if(lpstr != NULL)
// 		{
// 			nRet = GetLBText(nIndex, lpstr);
// 			strText.ReleaseBuffer();
// 		}
// 		return nRet;
// 	}
// #endif // defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)

	pub fn GetLBTextLen(&self,nIndex: c_int)->c_int {
		self.assert_window();
		self.SendMessage( CB_GETLBTEXTLEN, nIndex as WPARAM, 0) as c_int
	}

	pub fn GetItemHeight(&self,nIndex: c_int)->c_int {
		self.assert_window();
		self.SendMessage( CB_GETITEMHEIGHT, nIndex as WPARAM, 0) as c_int
	}

	pub fn SetItemHeight(&self,nIndex: c_int, cyItemHeight: UINT)->c_int {
		self.assert_window();
		self.SendMessage( CB_SETITEMHEIGHT, nIndex as WPARAM, MAKELONG(cyItemHeight as WORD, 0)  as LPARAM) as c_int
	}

	pub fn GetExtendedUI(&self)->BOOL {
		self.assert_window();
		self.SendMessage( CB_GETEXTENDEDUI, 0, 0) as BOOL
	}

	pub fn SetExtendedUI (&self,bExtended: BOOL)->c_int {
		self.assert_window();
		self.SendMessage( CB_SETEXTENDEDUI, bExtended as WPARAM, 0) as c_int
	}

	pub fn GetDroppedControlRect(&self,lprect: LPRECT) {
		self.assert_window();
		self.SendMessage( CB_GETDROPPEDCONTROLRECT, 0, lprect as LPARAM);
	}

	pub fn GetDroppedState(&self)->BOOL {
		self.assert_window();
		self.SendMessage( CB_GETDROPPEDSTATE, 0, 0) as BOOL
	}

//#if (_WIN32_WINNT >= 0x0501)
	pub fn GetMinVisible(&self)->c_int {
		self.assert_window();
		self.SendMessage( CB_GETMINVISIBLE, 0, 0) as c_int
	}

	pub fn SetMinVisible(&self,nMinVisible: c_int)->BOOL {
		self.assert_window();
		self.SendMessage( CB_SETMINVISIBLE, nMinVisible as WPARAM, 0) as BOOL
	}

	// Vista only
	pub fn GetCueBannerText(&self,lpwText: LPWSTR, cchText: c_int)->BOOL {
//#ifndef CB_GETCUEBANNER
		const CB_GETCUEBANNER:UINT = (CBM_FIRST + 4);
//#endif
		self.assert_window();
		self.SendMessage( CB_GETCUEBANNER, lpwText as WPARAM, cchText as LPARAM) as BOOL
	}

	// Vista only
	pub fn SetCueBannerText(&self,lpcwText: LPCWSTR)->BOOL {
//#ifndef CB_SETCUEBANNER
		const CB_SETCUEBANNER:UINT = (CBM_FIRST + 3);
//#endif
		self.assert_window();
		self.SendMessage( CB_SETCUEBANNER, 0, lpcwText as LPARAM) as BOOL
	}
//#endif // (_WIN32_WINNT >= 0x0501)

// Operations
	pub fn InitStorage(&self,nItems: c_int, nBytes: UINT)->c_int {
		self.assert_window();
		self.SendMessage( CB_INITSTORAGE, nItems as WPARAM, nBytes as LPARAM) as c_int
	}

	pub fn ResetContent(&self) {
		self.assert_window();
		self.SendMessage( CB_RESETCONTENT, 0, 0);
	}

	// for edit control
	pub fn LimitText(&self,nMaxChars: c_int)->BOOL {
		self.assert_window();
		self.SendMessage( CB_LIMITTEXT, nMaxChars as WPARAM, 0) as BOOL
	}

	// for drop-down combo boxes
	pub fn ShowDropDown (&self, bShowIt: BOOL) {
		self.assert_window();
		self.SendMessage( CB_SHOWDROPDOWN, bShowIt as WPARAM, 0);
	}

	// manipulating listbox items
	// pub fn AddString(&self,lpszString: LPCTSTR)->c_int {
	// 	self.assert_window();
	// 	self.SendMessage( CB_ADDSTRING, 0, lpszString as LPARAM) as c_int
	// }

	pub fn DeleteString(&self,nIndex: UINT)->c_int {
		self.assert_window();
		self.SendMessage( CB_DELETESTRING, nIndex as WPARAM, 0) as c_int
	}

	// pub fn InsertString(&self,nIndex: c_int, lpszString: LPCTSTR)->c_int {
	// 	self.assert_window();
	// 	self.SendMessage( CB_INSERTSTRING, nIndex, lpszString as LPARAM) as c_int
	// }

//#ifndef _WIN32_WCE
	// pub fn Dir(&self,attr: UINT, lpszWildCard: LPCTSTR)->c_int {
	// 	self.assert_window();
	// 	self.SendMessage( CB_DIR, attr, lpszWildCard as LPARAM) as c_int
	// }
//#endif // !_WIN32_WCE

	// selection helpers
	// pub fn FindString(&self,nStartAfter: c_int, lpszString: LPCTSTR)->c_int {
	// 	self.assert_window();
	// 	self.SendMessage( CB_FINDSTRING, nStartAfter, lpszString as LPARAM) as c_int
	// }

	// pub fn FindStringExact(&self,nIndexStart: c_int, lpszFind: LPCTSTR)->c_int {
	// 	self.assert_window();
	// 	self.SendMessage( CB_FINDSTRINGEXACT, nIndexStart, lpszFind as LPARAM) as c_int
	// }

	// pub fn SelectString(&self,nStartAfter: c_int, lpszString: LPCTSTR)->c_int {
	// 	self.assert_window();
	// 	self.SendMessage( CB_SELECTSTRING, nStartAfter, lpszString as LPARAM) as c_int
	// }

	// Clipboard operations
	pub fn Clear(&self) {
		self.assert_window();
		self.SendMessage( WM_CLEAR, 0, 0);
	}

	pub fn Copy(&self) {
		self.assert_window();
		self.SendMessage( WM_COPY, 0, 0);
	}

	pub fn Cut(&self) {
		self.assert_window();
		self.SendMessage( WM_CUT, 0, 0);
	}

	pub fn Paste(&self) {
		self.assert_window();
		self.SendMessage( WM_PASTE, 0, 0);
	}
}

//typedef CComboBoxT<ATL::CWindow>   CComboBox;


/////////////////////////////////////////////////////////
// expose all cwin methods

// currently racer not support macros,so add all functions manually
impl CComboBox {
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
    pub fn GetDlgItem2 (&self,nID:WORD) -> CWindow {
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

