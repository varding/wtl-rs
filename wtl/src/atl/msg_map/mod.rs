

//pub mod msg_handler;



//pub use self::msg_handler::MsgHandler;


//use Vec instead of BTreeMap,one message can add more than one listener,and maybe more efficient


//use std::ptr;
use user32;
use kernel32;
use winapi::*;
use std::collections::BTreeMap;

//type MsgFn = Fn(UINT,WPARAM,LPARAM)->LRESULT;
//type CmdFn = Fn(code:WORD,id:WORD,lParam:LPARAM)->LRESULT;

//struct RawPt<T: ?Sized>(*mut T);
// struct RawPt(*mut c_void);

// impl RawPt{
//     fn as_mut_ref<'a,T>(&self) -> &'a mut T {
//         unsafe{&mut (*(self.0 as *mut T)}
//     }

//     fn as_ref<'a,T>(&self) -> &'a T{
//     	unsafe{&(*(self.0 as *const T)}
//     }
    
//     fn new(t:&mut T) -> RawPt<T> {
//         RawPt(t as * mut T as c_void)
//     }

//     fn is_null(&self) ->bool {
//     	self.0 == 0
//     }
// }

pub struct Handler<T> {
	msg_entry : BTreeMap<UINT,Box<Fn(&T,UINT,WPARAM,LPARAM)->LRESULT>>,
	cmd_entry : BTreeMap<WORD,Box<Fn(&T,WORD,WORD,LPARAM)->LRESULT>>,
}

impl<T> Handler<T> {
	pub fn new() -> Handler<T> {
		Handler{
			//raw_t 	  : RawPt(0),
			msg_entry : BTreeMap::new(),
			cmd_entry : BTreeMap::new(),
		}
	}

	//message handler
	pub fn add_msg_listener<F>(&mut self,uMsg:UINT,f:F) where F:Fn(&T,UINT,WPARAM,LPARAM)->LRESULT + 'static {
		self.msg_entry.insert(uMsg, Box::new(f));
	}

	pub fn on_message(&self,t:&T,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL)->LRESULT {
		//debug_assert!(raw_t.is_null() == false,"you must call handler.init() first");

		let mut lRes = 0;
		// if uMsg == WM_INITDIALOG {
		// 	(self.init_dlg)(self.raw_t);
		// 	bHandled = TRUE;
		// } else 

		if let Some(f) = self.msg_entry.get(&uMsg) {
			lRes = f(t,uMsg,wParam,lParam);
			*bHandled = TRUE;
		}
		lRes
	}

	//commmand handler
	pub fn add_cmd_listener<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,WORD,LPARAM)->LRESULT + 'static {
		self.cmd_entry.insert(id, Box::new(f));
	}

	pub fn on_command(&self,t:&T,code:WORD,id:WORD,lParam:LPARAM,bHandled:&mut BOOL)->LRESULT {
		let mut lRes = 0;
		if let Some(f) = self.cmd_entry.get(&id) {
			lRes = f(t,code,id,lParam);
			*bHandled = TRUE;
		}
		lRes
	}

	////
	pub fn disptach_msg(&mut self,t:&T,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL {
		//debug_assert!(self.handler.is_none() == false);
		let mut bHandled = FALSE;
		//let mut bHandled:BOOL = TRUE;
		match uMsg{
			WM_NOTIFY=>{
				//*lResult = self.on_notify(wParam, lParam);
			},
			WM_COMMAND=>{
				let id = LOWORD(wParam as DWORD);
				let code = HIWORD(wParam as DWORD);
				//*lResult = self.on_command(code, id, lParam);
				*lResult = self.on_command(t,code, id, lParam,&mut bHandled);
			},
			_=>{
				//*lResult = self.on_message(uMsg, wParam, lParam);
				*lResult = self.on_message(t,uMsg, wParam, lParam,&mut bHandled);
				//return FALSE;
			},
		}
		bHandled
	}
}