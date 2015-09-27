class CPaintDC : public CDC
{
public:
// Data members
	HWND m_hWnd;
	PAINTSTRUCT m_ps;

// Constructor/destructor
	CPaintDC(HWND hWnd)
	{
		ATLASSERT(::IsWindow(hWnd));
		m_hWnd = hWnd;
		m_hDC = ::BeginPaint(hWnd, &m_ps);
	}

	~CPaintDC()
	{
		ATLASSERT(m_hDC != NULL);
		ATLASSERT(::IsWindow(m_hWnd));
		::EndPaint(m_hWnd, &m_ps);
		Detach();
	}
};

class CClientDC : public CDC
{
public:
// Data members
	HWND m_hWnd;

// Constructor/destructor
	CClientDC(HWND hWnd)
	{
		ATLASSERT(hWnd == NULL || ::IsWindow(hWnd));
		m_hWnd = hWnd;
		m_hDC = ::GetDC(hWnd);
	}

	~CClientDC()
	{
		ATLASSERT(m_hDC != NULL);
		::ReleaseDC(m_hWnd, Detach());
	}
};

class CWindowDC : public CDC
{
public:
// Data members
	HWND m_hWnd;

// Constructor/destructor
	CWindowDC(HWND hWnd)
	{
		ATLASSERT(hWnd == NULL || ::IsWindow(hWnd));
		m_hWnd = hWnd;
		m_hDC = ::GetWindowDC(hWnd);
	}

	~CWindowDC()
	{
		ATLASSERT(m_hDC != NULL);
		::ReleaseDC(m_hWnd, Detach());
	}
};

class CMemoryDC : public CDC
{
public:
// Data members
	HDC m_hDCOriginal;
	RECT m_rcPaint;
	CBitmap m_bmp;
	HBITMAP m_hBmpOld;

// Constructor/destructor
	CMemoryDC(HDC hDC, const RECT& rcPaint) : m_hDCOriginal(hDC), m_hBmpOld(NULL)
	{
		m_rcPaint = rcPaint;
		CreateCompatibleDC(m_hDCOriginal);
		ATLASSERT(m_hDC != NULL);
		m_bmp.CreateCompatibleBitmap(m_hDCOriginal, m_rcPaint.right - m_rcPaint.left, m_rcPaint.bottom - m_rcPaint.top);
		ATLASSERT(m_bmp.m_hBitmap != NULL);
		m_hBmpOld = SelectBitmap(m_bmp);
		SetViewportOrg(-m_rcPaint.left, -m_rcPaint.top);
	}

	~CMemoryDC()
	{
		::BitBlt(m_hDCOriginal, m_rcPaint.left, m_rcPaint.top, m_rcPaint.right - m_rcPaint.left, m_rcPaint.bottom - m_rcPaint.top, m_hDC, m_rcPaint.left, m_rcPaint.top, SRCCOPY);
		SelectBitmap(m_hBmpOld);
	}
};
