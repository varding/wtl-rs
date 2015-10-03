
use winapi::{MSG,HWND,POINT,WM_MOUSEMOVE};
use user32::{GetMessageW,TranslateMessage,DispatchMessageW};


// all dialog entry(not include child dialog)
pub struct MessageLoop;

impl MessageLoop {
	pub fn run(){
		let mut msg = MSG{hwnd:0 as HWND,message:0,wParam:0,lParam:0,time:0,pt:POINT{x:0,y:0}};
		unsafe{
			while GetMessageW( &mut msg, 0 as HWND, 0, 0 ) > 0 { 
				// if msg.message == WM_MOUSEMOVE {
				// 	println!("0x{:x}", msg.hwnd as usize);
				// }
				TranslateMessage(&msg); 
		        DispatchMessageW(&msg); 
		    }
		}
	}
}