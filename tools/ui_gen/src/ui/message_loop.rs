
use winapi::*;
use user32::*;

pub struct MessageLoop;

impl MessageLoop {
	pub fn run(){
		let mut msg = MSG{hwnd:0 as HWND,message:0,wParam:0,lParam:0,time:0,pt:POINT{x:0,y:0}};
		unsafe{
			while GetMessageW( &mut msg, 0 as HWND, 0, 0 ) > 0 { 
				TranslateMessage(&msg); 
				DispatchMessageW(&msg); 
			}
		}
	}
}
