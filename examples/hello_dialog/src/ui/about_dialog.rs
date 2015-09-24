#![allow(dead_code)]
//use winapi::*;
use wtl::atl::{Dialog,DlgMsg};
use wtl::ctrls::{Button,BtnMsg};

// modal dialog should not auto created
pub struct AboutDialog<T> {
    pub this: Dialog<T>,
    btn_ok:Button,
}


impl<T> AboutDialog<T> {
	pub fn new()->AboutDialog<T>{
		AboutDialog{
			this: Dialog::new(100),
			btn_ok:Button::new(),
		}
	}

	////////////////////////////////
	// handlers
	pub fn this_msg(&mut self)->DlgMsg<T>{
		DlgMsg::new(&mut self.this.handlers)
	}

	pub fn btn_ok_msg(&mut self)->BtnMsg<T>{
		// IDOK == 1
		self.this.btn_handler(1)
	}
}