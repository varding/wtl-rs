extern crate regex;
extern crate wtl;
extern crate user32;
extern crate winapi;

mod ui;
mod handler;

use ui::{Root,MessageLoop};
use handler::MainDlgHandler;

fn main() {
    let mut root = Root::new();
    
    let h = MainDlgHandler;
    h.register_handler(&mut root);

    root.create();
    MessageLoop::run();
}
