
use winapi::*;

pub struct Event {
    uMsg:u32,
    wParam:WPARAM,
    lParam:LPARAM,
}

impl Event {
	#[inline(always)]
	pub fn new(uMsg:UINT,wParam:WPARAM,lParam:LPARAM) ->Event {
		Event{
			uMsg:uMsg,
			wParam:wParam,
			lParam:lParam,
		}
	}
}