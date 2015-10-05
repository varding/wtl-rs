//! default handlers,bind all ctrls in dialog
use ui::Root;

mod main_dialog;

pub fn register_handler(r: &mut Root) {
	let m = main_dialog::MainDlgHandler;
	m.register_handler(r);
}