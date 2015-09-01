

//pub mod window_trait;

#[macro_use] #[allow(non_snake_case)]
pub mod cwindow;
pub mod thunk;

pub mod dialog;

//pub mod msg_map;

//pub use self::window_trait::{WindowTrait,NULL_HWND};
pub use self::cwindow::{CWindow,NULL_HWND};

pub use self::dialog::CDialogImpl;