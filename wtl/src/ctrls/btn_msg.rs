
// https://msdn.microsoft.com/en-us/library/windows/desktop/bb775941(v=vs.85).aspx
// Handling Messages from a Button

// Notifications from a button are sent as either WM_COMMAND or WM_NOTIFY messages. 
// Information about which message is used can be found on the reference page for each notification.


// https://msdn.microsoft.com/en-us/library/windows/desktop/bb775983(v=vs.85).aspx

use winapi::*;
//use user32;

//use super::super::cwindow::*;
//use super::consts::*;
//use super::Event;

use atl::{Handler,Event};
//use atl::CWindow;

pub struct BtnMsg <'a,T:'a> {
	id:WORD,
    h:&'a mut Vec<Handler<T>>,
}

impl<'a,T> BtnMsg<'a,T> {
	pub fn new(id:WORD,h:&'a mut Vec<Handler<T>>)->BtnMsg<'a,T>{
		BtnMsg{
			id:id,
			h:h,
		}
	}
}


impl<'a,T> BtnMsg<'a,T> {
	// BN_CLICKED
	// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761825(v=vs.85).aspx
	pub fn on_click<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, BN_CLICKED, priority, f));
    }
}