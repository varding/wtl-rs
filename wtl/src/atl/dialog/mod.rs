
//pub mod dialog;

#[allow(non_snake_case,dead_code,unused_variables)]
pub mod dialog_impl;

#[allow(dead_code)]
mod consts;

pub use self::dialog_impl::{CDialogImpl,CallBack};
//pub use self::dialog::{Dialog,MsgMapTrait};










/*
							impl_root
						/				\
					win_impl_base		dlg_impl_base
					/					/	  |		\
				win_impl		ax_dlg_impl	dlg_imp	simple_dlg

*/