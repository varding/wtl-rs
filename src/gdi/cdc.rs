

use winapi::*;
use user32;
use gdi32;
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
->c_void

(3)
debug_assert!(::IsWindow(m_hWnd));
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

// three paramter
pub fn (\w+)\s*\((\w+) (\w+), (\w+) (\w+), (\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\, \5: \4,\7: \6)

//four parameter
pub fn (\w+)\s*\((\w+) (\w+), (\w+) (\w+), (\w+) (\w+), (\w+) (\w+)\s*\)
=>
pub fn \1\(&self,\3: \2\, \5: \4,\7: \6,\9: \8)

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

(9) replace manually
WPARAM coercion
(self.SendMessage\(\s*\w+,\s*)(\w+)(.*)
=>
\1 \2 as WPARAM\3

LPARAM coercion
(self.SendMessage\(.*,.*,\s*)(\w+)(.*)
=>
\1 \2 as LPARAM\3

*/

pub struct CDC {
	h: cdc_inner,
}

pub const NULL_HDC: HDC = 0 as HDC;

struct cdc_inner {
    hdc: HDC,
}


impl cdc_inner{
// class CDCT
// {
// public:
// // Data members
// 	HDC self.hdc;

// Constructor/destructor/operators
	// CDCT(HDC hDC = NULL) : self.hdc(hDC)
	// {
	// }

	// ~CDCT()
	// {
	// 	if(t_bManaged && self.hdc != NULL)
	// 		::DeleteDC(Detach());
	// }

	// CDCT<t_bManaged>& operator =(HDC hDC)
	// {
	// 	Attach(hDC);
	// 	return *this;
	// }

	pub fn Attach(&self,hDC: HDC) {
		if(t_bManaged && self.hdc != NULL && self.hdc != hDC){
			::DeleteDC(self.hdc);
		}
		self.hdc = hDC;
	}

	pub fn Detach(&mut self)->HDC {
		let hDC = self.hdc;
		self.hdc = NULL;
		return hDC;
	}

	pub fn assert_dc(&self){
		debug_assert!(self.hdc != NULL_HDC);
	}

	pub fn HDC(&self)->operator { return self.hdc; }

	pub fn IsNull(&self)->bool { return (self.hdc == NULL); }

// Operations
//#ifndef _WIN32_WCE
	pub fn WindowFromDC(&self)->HWND {
		self.assert_dc();
		return ::WindowFromDC(self.hdc);
	}
//#endif // !_WIN32_WCE

	pub fn GetCurrentPen(&self)->CPenHandle {
		self.assert_dc();
		return CPenHandle(::GetCurrentObject(self.hdc, OBJ_PEN) as HPEN);
	}

	pub fn GetCurrentBrush(&self)->CBrushHandle {
		self.assert_dc();
		return CBrushHandle(::GetCurrentObject(self.hdc, OBJ_BRUSH) as HBRUSH);
	}

	pub fn GetCurrentPalette(&self)->CPaletteHandle {
		self.assert_dc();
		return CPaletteHandle(::GetCurrentObject(self.hdc, OBJ_PAL) as HPALETTE);
	}

	pub fn GetCurrentFont(&self)->CFontHandle {
		self.assert_dc();
		return CFontHandle(::GetCurrentObject(self.hdc, OBJ_FONT) as HFONT);
	}

	pub fn GetCurrentBitmap(&self)->CBitmapHandle {
		self.assert_dc();
		return CBitmapHandle(::GetCurrentObject(self.hdc, OBJ_BITMAP) as HBITMAP);
	}

	// pub fn CreateDC (&mut self,LPCTSTR lpszDriverName, LPCTSTR lpszDeviceName, LPCTSTR lpszOutput, const DEVMODE* lpInitData)->HDC {
	// 	debug_assert!(self.hdc == NULL);
	// 	self.hdc = ::CreateDC(lpszDriverName, lpszDeviceName, lpszOutput, lpInitData);
	// 	return self.hdc;
	// }

	pub fn CreateCompatibleDC (&mut self)->HDC {
		self.CreateCompatibleDC2(NULL_HDC)
	}

	pub fn CreateCompatibleDC2 (&mut self,hDC: HDC)->HDC {
		debug_assert!(self.hdc == NULL);
		self.hdc = ::CreateCompatibleDC(hDC);
		return self.hdc;
	}

	pub fn DeleteDC(&mut self)->BOOL {
		if self.hdc == NULL{
			return FALSE;
		}
		let bRet:BOOL = ::DeleteDC(self.hdc);
		if bRet > 0{
			self.hdc = NULL;
		}
		return bRet;
	}

// Device-Context Functions
	pub fn SaveDC(&self)->c_int {
		self.assert_dc();
		return ::SaveDC(self.hdc);
	}

	pub fn RestoreDC(&self,nSavedDC: c_int)->BOOL {
		self.assert_dc();
		return ::RestoreDC(self.hdc, nSavedDC);
	}

	pub fn GetDeviceCaps(&self,nIndex: c_int)->c_int {
		self.assert_dc();
		return ::GetDeviceCaps(self.hdc, nIndex);
	}

//#ifndef _WIN32_WCE
	pub fn SetBoundsRect(&self,lpRectBounds: LPCRECT, flags: UINT)->UINT {
		self.assert_dc();
		return ::SetBoundsRect(self.hdc, lpRectBounds, flags);
	}

	pub fn GetBoundsRect(&self,lpRectBounds: LPRECT, flags: UINT)->UINT {
		self.assert_dc();
		return ::GetBoundsRect(self.hdc, lpRectBounds, flags);
	}

	pub fn ResetDC (&self, lpDevMode: LPDEVMODEW)->BOOL {
		self.assert_dc();
		return ::ResetDC(self.hdc, lpDevMode) != NULL;
	}

// Drawing-Tool Functions
	pub fn GetBrushOrg(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::GetBrushOrgEx(self.hdc, lpPoint);
	}
//#endif // !_WIN32_WCE

	pub fn SetBrushOrg (&self, x: c_int , y: c_int, lpPoint: LPPOINT) -> BOOL {
		self.assert_dc();
		return ::SetBrushOrgEx(self.hdc, x, y, lpPoint);
	}

	pub fn SetBrushOrg (&self, point: POINT, lpPointRet: LPPOINT)->BOOL {
		self.assert_dc();
		return ::SetBrushOrgEx(self.hdc, point.x, point.y, lpPointRet);
	}

//#ifndef _WIN32_WCE
// 	pub fn EnumObjects (c_int nObjectType, c_int (CALLBACK* lpfn)(LPVOID, LPARAM), LPARAM lpData)->c_int {
// 		self.assert_dc();
// //#ifdef STRICT
// //		return ::EnumObjects(self.hdc, nObjectType, (GOBJENUMPROC)lpfn, lpData);
// //#else
// 		return ::EnumObjects(self.hdc, nObjectType, (GOBJENUMPROC)lpfn, (LPVOID)lpData);
// //#endif
// 	}
//#endif // !_WIN32_WCE

// Type-safe selection helpers
	pub fn SelectPen(&self,hPen: HPEN)->HPEN {
		self.assert_dc();
//#ifndef _WIN32_WCE
		debug_assert!(hPen == NULL || ::GetObjectType(hPen) == OBJ_PEN || ::GetObjectType(hPen) == OBJ_EXTPEN);
//#else // CE specific
//		debug_assert!(hPen == NULL || ::GetObjectType(hPen) == OBJ_PEN);
//#endif // _WIN32_WCE
		::SelectObject(self.hdc, hPen) as HPEN
	}

	pub fn SelectBrush(&self,hBrush: HBRUSH)->HBRUSH {
		self.assert_dc();
		debug_assert!(hBrush == NULL || ::GetObjectType(hBrush) == OBJ_BRUSH);
		::SelectObject(self.hdc, hBrush) as HBRUSH
	}

	pub fn SelectFont(&self,hFont: HFONT)->HFONT {
		self.assert_dc();
		debug_assert!(hFont == NULL || ::GetObjectType(hFont) == OBJ_FONT);
		::SelectObject(self.hdc, hFont) as HFONT
	}

	pub fn SelectBitmap(&self,hBitmap: HBITMAP)->HBITMAP {
		self.assert_dc();
		debug_assert!(hBitmap == NULL || ::GetObjectType(hBitmap) == OBJ_BITMAP);
		::SelectObject(self.hdc, hBitmap) as HBITMAP
	}

	pub fn SelectRgn(&self,hRgn: HRGN)->c_int{       // special return for regions
		self.assert_dc();
		debug_assert!(hRgn == NULL || ::GetObjectType(hRgn) == OBJ_REGION);
		return PtrToInt(::SelectObject(self.hdc, hRgn));
	}

// Type-safe selection helpers for stock objects
	pub fn SelectStockPen(&self,nPen: c_int)->HPEN {
		self.assert_dc();
//#if (_WIN32_WINNT >= 0x0500)
		debug_assert!(nPen == WHITE_PEN || nPen == BLACK_PEN || nPen == NULL_PEN || nPen == DC_PEN);
//#else
//		debug_assert!(nPen == WHITE_PEN || nPen == BLACK_PEN || nPen == NULL_PEN);
//#endif // !(_WIN32_WINNT >= 0x0500)
		return SelectPen(::GetStockObject(nPen) as HPEN);
	}

	pub fn SelectStockBrush(&self,nBrush: c_int)->HBRUSH {
//#if (_WIN32_WINNT >= 0x0500)
		debug_assert!((nBrush >= WHITE_BRUSH && nBrush <= HOLLOW_BRUSH) || nBrush == DC_BRUSH);
//#else
//		debug_assert!(nBrush >= WHITE_BRUSH && nBrush <= HOLLOW_BRUSH);
//#endif // !(_WIN32_WINNT >= 0x0500)
		return SelectBrush(::GetStockObject(nBrush) as HBRUSH);
	}

	pub fn SelectStockFont(&self,nFont: c_int)->HFONT {
//#ifndef _WIN32_WCE
		debug_assert!((nFont >= OEM_FIXED_FONT && nFont <= SYSTEM_FIXED_FONT) || nFont == DEFAULT_GUI_FONT);
//#else // CE specific
//		debug_assert!(nFont == SYSTEM_FONT);
//#endif // _WIN32_WCE
		return SelectFont(::GetStockObject(nFont) as HFONT);
	}

	pub fn SelectStockPalette(&self,nPalette: c_int, bForceBackground: BOOL)->HPALETTE {
		debug_assert!(nPalette == DEFAULT_PALETTE); // the only one supported
		return SelectPalette(::GetStockObject(nPalette) as HPALETTE, bForceBackground);
	}

// Color and Color Palette Functions
	pub fn GetNearestColor(&self,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		return ::GetNearestColor(self.hdc, crColor);
	}

	pub fn SelectPalette(&self,hPalette: HPALETTE, bForceBackground: BOOL)->HPALETTE {
		self.assert_dc();

		return ::SelectPalette(self.hdc, hPalette, bForceBackground);
	}

	pub fn RealizePalette(&self)->UINT {
		self.assert_dc();
		return ::RealizePalette(self.hdc);
	}

//#ifndef _WIN32_WCE
	pub fn UpdateColors(&self) {
		self.assert_dc();
		::UpdateColors(self.hdc);
	}
//#endif // !_WIN32_WCE

// Drawing-Attribute Functions
	pub fn GetBkColor(&self)->COLORREF {
		self.assert_dc();
		return ::GetBkColor(self.hdc);
	}

	pub fn GetBkMode(&self)->c_int {
		self.assert_dc();
		return ::GetBkMode(self.hdc);
	}

//#ifndef _WIN32_WCE
	pub fn GetPolyFillMode(&self)->c_int {
		self.assert_dc();
		return ::GetPolyFillMode(self.hdc);
	}

	pub fn GetROP2(&self)->c_int {
		self.assert_dc();
		return ::GetROP2(self.hdc);
	}

	pub fn GetStretchBltMode(&self)->c_int {
		self.assert_dc();
		return ::GetStretchBltMode(self.hdc);
	}
//#endif // !_WIN32_WCE

	pub fn GetTextColor(&self)->COLORREF {
		self.assert_dc();
		return ::GetTextColor(self.hdc);
	}

	pub fn SetBkColor(&self,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		return ::SetBkColor(self.hdc, crColor);
	}

	pub fn SetBkMode(&self,nBkMode: c_int)->c_int {
		self.assert_dc();
		return ::SetBkMode(self.hdc, nBkMode);
	}

//#ifndef _WIN32_WCE
	pub fn SetPolyFillMode(&self,nPolyFillMode: c_int)->c_int {
		self.assert_dc();
		return ::SetPolyFillMode(self.hdc, nPolyFillMode);
	}
//#endif // !_WIN32_WCE

	pub fn SetROP2(&self,nDrawMode: c_int)->c_int {
		self.assert_dc();
		return ::SetROP2(self.hdc, nDrawMode);
	}

//#ifndef _WIN32_WCE
	pub fn SetStretchBltMode(&self,nStretchMode: c_int)->c_int {
		self.assert_dc();
		return ::SetStretchBltMode(self.hdc, nStretchMode);
	}
//#endif // !_WIN32_WCE

	pub fn SetTextColor(&self,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		return ::SetTextColor(self.hdc, crColor);
	}

//#ifndef _WIN32_WCE
	pub fn GetColorAdjustment(&self,lpColorAdjust: LPCOLORADJUSTMENT)->BOOL {
		self.assert_dc();
		return ::GetColorAdjustment(self.hdc, lpColorAdjust);
	}

	pub fn SetColorAdjustment (&self, lpColorAdjust: COLORADJUSTMENT)->BOOL {
		self.assert_dc();
		return ::SetColorAdjustment(self.hdc, lpColorAdjust);
	}

// Mapping Functions
	pub fn GetMapMode(&self)->c_int {
		self.assert_dc();
		return ::GetMapMode(self.hdc);
	}

	pub fn GetViewportOrg(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::GetViewportOrgEx(self.hdc, lpPoint);
	}

	pub fn SetMapMode(&self,nMapMode: c_int)->c_int {
		self.assert_dc();
		return ::SetMapMode(self.hdc, nMapMode);
	}
//#endif // !_WIN32_WCE

	// Viewport Origin
	pub fn SetViewportOrg(&self,x: c_int, y: c_int,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::SetViewportOrgEx(self.hdc, x, y, lpPoint);
	}

	pub fn SetViewportOrg(&self,point: POINT, lpPointRet: LPPOINT)->BOOL {
		self.assert_dc();
		return SetViewportOrg(point.x, point.y, lpPointRet);
	}

//#ifndef _WIN32_WCE
	pub fn OffsetViewportOrg(&self,nWidth: c_int, nHeight: c_int,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::OffsetViewportOrgEx(self.hdc, nWidth, nHeight, lpPoint);
	}

	// Viewport Extent
	pub fn GetViewportExt(&self,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		return ::GetViewportExtEx(self.hdc, lpSize);
	}

	pub fn SetViewportExt(&self,x: c_int, y: c_int,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		return ::SetViewportExtEx(self.hdc, x, y, lpSize);
	}

	pub fn SetViewportExt(&self,size: SIZE, lpSizeRet: LPSIZE)->BOOL {
		self.assert_dc();
		return SetViewportExt(size.cx, size.cy, lpSizeRet);
	}

	pub fn ScaleViewportExt (c_int xNum, c_int xDenom, c_int yNum, c_int yDenom, LPSIZE lpSize )->BOOL {
		self.assert_dc();
		return ::ScaleViewportExtEx(self.hdc, xNum, xDenom, yNum, yDenom, lpSize);
	}
//#endif // !_WIN32_WCE

	// Window Origin
//#ifndef _WIN32_WCE
	pub fn GetWindowOrg(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::GetWindowOrgEx(self.hdc, lpPoint);
	}

	pub fn SetWindowOrg(&self,x: c_int, y: c_int,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::SetWindowOrgEx(self.hdc, x, y, lpPoint);
	}

	pub fn SetWindowOrg(&self,point: POINT, lpPointRet: LPPOINT)->BOOL {
		self.assert_dc();
		return SetWindowOrg(point.x, point.y, lpPointRet);
	}

	pub fn OffsetWindowOrg(&self,nWidth: c_int, nHeight: c_int,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::OffsetWindowOrgEx(self.hdc, nWidth, nHeight, lpPoint);
	}

	// Window extent
	pub fn GetWindowExt(&self,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		return ::GetWindowExtEx(self.hdc, lpSize);
	}

	pub fn SetWindowExt(&self,x: c_int, y: c_int,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		return ::SetWindowExtEx(self.hdc, x, y, lpSize);
	}

	pub fn SetWindowExt(&self,size: SIZE, lpSizeRet: LPSIZE)->BOOL {
		self.assert_dc();
		return SetWindowExt(size.cx, size.cy, lpSizeRet);
	}

	pub fn ScaleWindowExt (c_int xNum, c_int xDenom, c_int yNum, c_int yDenom, LPSIZE lpSize )->BOOL {
		self.assert_dc();
		return ::ScaleWindowExtEx(self.hdc, xNum, xDenom, yNum, yDenom, lpSize);
	}

// Coordinate Functions
	pub fn DPtoLP (LPPOINT lpPoints, c_int nCount = 1)->BOOL {
		self.assert_dc();
		return ::DPtoLP(self.hdc, lpPoints, nCount);
	}

	pub fn DPtoLP(&self,lpRect: LPRECT)->BOOL {
		self.assert_dc();
		return ::DPtoLP(self.hdc, (LPPOINT)lpRect, 2);
	}

	pub fn DPtoLP(&self,lpSize: LPSIZE)->BOOL {
		SIZE sizeWinExt = { 0, 0 };
		if(!GetWindowExt(&sizeWinExt))
			return FALSE;
		SIZE sizeVpExt = { 0, 0 };
		if(!GetViewportExt(&sizeVpExt))
			return FALSE;
		lpSize->cx = ::MulDiv(lpSize->cx, abs(sizeWinExt.cx), abs(sizeVpExt.cx));
		lpSize->cy = ::MulDiv(lpSize->cy, abs(sizeWinExt.cy), abs(sizeVpExt.cy));
		return TRUE;
	}

	pub fn LPtoDP (LPPOINT lpPoints, c_int nCount = 1)->BOOL {
		self.assert_dc();
		return ::LPtoDP(self.hdc, lpPoints, nCount);
	}

	pub fn LPtoDP(&self,lpRect: LPRECT)->BOOL {
		self.assert_dc();
		return ::LPtoDP(self.hdc, (LPPOINT)lpRect, 2);
	}

	pub fn LPtoDP(&self,lpSize: LPSIZE)->BOOL {
		SIZE sizeWinExt = { 0, 0 };
		if(!GetWindowExt(&sizeWinExt))
			return FALSE;
		SIZE sizeVpExt = { 0, 0 };
		if(!GetViewportExt(&sizeVpExt))
			return FALSE;
		lpSize->cx = ::MulDiv(lpSize->cx, abs(sizeVpExt.cx), abs(sizeWinExt.cx));
		lpSize->cy = ::MulDiv(lpSize->cy, abs(sizeVpExt.cy), abs(sizeWinExt.cy));
		return TRUE;
	}

// Special Coordinate Functions (useful for dealing with metafiles and OLE)
	#define HIMETRIC_INCH   2540    // HIMETRIC units per inch

	pub fn DPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.assert_dc();
		c_int nMapMode;
		if((nMapMode = GetMapMode()) < MM_ISOTROPIC && nMapMode != MM_TEXT)
		{
			// when using a constrained map mode, map against physical inch
			((CDCHandle*)this)->SetMapMode(MM_HIMETRIC);
			DPtoLP(lpSize);
			((CDCHandle*)this)->SetMapMode(nMapMode);
		}
		else
		{
			// map against logical inch for non-constrained mapping modes
			c_int cxPerInch = GetDeviceCaps(LOGPIXELSX);
			c_int cyPerInch = GetDeviceCaps(LOGPIXELSY);
			debug_assert!(cxPerInch != 0 && cyPerInch != 0);
			lpSize->cx = ::MulDiv(lpSize->cx, HIMETRIC_INCH, cxPerInch);
			lpSize->cy = ::MulDiv(lpSize->cy, HIMETRIC_INCH, cyPerInch);
		}
	}

	pub fn HIMETRICtoDP(&self,lpSize: LPSIZE) {
		self.assert_dc();
		c_int nMapMode;
		if((nMapMode = GetMapMode()) < MM_ISOTROPIC && nMapMode != MM_TEXT)
		{
			// when using a constrained map mode, map against physical inch
			((CDCHandle*)this)->SetMapMode(MM_HIMETRIC);
			LPtoDP(lpSize);
			((CDCHandle*)this)->SetMapMode(nMapMode);
		}
		else
		{
			// map against logical inch for non-constrained mapping modes
			c_int cxPerInch = GetDeviceCaps(LOGPIXELSX);
			c_int cyPerInch = GetDeviceCaps(LOGPIXELSY);
			debug_assert!(cxPerInch != 0 && cyPerInch != 0);
			lpSize->cx = ::MulDiv(lpSize->cx, cxPerInch, HIMETRIC_INCH);
			lpSize->cy = ::MulDiv(lpSize->cy, cyPerInch, HIMETRIC_INCH);
		}
	}

	pub fn LPtoHIMETRIC(&self,lpSize: LPSIZE) {
		LPtoDP(lpSize);
		DPtoHIMETRIC(lpSize);
	}

	pub fn HIMETRICtoLP(&self,lpSize: LPSIZE) {
		HIMETRICtoDP(lpSize);
		DPtoLP(lpSize);
	}
//#endif // !_WIN32_WCE

// Region Functions
	pub fn FillRgn(&self,hRgn: HRGN, hBrush: HBRUSH)->BOOL {
		self.assert_dc();
		return ::FillRgn(self.hdc, hRgn, hBrush);
	}

//#ifndef _WIN32_WCE
	pub fn FrameRgn(&self,hRgn: HRGN, hBrush: HBRUSH,nWidth: c_int,nHeight: c_int)->BOOL {
		self.assert_dc();
		return ::FrameRgn(self.hdc, hRgn, hBrush, nWidth, nHeight);
	}

	pub fn InvertRgn(&self,hRgn: HRGN)->BOOL {
		self.assert_dc();
		return ::InvertRgn(self.hdc, hRgn);
	}

	pub fn PaintRgn(&self,hRgn: HRGN)->BOOL {
		self.assert_dc();
		return ::PaintRgn(self.hdc, hRgn);
	}
//#endif // !_WIN32_WCE

// Clipping Functions
	pub fn GetClipBox(&self,lpRect: LPRECT)->c_int {
		self.assert_dc();
		return ::GetClipBox(self.hdc, lpRect);
	}

	pub fn GetClipRgn (CRgn& region)->c_int {
		self.assert_dc();
		if(region.IsNull())
			region.CreateRectRgn(0, 0, 0, 0);

		c_int nRet = ::GetClipRgn(self.hdc, region);
		if(nRet != 1)
			region.DeleteObject();

		return nRet;
	}

//#ifndef _WIN32_WCE
	pub fn PtVisible(&self,x: c_int, y: c_int)->BOOL {
		self.assert_dc();
		return ::PtVisible(self.hdc, x, y);
	}

	pub fn PtVisible(&self,point: POINT)->BOOL {
		self.assert_dc();
		return ::PtVisible(self.hdc, point.x, point.y);
	}
//#endif // !_WIN32_WCE

	pub fn RectVisible(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		return ::RectVisible(self.hdc, lpRect);
	}

	pub fn SelectClipRgn(&self,hRgn: HRGN)->c_int {
		self.assert_dc();
		return ::SelectClipRgn(self.hdc, (HRGN)hRgn);
	}

	pub fn ExcludeClipRect(&self,x1: c_int, y1: c_int,x2: c_int,y2: c_int)->c_int {
		self.assert_dc();
		return ::ExcludeClipRect(self.hdc, x1, y1, x2, y2);
	}

	pub fn ExcludeClipRect(&self,lpRect: LPCRECT)->c_int {
		self.assert_dc();
		return ::ExcludeClipRect(self.hdc, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom);
	}

//#ifndef _WIN32_WCE
	pub fn ExcludeUpdateRgn(&self,hWnd: HWND)->c_int {
		self.assert_dc();
		return ::ExcludeUpdateRgn(self.hdc, hWnd);
	}
//#endif // !_WIN32_WCE

	pub fn IntersectClipRect(&self,x1: c_int, y1: c_int,x2: c_int,y2: c_int)->c_int {
		self.assert_dc();
		return ::IntersectClipRect(self.hdc, x1, y1, x2, y2);
	}

	pub fn IntersectClipRect(&self,lpRect: LPCRECT)->c_int {
		self.assert_dc();
		return ::IntersectClipRect(self.hdc, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom);
	}

//#ifndef _WIN32_WCE
	pub fn OffsetClipRgn(&self,x: c_int, y: c_int)->c_int {
		self.assert_dc();
		return ::OffsetClipRgn(self.hdc, x, y);
	}

	pub fn OffsetClipRgn(&self,size: SIZE)->c_int {
		self.assert_dc();
		return ::OffsetClipRgn(self.hdc, size.cx, size.cy);
	}

	pub fn SelectClipRgn(&self,hRgn: HRGN, nMode: c_int)->c_int {
		self.assert_dc();
		return ::ExtSelectClipRgn(self.hdc, hRgn, nMode);
	}
//#endif // !_WIN32_WCE

// Line-Output Functions
//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)
	pub fn GetCurrentPosition(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::GetCurrentPositionEx(self.hdc, lpPoint);
	}

	pub fn MoveTo(&self,x: c_int, y: c_int,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		return ::MoveToEx(self.hdc, x, y, lpPoint);
	}

	pub fn MoveTo(&self,point: POINT, lpPointRet: LPPOINT)->BOOL {
		self.assert_dc();
		return MoveTo(point.x, point.y, lpPointRet);
	}

	pub fn LineTo(&self,x: c_int, y: c_int)->BOOL {
		self.assert_dc();
		return ::LineTo(self.hdc, x, y);
	}

	pub fn LineTo(&self,point: POINT)->BOOL {
		self.assert_dc();
		return LineTo(point.x, point.y);
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)

//#ifndef _WIN32_WCE
	pub fn Arc (c_int x1, c_int y1, c_int x2, c_int y2, c_int x3, c_int y3, c_int x4, c_int y4)->BOOL {
		self.assert_dc();
		return ::Arc(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4);
	}

	pub fn Arc(&self,lpRect: LPCRECT, ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		return ::Arc(self.hdc, lpRect->left, lpRect->top,
			lpRect->right, lpRect->bottom, ptStart.x, ptStart.y,
			ptEnd.x, ptEnd.y);
	}
//#endif // !_WIN32_WCE

	pub fn Polyline (const POINT* lpPoints, c_int nCount)->BOOL {
		self.assert_dc();
		return ::Polyline(self.hdc, lpPoints, nCount);
	}

//#ifndef _WIN32_WCE
	pub fn AngleArc (c_int x, c_int y, c_int nRadius, float fStartAngle, float fSweepAngle)->BOOL {
		self.assert_dc();
		return ::AngleArc(self.hdc, x, y, nRadius, fStartAngle, fSweepAngle);
	}

	pub fn ArcTo (c_int x1, c_int y1, c_int x2, c_int y2, c_int x3, c_int y3, c_int x4, c_int y4)->BOOL {
		self.assert_dc();
		return ::ArcTo(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4);
	}

	pub fn ArcTo(&self,lpRect: LPCRECT, ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		return ArcTo(lpRect->left, lpRect->top, lpRect->right,
		lpRect->bottom, ptStart.x, ptStart.y, ptEnd.x, ptEnd.y);
	}

	pub fn GetArcDirection(&self)->c_int {
		self.assert_dc();
		return ::GetArcDirection(self.hdc);
	}

	pub fn SetArcDirection(&self,nArcDirection: c_int)->c_int {
		self.assert_dc();
		return ::SetArcDirection(self.hdc, nArcDirection);
	}

	pub fn PolyDraw (const POINT* lpPoints, const BYTE* lpTypes, c_int nCount)->BOOL {
		self.assert_dc();
		return ::PolyDraw(self.hdc, lpPoints, lpTypes, nCount);
	}

	pub fn PolylineTo (const POINT* lpPoints, c_int nCount)->BOOL {
		self.assert_dc();
		return ::PolylineTo(self.hdc, lpPoints, nCount);
	}

	BOOL PolyPolyline(const POINT* lpPoints,
		const DWORD* lpPolyPoints, c_int nCount)
	{
		self.assert_dc();
		return ::PolyPolyline(self.hdc, lpPoints, lpPolyPoints, nCount);
	}

	pub fn PolyBezier (const POINT* lpPoints, c_int nCount)->BOOL {
		self.assert_dc();
		return ::PolyBezier(self.hdc, lpPoints, nCount);
	}

	pub fn PolyBezierTo (const POINT* lpPoints, c_int nCount)->BOOL {
		self.assert_dc();
		return ::PolyBezierTo(self.hdc, lpPoints, nCount);
	}
//#endif // !_WIN32_WCE

// Simple Drawing Functions
	pub fn FillRect(&self,lpRect: LPCRECT, hBrush: HBRUSH)->BOOL {
		self.assert_dc();
		return ::FillRect(self.hdc, lpRect, hBrush);
	}

	pub fn FillRect(&self,lpRect: LPCRECT, nColorIndex: c_int)->BOOL {
		self.assert_dc();
//#ifndef _WIN32_WCE
		return ::FillRect(self.hdc, lpRect, (HBRUSH)LongToPtr(nColorIndex + 1));
//#else // CE specific
//		return ::FillRect(self.hdc, lpRect, ::GetSysColorBrush(nColorIndex));
//#endif // _WIN32_WCE
	}

//#ifndef _WIN32_WCE
	pub fn FrameRect(&self,lpRect: LPCRECT, hBrush: HBRUSH)->BOOL {
		self.assert_dc();
		return ::FrameRect(self.hdc, lpRect, hBrush);
	}
//#endif // !_WIN32_WCE

//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 420)
	pub fn InvertRect(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		return ::InvertRect(self.hdc, lpRect);
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 420)

	pub fn DrawIcon(&self,x: c_int, y: c_int,hIcon: HICON)->BOOL {
		self.assert_dc();
//#ifndef _WIN32_WCE
		return ::DrawIcon(self.hdc, x, y, hIcon);
//#else // CE specific
//		return ::DrawIconEx(self.hdc, x, y, hIcon, 0, 0, 0, NULL, DI_NORMAL);
//#endif // _WIN32_WCE
	}

	pub fn DrawIcon(&self,point: POINT, hIcon: HICON)->BOOL {
		self.assert_dc();
//#ifndef _WIN32_WCE
		return ::DrawIcon(self.hdc, point.x, point.y, hIcon);
//#else // CE specific
//		return ::DrawIconEx(self.hdc, point.x, point.y, hIcon, 0, 0, 0, NULL, DI_NORMAL);
//#endif // _WIN32_WCE
	}

	pub fn DrawIconEx (c_int x, c_int y, HICON hIcon, c_int cxWidth, c_int cyWidth, UINT uStepIfAniCur = 0, HBRUSH hbrFlickerFreeDraw , UINT uFlags = DI_NORMAL)->BOOL {
		self.assert_dc();
		return ::DrawIconEx(self.hdc, x, y, hIcon, cxWidth, cyWidth, uStepIfAniCur, hbrFlickerFreeDraw, uFlags);
	}

	pub fn DrawIconEx (POINT point, HICON hIcon, SIZE size, UINT uStepIfAniCur = 0, HBRUSH hbrFlickerFreeDraw , UINT uFlags = DI_NORMAL)->BOOL {
		self.assert_dc();
		return ::DrawIconEx(self.hdc, point.x, point.y, hIcon, size.cx, size.cy, uStepIfAniCur, hbrFlickerFreeDraw, uFlags);
	}

//#ifndef _WIN32_WCE
	pub fn DrawState (POINT pt, SIZE size, HBITMAP hBitmap, UINT nFlags, HBRUSH hBrush )->BOOL {
		self.assert_dc();
		return ::DrawState(self.hdc, hBrush, NULL, hBitmap as LPARAM, 0, pt.x, pt.y, size.cx, size.cy, nFlags | DST_BITMAP);
	}

	pub fn DrawState (POINT pt, SIZE size, HICON hIcon, UINT nFlags, HBRUSH hBrush )->BOOL {
		self.assert_dc();
		return ::DrawState(self.hdc, hBrush, NULL, hIcon as LPARAM, 0, pt.x, pt.y, size.cx, size.cy, nFlags | DST_ICON);
	}

	pub fn DrawState (POINT pt, SIZE size, LPCTSTR lpszText, UINT nFlags, BOOL bPrefixText = TRUE, c_int nTextLen = 0, HBRUSH hBrush )->BOOL {
		self.assert_dc();
		return ::DrawState(self.hdc, hBrush, NULL, lpszText as LPARAM, nTextLen as WPARAM, pt.x, pt.y, size.cx, size.cy, nFlags | (bPrefixText ? DST_PREFIXTEXT : DST_TEXT));
	}

	pub fn DrawState (POINT pt, SIZE size, DRAWSTATEPROC lpDrawProc, LPARAM lData, UINT nFlags, HBRUSH hBrush )->BOOL {
		self.assert_dc();
		return ::DrawState(self.hdc, hBrush, lpDrawProc, lData, 0, pt.x, pt.y, size.cx, size.cy, nFlags | DST_COMPLEX);
	}
//#endif // !_WIN32_WCE

// Ellipse and Polygon Functions
//#ifndef _WIN32_WCE
	pub fn Chord (c_int x1, c_int y1, c_int x2, c_int y2, c_int x3, c_int y3, c_int x4, c_int y4)->BOOL {
		self.assert_dc();
		return ::Chord(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4);
	}

	pub fn Chord(&self,lpRect: LPCRECT, ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		return ::Chord(self.hdc, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom, ptStart.x, ptStart.y, ptEnd.x, ptEnd.y);
	}
//#endif // !_WIN32_WCE

	pub fn DrawFocusRect(&self,lpRect: LPCRECT) {
		self.assert_dc();
		::DrawFocusRect(self.hdc, lpRect);
	}

	pub fn Ellipse(&self,x1: c_int, y1: c_int,x2: c_int,y2: c_int)->BOOL {
		self.assert_dc();
		return ::Ellipse(self.hdc, x1, y1, x2, y2);
	}

	pub fn Ellipse(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		return ::Ellipse(self.hdc, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom);
	}

//#ifndef _WIN32_WCE
	pub fn Pie (c_int x1, c_int y1, c_int x2, c_int y2, c_int x3, c_int y3, c_int x4, c_int y4)->BOOL {
		self.assert_dc();
		return ::Pie(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4);
	}

	pub fn Pie(&self,lpRect: LPCRECT, ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		return ::Pie(self.hdc, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom, ptStart.x, ptStart.y, ptEnd.x, ptEnd.y);
	}
//#endif // !_WIN32_WCE

	pub fn Polygon (const POINT* lpPoints, c_int nCount)->BOOL {
		self.assert_dc();
		return ::Polygon(self.hdc, lpPoints, nCount);
	}

//#ifndef _WIN32_WCE
	pub fn PolyPolygon (const POINT* lpPoints, const INT* lpPolyCounts, c_int nCount)->BOOL {
		self.assert_dc();
		return ::PolyPolygon(self.hdc, lpPoints, lpPolyCounts, nCount);
	}
//#endif // !_WIN32_WCE

	pub fn Rectangle(&self,x1: c_int, y1: c_int,x2: c_int,y2: c_int)->BOOL {
		self.assert_dc();
		return ::Rectangle(self.hdc, x1, y1, x2, y2);
	}

	pub fn Rectangle(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		return ::Rectangle(self.hdc, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom);
	}

	pub fn RoundRect (c_int x1, c_int y1, c_int x2, c_int y2, c_int x3, c_int y3)->BOOL {
		self.assert_dc();
		return ::RoundRect(self.hdc, x1, y1, x2, y2, x3, y3);
	}

	pub fn RoundRect(&self,lpRect: LPCRECT, point: POINT)->BOOL {
		self.assert_dc();
		return ::RoundRect(self.hdc, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom, point.x, point.y);
	}

// Bitmap Functions
	pub fn PatBlt (c_int x, c_int y, c_int nWidth, c_int nHeight, DWORD dwRop)->BOOL {
		self.assert_dc();
		return ::PatBlt(self.hdc, x, y, nWidth, nHeight, dwRop);
	}

	BOOL BitBlt(c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC,
		c_int xSrc, c_int ySrc, DWORD dwRop)
	{
		self.assert_dc();
		return ::BitBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, dwRop);
	}

	pub fn StretchBlt (c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC, c_int xSrc, c_int ySrc, c_int nSrcWidth, c_int nSrcHeight, DWORD dwRop)->BOOL {
		self.assert_dc();
		return ::StretchBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, dwRop);
	}

	pub fn GetPixel(&self,x: c_int, y: c_int)->COLORREF {
		self.assert_dc();
		return ::GetPixel(self.hdc, x, y);
	}

	pub fn GetPixel(&self,point: POINT)->COLORREF {
		self.assert_dc();
		return ::GetPixel(self.hdc, point.x, point.y);
	}

	pub fn SetPixel(&self,x: c_int, y: c_int,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		return ::SetPixel(self.hdc, x, y, crColor);
	}

	pub fn SetPixel(&self,point: POINT, crColor: COLORREF)->COLORREF {
		self.assert_dc();
		return ::SetPixel(self.hdc, point.x, point.y, crColor);
	}

//#ifndef _WIN32_WCE
	pub fn FloodFill(&self,x: c_int, y: c_int,crColor: COLORREF)->BOOL {
		self.assert_dc();
		return ::FloodFill(self.hdc, x, y, crColor);
	}

	pub fn ExtFloodFill(&self,x: c_int, y: c_int,crColor: COLORREF,nFillType: UINT)->BOOL {
		self.assert_dc();
		return ::ExtFloodFill(self.hdc, x, y, crColor, nFillType);
	}
//#endif // !_WIN32_WCE

	pub fn MaskBlt (c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC, c_int xSrc, c_int ySrc, HBITMAP hMaskBitmap, c_int xMask, c_int yMask, DWORD dwRop)->BOOL {
		self.assert_dc();
		return ::MaskBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, hMaskBitmap, xMask, yMask, dwRop);
	}

//#ifndef _WIN32_WCE
	pub fn PlgBlt (LPPOINT lpPoint, HDC hSrcDC, c_int xSrc, c_int ySrc, c_int nWidth, c_int nHeight, HBITMAP hMaskBitmap, c_int xMask, c_int yMask)->BOOL {
		self.assert_dc();
		return ::PlgBlt(self.hdc, lpPoint, hSrcDC, xSrc, ySrc, nWidth, nHeight, hMaskBitmap, xMask, yMask);
	}

	pub fn SetPixelV(&self,x: c_int, y: c_int,crColor: COLORREF)->BOOL {
		self.assert_dc();
		return ::SetPixelV(self.hdc, x, y, crColor);
	}

	pub fn SetPixelV(&self,point: POINT, crColor: COLORREF)->BOOL {
		self.assert_dc();
		return ::SetPixelV(self.hdc, point.x, point.y, crColor);
	}
//#endif // !_WIN32_WCE

//#if !defined(_ATL_NO_MSIMG) || defined(_WIN32_WCE)
//#ifndef _WIN32_WCE
	pub fn TransparentBlt (c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC, c_int xSrc, c_int ySrc, c_int nSrcWidth, c_int nSrcHeight, UINT crTransparent)->BOOL {
		self.assert_dc();
		return ::TransparentBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, crTransparent);
	}
// #else // CE specific
// 	pub fn TransparentImage (c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC, c_int xSrc, c_int ySrc, c_int nSrcWidth, c_int nSrcHeight, UINT crTransparent)->BOOL {
// 		self.assert_dc();
// 		return ::TransparentImage(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, crTransparent);
// 	}
// #endif // _WIN32_WCE

//#if (!defined(_WIN32_WCE) || (_WIN32_WCE >= 420))
	pub fn GradientFill (const PTRIVERTEX pVertices, DWORD nVertices, void* pMeshElements, DWORD nMeshElements, DWORD dwMode)->BOOL {
		self.assert_dc();
		return ::GradientFill(self.hdc, pVertices, nVertices, pMeshElements, nMeshElements, dwMode);
	}

	pub fn GradientFillRect (RECT& rect, COLORREF clr1, COLORREF clr2, bool bHorizontal)->BOOL {
		self.assert_dc();

		TRIVERTEX arrTvx[2] = { { 0 }, { 0 } };

		arrTvx[0].x = rect.left;
		arrTvx[0].y = rect.top;
		arrTvx[0].Red = MAKEWORD(0, GetRValue(clr1));
		arrTvx[0].Green = MAKEWORD(0, GetGValue(clr1));
		arrTvx[0].Blue = MAKEWORD(0, GetBValue(clr1));
		arrTvx[0].Alpha = 0;

		arrTvx[1].x = rect.right;
		arrTvx[1].y = rect.bottom;
		arrTvx[1].Red = MAKEWORD(0, GetRValue(clr2));
		arrTvx[1].Green = MAKEWORD(0, GetGValue(clr2));
		arrTvx[1].Blue = MAKEWORD(0, GetBValue(clr2));
		arrTvx[1].Alpha = 0;

		GRADIENT_RECT gr = { 0, 1 };

		return ::GradientFill(self.hdc, arrTvx, 2, &gr, 1, bHorizontal ? GRADIENT_FILL_RECT_H : GRADIENT_FILL_RECT_V);
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 420)

//#if !defined(_WIN32_WCE) || (_WIN32_WCE > 0x500)
	pub fn AlphaBlend (c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC, c_int xSrc, c_int ySrc, c_int nSrcWidth, c_int nSrcHeight, BLENDFUNCTION bf)->BOOL {
		self.assert_dc();
		return ::AlphaBlend(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, bf);
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE > 0x500)
//#endif //  !defined(_ATL_NO_MSIMG) || defined(_WIN32_WCE)

// Extra bitmap functions
	// Helper function for painting a disabled toolbar or menu bitmap
	// This function can take either an HBITMAP (for SS) or a DC with 
	//           the bitmap already painted (for cmdbar)
	BOOL DitherBlt(c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC, HBITMAP hBitmap, c_int xSrc, c_int ySrc,
			HBRUSH hBrushBackground = ::GetSysColorBrush(COLOR_3DFACE),
			HBRUSH hBrush3DEffect = ::GetSysColorBrush(COLOR_3DHILIGHT),
			HBRUSH hBrushDisabledImage = ::GetSysColorBrush(COLOR_3DSHADOW))
	{
		debug_assert!(self.hdc != NULL || hBitmap != NULL);
		debug_assert!(nWidth > 0 && nHeight > 0);
		
		// Create a generic DC for all BitBlts
		CDCHandle dc = (hSrcDC != NULL) ? hSrcDC : ::CreateCompatibleDC(self.hdc);
		debug_assert!(dc.self.hdc != NULL);
		if(dc.self.hdc == NULL)
			return FALSE;
		
		// Create a DC for the monochrome DIB section
		CDC dcBW = ::CreateCompatibleDC(self.hdc);
		debug_assert!(dcBW.self.hdc != NULL);
		if(dcBW.self.hdc == NULL)
		{
			if(hSrcDC == NULL)
				dc.DeleteDC();
			return FALSE;
		}

		// Create the monochrome DIB section with a black and white palette
		struct RGBBWBITMAPINFO
		{
			BITMAPINFOHEADER bmiHeader; 
			RGBQUAD bmiColors[2]; 
		};

		RGBBWBITMAPINFO rgbBWBitmapInfo = 
		{
			{ sizeof(BITMAPINFOHEADER), nWidth, nHeight, 1, 1, BI_RGB, 0, 0, 0, 0, 0 },
			{ { 0x00, 0x00, 0x00, 0x00 }, { 0xFF, 0xFF, 0xFF, 0x00 } }
		};

		VOID* pbitsBW;
		CBitmap bmpBW = ::CreateDIBSection(dcBW, (LPBITMAPINFO)&rgbBWBitmapInfo, DIB_RGB_COLORS, &pbitsBW, NULL, 0);
		debug_assert!(bmpBW.m_hBitmap != NULL);
		if(bmpBW.m_hBitmap == NULL)
		{
			if(hSrcDC == NULL)
				dc.DeleteDC();
			return FALSE;
		}
		
		// Attach the monochrome DIB section and the bitmap to the DCs
		HBITMAP hbmOldBW = dcBW.SelectBitmap(bmpBW);
		HBITMAP hbmOldDC = NULL;
		if(hBitmap != NULL)
			hbmOldDC = dc.SelectBitmap(hBitmap);

		// Block: Dark gray removal: we want (128, 128, 128) pixels to become black and not white
		{
			CDC dcTemp1 = ::CreateCompatibleDC(self.hdc);
			CDC dcTemp2 = ::CreateCompatibleDC(self.hdc);
			CBitmap bmpTemp1;
			bmpTemp1.CreateCompatibleBitmap(dc, nWidth, nHeight);
			CBitmap bmpTemp2;
			bmpTemp2.CreateBitmap(nWidth, nHeight, 1, 1, NULL);
			HBITMAP hOldBmp1 = dcTemp1.SelectBitmap(bmpTemp1);
			HBITMAP hOldBmp2 = dcTemp2.SelectBitmap(bmpTemp2);
			// Let's copy our image, it will be altered
			dcTemp1.BitBlt(0, 0, nWidth, nHeight, dc, xSrc, ySrc, SRCCOPY);

			// All dark gray pixels will become white, the others black
			dcTemp1.SetBkColor(RGB(128, 128, 128));
			dcTemp2.BitBlt(0, 0, nWidth, nHeight, dcTemp1, 0, 0, SRCCOPY);
			// Do an XOR to set to black these white pixels
			dcTemp1.BitBlt(0, 0, nWidth, nHeight, dcTemp2, 0, 0, SRCINVERT);

			// BitBlt the bitmap into the monochrome DIB section
			// The DIB section will do a true monochrome conversion
			// The magenta background being closer to white will become white
			dcBW.BitBlt(0, 0, nWidth, nHeight, dcTemp1, 0, 0, SRCCOPY);

			// Cleanup
			dcTemp1.SelectBitmap(hOldBmp1);
			dcTemp2.SelectBitmap(hOldBmp2);
		}
		
		// Paint the destination rectangle using hBrushBackground
		if(hBrushBackground != NULL)
		{
			RECT rc = { x, y, x + nWidth, y + nHeight };
			FillRect(&rc, hBrushBackground);
		}

		// BitBlt the black bits in the monochrome bitmap into hBrush3DEffect color in the destination DC
		// The magic ROP comes from the Charles Petzold's book
		HBRUSH hOldBrush = SelectBrush(hBrush3DEffect);
		BitBlt(x + 1, y + 1, nWidth, nHeight, dcBW, 0, 0, 0xB8074A);

		// BitBlt the black bits in the monochrome bitmap into hBrushDisabledImage color in the destination DC
		SelectBrush(hBrushDisabledImage);
		BitBlt(x, y, nWidth, nHeight, dcBW, 0, 0, 0xB8074A);

		SelectBrush(hOldBrush);
		dcBW.SelectBitmap(hbmOldBW);
		dc.SelectBitmap(hbmOldDC);

		if(hSrcDC == NULL)
			dc.DeleteDC();

		return TRUE;
	}

// Text Functions
//#ifndef _WIN32_WCE
	pub fn TextOut (c_int x, c_int y, LPCTSTR lpszString, c_int nCount = -1)->BOOL {
		self.assert_dc();
		if(nCount == -1)
			nCount = lstrlen(lpszString);
		return ::TextOut(self.hdc, x, y, lpszString, nCount);
	}
//#endif // !_WIN32_WCE

	pub fn ExtTextOut (c_int x, c_int y, UINT nOptions, LPCRECT lpRect, LPCTSTR lpszString, UINT nCount = -1, LPINT lpDxWidths )->BOOL {
		self.assert_dc();
		if(nCount == -1)
			nCount = lstrlen(lpszString);
		return ::ExtTextOut(self.hdc, x, y, nOptions, lpRect, lpszString, nCount, lpDxWidths);
	}

//#ifndef _WIN32_WCE
	pub fn TabbedTextOut (c_int x, c_int y, LPCTSTR lpszString, c_int nCount = -1, c_int nTabPositions = 0, LPINT lpnTabStopPositions , c_int nTabOrigin = 0)->SIZE {
		self.assert_dc();
		if(nCount == -1)
			nCount = lstrlen(lpszString);
		LONG lRes = ::TabbedTextOut(self.hdc, x, y, lpszString, nCount, nTabPositions, lpnTabStopPositions, nTabOrigin);
		SIZE size = { GET_X_LPARAM(lRes), GET_Y_LPARAM(lRes) };
		return size;
	}
//#endif // !_WIN32_WCE

	pub fn DrawText(&self,lpstrText: LPCTSTR, cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
		self.assert_dc();
//#ifndef _WIN32_WCE
		debug_assert!((uFormat & DT_MODIFYSTRING) == 0);
//#endif // !_WIN32_WCE
		return ::DrawText(self.hdc, lpstrText, cchText, lpRect, uFormat);
	}

	pub fn DrawText(&self,lpstrText: LPTSTR, cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
		self.assert_dc();
		return ::DrawText(self.hdc, lpstrText, cchText, lpRect, uFormat);
	}

//#ifndef _WIN32_WCE
	pub fn DrawTextEx (LPTSTR lpstrText, c_int cchText, LPRECT lpRect, UINT uFormat, LPDRAWTEXTPARAMS lpDTParams )->c_int {
		self.assert_dc();
		return ::DrawTextEx(self.hdc, lpstrText, cchText, lpRect, uFormat, lpDTParams);
	}
//#endif // !_WIN32_WCE

//#if (_WIN32_WINNT >= 0x0501)
	pub fn DrawShadowText (LPCWSTR lpstrText, c_int cchText, LPRECT lpRect, DWORD dwFlags, COLORREF clrText, COLORREF clrShadow, c_int xOffset, c_int yOffset)->c_int {
		self.assert_dc();
		// This function is present only if comctl32.dll version 6 is loaded;
		// we use LoadLibrary/GetProcAddress to allow apps compiled with
		// _WIN32_WINNT >= 0x0501 to run on older Windows/CommCtrl
		c_int nRet = 0;
		HMODULE hCommCtrlDLL = ::LoadLibrary(_T("comctl32.dll"));
		debug_assert!(hCommCtrlDLL != NULL);
		if(hCommCtrlDLL != NULL)
		{
			typedef c_int (WINAPI *PFN_DrawShadowText)(HDC hDC, LPCWSTR lpstrText, UINT cchText, LPRECT lpRect, DWORD dwFlags, COLORREF clrText, COLORREF clrShadow, c_int xOffset, c_int yOffset);
			PFN_DrawShadowText pfnDrawShadowText = (PFN_DrawShadowText)::GetProcAddress(hCommCtrlDLL, "DrawShadowText");
			debug_assert!(pfnDrawShadowText != NULL);   // this function requires CommCtrl6
			if(pfnDrawShadowText != NULL)
				nRet = pfnDrawShadowText(self.hdc, lpstrText, cchText, lpRect, dwFlags, clrText, clrShadow, xOffset, yOffset);
			::FreeLibrary(hCommCtrlDLL);
		}
		return nRet;
	}
//#endif // (_WIN32_WINNT >= 0x0501)

	pub fn GetTextExtent(&self,lpszString: LPCTSTR, nCount: c_int,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		if(nCount == -1)
			nCount = lstrlen(lpszString);
		return ::GetTextExtentPoint32(self.hdc, lpszString, nCount, lpSize);
	}

	pub fn GetTextExtentExPoint (LPCTSTR lpszString, c_int cchString, LPSIZE lpSize, c_int nMaxExtent, LPINT lpnFit , LPINT alpDx )->BOOL {
		self.assert_dc();
		return ::GetTextExtentExPoint(self.hdc, lpszString, cchString, nMaxExtent, lpnFit, alpDx, lpSize);
	}

//#ifndef _WIN32_WCE
	pub fn GetTabbedTextExtent (LPCTSTR lpszString, c_int nCount = -1, c_int nTabPositions = 0, LPINT lpnTabStopPositions )->DWORD {
		self.assert_dc();
		if(nCount == -1)
			nCount = lstrlen(lpszString);
		return ::GetTabbedTextExtent(self.hdc, lpszString, nCount, nTabPositions, lpnTabStopPositions);
	}

	pub fn GrayString (HBRUSH hBrush, BOOL (CALLBACK* lpfnOutput)(HDC, LPARAM, c_int), LPARAM lpData, c_int nCount, c_int x, c_int y, c_int nWidth, c_int nHeight)->BOOL {
		self.assert_dc();
		return ::GrayString(self.hdc, hBrush, (GRAYSTRINGPROC)lpfnOutput, lpData, nCount, x, y, nWidth, nHeight);
	}
//#endif // !_WIN32_WCE

//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)
	pub fn GetTextAlign(&self)->UINT {
		self.assert_dc();
		return ::GetTextAlign(self.hdc);
	}

	pub fn SetTextAlign(&self,nFlags: UINT)->UINT {
		self.assert_dc();
		return ::SetTextAlign(self.hdc, nFlags);
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)

	pub fn GetTextFace(&self,lpszFacename: LPTSTR, nCount: c_int)->c_int {
		self.assert_dc();
		return ::GetTextFace(self.hdc, nCount, lpszFacename);
	}

	pub fn GetTextFaceLen(&self)->c_int {
		self.assert_dc();
		return ::GetTextFace(self.hdc, 0, NULL);
	}

// #ifndef _ATL_NO_COM
// #ifdef _OLEAUTO_H_
// 	pub fn GetTextFace (BSTR& bstrFace)->BOOL {
// 		USES_CONVERSION;
// 		self.assert_dc();
// 		debug_assert!(bstrFace == NULL);

// 		c_int nLen = GetTextFaceLen();
// 		if(nLen == 0)
// 			return FALSE;

// 		CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
// 		LPTSTR lpszText = buff.Allocate(nLen);
// 		if(lpszText == NULL)
// 			return FALSE;

// 		if(!GetTextFace(lpszText, nLen))
// 			return FALSE;

// 		bstrFace = ::SysAllocString(T2OLE(lpszText));
// 		return (bstrFace != NULL) ? TRUE : FALSE;
// 	}
// #endif
// #endif // !_ATL_NO_COM

// #if defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)
// 	pub fn GetTextFace (_CSTRING_NS::CString& strFace)->c_int {
// 		self.assert_dc();

// 		c_int nLen = GetTextFaceLen();
// 		if(nLen == 0)
// 			return 0;

// 		LPTSTR lpstr = strFace.GetBufferSetLength(nLen);
// 		if(lpstr == NULL)
// 			return 0;
// 		c_int nRet = GetTextFace(lpstr, nLen);
// 		strFace.ReleaseBuffer();
// 		return nRet;
// 	}
// #endif // defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)

	pub fn GetTextMetrics(&self,lpMetrics: LPTEXTMETRIC)->BOOL {
		self.assert_dc();
		return ::GetTextMetrics(self.hdc, lpMetrics);
	}

//#ifndef _WIN32_WCE
	pub fn SetTextJustification(&self,nBreakExtra: c_int, nBreakCount: c_int)->c_int {
		self.assert_dc();
		return ::SetTextJustification(self.hdc, nBreakExtra, nBreakCount);
	}

	pub fn GetTextCharacterExtra(&self)->c_int {
		self.assert_dc();
		return ::GetTextCharacterExtra(self.hdc);
	}

	pub fn SetTextCharacterExtra(&self,nCharExtra: c_int)->c_int {
		self.assert_dc();
		return ::SetTextCharacterExtra(self.hdc, nCharExtra);
	}
//#endif // !_WIN32_WCE

// Advanced Drawing
	pub fn DrawEdge(&self,lpRect: LPRECT, nEdge: UINT,nFlags: UINT)->BOOL {
		self.assert_dc();
		return ::DrawEdge(self.hdc, lpRect, nEdge, nFlags);
	}

	pub fn DrawFrameControl(&self,lpRect: LPRECT, nType: UINT,nState: UINT)->BOOL {
		self.assert_dc();
		return ::DrawFrameControl(self.hdc, lpRect, nType, nState);
	}

// Scrolling Functions
	pub fn ScrollDC (c_int dx, c_int dy, LPCRECT lpRectScroll, LPCRECT lpRectClip, HRGN hRgnUpdate, LPRECT lpRectUpdate)->BOOL {
		self.assert_dc();
		return ::ScrollDC(self.hdc, dx, dy, lpRectScroll, lpRectClip, hRgnUpdate, lpRectUpdate);
	}

// Font Functions
//#ifndef _WIN32_WCE
	pub fn GetCharWidth(&self,nFirstChar: UINT, nLastChar: UINT,lpBuffer: LPINT)->BOOL {
		self.assert_dc();
		return ::GetCharWidth(self.hdc, nFirstChar, nLastChar, lpBuffer);
	}

	// GetCharWidth32 is not supported under Win9x
	pub fn GetCharWidth32(&self,nFirstChar: UINT, nLastChar: UINT,lpBuffer: LPINT)->BOOL {
		self.assert_dc();
		return ::GetCharWidth32(self.hdc, nFirstChar, nLastChar, lpBuffer);
	}

	pub fn SetMapperFlags(&self,dwFlag: DWORD)->DWORD {
		self.assert_dc();
		return ::SetMapperFlags(self.hdc, dwFlag);
	}

	pub fn GetAspectRatioFilter(&self,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		return ::GetAspectRatioFilterEx(self.hdc, lpSize);
	}

	pub fn GetCharABCWidths(&self,nFirstChar: UINT, nLastChar: UINT,lpabc: LPABC)->BOOL {
		self.assert_dc();
		return ::GetCharABCWidths(self.hdc, nFirstChar, nLastChar, lpabc);
	}

	pub fn GetFontData(&self,dwTable: DWORD, dwOffset: DWORD,lpData: LPVOID,cbData: DWORD)->DWORD {
		self.assert_dc();
		return ::GetFontData(self.hdc, dwTable, dwOffset, lpData, cbData);
	}

	pub fn GetKerningPairs(&self,nPairs: c_int, lpkrnpair: LPKERNINGPAIR)->c_int {
		self.assert_dc();
		return ::GetKerningPairs(self.hdc, nPairs, lpkrnpair);
	}

	pub fn GetOutlineTextMetrics(&self,cbData: UINT, lpotm: LPOUTLINETEXTMETRIC)->UINT {
		self.assert_dc();
		return ::GetOutlineTextMetrics(self.hdc, cbData, lpotm);
	}

	pub fn GetGlyphOutline (UINT nChar, UINT nFormat, LPGLYPHMETRICS lpgm, DWORD cbBuffer, LPVOID lpBuffer, const MAT2* lpmat2)->DWORD {
		self.assert_dc();
		return ::GetGlyphOutline(self.hdc, nChar, nFormat, lpgm, cbBuffer, lpBuffer, lpmat2);
	}

	pub fn GetCharABCWidths(&self,nFirstChar: UINT, nLastChar: UINT,lpABCF: LPABCFLOAT)->BOOL {
		self.assert_dc();
		return ::GetCharABCWidthsFloat(self.hdc, nFirstChar, nLastChar, lpABCF);
	}

	pub fn GetCharWidth (UINT nFirstChar, UINT nLastChar, float* lpFloatBuffer)->BOOL {
		self.assert_dc();
		return ::GetCharWidthFloat(self.hdc, nFirstChar, nLastChar, lpFloatBuffer);
	}
//#endif // !_WIN32_WCE

// Printer/Device Escape Functions
//#ifndef _WIN32_WCE
	pub fn Escape(&self,nEscape: c_int, nCount: c_int,lpszInData: LPCSTR,lpOutData: LPVOID)->c_int {
		self.assert_dc();
		return ::Escape(self.hdc, nEscape, nCount, lpszInData, lpOutData);
	}
//#endif // !_WIN32_WCE

	c_int Escape(c_int nEscape, c_int nInputSize, LPCSTR lpszInputData,
		c_int nOutputSize, LPSTR lpszOutputData)
	{
		self.assert_dc();
		return ::ExtEscape(self.hdc, nEscape, nInputSize, lpszInputData, nOutputSize, lpszOutputData);
	}

//#ifndef _WIN32_WCE
	pub fn DrawEscape(&self,nEscape: c_int, nInputSize: c_int,lpszInputData: LPCSTR)->c_int {
		self.assert_dc();
		return ::DrawEscape(self.hdc, nEscape, nInputSize, lpszInputData);
	}
//#endif // !_WIN32_WCE

	// Escape helpers
//#if !defined(_WIN32_WCE) || ((_WIN32_WCE >= 200) && defined(StartDoc))
	c_int StartDoc(LPCTSTR lpszDocName)  // old Win3.0 version
	{
		DOCINFO di = { 0 };
		di.cbSize = sizeof(DOCINFO);
		di.lpszDocName = lpszDocName;
		return StartDoc(&di);
	}

	pub fn StartDoc(&self,lpDocInfo: LPDOCINFO)->c_int {
		self.assert_dc();
		return ::StartDoc(self.hdc, lpDocInfo);
	}

	pub fn StartPage(&self)->c_int {
		self.assert_dc();
		return ::StartPage(self.hdc);
	}

	pub fn EndPage(&self)->c_int {
		self.assert_dc();
		return ::EndPage(self.hdc);
	}

	pub fn SetAbortProc (BOOL (CALLBACK* lpfn)(HDC, c_int))->c_int {
		self.assert_dc();
		return ::SetAbortProc(self.hdc, (ABORTPROC)lpfn);
	}

	pub fn AbortDoc(&self)->c_int {
		self.assert_dc();
		return ::AbortDoc(self.hdc);
	}

	pub fn EndDoc(&self)->c_int {
		self.assert_dc();
		return ::EndDoc(self.hdc);
	}
//#endif // !defined(_WIN32_WCE) || ((_WIN32_WCE >= 200) && defined(StartDoc))

// MetaFile Functions
//#ifndef _WIN32_WCE
	pub fn PlayMetaFile(&self,hMF: HMETAFILE)->BOOL {
		self.assert_dc();
		if(::GetDeviceCaps(self.hdc, TECHNOLOGY) == DT_METAFILE)
		{
			// playing metafile in metafile, just use core windows API
			return ::PlayMetaFile(self.hdc, hMF);
		}

		// for special playback, lParam == pDC
		return ::EnumMetaFile(self.hdc, hMF, EnumMetaFileProc, this as LPARAM);
	}

	pub fn PlayMetaFile(&self,hEnhMetaFile: HENHMETAFILE, lpBounds: LPCRECT)->BOOL {
		self.assert_dc();
		return ::PlayEnhMetaFile(self.hdc, hEnhMetaFile, lpBounds);
	}

	BOOL AddMetaFileComment(UINT nDataSize, const BYTE* pCommentData) // can be used for enhanced metafiles only
	{
		self.assert_dc();
		return ::GdiComment(self.hdc, nDataSize, pCommentData);
	}

	// Special handling for metafile playback
	static c_int CALLBACK EnumMetaFileProc(HDC hDC, HANDLETABLE* pHandleTable, METARECORD* pMetaRec, c_int nHandles, LPARAM lParam)
	{
		CDCHandle* pDC = (CDCHandle*)lParam;

		switch (pMetaRec->rdFunction)
		{
		case META_SETMAPMODE:
			pDC->SetMapMode((c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETWINDOWEXT:
			pDC->SetWindowExt((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETWINDOWORG:
			pDC->SetWindowOrg((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETVIEWPORTEXT:
			pDC->SetViewportExt((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETVIEWPORTORG:
			pDC->SetViewportOrg((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SCALEWINDOWEXT:
			pDC->ScaleWindowExt((c_int)(short)pMetaRec->rdParm[3], (c_int)(short)pMetaRec->rdParm[2], 
				(c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SCALEVIEWPORTEXT:
			pDC->ScaleViewportExt((c_int)(short)pMetaRec->rdParm[3], (c_int)(short)pMetaRec->rdParm[2],
				(c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_OFFSETVIEWPORTORG:
			pDC->OffsetViewportOrg((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SAVEDC:
			pDC->SaveDC();
			break;
		case META_RESTOREDC:
			pDC->RestoreDC((c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETBKCOLOR:
			pDC->SetBkColor(*(UNALIGNED COLORREF*)&pMetaRec->rdParm[0]);
			break;
		case META_SETTEXTCOLOR:
			pDC->SetTextColor(*(UNALIGNED COLORREF*)&pMetaRec->rdParm[0]);
			break;

		// need to watch out for SelectObject(HFONT), for custom font mapping
		case META_SELECTOBJECT:
			{
				HGDIOBJ hObject = pHandleTable->objectHandle[pMetaRec->rdParm[0]];
				UINT nObjType = ::GetObjectType(hObject);
				if(nObjType == 0)
				{
					// object type is unknown, determine if it is a font
					HFONT hStockFont = (HFONT)::GetStockObject(SYSTEM_FONT);
					HFONT hFontOld = (HFONT)::SelectObject(pDC->self.hdc, hStockFont);
					HGDIOBJ hObjOld = ::SelectObject(pDC->self.hdc, hObject);
					if(hObjOld == hStockFont)
					{
						// got the stock object back, so must be selecting a font
						pDC->SelectFont((HFONT)hObject);
						break;  // don't play the default record
					}
					else
					{
						// didn't get the stock object back, so restore everything
						::SelectObject(pDC->self.hdc, hFontOld);
						::SelectObject(pDC->self.hdc, hObjOld);
					}
					// and fall through to PlayMetaFileRecord...
				}
				else if(nObjType == OBJ_FONT)
				{
					// play back as CDCHandle::SelectFont(HFONT)
					pDC->SelectFont((HFONT)hObject);
					break;  // don't play the default record
				}
			}
			// fall through...

		default:
			::PlayMetaFileRecord(hDC, pHandleTable, pMetaRec, nHandles);
			break;
		}

		return 1;
	}
//#endif // !_WIN32_WCE

// Path Functions
//#ifndef _WIN32_WCE
	pub fn AbortPath(&self)->BOOL {
		self.assert_dc();
		return ::AbortPath(self.hdc);
	}

	pub fn BeginPath(&self)->BOOL {
		self.assert_dc();
		return ::BeginPath(self.hdc);
	}

	pub fn CloseFigure(&self)->BOOL {
		self.assert_dc();
		return ::CloseFigure(self.hdc);
	}

	pub fn EndPath(&self)->BOOL {
		self.assert_dc();
		return ::EndPath(self.hdc);
	}

	pub fn FillPath(&self)->BOOL {
		self.assert_dc();
		return ::FillPath(self.hdc);
	}

	pub fn FlattenPath(&self)->BOOL {
		self.assert_dc();
		return ::FlattenPath(self.hdc);
	}

	pub fn StrokeAndFillPath(&self)->BOOL {
		self.assert_dc();
		return ::StrokeAndFillPath(self.hdc);
	}

	pub fn StrokePath(&self)->BOOL {
		self.assert_dc();
		return ::StrokePath(self.hdc);
	}

	pub fn WidenPath(&self)->BOOL {
		self.assert_dc();
		return ::WidenPath(self.hdc);
	}

	pub fn GetMiterLimit(&self,pfMiterLimit: PFLOAT)->BOOL {
		self.assert_dc();
		return ::GetMiterLimit(self.hdc, pfMiterLimit);
	}

	pub fn SetMiterLimit(&self,fMiterLimit: float)->BOOL {
		self.assert_dc();
		return ::SetMiterLimit(self.hdc, fMiterLimit, NULL);
	}

	pub fn GetPath(&self,lpPoints: LPPOINT, lpTypes: LPBYTE,nCount: c_int)->c_int {
		self.assert_dc();
		return ::GetPath(self.hdc, lpPoints, lpTypes, nCount);
	}

	pub fn SelectClipPath(&self,nMode: c_int)->BOOL {
		self.assert_dc();
		return ::SelectClipPath(self.hdc, nMode);
	}
//#endif // !_WIN32_WCE

// Misc Helper Functions
	static CBrushHandle PASCAL GetHalftoneBrush()
	{
		HBRUSH halftoneBrush = NULL;
		WORD grayPattern[8] = { 0 };
		for(c_int i = 0; i < 8; i++)
			grayPattern[i] = (WORD)(0x5555 << (i & 1));
		HBITMAP grayBitmap = CreateBitmap(8, 8, 1, 1, &grayPattern);
		if(grayBitmap != NULL)
		{
			halftoneBrush = ::CreatePatternBrush(grayBitmap);
			DeleteObject(grayBitmap);
		}
		return CBrushHandle(halftoneBrush);
	}

	pub fn DrawDragRect (LPCRECT lpRect, SIZE size, LPCRECT lpRectLast, SIZE sizeLast, HBRUSH hBrush , HBRUSH hBrushLast ) {
		// first, determine the update region and select it
		CRgn rgnOutside;
		rgnOutside.CreateRectRgnIndirect(lpRect);
		RECT rect = *lpRect;
		::InflateRect(&rect, -size.cx, -size.cy);
		::IntersectRect(&rect, &rect, lpRect);
		CRgn rgnInside;
		rgnInside.CreateRectRgnIndirect(&rect);
		CRgn rgnNew;
		rgnNew.CreateRectRgn(0, 0, 0, 0);
		rgnNew.CombineRgn(rgnOutside, rgnInside, RGN_XOR);

		HBRUSH hBrushOld = NULL;
		CBrush brushHalftone;
		if(hBrush == NULL)
			brushHalftone = hBrush = CDCHandle::GetHalftoneBrush();
		if(hBrushLast == NULL)
			hBrushLast = hBrush;

		CRgn rgnLast;
		CRgn rgnUpdate;
		if(lpRectLast != NULL)
		{
			// find difference between new region and old region
			rgnLast.CreateRectRgn(0, 0, 0, 0);
			rgnOutside.SetRectRgn(lpRectLast->left, lpRectLast->top, lpRectLast->right, lpRectLast->bottom);
			rect = *lpRectLast;
			::InflateRect(&rect, -sizeLast.cx, -sizeLast.cy);
			::IntersectRect(&rect, &rect, lpRectLast);
			rgnInside.SetRectRgn(rect.left, rect.top, rect.right, rect.bottom);
			rgnLast.CombineRgn(rgnOutside, rgnInside, RGN_XOR);

			// only diff them if brushes are the same
			if(hBrush == hBrushLast)
			{
				rgnUpdate.CreateRectRgn(0, 0, 0, 0);
				rgnUpdate.CombineRgn(rgnLast, rgnNew, RGN_XOR);
			}
		}
		if(hBrush != hBrushLast && lpRectLast != NULL)
		{
			// brushes are different -- erase old region first
			SelectClipRgn(rgnLast);
			GetClipBox(&rect);
			hBrushOld = SelectBrush(hBrushLast);
			PatBlt(rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, PATINVERT);
			SelectBrush(hBrushOld);
			hBrushOld = NULL;
		}

		// draw into the update/new region
		SelectClipRgn(rgnUpdate.IsNull() ? rgnNew : rgnUpdate);
		GetClipBox(&rect);
		hBrushOld = SelectBrush(hBrush);
		PatBlt(rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, PATINVERT);

		// cleanup DC
		if(hBrushOld != NULL)
			SelectBrush(hBrushOld);
		SelectClipRgn(NULL);
	}

	pub fn FillSolidRect(&self,lpRect: LPCRECT, clr: COLORREF) {
		self.assert_dc();

		COLORREF clrOld = ::SetBkColor(self.hdc, clr);
		debug_assert!(clrOld != CLR_INVALID);
		if(clrOld != CLR_INVALID)
		{
			::ExtTextOut(self.hdc, 0, 0, ETO_OPAQUE, lpRect, NULL, 0, NULL);
			::SetBkColor(self.hdc, clrOld);
		}
	}

	pub fn FillSolidRect (c_int x, c_int y, c_int cx, c_int cy, COLORREF clr) {
		self.assert_dc();

		RECT rect = { x, y, x + cx, y + cy };
		FillSolidRect(&rect, clr);
	}

	pub fn Draw3dRect(&self,lpRect: LPCRECT, clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		Draw3dRect(lpRect->left, lpRect->top, lpRect->right - lpRect->left,
			lpRect->bottom - lpRect->top, clrTopLeft, clrBottomRight);
	}

	pub fn Draw3dRect (c_int x, c_int y, c_int cx, c_int cy, COLORREF clrTopLeft, COLORREF clrBottomRight) {
		FillSolidRect(x, y, cx - 1, 1, clrTopLeft);
		FillSolidRect(x, y, 1, cy - 1, clrTopLeft);
		FillSolidRect(x + cx, y, -1, cy, clrBottomRight);
		FillSolidRect(x, y + cy, cx, -1, clrBottomRight);
	}

// DIB support
//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 410)
	pub fn SetDIBitsToDevice (c_int x, c_int y, DWORD dwWidth, DWORD dwHeight, c_int xSrc, c_int ySrc, UINT uStartScan, UINT cScanLines, CONST VOID* lpvBits, CONST BITMAPINFO* lpbmi, UINT uColorUse)->c_int {
		self.assert_dc();
		return ::SetDIBitsToDevice(self.hdc, x, y, dwWidth, dwHeight, xSrc, ySrc, uStartScan, cScanLines, lpvBits, lpbmi, uColorUse);
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 410)

//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)
	pub fn StretchDIBits (c_int x, c_int y, c_int nWidth, c_int nHeight, c_int xSrc, c_int ySrc, c_int nSrcWidth, c_int nSrcHeight, CONST VOID* lpvBits, CONST BITMAPINFO* lpbmi, UINT uColorUse, DWORD dwRop)->c_int {
		self.assert_dc();
		return ::StretchDIBits(self.hdc, x, y, nWidth, nHeight, xSrc, ySrc, nSrcWidth, nSrcHeight, lpvBits, lpbmi, uColorUse, dwRop);
	}

	pub fn GetDIBColorTable (UINT uStartIndex, UINT cEntries, RGBQUAD* pColors)->UINT {
		self.assert_dc();
		return ::GetDIBColorTable(self.hdc, uStartIndex, cEntries, pColors);
	}

	pub fn SetDIBColorTable (UINT uStartIndex, UINT cEntries, CONST RGBQUAD* pColors)->UINT {
		self.assert_dc();
		return ::SetDIBColorTable(self.hdc, uStartIndex, cEntries, pColors);
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)

// OpenGL support
//#if !defined(_ATL_NO_OPENGL) && !defined(_WIN32_WCE)
	pub fn ChoosePixelFormat (CONST PIXELFORMATDESCRIPTOR* ppfd)->c_int {
		self.assert_dc();
		return ::ChoosePixelFormat(self.hdc, ppfd);
	}

	pub fn DescribePixelFormat(&self,iPixelFormat: c_int, nBytes: UINT,ppfd: LPPIXELFORMATDESCRIPTOR)->c_int {
		self.assert_dc();
		return ::DescribePixelFormat(self.hdc, iPixelFormat, nBytes, ppfd);
	}

	pub fn GetPixelFormat(&self)->c_int {
		self.assert_dc();
		return ::GetPixelFormat(self.hdc);
	}

	pub fn SetPixelFormat (c_int iPixelFormat, CONST PIXELFORMATDESCRIPTOR* ppfd)->BOOL {
		self.assert_dc();
		return ::SetPixelFormat(self.hdc, iPixelFormat, ppfd);
	}

	pub fn SwapBuffers(&self)->BOOL {
		self.assert_dc();
		return ::SwapBuffers(self.hdc);
	}

	pub fn wglCreateContext(&self)->HGLRC {
		self.assert_dc();
		return ::wglCreateContext(self.hdc);
	}

	pub fn wglCreateLayerContext(&self,iLayerPlane: c_int)->HGLRC {
		self.assert_dc();
		return ::wglCreateLayerContext(self.hdc, iLayerPlane);
	}

	pub fn wglMakeCurrent(&self,hglrc: HGLRC)->BOOL {
		self.assert_dc();
		return ::wglMakeCurrent(self.hdc, hglrc);
	}

	pub fn wglUseFontBitmaps(&self,dwFirst: DWORD, dwCount: DWORD,listBase: DWORD)->BOOL {
		self.assert_dc();
		return ::wglUseFontBitmaps(self.hdc, dwFirst, dwCount, listBase);
	}

	pub fn wglUseFontOutlines (DWORD dwFirst, DWORD dwCount, DWORD listBase, FLOAT deviation, FLOAT extrusion, c_int format, LPGLYPHMETRICSFLOAT lpgmf)->BOOL {
		self.assert_dc();
		return ::wglUseFontOutlines(self.hdc, dwFirst, dwCount, listBase, deviation, extrusion, format, lpgmf);
	}

	pub fn wglDescribeLayerPlane(&self,iPixelFormat: c_int, iLayerPlane: c_int,nBytes: UINT,plpd: LPLAYERPLANEDESCRIPTOR)->BOOL {
		self.assert_dc();
		return ::wglDescribeLayerPlane(self.hdc, iPixelFormat, iLayerPlane, nBytes, plpd);
	}

	pub fn wglSetLayerPaletteEntries (c_int iLayerPlane, c_int iStart, c_int cEntries, CONST COLORREF* pclr)->c_int {
		self.assert_dc();
		return ::wglSetLayerPaletteEntries(self.hdc, iLayerPlane, iStart, cEntries, pclr);
	}

	pub fn wglGetLayerPaletteEntries (c_int iLayerPlane, c_int iStart, c_int cEntries, COLORREF* pclr)->c_int {
		self.assert_dc();
		return ::wglGetLayerPaletteEntries(self.hdc, iLayerPlane, iStart, cEntries, pclr);
	}

	pub fn wglRealizeLayerPalette(&self,iLayerPlane: c_int, bRealize: BOOL)->BOOL {
		self.assert_dc();
		return ::wglRealizeLayerPalette(self.hdc, iLayerPlane, bRealize);
	}

	pub fn wglSwapLayerBuffers(&self,uPlanes: UINT)->BOOL {
		self.assert_dc();
		return ::wglSwapLayerBuffers(self.hdc, uPlanes);
	}
//#endif // !defined(_ATL_NO_OPENGL) && !defined(_WIN32_WCE)

// New for Windows 2000 only
//#if (_WIN32_WINNT >= 0x0500)
	pub fn GetDCPenColor(&self)->COLORREF {
		self.assert_dc();
		return ::GetDCPenColor(self.hdc);
	}

	pub fn SetDCPenColor(&self,clr: COLORREF)->COLORREF {
		self.assert_dc();
		return ::SetDCPenColor(self.hdc, clr);
	}

	pub fn GetDCBrushColor(&self)->COLORREF {
		self.assert_dc();
		return ::GetDCBrushColor(self.hdc);
	}

	pub fn SetDCBrushColor(&self,clr: COLORREF)->COLORREF {
		self.assert_dc();
		return ::SetDCBrushColor(self.hdc, clr);
	}

//#ifndef _WIN32_WCE
	pub fn GetFontUnicodeRanges(&self,lpgs: LPGLYPHSET)->DWORD {
		self.assert_dc();
		return ::GetFontUnicodeRanges(self.hdc, lpgs);
	}
//#endif // !_WIN32_WCE

	pub fn GetGlyphIndices(&self,lpstr: LPCTSTR, cch: c_int,pgi: LPWORD,dwFlags: DWORD)->DWORD {
		self.assert_dc();
		return ::GetGlyphIndices(self.hdc, lpstr, cch, pgi, dwFlags);
	}

	pub fn GetTextExtentPointI(&self,pgiIn: LPWORD, cgi: c_int,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		return ::GetTextExtentPointI(self.hdc, pgiIn, cgi, lpSize);
	}

	pub fn GetTextExtentExPointI (LPWORD pgiIn, c_int cgi, c_int nMaxExtent, LPINT lpnFit, LPINT alpDx, LPSIZE lpSize)->BOOL {
		self.assert_dc();
		return ::GetTextExtentExPointI(self.hdc, pgiIn, cgi, nMaxExtent, lpnFit, alpDx, lpSize);
	}

	pub fn GetCharWidthI(&self,giFirst: UINT, cgi: UINT,pgi: LPWORD,lpBuffer: LPINT)->BOOL {
		self.assert_dc();
		return ::GetCharWidthI(self.hdc, giFirst, cgi, pgi, lpBuffer);
	}

	pub fn GetCharABCWidthsI(&self,giFirst: UINT, cgi: UINT,pgi: LPWORD,lpabc: LPABC)->BOOL {
		self.assert_dc();
		return ::GetCharABCWidthsI(self.hdc, giFirst, cgi, pgi, lpabc);
	}
//#endif // (_WIN32_WINNT >= 0x0500)

// New for Windows 2000 and Windows 98
//#if (WINVER >= 0x0500) && !defined(_WIN32_WCE)
	pub fn ColorCorrectPalette(&self,hPalette: HPALETTE, dwFirstEntry: DWORD,dwNumOfEntries: DWORD)->BOOL {
		self.assert_dc();
		return ::ColorCorrectPalette(self.hdc, hPalette, dwFirstEntry, dwNumOfEntries);
	}
//#endif // (WINVER >= 0x0500) && !defined(_WIN32_WCE)
}

//typedef CDCT<false>   CDCHandle;
//typedef CDCT<true>    CDC;