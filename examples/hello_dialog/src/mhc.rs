use user32;
// use kernel32;
use winapi::*;
use wtl::atl::*;

use about;

pub struct MainDlg {
    dlg 		: CDialogImpl,
    handler 	: Handler<MainDlg>,
    about_dlg 	: about::AboutDlg,
}

impl MainDlg {
	pub fn new()-> MainDlg{
		MainDlg{
			dlg:CDialogImpl::new(129,Self::ProcessWindowMessage,None),
			about_dlg:about::AboutDlg::new(),
			handler:Handler::new(),
		}
	}

	pub fn create(&mut self) {
		self.handler.add_msg_listener(WM_CLOSE, |pself:&Self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM|->LRESULT{
			println!("close main dlg");
			unsafe{user32::PostQuitMessage(0)};
			0
		});

		self.handler.add_cmd_listener(101, |pself:&Self,code:WORD,id:WORD,lParam:LPARAM|->LRESULT {
			pself.about_dlg.dlg.ShowWindow(SW_SHOW);
			0
		});

		let p = self as *mut Self as *mut c_void;
		self.dlg.DoModal2(p);
	}

	pub fn OnInitDialog(&mut self) {
		self.dlg.CenterWindow(NULL_HWND);
		self.about_dlg.Create(self.dlg.GetHwnd());
	}

	//pass OnInitDialog as param
	impl_proc_msg!(OnInitDialog);
}	
