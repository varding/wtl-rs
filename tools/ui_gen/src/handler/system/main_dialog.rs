
use ui::Root;
use user32;
use winapi::*;
use comctl32;

pub struct MainDlgHandler;

impl MainDlgHandler {
	pub fn register_handler(&self, r: &mut Root) {
		r.main_dialog.this_msg().on_close(|_,_|{
			unsafe{user32::PostQuitMessage(0)};
		});

		r.main_dialog.this_msg().on_init_dialog(|_,t|{
            //println!("hello main dlg");
            unsafe{
                  comctl32::InitCommonControls();
            }
            t.main_dialog.this.CenterWindow(0 as HWND);
            //t.main_dialog.this.SetWindowText("GUI Generator");

            let this = &t.main_dialog.this;

            //bind all controls
		t.main_dialog.tree_selected_dlgs.Attach(this.GetDlgItem(1002));
            t.main_dialog.btn_parse.Attach(this.GetDlgItem(1005));
            t.main_dialog.btn_select.Attach(this.GetDlgItem(1003));
            t.main_dialog.btn_unselect.Attach(this.GetDlgItem(1004));
            t.main_dialog.edt_rc_path.Attach(t.main_dialog.this.GetDlgItem(1006));
            t.main_dialog.lst_all_dlgs.Attach(t.main_dialog.this.GetDlgItem(1001));
        }).set_system_priority(0);
	}
}