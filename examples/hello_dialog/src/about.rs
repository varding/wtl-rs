

use wtl::atl::*;

//100
pub struct AboutDlg {
    dlg : CDialogImpl,
}

impl AboutDlg {
	pub fn new()->AboutDlg{
		AboutDlg{
			dlg:CDialogImpl::new(100),
		}
	}

	pub fn DoModal(&mut self){
		self.dlg.DoModal2();
	}

	pub fn Create(&mut self){
		self.dlg.Create2();
	}
}