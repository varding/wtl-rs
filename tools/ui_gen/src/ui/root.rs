#![allow(dead_code)]
use wtl::*;
use ui::consts::*;
use super::sub_root::*;
pub struct Root {
    pub main_dialog: MainDialog<Root>,
}
impl Root {
    
    pub fn new()->Root{
        Root{
            main_dialog: MainDialog::new(),
        }
    }
    
    pub fn create(&mut self){
        let r = self as *mut _ ;
        self.main_dialog.create(r);
    }
}