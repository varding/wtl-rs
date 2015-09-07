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


//use Vec instead of BTreeMap,one message can add more than one listener,and maybe more efficient

//use std::ptr;
use user32;
use kernel32;
use winapi::*;
use std::collections::BTreeMap;
use super::CWindow;
use std::cmp::{Ordering, Ord, Eq};
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

/// The key of a map is unique, so we use vector to implement multi listener and multi entry 
pub struct Handler<T> {
    msg_entry: BTreeMap<UINT, Box<Fn(&T, UINT, WPARAM, LPARAM) -> LRESULT>>,
    cmd_entry: BTreeMap<WORD, Box<Fn(&T, WORD, WORD, LPARAM) -> LRESULT>>, 
}

impl<T> Handler<T> {
    pub fn new() -> Handler<T> {
        Handler {
            // raw_t 	  : RawPt(0),
            msg_entry: BTreeMap::new(),
            cmd_entry: BTreeMap::new(), /* entry     : Vec::new(),
                                         * call_vec  : Vec::new(), */
        }
    }

	//message handler
    pub fn add_msg_listener<F>(&mut self, uMsg: UINT, f: F)
        where F: Fn(&T, UINT, WPARAM, LPARAM) -> LRESULT + 'static
    {
        self.msg_entry.insert(uMsg, Box::new(f));
    }

    pub fn on_message(&self,
                      t: &T,
                      uMsg: UINT,
                      wParam: WPARAM,
                      lParam: LPARAM,
                      bHandled: &mut BOOL)
                      -> LRESULT {
		//debug_assert!(raw_t.is_null() == false,"you must call handler.init() first");

        let mut lRes = 0;
		// if uMsg == WM_INITDIALOG {
		// 	(self.init_dlg)(self.raw_t);
		// 	bHandled = TRUE;
		// } else

        if let Some(f) = self.msg_entry.get(&uMsg) {
            lRes = f(t, uMsg, wParam, lParam);
            *bHandled = TRUE;
        }
        lRes
    }

	//commmand handler
    pub fn add_cmd_listener<F>(&mut self, id: WORD, f: F)
        where F: Fn(&T, WORD, WORD, LPARAM) -> LRESULT + 'static
    {
        self.cmd_entry.insert(id, Box::new(f));
    }

    pub fn on_command(&self,
                      t: &T,
                      code: WORD,
                      id: WORD,
                      lParam: LPARAM,
                      bHandled: &mut BOOL)
                      -> LRESULT {
        let mut lRes = 0;
        if let Some(f) = self.cmd_entry.get(&id) {
            lRes = f(t, code, id, lParam);
            *bHandled = TRUE;
        }
        lRes
    }

	////
    pub fn disptach_msg(&mut self,
                        t: &T,
                        hWnd: HWND,
                        uMsg: UINT,
                        wParam: WPARAM,
                        lParam: LPARAM,
                        lResult: &mut LRESULT,
                        dwMsgMapID: DWORD)
                        -> BOOL {
		//debug_assert!(self.handler.is_none() == false);
        let mut bHandled = FALSE;
		//let mut bHandled:BOOL = TRUE;
        match uMsg {
            WM_NOTIFY => {
				//*lResult = self.on_notify(wParam, lParam);
            }
            WM_COMMAND => {
                let id = LOWORD(wParam as DWORD);
                let code = HIWORD(wParam as DWORD);
				//*lResult = self.on_command(code, id, lParam);
                *lResult = self.on_command(t,code, id, lParam,&mut bHandled);
            }
            _ => {
				//*lResult = self.on_message(uMsg, wParam, lParam);
                *lResult = self.on_message(t,uMsg, wParam, lParam,&mut bHandled);
				//return FALSE;
            }
        }
        bHandled
    }

	// fn add_entry<F>(&mut self,uMsg:WORD,id:WORD,code:WORD,f:FnCb) {

	// }

	// fn add_entry_vec<F>(&mut self,ids:&[WORD],f:F) where F:Fn(&T,UINT,WPARAM,LPARAM)->LRESULT + 'static {

	// }

	//
	// pub fn on_btn_click<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,CWindow) + 'static {
	// 	let p = FnCb::BtnClick(Box::new(f));
	// 	self.call_vec.push(p);
	// 	let idx = self.call_vec.len();
	// 	self.entry.push(Entry::new(WM_COMMAND as WORD, id, BN_CLICKED,idx as u16));
	// 	self.entry.sort();
	// }

	// pub fn on_btn_click_vec<F>(&mut self,ids:&[WORD],f:F) where F:Fn(&T,UINT,WPARAM,LPARAM)->LRESULT + 'static {

	// }
}

macro_rules! impl_msg_map {
	() => ()
}
