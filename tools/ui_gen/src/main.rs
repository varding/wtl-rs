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

use handler::MainDlgHandler;

fn main() {
    let mut root = ui::Root::new();
    
    //register some default behavior generate by ui_gen tool
    ui::register_handler(&mut root);

    //register user defined handlers
    let h = MainDlgHandler::new();
    h.register_handler(&mut root);

    //create dialogs that managered by ui_gen tool
    root.create();

    //run message loop
    ui::MessageLoop::run();
}
