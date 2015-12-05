
use ui;
use user32;
use winapi::*;
use super::{RcFile,RcRoot};
use std::path::{Path,PathBuf};
use std::rc::Rc;
use std::cell::RefCell;
use wtl::ctrls::CTreeItem;

pub struct MainDlgHandler{
    rc_root: Rc<RefCell<RcRoot>>,
}

impl MainDlgHandler {
    pub fn new()->MainDlgHandler {
        MainDlgHandler{
            rc_root: Rc::new(RefCell::new(RcRoot::new())),
        }
    }

	pub fn register_handler(&self, r: &mut ui::Root) {
        r.main_dialog.this_msg().on_close(|_,_|{
            unsafe{user32::PostQuitMessage(0)};
        });

		r.main_dialog.this_msg().on_init_dialog(|_,t|{
            println!("hello main dlg");
            t.main_dialog.this.SetWindowText("GUI Generator");

            //t.main_dialog.edt_rc_path.SetWindowText("K:\\software\\pc\\rust\\wtl-rs\\tools\\ui_gen\\src\\del\\mhc.rc");
            t.main_dialog.edt_rc_path.SetWindowText("K:\\software\\pc\\rust\\wtl-rs\\tools\\ui_gen\\src\\design\\design.rc");
        });

        let rt1 = self.rc_root.clone();
		r.main_dialog.btn_parse_msg().on_click(move|_,t|{
            *rt1.borrow_mut() = parse_msg(t);
		});

        let rt2 = self.rc_root.clone();
        r.main_dialog.btn_select_msg().on_click(move|_,t|{
            let mut rt = rt2.borrow_mut();
            SelectDialog::call(&mut *rt,t);
        });

        let rt3 = self.rc_root.clone();
        r.main_dialog.btn_unselect_msg().on_click(move|_,t|{
            let mut rt = rt3.borrow_mut();
            UnSelectDialog::call(&mut *rt,t);
        });

        let rt4 = self.rc_root.clone();
        r.main_dialog.btn_generate_msg().on_click(move|_,t|{
            let mut rt = rt4.borrow_mut();
            rt.write_files();
        });
	}
}

////////////////////////////////////////////
// use struct here intead of mod to avoid import all mod used here
struct UnSelectDialog;
impl UnSelectDialog {
    // make_path get selected dlgs in RcRoot and then make the path by given path
    // all dialogs should put back to ui::Root after item deleted
    // otherwise ,next time make_path will not work(can't find dlgs in RcRoot)
    fn call(rc_root: &mut RcRoot, t: &mut ui::Root) {
        let item = t.main_dialog.tree_selected_dlgs.GetSelectedItem();
            let sel_txt = item.GetText();

            let del_strings = Self::delete_tree(&item);
            for s in &del_strings {
                t.main_dialog.lst_all_dlgs.AddString(&s);
        }

        // check unselected item and it's children,if the child is dlg,then add to RcRoot and listbox;if child is other container,then just delete it
    }

    fn delete_child(item: &CTreeItem,item_strings: &mut Vec<String>)->bool {
        if item.IsNull() {
            return false;
        }
        
        //delete all children first
        while Self::delete_child(&item.GetChild(), item_strings){}

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
        
        Self::delete_child(item, &mut item_strings);

        item_strings
    }   
}

fn parse_msg(t: &mut ui::Root)->RcRoot {
    //delete existing items
    t.main_dialog.lst_all_dlgs.ResetContent();
    //this does not work
    //t.main_dialog.tree_selected_dlgs.DeleteAllItems();

    //delete orignal root
    let a = t.main_dialog.tree_selected_dlgs.GetRootItem();
    a.Delete();
    // add a new root item
    let a = t.main_dialog.tree_selected_dlgs.GetRootItem();
    let b = a.AddHead("Root",-1);
    b.Select();

    let rf = RcFile;
    let p = t.main_dialog.edt_rc_path.GetWindowText();
    let (mut rc_root,ids) = rf.parse_rc(&p);

    // show all dialog names
    for id in &ids {
        t.main_dialog.lst_all_dlgs.AddString(id);
    }
    
    //*rt1.borrow_mut() = rc_root;

    //assume resource.h in the same directory of .rc file
    let mut header_path = PathBuf::from(Path::new(&p).parent().unwrap());
    header_path.push("resource.h");
    let consts = rf.parse_header(header_path.to_str().unwrap());
    rc_root.set_consts(consts);

    rc_root
}

struct SelectDialog;
impl SelectDialog {
    fn call(rc_root: &mut RcRoot, t: &mut ui::Root) {
        /*
        items in listbox are all dialogs,when selecting a dialog,all containers in it will be added as children in tree_ctrl
        items in tree_ctrl store node type(dialog/container) in itemdata
        when unselect a item,the item type must be check,if the item is container then delete it directly,else push it to the listbox
        */

        //get selected list item
        let mut lst_sel = t.main_dialog.lst_all_dlgs.GetCurSel();
        if lst_sel == -1 {
            return;
        }
        //get text of selected item
        let lst_sel_txt = t.main_dialog.lst_all_dlgs.GetText(lst_sel);

        //delete the selected item in listbox
        t.main_dialog.lst_all_dlgs.DeleteString(lst_sel as UINT);
        //select the item before the deleted one
        if lst_sel > 0 {
            lst_sel -= 1;
        }
        //do selection
        t.main_dialog.lst_all_dlgs.SetCurSel(lst_sel);

        //get selected item of tree_ctrl
        let item = t.main_dialog.tree_selected_dlgs.GetSelectedItem();

        //add the selected item in listbox to the tree_ctrl
        //return the item inserted just now
        let new_item = item.AddTail(&lst_sel_txt,-1);

        //get abs path of the selected item in tree_view
        let mut path = Self::get_item_path(&item);
        println!("{:?}", path);
        
        let child_container = rc_root.make_path(&lst_sel_txt,&mut path);

        // if the selected item has child container,then add to the tree_view
        for c in child_container {
            new_item.AddTail(&c,-1);
        }

        //println!("{}", rc_root);
        rc_root.print();

        //check if the container has child containers



        //expand the button of a new item manually
        //http://www.go4expert.com/forums/i-refresh-expand-sign-treeview-control-t15764/
        item.Expand(None);
    }

    //get path in the reverse order
    //e.g  [about_dlg,main_dlg,root]
    // fn get_item_path(item: &CTreeItem,p: &mut Vec<String>) {
    //     if item.IsNull() {
    //         return;
    //     }

    //     // put the deeper item in the front,it is useful for parse the path
    //     p.push(item.GetText());

    //     let parent = item.GetParent();
        
    //     Self::get_item_path(&parent,p);
    // }

    fn get_item_path(item: &CTreeItem)->Vec<String> {
        let parent = item.GetParent();
        if parent.IsNull() {
            //Root
            let mut v = Vec::new();
            v.push(item.GetText());
            return v;
        }

        let mut v = Self::get_item_path(&parent);
        // put the deeper item in the front,it is useful for parse the path
        v.push(item.GetText());
        //println!("get_item_path:{:?}", v);
        v
    }
}
