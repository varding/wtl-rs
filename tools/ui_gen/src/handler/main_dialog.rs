
use ui::Root;
use user32;
use winapi::*;
use super::RcFile;
use std::path::{Path,PathBuf};
use std::rc::Rc;
use std::cell::RefCell;
use wtl::ctrls::CTreeItem;

pub struct MainDlgHandler{
    rc_file: Rc<RefCell<RcFile>>,
}

impl MainDlgHandler {
    pub fn new()->MainDlgHandler {
        MainDlgHandler{
            rc_file: Rc::new(RefCell::new(RcFile)),
        }
    }

	pub fn register_handler(&self, r: &mut Root) {
		r.main_dlg.this_msg().on_init_dialog(|_,t|{
            println!("hello main dlg");
            t.main_dlg.this.SetWindowText("GUI Generator");

            let a = t.main_dlg.dlg_tree.GetRootItem();
            let b = a.AddHead("Root",-1);
            b.Select(TVGN_CARET as UINT);
            //let b= a.AddHead("Root",0);

            t.main_dlg.edt_rc_path.SetWindowText("K:\\software\\pc\\rust\\wtl-rs\\tools\\ui_gen\\src\\del\\mhc.rc");
        });

        let rf1 = self.rc_file.clone();
		r.main_dlg.btn_parse_msg().on_click(move|_,t|{
            let rf = rf1.borrow_mut();
            let p = t.main_dlg.edt_rc_path.GetWindowText();
            let dlg_ids = rf.parse_rc(&p);
            
            for id in &dlg_ids {
                t.main_dlg.lst_all_dlgs.AddString(id);
            }
            //let header_path = Path::new(p);
            let mut header_path = PathBuf::from(Path::new(&p).parent().unwrap());
            header_path.push("resource.h");
            rf.parse_header(header_path.to_str().unwrap());
		});

        let rf2 = self.rc_file.clone();
        r.main_dlg.btn_select_msg().on_click(move|_,t|{
            let rf = rf2.borrow_mut();
            //get selected list item
            let lst_sel = t.main_dlg.lst_all_dlgs.GetCurSel();
            if lst_sel == -1 {
                return;
            }
            let lst_sel_txt = t.main_dlg.lst_all_dlgs.GetText(lst_sel);
            t.main_dlg.lst_all_dlgs.DeleteString(lst_sel as UINT);
            //let tree_sel_txt = t.main_dlg.dlg_tree.GetSelectedItem().GetText();
            let item = t.main_dlg.dlg_tree.GetSelectedItem();
            item.AddTail(&lst_sel_txt[..],-1);
            //println!("lst sel:{}, txt:{}", lst_sel,lst_sel_txt);
            //println!("tree sel: {}", tree_sel_txt);
        });

        let rf3 = self.rc_file.clone();
        r.main_dlg.btn_unselect_msg().on_click(move|_,t|{
            let item = t.main_dlg.dlg_tree.GetSelectedItem();
            let sel_txt = item.GetText();

            let del_strings = delete_tree(&item);
            for s in &del_strings {
                t.main_dlg.lst_all_dlgs.AddString(&s[..]);
            }
        });
	}
}

fn delete_child(item: &CTreeItem,item_strings: &mut Vec<String>)->bool {
    if item.IsNull() {
        return false;
    }
    
    //delete all children first
    while delete_child(&item.GetChild(), item_strings){}

    let t = item.GetText();
    //Root can't be deleted
    if t != "Root" {
        item_strings.push(t);
        item.Delete();
    }
    
    return true;
}

fn delete_tree(item: &CTreeItem) ->Vec<String> {
    let mut item_strings: Vec<String> = Vec::new();
    
    delete_child(item, &mut item_strings);

    item_strings
}