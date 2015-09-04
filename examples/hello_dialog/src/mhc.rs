use user32;
// use kernel32;
use winapi::*;
use wtl::atl::*;

//use wtl::atl::

use about;

pub struct MainDlg {
    dlg: CDialogImpl,
    handler:Handler<MainDlg>,
    about_dlg:about::AboutDlg,
}

impl MainDlg {
	pub fn new()-> MainDlg{
		MainDlg{
			dlg:CDialogImpl::new(129,Self::ProcessWindowMessage,None),
			about_dlg:about::AboutDlg::new(),
			handler:Handler::new(),
		}
	}

	pub fn OnInitDialog(&mut self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL) -> LRESULT {

		self.handler.add_msg_listener(WM_CLOSE, |t:&Self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM|->LRESULT{
			println!("close main dlg");
			unsafe{user32::PostQuitMessage(0)};
			0
		});

		self.handler.add_cmd_listener(101, |t:&Self,code:WORD,id:WORD,lParam:LPARAM|->LRESULT{
			t.about_dlg.dlg.ShowWindow(SW_SHOW);
			0
		});

		self.about_dlg.Create();
		0
	}

	pub fn do_modal(&mut self){
		let pself = self as *mut Self as *mut c_void;
		self.dlg.DoModal2(pself);
	}

	// fn on_notify(&mut self,wParam:WPARAM,lParam:LPARAM) -> LRESULT{
	// 	let ph = lParam as LPNMHDR;
	// 	let id = unsafe{(*ph).idFrom};
	// 	let code = unsafe{(*ph).code};
	// 	0
	// }

	pub fn ProcessWindowMessageLocal(&mut self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL{
		let mut bHandled:BOOL = TRUE;

		match uMsg{
			WM_INITDIALOG=>{
				//self.OnInitDialog();
				*lResult = self.OnInitDialog(uMsg, wParam, lParam, &mut bHandled);
			},
			WM_NOTIFY=>{
				//*lResult = self.on_notify(wParam, lParam);
			},
			WM_COMMAND=>{
				let id = LOWORD(wParam as DWORD);
				let code = HIWORD(wParam as DWORD);
				//*lResult = self.on_command(code, id, lParam);
				*lResult = self.handler.on_command(self,code, id, lParam);
			},
			_=>{
				//*lResult = self.on_message(uMsg, wParam, lParam);
				*lResult = self.handler.on_message(self,uMsg, wParam, lParam);
				return FALSE;
			},
		}
		TRUE
	}

	//convert the first param *mut c_void to self
	pub fn ProcessWindowMessage(pself:*mut c_void,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL{
		unsafe{(*(pself as *mut Self)).ProcessWindowMessageLocal(hWnd,uMsg,wParam,lParam,lResult,0)}
	}
}	


// fn show_msg_dlg(){
// 	let hello = "hello大家好";

// 	unsafe{
// 		let out = [0u16,24];
// 		let wcsLen = kernel32::MultiByteToWideChar(CP_UTF8, 0, hello as *const str as LPCCH , hello.len() as c_int, out.as_ptr() as LPWSTR, 24);
// 		//println!("{}", wcsLen);
// 		user32::MessageBoxW(NULL_HWND, out.as_ptr() as LPCWSTR, out.as_ptr() as LPCWSTR, 0u32);
// 	}
// }