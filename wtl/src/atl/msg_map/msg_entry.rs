use winapi::*;
use std::collections::BTreeMap;
use super::super::CWindow;


/// only one call for message entry
pub struct MsgEntry<T> {
    //OnClose()
    //pub on_close:OptCall<Fn(&T)>,
    pub on_close : Option<Box<Fn(&T)>>,
}

impl<T> MsgEntry<T> {
    pub fn new()->MsgEntry<T>{
        MsgEntry{
            on_close:None,
        }
    }

    pub fn on_message(&self,t: &T,uMsg: UINT,wParam: WPARAM,lParam: LPARAM,bHandled: &mut BOOL) -> LRESULT {
        let mut lRes:LRESULT = 0;
        match uMsg{
            WM_CLOSE=>{
                //self.on_close();
                if let Some(ref optc) = self.on_close {
                    optc(t);
                    *bHandled = TRUE;
                }
            },
            _=>{

            },
        }
        lRes
    }   
}