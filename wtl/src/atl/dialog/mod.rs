










pub use self::dialog::{Dialog};
//pub use self::dialog::{Dialog,MsgMapTrait};
pub use self::event::Event;


#[allow(non_snake_case,dead_code,unused_variables)]
pub mod dialog;

#[allow(dead_code)]
mod consts;

mod event;









/*
							impl_root
						/				\
					win_impl_base		dlg_impl_base
					/					/	  |		\
				win_impl		ax_dlg_impl	dlg_imp	simple_dlg

*/
