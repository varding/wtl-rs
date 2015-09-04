

use winapi::*;
use wtl::atl::*;

//100
pub struct AboutDlg {
    pub dlg : CDialogImpl,
}

impl AboutDlg {
	pub fn new()->AboutDlg{
		AboutDlg{
			dlg:CDialogImpl::new(100,Self::DialogProc,None),
		}
	}

	// pub fn DoModal(&mut self){
	// 	self.dlg.DoModal2();
	// }

	pub fn Create(&mut self){
		let pself = self as *mut Self as *mut c_void;
		self.dlg.Create2(pself);
	}

	pub fn DialogProc(pself:*mut c_void,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL{
		if uMsg == WM_CLOSE{
			println!("close about dlg");
			//unsafe{user32::PostQuitMessage(0)};
			unsafe{
				(*(pself as *mut Self)).dlg.ShowWindow(SW_HIDE);
			}
		}
		0
	}
}