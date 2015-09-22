

use winapi::*;
use wtl::atl::{Dialog,DlgMsg};
use super::AboutDialog;

pub struct MainDialog<T> {
    pub this: Dialog<T>,
    pub about_dialog: AboutDialog<T>,
    //btn_about:Button,
}

impl<T> MainDialog<T> {
	pub fn new()->MainDialog<T>{
		MainDialog{
			this: Dialog::new(129),
			about_dialog: AboutDialog::new(),
			//btn_about:Button::new(101),
		}
	}

	pub fn create(&mut self,t:*mut T){
		let h = self.this.Create3(t);
		self.about_dialog.this.Create2(h,t);
		self.about_dialog.this.ShowWindow(SW_SHOW);
	}

	////////////////////////////////////////
	// handlers
	pub fn this_msg(&mut self)->DlgMsg<T>{
		DlgMsg::new(&mut self.this.handlers)
	}
}
