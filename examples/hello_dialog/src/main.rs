#[macro_use]
extern crate wtl;
extern crate winapi;
extern crate user32;
extern crate kernel32;
#[link(name="ui", kind="static")]

use winapi::*;
//use win32::atl::cdlg_impl::CDialogImpl;
use wtl::atl::*;
struct Dialog {
    m_hWnd: HWND,
    thk   : &'static mut thunk::Thunk
}

impl Dialog {
	fn new()->Dialog{
		Dialog{
			m_hWnd:NULL_HWND,
			thk   :thunk::get_thunk(),
		}
	}
}
//auto impl winTrait
impl_hwnd_trait!(Dialog);

impl CDialogImpl for Dialog {}
impl CWindowImplRoot for Dialog{}
impl CDialogImplBaseT for Dialog {}
//after impl the two trait,Dialog auto impl CDialogImpl
impl DialogTrait for Dialog {
	fn NewThunk(&mut self) -> bool{
		true
	}

	fn InitThunk(&mut self,h:HWND,dlg_proc:DLGPROC) -> DLGPROC { //convert &mut self to *const T in this method
		self.m_hWnd = h;
		dlg_proc
	}		

	fn ProcessWindowMessage(&self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL{
		println!("msg:{}", uMsg);
		FALSE
	}

	fn State(&self)->DWORD {
		1
	}

	fn AddState(&self,s:DWORD) {

	}

	fn OnFinalMessage(&self){

	}

	fn IDD(&self)->WORD{
		101
	}
}

//include!("foo.rs");
fn main() {
	println!("hello");
	let mut d = Dialog::new();
	let r = d.DoModal(NULL_HWND,0 as LPARAM);
	println!("{}", r);
	let hello = "hello大家好";
	//CodePage: UINT, dwFlags: DWORD, lpMultiByteStr: LPCCH, cbMultiByte: c_int,
	//        lpWideCharStr: LPWSTR, cchWideChar: c_int,
	unsafe{
		let out = [0u16,24];
		let wcsLen = kernel32::MultiByteToWideChar(CP_UTF8, 0, hello as *const str as LPCCH , hello.len() as c_int, out.as_ptr() as LPWSTR, 24);
		println!("{}", wcsLen);
		user32::MessageBoxW(NULL_HWND, out.as_ptr() as LPCWSTR, out.as_ptr() as LPCWSTR, 0u32);
	}
}

