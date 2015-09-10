#![feature(custom_attribute, plugin)]
#![plugin(trace)]

//the three lines below pass argments to linker to make the programe a win32 gui without a console 
#![feature(link_args)]
#[link_args = "-Wl,--subsystem,windows"]
extern {}


#[macro_use]
extern crate wtl;
extern crate winapi;
extern crate user32;
extern crate kernel32;


mod mhc;
// mod simple;
mod about;


fn main() {
	// let mut s = simple::SimpleDlg::new();
	// s.Create();

    let mut d = mhc::MainDlg::new();
    d.create();
}

/*
rustc c2tryMessageBoxW.rs -C link-args="-Wl,--subsystem,windows"
https://www.reddit.com/r/rust/comments/2dfp47/how_to_build_a_win32_gui_application_my_app_code/
*/
