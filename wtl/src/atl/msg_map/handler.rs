
use user32;
use kernel32;
use winapi::*;
use std::collections::BTreeMap;
use super::CWindow;
use super::OptCall;
use super::MsgEntry;
use super::CmdEntry;

pub struct Handler<T> {
    msg_entry: MsgEntry<T>,
    cmd_entry: CmdEntry<T>,
}

impl<T> Handler<T> {
    pub fn new() -> Handler<T> {
        Handler {
            msg_entry: MsgEntry::new(),
            cmd_entry: CmdEntry::new(), 
        }
    }
}

// msg
impl<T> Handler<T> {
	//message handler
    // pub fn add_msg_listener<F>(&mut self, uMsg: UINT, f: F)
    //     where F: Fn(&T, UINT, WPARAM, LPARAM) -> LRESULT + 'static
    // {
    //     self.msg_entry.insert(uMsg, Box::new(f));
    // }
    pub fn on_close<F>(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_close = Some(Box::new(f));
    }

    pub fn on_message(&self,
                      t: &T,
                      uMsg: UINT,
                      wParam: WPARAM,
                      lParam: LPARAM,
                      bHandled: &mut BOOL)
                      -> LRESULT {
        self.msg_entry.on_message(t, uMsg, wParam, lParam,bHandled)
    }

}

// command
impl<T> Handler<T> {
	/// commmand handler
    pub fn add_cmd_listener_before<F>(&mut self, id: WORD, f: F)
        where F: Fn(&T, WORD, WORD, LPARAM) -> LRESULT + 'static
    {
        //self.cmd_entry.insert(id, Box::new(f));
    }

    pub fn add_cmd_listener<F>(&mut self, id: WORD, f: F)
        where F: Fn(&T, WORD, WORD, LPARAM) -> LRESULT + 'static
    {
        //self.cmd_entry.insert(id, Box::new(f));
    }

    pub fn add_cmd_listener_after<F>(&mut self, id: WORD, f: F)
        where F: Fn(&T, WORD, WORD, LPARAM) -> LRESULT + 'static
    {
        //self.cmd_entry.insert(id, Box::new(f));
    }

    pub fn on_command(&self,
                      t: &T,
                      code: WORD,
                      id: WORD,
                      lParam: LPARAM,
                      bHandled: &mut BOOL)
                      -> LRESULT {
        let mut lRes = 0;
        lRes = self.cmd_entry.on_command(t, code, id, lParam, bHandled);
        lRes
    }

    /// on_btn_click:&T,id,Fn
    pub fn on_btn_click_before<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,&CWindow)+'static {
        let opt_call = self.cmd_entry.on_btn_click.entry(id).or_insert(OptCall::new());
        opt_call.before = Some(Box::new(f));
    }

    pub fn on_btn_click<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,&CWindow)+'static{
        let opt_call = self.cmd_entry.on_btn_click.entry(id).or_insert(OptCall::new());
        opt_call.around = Some(Box::new(f));
    }

    pub fn on_btn_click_after<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,&CWindow)+'static{
        let opt_call = self.cmd_entry.on_btn_click.entry(id).or_insert(OptCall::new());
        opt_call.after = Some(Box::new(f));
    }

}

// dispatch
impl<T> Handler<T> {
	/// return TRUE if processed
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
}
