

/*
::DialogBoxParam(_AtlBaseModule.GetResourceInstance(), MAKEINTRESOURCE(static_cast<T*>(this)->IDD),
					hWndParent, T::StartDialogProc, dwInitParam);
*/

use winapi::*;
use user32;
//https://github.com/klutzy/rust-windows/blob/master/src/window.rs
//https://github.com/klutzy/rust-windows/blob/master/src/lib.rs
pub unsafe extern "system" fn main_wnd_proc(hwnd: HWND,msg: UINT,w: WPARAM,l: LPARAM) -> LRESULT {
	user32::DefWindowProcW(hwnd, msg, w, l)
}


pub trait CDialogImpl {
	fn DoModal2(){
		 HWND hWndParent = ::GetActiveWindow(),
		 LPARAM dwInitParam = NULL
	}
	
	fn DoModal(hWndParent:HWND,dwInitParam:LPARAM) -> INT_PTR {
		BOOL result;

		//ATLASSUME(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail
		// gracefully.

		result = m_thunk.Init(NULL,NULL);
		if (result == FALSE)
		{
			SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		_AtlWinModule.AddCreateWndData(&m_thunk.cd, (CDialogImplBaseT< TBase >*)this);
#ifdef _DEBUG
		m_bModal = true;
#endif //_DEBUG
		return ::DialogBoxParam(_AtlBaseModule.GetResourceInstance(), MAKEINTRESOURCE(static_cast<T*>(this)->IDD),
					hWndParent, T::StartDialogProc, dwInitParam);
	}

	BOOL EndDialog( int nRetCode)
	{
		ATLASSERT(::IsWindow(m_hWnd));
#ifdef _DEBUG
		ATLASSUME(m_bModal);	// must be a modal dialog
#endif //_DEBUG
		return ::EndDialog(m_hWnd, nRetCode);
	}
	// modeless dialogs
	HWND Create(
		 HWND hWndParent,
		 LPARAM dwInitParam = NULL)
	{
		BOOL result;

		ATLASSUME(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail
		// gracefully.

		result = m_thunk.Init(NULL,NULL);
		if (result == FALSE)
		{
			SetLastError(ERROR_OUTOFMEMORY);
			return NULL;
		}

		_AtlWinModule.AddCreateWndData(&m_thunk.cd, (CDialogImplBaseT< TBase >*)this);
#ifdef _DEBUG
		m_bModal = false;
#endif //_DEBUG
		HWND hWnd = ::CreateDialogParam(_AtlBaseModule.GetResourceInstance(), MAKEINTRESOURCE(static_cast<T*>(this)->IDD),
					hWndParent, T::StartDialogProc, dwInitParam);
		ATLASSUME(m_hWnd == hWnd);
		return hWnd;
	}
	// for CComControl
	HWND Create(
		 HWND hWndParent,
		 RECT&,
		 LPARAM dwInitParam = NULL)
	{
		return Create(hWndParent, dwInitParam);
	}
	BOOL DestroyWindow()
	{
		ATLASSERT(::IsWindow(m_hWnd));
#ifdef _DEBUG
		ATLASSERT(!m_bModal);	// must not be a modal dialog
#endif //_DEBUG

		if (!::DestroyWindow(m_hWnd))
		{
			return FALSE;
		}

		return TRUE;
	}
}