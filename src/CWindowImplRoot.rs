

//include base message operations like ForwardNotifications,ReflectNotifications,DefaultReflectionHandler and handle state

/*
							impl_root
						/				\
					win_impl_base		dlg_impl_base
					/					/	  |		\
				win_impl		ax_dlg_impl	dlg_imp	simple_dlg

*/
use winapi::{MSG};

use thunk;
pub struct CWindowImplRoot {
    proc_thunk: thunk::Thunk,

}