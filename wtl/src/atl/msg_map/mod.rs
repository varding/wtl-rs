

//pub mod msg_handler;



//pub use self::msg_handler::MsgHandler;


//use Vec instead of BTreeMap,one message can add more than one listener,and maybe more efficient



use user32;
use kernel32;
use winapi::*;
use std::collections::BTreeMap;

//type MsgFn = Fn(UINT,WPARAM,LPARAM)->LRESULT;
//type CmdFn = Fn(code:WORD,id:WORD,lParam:LPARAM)->LRESULT;

// struct RawPt<T: ?Sized>(*mut T);

// impl<T: ?Sized> RawPt<T>{
//     fn as_mut_ref<'a>(&self) -> &'a mut T {
//         unsafe{&mut (*self.0)}
//     }
    
//     fn new(t:&mut T) -> RawPt<T> {
//         RawPt(t as * mut T)
//     }
// }

pub struct Handler<T> {
	//raw_t	  : Option<&'a mut T>,//RawPt<T>,
	msg_entry : BTreeMap<UINT,Box<Fn(&T,UINT,WPARAM,LPARAM)->LRESULT>>,
	cmd_entry : BTreeMap<WORD,Box<Fn(&T,WORD,WORD,LPARAM)->LRESULT>>,
}

impl<T> Handler<T>{
	pub fn new()->Handler<T>{
		Handler{
			//raw_t 	  : None,//RawPt::new(t),
			msg_entry : BTreeMap::new(),
			cmd_entry : BTreeMap::new(),
		}
	}

	// pub fn set_self(&mut self,t:&'a mut T){
	// 	self.raw_t = t;
	// }

	//message handler
	pub fn add_msg_listener<F>(&mut self,uMsg:UINT,f:F) where F:Fn(&T,UINT,WPARAM,LPARAM)->LRESULT + 'static {
		self.msg_entry.insert(uMsg, Box::new(f));
	}

	pub fn on_message(&self,t:&T,uMsg:UINT,wParam:WPARAM,lParam:LPARAM)->LRESULT {
		let mut lRes = 0;
		if let Some(f) = self.msg_entry.get(&uMsg) {
			lRes = f(t,uMsg,wParam,lParam);
		}
		lRes
	}

	//commmand handler
	pub fn add_cmd_listener<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,WORD,LPARAM)->LRESULT + 'static {
		self.cmd_entry.insert(id, Box::new(f));
	}

	pub fn on_command(&self,t:&T,code:WORD,id:WORD,lParam:LPARAM)->LRESULT {
		let mut lRes = 0;
		if let Some(f) = self.cmd_entry.get(&id) {
			lRes = f(t,code,id,lParam);
		}
		lRes
	}
}