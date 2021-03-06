//! wtl: GUI operation library ,include dialogs,buttons,cmd_bar,mdi frame
//! atl: Message loop and disptach


extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate shell32;
extern crate gdi32;
extern crate opengl32;


pub use atl::*;
pub use wtl::*;

pub mod atl;
pub mod wtl;