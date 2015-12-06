use ui::Root;
use user32;
use winapi::*;
use ui::consts::*;

pub fn register_handler(r: &mut Root) {
    
    r.main_dialog.this_msg().on_init_dialog(|_,t|{
        t.main_dialog.this.CenterWindow(0 as HWND);
        let this = &t.main_dialog.this;
        t.main_dialog.lst_all_dlgs.Attach(this.GetDlgItem(IDC_LST_ALL_DLGS));
        t.main_dialog.tree_selected_dlgs.Attach(this.GetDlgItem(IDC_TREE_SELECTED_DLGS));
        t.main_dialog.btn_select.Attach(this.GetDlgItem(IDC_BTN_SELECT));
        t.main_dialog.btn_unselect.Attach(this.GetDlgItem(IDC_BTN_UNSELECT));
        t.main_dialog.btn_parse.Attach(this.GetDlgItem(IDC_BTN_PARSE));
        t.main_dialog.edt_rc_path.Attach(this.GetDlgItem(IDC_EDT_RC_PATH));
        t.main_dialog.btn_generate.Attach(this.GetDlgItem(IDC_BTN_GENERATE));
        t.main_dialog.edt_dlg_name.Attach(this.GetDlgItem(IDC_EDT_DLG_NAME));
    }).set_system_priority(0);
    
}