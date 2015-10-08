// #![feature(link_args)]
// #[link_args = "-Wl,--subsystem,windows"]
// extern {}


extern crate regex;
extern crate wtl;
extern crate user32;
extern crate winapi;
extern crate comctl32;

mod ui;
mod handler;

use ui::{Root,MessageLoop};
use handler::MainDlgHandler;

fn main() {
    let mut root = Root::new();
    
    //register some default behavior generate by ui_gen tool
    handler::system::register_handler(&mut root);
    
    //register user defined handlers
    let h = MainDlgHandler::new();
    h.register_handler(&mut root);

    //create dialogs that managered by ui_gen tool
    root.create();

    //run message loop
    MessageLoop::run();
}
