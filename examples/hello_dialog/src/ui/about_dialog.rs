
use winapi::*;
use wtl::atl::{Dialog,DlgMsg};

pub struct AboutDialog<T> {
    pub this: Dialog<T>,
}


impl<T> AboutDialog<T> {
	pub fn new()->AboutDialog<T>{
		AboutDialog{
			this: Dialog::new(100),
		}
	}

	////////////////////////////////
	// handlers
	pub fn this_msg(&mut self)->DlgMsg<T>{
		DlgMsg::new(&mut self.this.handlers)
	}
}