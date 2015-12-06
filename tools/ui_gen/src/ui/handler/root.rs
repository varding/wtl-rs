use ui::Root;
use user32;
use winapi::*;
use ui::consts::*;
use super::main_dialog;
pub fn register_handler(r: &mut Root) {
    
    main_dialog::register_handler(r);
}