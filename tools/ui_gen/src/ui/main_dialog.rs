
use wtl::ctrls::{CTreeViewCtrlEx,CTreeItem,Button,BtnMsg,Edit,EdtMsg};
use wtl::atl::{Dialog,DlgMsg,NULL_HWND};

pub struct MainDialog<T> {
    pub this: Dialog<T>,
    pub dlg_tree: CTreeViewCtrlEx,
    pub btn_parse: Button,
    pub edt_rc_path: Edit,
}

impl<T> MainDialog<T> {
	pub fn new()->MainDialog<T> {
		MainDialog{
			this: Dialog::new(101),
			dlg_tree: CTreeViewCtrlEx::new(),
			btn_parse: Button::new(),
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
}