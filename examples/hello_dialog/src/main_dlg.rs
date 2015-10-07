use user32;
// use kernel32;
use winapi::*;
use wtl::atl::*;

use super::ui;
use about;


pub struct MainDialogHandler{
    about_dlg_handler: about::AboutDialogHandler,
}

impl MainDialogHandler {
    #[inline(always)]    
    pub fn new()->MainDialogHandler{
        MainDialogHandler{
            about_dlg_handler: about::AboutDialogHandler,
        }
    }
}

impl ui::DialogHandler for MainDialogHandler {
    fn register_handler(&self,r:&mut ui::Root){
        r.main_dlg.this_msg().on_init_dialog(|_,t|{
            println!("hello main dlg");
            t.main_dlg.this.CenterWindow(NULL_HWND);
            t.main_dlg.this.SetWindowText("hello");
        });

        r.main_dlg.this_msg().on_close(|_,_|{
            println!("bye main dlg");
            unsafe{user32::PostQuitMessage(0)};
        }).set_user_priority(100);       // only for test

        r.main_dlg.btn_about_msg().on_click(|_,t|{
            println!("show about dlg");
            t.main_dlg.about_dialog.this.ShowWindow(SW_SHOW);
        });

        self.about_dlg_handler.register_handler(r);
    }
}