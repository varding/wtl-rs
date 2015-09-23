
use kernel32;
use user32;
use winapi::*;
use wtl::atl::*;

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

    unsafe {
        let out = [0u16,24];
        kernel32::MultiByteToWideChar(CP_UTF8,
                                                   0,
                                                   hello as *const str as LPCCH,
                                                   hello.len() as c_int,
                                                   out.as_ptr() as LPWSTR,
                                                   24);
     //println!("{}", wcsLen);
        user32::MessageBoxW(h, out.as_ptr() as LPCWSTR, out.as_ptr() as LPCWSTR, 0u32);
    }
}