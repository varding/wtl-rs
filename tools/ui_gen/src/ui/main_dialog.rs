#![allow(dead_code)]
use wtl::ctrls::{CTreeViewCtrlEx,Button,BtnMsg,Edit,ListBox};
use wtl::atl::{Dialog,DlgMsg};

pub struct MainDialog<T> {
    pub this: Dialog<T>,
    pub lst_all_dlgs: ListBox,
    pub dlg_tree: CTreeViewCtrlEx,
    pub btn_parse: Button,
    pub btn_select: Button,
    pub btn_unselect: Button,
    pub btn_generate: Button,
    pub edt_rc_path: Edit,
}

impl<T> MainDialog<T> {
	pub fn new()->MainDialog<T> {
		MainDialog{
			this: Dialog::new(101),
			lst_all_dlgs: ListBox::new(),
			dlg_tree: CTreeViewCtrlEx::new(),
			btn_parse: Button::new(),
			btn_select: Button::new(),
			btn_unselect: Button::new(),
			btn_generate: Button::new(),
			edt_rc_path: Edit::new(),
		}
	}

	pub fn create(&mut self,r: *mut T){
		self.this.Create3(r);
	}

	////////////////////////
	pub fn this_msg(&mut self)->DlgMsg<T> {
		self.this.msg_handler()
	}

	pub fn btn_parse_msg(&mut self)->BtnMsg<T> {
		self.this.btn_handler(1005)
	}

	pub fn btn_select_msg(&mut self)->BtnMsg<T> {
		self.this.btn_handler(1003)
	}

	pub fn btn_unselect_msg(&mut self)->BtnMsg<T> {
		self.this.btn_handler(1004)
	}

	pub fn btn_generate_msg(&mut self)->BtnMsg<T> {
		self.this.btn_handler(1007)
	}
}