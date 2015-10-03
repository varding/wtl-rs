
use ui::Root;
use user32;
use winapi::*;
use super::RcFile;
use std::path::{Path,PathBuf};

pub struct MainDlgHandler;

impl MainDlgHandler {
	pub fn register_handler(&self, r: &mut Root) {
		r.main_dlg.this_msg().on_close(|_,_|{
			unsafe{user32::PostQuitMessage(0)};
		});

		r.main_dlg.this_msg().on_init_dialog(|_,t|{
            println!("hello main dlg");
            t.main_dlg.this.CenterWindow(0 as HWND);
            t.main_dlg.this.SetWindowText("GUI Generator");

			t.main_dlg.dlg_tree.Attach(t.main_dlg.this.GetDlgItem(1002));
            let a = t.main_dlg.dlg_tree.GetRootItem();
            let b= a.AddHead("hello",0);
            b.AddHead("hi",0);
            a.AddHead("hello",0);
            a.AddHead("hello",0);

            //bind btn_parse
            t.main_dlg.btn_parse.Attach(t.main_dlg.this.GetDlgItem(1005));

            //bind edt_rc_path
            t.main_dlg.edt_rc_path.Attach(t.main_dlg.this.GetDlgItem(1006));
        });

		r.main_dlg.btn_parse_msg().on_click(|_,t|{
            let rf = RcFile;
            let p = t.main_dlg.edt_rc_path.GetWindowText();
            rf.parse_rc(&p);
            
            //let header_path = Path::new(p);
            let mut header_path = PathBuf::from(Path::new(&p).parent().unwrap());
            header_path.push("resource.h");
            rf.parse_header(header_path.to_str().unwrap());
		});
        // r.main_dlg.this_msg().on_mouse_move(|e,_|{
        // 	//println!("0x:{:x}", e.get_hwnd() as usize);
        // });
	}
}