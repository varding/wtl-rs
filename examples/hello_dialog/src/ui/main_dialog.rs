

use winapi::*;
use wtl::atl::{Dialog,DlgMsg};
use wtl::ctrls::{CButton,BtnMsg};

use super::AboutDialog;

pub struct MainDialog<T> {
    pub this: Dialog<T>,
    pub about_dialog: AboutDialog<T>,
    btn_about:CButton,
}

impl<T> MainDialog<T> {
	pub fn new()->MainDialog<T>{
		MainDialog{
			this: Dialog::new(129),
			about_dialog: AboutDialog::new(),
			btn_about:CButton::new(),
		}
	}

	pub fn create(&mut self,t:*mut T){
		let h = self.this.Create3(t);
		self.about_dialog.this.Create2(h,t);
		self.about_dialog.this.ShowWindow(SW_SHOW);

		//println!("{}", self.this);
		// buttons 
		// these binders should call in OnInitDialog?
		let h = self.this.GetDlgItem(101);
		//println!("0x{:x}", h as usize);
		self.btn_about.Attach(h);
	}

	////////////////////////////////////////
	// handlers
	pub fn this_msg(&mut self)->DlgMsg<T>{
		//DlgMsg::new(&mut self.this.handlers)
		self.this.msg_handler()
	}

	pub fn btn_about_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(101)
	}
}
