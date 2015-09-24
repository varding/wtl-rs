//#![feature(custom_attribute, plugin)]
//#![plugin(trace)]

//the three lines below pass argments to linker to make the programe a win32 gui without a console 
// #![feature(link_args)]
// #[link_args = "-Wl,--subsystem,windows"]
extern {}


#[macro_use]
extern crate wtl;
extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate shell32;
// extern crate rand;

// use rand::Rng;
// use std::rc::Rc;

mod ui;
mod main_dlg;
mod about;
pub mod bloat;
mod make_bloat;


use ui::DialogHandler;

fn main() {
    
    make_bloat::make_call();
    let mut root = ui::Root::new();

    //////////////////////////////////////////////
    // register all handlers
    let main_dlg_handler = main_dlg::MainDialogHandler::new();
    //register all handlers before create
    main_dlg_handler.register_handler(&mut root);

    //////////////////////////////////////////////
    //create root dialog or win
    root.create();

    //////////////////////////////////////////////
    // run message loop
    ui::MessageLoop::run();
}

/*
rustc c2tryMessageBoxW.rs -C link-args="-Wl,--subsystem,windows"
https://www.reddit.com/r/rust/comments/2dfp47/how_to_build_a_win32_gui_application_my_app_code/
*/