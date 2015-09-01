

use wtl::atl::*;

//201
pub struct SimpleDlg {
    dlg : CDialogImpl,
}

impl SimpleDlg {
	pub fn new()->SimpleDlg{
		SimpleDlg{
			dlg:CDialogImpl::new(201),
		}
	}

	pub fn DoModal(&mut self){
		self.dlg.DoModal2();
	}

	pub fn Create(&mut self){
		self.dlg.Create2();
	}
}