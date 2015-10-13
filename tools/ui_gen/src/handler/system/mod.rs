use ui::Root;
mod main_dialog;
pub fn register_handler(r: &mut Root) {
	main_dialog::register_handler(r);
}
