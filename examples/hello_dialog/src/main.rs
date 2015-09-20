#![feature(custom_attribute, plugin,rc_counts)]
#![plugin(trace)]

//the three lines below pass argments to linker to make the programe a win32 gui without a console 
// #![feature(link_args)]
// #[link_args = "-Wl,--subsystem,windows"]
extern {}


#[macro_use]
extern crate wtl;
extern crate winapi;
extern crate user32;
extern crate kernel32;
// extern crate rand;

// use rand::Rng;
// use std::rc::Rc;

mod ui;
mod mhc;

use ui::DialogHandler;
// mod simple;
//mod about;

fn main() {
    // message loop
    let mut msg_loop = ui::MessageLoop::new();

    let main_dlg_handler = mhc::MainDialogHandler;

    //register all handlers
    main_dlg_handler.register_handler(&mut msg_loop);

    //create root dialog or win
    msg_loop.create();

    msg_loop.run();
}

/*
rustc c2tryMessageBoxW.rs -C link-args="-Wl,--subsystem,windows"
https://www.reddit.com/r/rust/comments/2dfp47/how_to_build_a_win32_gui_application_my_app_code/
*/