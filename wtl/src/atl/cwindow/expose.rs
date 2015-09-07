

#[macro_export]
macro_rules! expose_cwindow {
	($typ:ident) => {
		impl $typ {
			pub fn GetHwnd(&self) -> HWND {
				self.cwin.GetHwnd()
			}

			pub fn Detach(&mut self)-> HWND {
				self.cwin.Detach()
			}

			pub fn Attach (&mut self,hWndNew:HWND){
				self.cwin.Attach(hWndNew)
			}

			pub fn assert_window(&self) {
				self.cwin.assert_window()
			}

			pub fn GetParent2 (&self) -> CWindow {
				self.cwin.GetParent2()
			}

			pub fn SetParent2 (&self,hWndNewParent:HWND) -> CWindow {
				self.cwin.SetParent2(hWndNewParent)
			}

			pub fn GetDlgItem2 (&self,nID:c_int) -> CWindow {
				self.cwin.GetDlgItem2(nID)
			}

			pub fn GetTopWindow (&self) -> CWindow {
				self.cwin.GetTopWindow()
			}

			pub fn GetWindow (&self,nCmd:UINT) -> CWindow {
				self.cwin.GetWindow(nCmd)
			}

			pub fn GetLastActivePopup (&self) -> CWindow {
				self.cwin.GetLastActivePopup()
			}

			pub fn ChildWindowFromPoint (&self,point:POINT) -> CWindow {
				self.cwin.ChildWindowFromPoint(point)
			}

			pub fn ChildWindowFromPointEx (&self,point:POINT,uFlags:UINT) -> CWindow {
				self.cwin.ChildWindowFromPointEx(point,uFlags)
			}

			pub fn GetNextDlgGroupItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
				self.cwin.GetNextDlgGroupItem(hWndCtl,bPrevious)
			}

			pub fn GetNextDlgTabItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
				self.cwin.GetNextDlgTabItem(hWndCtl,bPrevious)
			}

			pub fn GetTopLevelParent (&self) -> CWindow {
				self.cwin.GetTopLevelParent()
			}

			pub fn GetTopLevelWindow (&self) -> CWindow {
				self.cwin.GetTopLevelWindow()
			}

			pub fn GetDescendantWindow (&self,nID:c_int) -> CWindow {
				self.cwin.GetDescendantWindow(nID)
			}

			// pub fn Create(lpstrWndClass:LPCWSTR ,hWndParent:HWND ,rect:&RECT,szWindowName:LPCWSTR,dwStyle:DWORD,dwExStyle:DWORD,hMenu:HMENU,lpCreateParam:LPVOID) -> HWND {

			// }


			pub fn DestroyWindow (&mut self) -> bool {
				self.cwin.DestroyWindow()
			}

			pub fn GetStyle (&self) -> DWORD {
				self.cwin.GetStyle()
			}

			pub fn GetExStyle (&self) -> DWORD {
				self.cwin.GetExStyle()
			}

			pub fn GetWindowLong (&self,nIndex:c_int) -> LONG {
				self.cwin.GetWindowLong(nIndex)
			}

			pub fn GetWindowLongPtr (&self,nIndex:c_int) -> LONG_PTR {
				self.cwin.GetWindowLongPtr(nIndex)
			}

			pub fn SetWindowLong (&self,nIndex:c_int,dwNewLong:LONG) -> LONG {
				self.cwin.SetWindowLong(nIndex,dwNewLong)
			}

			pub fn SetWindowLongPtr (&self,nIndex:c_int,dwNewLong:LONG_PTR) -> LONG_PTR {
				self.cwin.SetWindowLongPtr(nIndex,dwNewLong)
			}

			pub fn GetWindowWord (&self,nIndex:c_int) -> WORD {
				self.cwin.GetWindowWord(nIndex)
			}

			pub fn SetWindowWord (&self,nIndex:c_int,wNewWord:WORD) -> WORD {
				self.cwin.SetWindowWord(nIndex,wNewWord)
			}

			pub fn SendMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
				self.cwin.SendMessage(message,wParam,lParam)
			}

			pub fn PostMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
				self.cwin.PostMessage(message,wParam,lParam)
			}

			pub fn SendNotifyMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
				self.cwin.SendNotifyMessage(message,wParam,lParam)
			}

			pub fn GetWindowTextLength (&self) -> c_int {
				self.cwin.GetWindowTextLength()
			}

			pub fn SetFont (&self,hFont:HFONT,bRedraw:BOOL)  {
				self.cwin.SetFont(hFont,bRedraw)
			}

			pub fn GetFont (&self) -> HFONT {
				self.cwin.GetFont()
			}

			pub fn GetMenu (&self) -> HMENU {
				self.cwin.GetMenu()
			}

			pub fn SetMenu (&self,hMenu:HMENU) -> bool {
				self.cwin.SetMenu(hMenu)
			}

			pub fn DrawMenuBar (&self) -> bool {
				self.cwin.DrawMenuBar()
			}

			pub fn GetSystemMenu (&self,bRevert:BOOL) -> HMENU {
				self.cwin.GetSystemMenu(bRevert)
			}

			pub fn HiliteMenuItem (&self,hMenu:HMENU,uItemHilite:UINT,uHilite:UINT) -> bool {
				self.cwin.HiliteMenuItem(hMenu,uItemHilite,uHilite)
			}

			pub fn IsIconic (&self) -> bool {
				self.cwin.IsIconic()
			}

			pub fn IsZoomed (&self) -> bool {
				self.cwin.IsZoomed()
			}

			pub fn MoveWindow(&self,x:c_int,y:c_int,nWidth:c_int,nHeight:c_int,bRepaint:BOOL) -> bool{
				self.cwin.MoveWindow(x,y,nWidth,nHeight,bRepaint)
			}

			pub fn MoveWindow2 (&self,lpRect:&RECT,bRepaint:BOOL) -> bool {
				self.cwin.MoveWindow2(lpRect,bRepaint)
			}

			pub fn SetWindowPos(&self,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,nFlags:UINT) -> bool {
				self.cwin.SetWindowPos(hWndInsertAfter,x,y,cx,cy,nFlags)
			}

			pub fn SetWindowPos2 (&self,hWndInsertAfter:HWND,lpRect:&RECT,nFlags:UINT) -> bool {
				self.cwin.SetWindowPos2(hWndInsertAfter,lpRect,nFlags)
			}

			pub fn ArrangeIconicWindows (&self) -> UINT {
				self.cwin.ArrangeIconicWindows()
			}

			pub fn BringWindowToTop (&self) -> bool {
				self.cwin.BringWindowToTop()
			}

			pub fn GetWindowRect (&self,lpRect:LPRECT) -> bool {
				self.cwin.GetWindowRect(lpRect)
			}

			pub fn GetClientRect (&self,lpRect:&mut RECT) -> bool {
				self.cwin.GetClientRect(lpRect)
			}

			pub fn GetWindowPlacement(&self,lpwndpl:&mut WINDOWPLACEMENT) -> bool {
				self.cwin.GetWindowPlacement(lpwndpl)
			}

			pub fn SetWindowPlacement(&self,lpwndpl:&WINDOWPLACEMENT) -> bool {
				self.cwin.SetWindowPlacement(lpwndpl)
			}

			pub fn ClientToScreen (&self,lpPoint:LPPOINT) -> bool {
				self.cwin.ClientToScreen(lpPoint)
			}

			pub fn ClientToScreen2 (&self,lpRect:&mut RECT) -> bool {
				self.cwin.ClientToScreen2(lpRect)
			}

			pub fn ScreenToClient (&self,lpPoint:LPPOINT) -> bool {
				self.cwin.ScreenToClient(lpPoint)
			}

			pub fn ScreenToClient2 (&self,lpRect:&mut RECT) -> bool {
				self.cwin.ScreenToClient2(lpRect)
			}

			pub fn MapWindowPoints (&self,hWndTo:HWND,lpPoint:LPPOINT,nCount:UINT) -> c_int {
				self.cwin.MapWindowPoints(hWndTo,lpPoint,nCount)
			}

			pub fn MapWindowPoints2 (&self,hWndTo:HWND,lpRect:LPRECT) -> c_int {
				self.cwin.MapWindowPoints2(hWndTo,lpRect)
			}

			pub fn BeginPaint (&self,lpPaint:LPPAINTSTRUCT) -> HDC {
				self.cwin.BeginPaint(lpPaint)
			}

			pub fn EndPaint (&self,lpPaint:LPPAINTSTRUCT)  {
				self.cwin.EndPaint(lpPaint)
			}

			pub fn GetDC (&self) -> HDC {
				self.cwin.GetDC()
			}

			pub fn GetWindowDC (&self) -> HDC {
				self.cwin.GetWindowDC()
			}

			pub fn ReleaseDC (&self,hDC:HDC) -> c_int {
				self.cwin.ReleaseDC(hDC)
			}

			pub fn Print (&self,hDC:HDC,dwFlags:DWORD)  {
				self.cwin.Print(hDC,dwFlags)
			}

			pub fn PrintClient (&self,hDC:HDC,dwFlags:DWORD)  {
				self.cwin.PrintClient(hDC,dwFlags)
			}

			pub fn UpdateWindow (&self) -> bool {
				self.cwin.UpdateWindow()
			}

			pub fn SetRedraw (&self,bRedraw:BOOL)  {
				self.cwin.SetRedraw(bRedraw)
			}

			pub fn GetUpdateRect (&self,lpRect:LPRECT,bErase:BOOL) -> bool {
				self.cwin.GetUpdateRect(lpRect,bErase)
			}

			pub fn GetUpdateRgn (&self,hRgn:HRGN,bErase:BOOL) -> c_int {
				self.cwin.GetUpdateRgn(hRgn,bErase)
			}

			pub fn Invalidate (&self,bErase:BOOL) -> bool {
				self.cwin.Invalidate(bErase)
			}

			pub fn Invalidate2 (&self,lpRect:LPCRECT,bErase:BOOL) -> bool {
				self.cwin.Invalidate2(lpRect,bErase)
			}

			pub fn ValidateRect (&self,lpRect:LPCRECT) -> bool {
				self.cwin.ValidateRect(lpRect)
			}

			pub fn InvalidateRgn (&self,hRgn:HRGN,bErase:BOOL)  {
				self.cwin.InvalidateRgn(hRgn,bErase)
			}

			pub fn ValidateRgn (&self,hRgn:HRGN) -> bool {
				self.cwin.ValidateRgn(hRgn)
			}

			pub fn ShowWindow (&self,nCmdShow:c_int) -> bool {
				self.cwin.ShowWindow(nCmdShow)
			}

			pub fn IsWindowVisible (&self) -> bool {
				self.cwin.IsWindowVisible()
			}

			pub fn ShowOwnedPopups (&self,bShow:BOOL) -> bool {
				self.cwin.ShowOwnedPopups(bShow)
			}

			pub fn GetDCEx (&self,hRgnClip:HRGN,flags:DWORD) -> HDC {
				self.cwin.GetDCEx(hRgnClip,flags)
			}

			pub fn LockWindowUpdate (&self,bLock:bool) -> bool {
				self.cwin.LockWindowUpdate(bLock)
			}

			pub fn RedrawWindow2(&self) -> bool{
				self.cwin.RedrawWindow2()
			}

			pub fn RedrawWindow(&self,lpRectUpdate:LPCRECT,hRgnUpdate:HRGN,flags:UINT)->bool{
				self.cwin.RedrawWindow(lpRectUpdate,hRgnUpdate,flags)
			}

			pub fn SetTimer(&self,nIDEvent:UINT_PTR,nElapse:UINT) -> UINT_PTR {
				self.cwin.SetTimer(nIDEvent,nElapse)
			}

			pub fn SetTimer2(&self,nIDEvent:UINT_PTR,nElapse:UINT,lpfnTimer:TimerProc)->UINT_PTR{
				self.cwin.SetTimer2(nIDEvent,nElapse,lpfnTimer)
			}

			pub fn KillTimer (&self,nIDEvent:UINT_PTR) -> bool {
				self.cwin.KillTimer(nIDEvent)
			}

			pub fn IsWindowEnabled (&self) -> bool {
				self.cwin.IsWindowEnabled()
			}

			pub fn EnableWindow (&self,bEnable:BOOL) -> bool {
				self.cwin.EnableWindow(bEnable)
			}

			pub fn SetActiveWindow (&self) -> HWND {
				self.cwin.SetActiveWindow()
			}

			pub fn SetCapture (&self) -> HWND {
				self.cwin.SetCapture()
			}

			pub fn SetFocus (&self) -> HWND {
				self.cwin.SetFocus()
			}

			pub fn CheckDlgButton (&self,nIDButton:c_int,nCheck:UINT) -> bool {
				self.cwin.CheckDlgButton(nIDButton,nCheck)
			}

			pub fn CheckRadioButton (&self,nIDFirstButton:c_int,nIDLastButton:c_int,nIDCheckButton:c_int) -> bool {
				self.cwin.CheckRadioButton(nIDFirstButton,nIDLastButton,nIDCheckButton)
			}

			pub fn GetDlgItemInt(&self,nID:c_int) -> UINT {
				self.cwin.GetDlgItemInt(nID)
			}

			pub fn GetDlgItemInt2(&self,nID:c_int,lpTrans:&mut BOOL,bSigned:BOOL) -> UINT {
				self.cwin.GetDlgItemInt2(nID,lpTrans,bSigned)
			}

			pub fn IsDlgButtonChecked (&self,nIDButton:c_int) -> UINT {
				self.cwin.IsDlgButtonChecked(nIDButton)
			}

			pub fn SendDlgItemMessage (&self,nID:c_int,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
				self.cwin.SendDlgItemMessage(nID,message,wParam,lParam)
			}

			pub fn SetDlgItemInt (&self,nID:c_int,nValue:UINT,bSigned:BOOL) -> bool {
				self.cwin.SetDlgItemInt(nID,nValue,bSigned)
			}

			pub fn GetScrollPos (&self,nBar:c_int) -> c_int {
				self.cwin.GetScrollPos(nBar)
			}

			pub fn GetScrollRange (&self,nBar:c_int,lpMinPos:LPINT,lpMaxPos:LPINT) -> bool {
				self.cwin.GetScrollRange(nBar,lpMinPos,lpMaxPos)
			}

			pub fn ScrollWindow (&self,xAmount:c_int,yAmount:c_int,lpRect:LPCRECT,lpClipRect:LPCRECT) -> bool {
				self.cwin.ScrollWindow(xAmount,yAmount,lpRect,lpClipRect)
			}

			pub fn ScrollWindowEx(&self,dx:c_int,dy:c_int,lpRectScroll:LPCRECT ,lpRectClip:LPCRECT ,hRgnUpdate:HRGN ,lpRectUpdate:LPRECT ,uFlags:UINT ) -> c_int {
				self.cwin.ScrollWindowEx(dx,dy,lpRectScroll ,lpRectClip ,hRgnUpdate ,lpRectUpdate ,uFlags )
			}

			pub fn ScrollWindowExDefault(&self,dx:c_int,dy:c_int,uFlags:UINT)->c_int{
				self.cwin.ScrollWindowExDefault(dx,dy,uFlags)
			}

			pub fn SetScrollPos (&self,nBar:c_int,nPos:c_int,bRedraw:BOOL) -> c_int {
				self.cwin.SetScrollPos(nBar,nPos,bRedraw)
			}

			pub fn SetScrollRange (&self,nBar:c_int,nMinPos:c_int,nMaxPos:c_int,bRedraw:BOOL) -> bool {
				self.cwin.SetScrollRange(nBar,nMinPos,nMaxPos,bRedraw)
			}

			pub fn ShowScrollBar (&self,nBar:c_int,bShow:BOOL) -> bool {
				self.cwin.ShowScrollBar(nBar,bShow)
			}

			pub fn EnableScrollBar (&self,uSBFlags:UINT,uArrowFlags:UINT) -> bool {
				self.cwin.EnableScrollBar(uSBFlags,uArrowFlags)
			}

			pub fn IsChild (&self,hWnd:HWND) -> bool {
				self.cwin.IsChild(hWnd)
			}

			pub fn GetDlgCtrlID (&self) -> c_int {
				self.cwin.GetDlgCtrlID()
			}

			pub fn SetDlgCtrlID (&self,nID:c_int) -> c_int {
				self.cwin.SetDlgCtrlID(nID)
			}

			pub fn FlashWindow (&self,bInvert:BOOL) -> bool {
				self.cwin.FlashWindow(bInvert)
			}

			pub fn ChangeClipboardChain (&self,hWndNewNext:HWND) -> bool {
				self.cwin.ChangeClipboardChain(hWndNewNext)
			}

			pub fn SetClipboardViewer (&self) -> HWND {
				self.cwin.SetClipboardViewer()
			}

			pub fn OpenClipboard (&self) -> bool {
				self.cwin.OpenClipboard()
			}

			pub fn CreateCaret (&self,hBitmap:HBITMAP) -> bool {
				self.cwin.CreateCaret(hBitmap)
			}

			pub fn CreateSolidCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
				self.cwin.CreateSolidCaret(nWidth,nHeight)
			}

			pub fn CreateGrayCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
				self.cwin.CreateGrayCaret(nWidth,nHeight)
			}

			pub fn HideCaret (&self) -> bool {
				self.cwin.HideCaret()
			}

			pub fn ShowCaret (&self) -> bool {
				self.cwin.ShowCaret()
			}

			pub fn DragAcceptFiles (&self,bAccept:BOOL)  {
				self.cwin.DragAcceptFiles(bAccept)
			}

			pub fn SetIcon (&self,hIcon:HICON,bBigIcon:BOOL) -> HICON {
				self.cwin.SetIcon(hIcon,bBigIcon)
			}

			pub fn GetIcon (&self,bBigIcon:BOOL) -> HICON {
				self.cwin.GetIcon(bBigIcon)
			}

			pub fn SetWindowContextHelpId (&self,dwContextHelpId:DWORD) -> bool {
				self.cwin.SetWindowContextHelpId(dwContextHelpId)
			}

			pub fn GetWindowContextHelpId (&self) -> DWORD {
				self.cwin.GetWindowContextHelpId()
			}

			pub fn SetHotKey (&self,wVirtualKeyCode:WORD,wModifiers:WORD) -> c_int {
				self.cwin.SetHotKey(wVirtualKeyCode,wModifiers)
			}

			pub fn GetHotKey (&self) -> DWORD {
				self.cwin.GetHotKey()
			}

			pub fn GetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO) -> bool {
				self.cwin.GetScrollInfo(nBar,lpScrollInfo)
			}

			pub fn SetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO,bRedraw:BOOL) -> c_int {
				self.cwin.SetScrollInfo(nBar,lpScrollInfo,bRedraw)
			}

			pub fn IsDialogMessage (&self,lpMsg:LPMSG) -> bool {
				self.cwin.IsDialogMessage(lpMsg)
			}

			pub fn NextDlgCtrl (&self)  {
				self.cwin.NextDlgCtrl()
			}

			pub fn PrevDlgCtrl (&self)  {
				self.cwin.PrevDlgCtrl()
			}

			pub fn GotoDlgCtrl (&self,hWndCtrl:HWND)  {
				self.cwin.GotoDlgCtrl(hWndCtrl)
			}

			pub fn ResizeClient (&self,nWidth:c_int,nHeight:c_int,bRedraw:BOOL) -> bool {
				self.cwin.ResizeClient(nWidth,nHeight,bRedraw)
			}

			pub fn GetWindowRgn (&self,hRgn:HRGN) -> c_int {
				self.cwin.GetWindowRgn(hRgn)
			}

			pub fn SetWindowRgn (&self,hRgn:HRGN,bRedraw:BOOL) -> c_int {
				self.cwin.SetWindowRgn(hRgn,bRedraw)
			}

			pub fn DeferWindowPos(&self,hWinPosInfo:HDWP,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,uFlags:UINT) -> HDWP {
				self.cwin.DeferWindowPos(hWinPosInfo,hWndInsertAfter,x,y,cx,cy,uFlags)
			}

			pub fn GetWindowThreadID (&self) -> DWORD {
				self.cwin.GetWindowThreadID()
			}

			pub fn GetWindowProcessID (&self) -> DWORD {
				self.cwin.GetWindowProcessID()
			}

			pub fn IsWindow (&self) -> bool {
				self.cwin.IsWindow()
			}

			pub fn IsWindowUnicode (&self) -> bool {
				self.cwin.IsWindowUnicode()
			}

			pub fn ShowWindowAsync (&self,nCmdShow:c_int) -> bool {
				self.cwin.ShowWindowAsync(nCmdShow)
			}

			pub fn CenterWindow (&self,hCenter:HWND) -> BOOL {
				self.cwin.CenterWindow(hCenter)
			}

			pub fn ModifyStyle (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
				self.cwin.ModifyStyle(dwRemove,dwAdd,nFlags)
			}

			pub fn ModifyStyleEx (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
				self.cwin.ModifyStyleEx(dwRemove,dwAdd,nFlags)
			}
		}
	};
}
