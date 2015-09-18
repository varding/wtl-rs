
use winapi::*;
use std::collections::BTreeMap;
use super::CWindow;
use super::OptCall;

pub struct CmdEntry<T> {
    //id=>(id,CWindow)
    pub on_btn_click: BTreeMap<WORD, OptCall<Fn(&T, WORD,&CWindow)>>,
}

impl<T> CmdEntry<T> {
    pub fn new()->CmdEntry<T>{
        CmdEntry{
            on_btn_click:BTreeMap::new(),
        }
    }

    pub fn on_command(&self,t: &T,code: WORD,id: WORD,lParam: LPARAM,bHandled: &mut BOOL) -> LRESULT {
        let mut lRes:LRESULT = 0;
        match code{
            BN_CLICKED => {
                if let Some(ref opt_call) = self.on_btn_click.get(&id) {
                    let c = CWindow::new(lParam as HWND);
                    if let Some(ref before) = opt_call.before{
                        before(t, id, &c);
                        *bHandled = TRUE;
                    }

                    if let Some(ref around) = opt_call.around{
                        around(t, id, &c);
                        *bHandled = TRUE;
                    }

                    if let Some(ref after) = opt_call.after{
                        after(t, id, &c);
                        *bHandled = TRUE;
                    }
                }
            },
            _=>{

            },
        }
        lRes
    }
}



macro_rules! on_opt_call {
    ($name:ident,$($p,)*) => {
        if let Some(ref optc) = self.$name{
            if let Some(ref before) = optc.before{
                before($($p,)*);
            }

            if let Some(ref around) = optc.around{
                around($($p,)*);
            }

            if let Some(ref after) = optc.after{
                after($($p,)*);
            }
        }
    }
}