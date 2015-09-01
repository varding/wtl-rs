#[macro_use]
extern crate wtl;
extern crate winapi;
extern crate user32;
extern crate kernel32;

mod simple;
mod mhc;
mod about;

fn main() {
	println!("hello");
	let mut a = about::AboutDlg::new();
	a.Create();

	let mut s = simple::SimpleDlg::new();
	s.Create();

	let mut d = mhc::MainDlg::new();
	d.do_modal();
}

/*
rustc c2tryMessageBoxW.rs -C link-args="-Wl,--subsystem,windows"
https://www.reddit.com/r/rust/comments/2dfp47/how_to_build_a_win32_gui_application_my_app_code/
*/