//! Multi listener for the same message or command:
//! In GUI project ,sometimes we need to read or write some sth in more than one events,and then do other logic stuffs
//! wo could forget to call the read or write for they write apart in the callback functions
//! Here we provide some `multi listener and multi entry` listeners to solve this.
//! You can add multiple object event callback do the same thing
//! ```
//! handler.on_btn_click_vec(&[ID_BTN1,ID_BTN2,ID_BTN3],||xxx.Enable());
//! ```
//! Then you can also the logic stuffs in another callbacks together or sepreatly
//! ``` handler.on_btn_click_vec(&[ID_BTN1,ID_BTN3],||logic stuffs); ```
//! ``` handler.on_btn_click(ID_BTN2||logic stuffs; ```

pub use super::CWindow;

pub use self::opt_call::OptCall;
pub use self::cmd_entry::CmdEntry;
pub use self::msg_entry::MsgEntry;
pub use self::handler::Handler;

mod opt_call;
mod cmd_entry;
mod msg_entry;
mod handler;

/*
/// call entry
pub enum FnCb<T> {
	//https://msdn.microsoft.com/zh-cn/library/windows/desktop/bb761825(v=vs.85).aspx
	/// wID:WORD, hWndCtl:CWindow
    BtnClick(Box<Fn(&T,WORD,CWindow)>),

    /// atlcrack.h
    OnCreate(Box<Fn(&T,LPCREATESTRUCTW)>),
}

///  #Multi Entry Multi Listener
///  Entries of three buttons
///  e1 = Entry{WM_COMMAND,100,BN_CLICK,f1_idx}  e2 = Entry{WM_COMMAND,100,BN_CLICK,f2_idx}  e3 = Entry{WM_COMMAND,100,BN_CLICK,f3_idx}
///  e3 = Entry{WM_COMMAND,101,BN_CLICK,f1_idx}  e3 = Entry{WM_COMMAND,102,BN_CLICK,f3_idx}
///  there different function calls
///  f1 = BtnClick(Box<Fn(WORD,WORD,CWindow)>),f2 = BtnClick(Box<Fn(WORD,WORD,CWindow)>),f3 = BtnClick(Box<Fn(WORD,WORD,CWindow)>)
///  btn id = 100 has three function calls (multi call),btn = 101 and btn 102 hava one each.
///  function f1 and f2 have two entries each (multi entry)
///  the pseudo code could be:
///  ```
///  handler.on_btn_click_vec(&[100,101],f1)     //sth common like update gui can put together
///  handler.on_btn_click(&[100,102],f3)		 //the logic 100 and 102 are both need to execute
///  handler.on_btn_click(100,f2)				 //sth special 100 need to do
///  ```
/// message entry

#[derive(Debug,Default)]
pub struct Entry {
    uMsg 	: WORD,			//msg type,WORD is enough
    id 		: WORD,			//control id
    code 	: WORD,			//
    call_idx: WORD,			//call index in call_vec,WORD is enough
}

impl Entry{
	fn new(uMsg:WORD,id:WORD,code:WORD,call_idx:WORD)->Entry{
		Entry{
			uMsg 	:uMsg,
			id 	 	:id,
			code 	:code,
			call_idx:call_idx,
		}
	}
}

impl Ord for Entry {
	fn cmp(&self, other: &Self) -> Ordering {
		(*self as u64).cmp(*other as u64)
	}
}

*/