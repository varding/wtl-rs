class CPenT
{
public:
// Data members
	HPEN m_hPen;

// Constructor/destructor/operators
	CPenT(HPEN hPen = NULL) : m_hPen(hPen)
	{ }

	~CPenT()
	{
		if(t_bManaged && m_hPen != NULL)
			DeleteObject();
	}

	CPenT<t_bManaged>& operator =(HPEN hPen)
	{
		Attach(hPen);
		return *this;
	}

	void Attach(HPEN hPen)
	{
		if(t_bManaged && m_hPen != NULL && m_hPen != hPen)
			::DeleteObject(m_hPen);
		m_hPen = hPen;
	}

	HPEN Detach()
	{
		HPEN hPen = m_hPen;
		m_hPen = NULL;
		return hPen;
	}

	operator HPEN() const { return m_hPen; }

	bool IsNull() const { return (m_hPen == NULL); }

// Create methods
	HPEN CreatePen(int nPenStyle, int nWidth, COLORREF crColor)
	{
		ATLASSERT(m_hPen == NULL);
		m_hPen = ::CreatePen(nPenStyle, nWidth, crColor);
		return m_hPen;
	}

#ifndef _WIN32_WCE
	HPEN CreatePen(int nPenStyle, int nWidth, const LOGBRUSH* pLogBrush, int nStyleCount = 0, const DWORD* lpStyle = NULL)
	{
		ATLASSERT(m_hPen == NULL);
		m_hPen = ::ExtCreatePen(nPenStyle, nWidth, pLogBrush, nStyleCount, lpStyle);
		return m_hPen;
	}
#endif // !_WIN32_WCE

	HPEN CreatePenIndirect(LPLOGPEN lpLogPen)
	{
		ATLASSERT(m_hPen == NULL);
		m_hPen = ::CreatePenIndirect(lpLogPen);
		return m_hPen;
	}

	BOOL DeleteObject()
	{
		ATLASSERT(m_hPen != NULL);
		BOOL bRet = ::DeleteObject(m_hPen);
		if(bRet)
			m_hPen = NULL;
		return bRet;
	}

// Attributes
	int GetLogPen(LOGPEN* pLogPen) const
	{
		ATLASSERT(m_hPen != NULL);
		return ::GetObject(m_hPen, sizeof(LOGPEN), pLogPen);
	}

	bool GetLogPen(LOGPEN& LogPen) const
	{
		ATLASSERT(m_hPen != NULL);
		return (::GetObject(m_hPen, sizeof(LOGPEN), &LogPen) == sizeof(LOGPEN));
	}

#ifndef _WIN32_WCE
	int GetExtLogPen(EXTLOGPEN* pLogPen, int nSize = sizeof(EXTLOGPEN)) const
	{
		ATLASSERT(m_hPen != NULL);
		return ::GetObject(m_hPen, nSize, pLogPen);
	}

	bool GetExtLogPen(EXTLOGPEN& ExtLogPen, int nSize = sizeof(EXTLOGPEN)) const
	{
		ATLASSERT(m_hPen != NULL);
		int nRet = ::GetObject(m_hPen, nSize, &ExtLogPen);
		return ((nRet > 0) && (nRet <= nSize));
	}
#endif // !_WIN32_WCE
};

typedef CPenT<false>   CPenHandle;
typedef CPenT<true>    CPen;