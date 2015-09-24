
//use kernel32;
use user32;
use winapi::*;
use wtl::atl::*;

use super::wchar::ToCU16Str;
use super::ui;

pub struct AboutDialogHandler;

impl ui::DialogHandler for AboutDialogHandler {
    fn register_handler(&self,r:&mut ui::Root){
        r.main_dlg.about_dialog.this_msg().on_init_dialog(0, |_,t|{
            println!("hello about");
            t.main_dlg.about_dialog.this.CenterWindow(t.main_dlg.this.GetHwnd());
        });

        r.main_dlg.about_dialog.this_msg().on_close(0, |_,t|{
            println!("hide about dlg");
            //unsafe{user32::PostQuitMessage(0)};
            //t.main_dlg.about_dialog.this.DestroyWindow();
            t.main_dlg.about_dialog.this.ShowWindow(SW_HIDE);
        });

        r.main_dlg.about_dialog.btn_ok_msg().on_click(0, |e:&mut Event,_|{
            show_msg_dlg(e.get_hwnd());
        });
    }
}

pub fn show_msg_dlg(h: HWND) {
    let hello = "hello大家好";
    let t = hello.to_c_u16();
    unsafe {
        user32::MessageBoxW(h, t.as_ptr() as LPCWSTR, 0 as LPCWSTR, 0u32);
    }
}