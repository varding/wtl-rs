
#![allow(dead_code,unused_assignments,non_snake_case)]
use std;
use winapi::*;
use user32;
use shell32;

use super::bloat::*;

macro_rules! make_impl {
	($name:ident) => {
		pub struct $name(HWND);

		impl MyCWin for $name {
			fn GetHwnd(&self) -> HWND {
		        self.0
		    }

		    fn Detach(&mut self) -> HWND { //only set m_hWnd = 0,this prevent most write ability from a hwndTrait
		        let hWnd = self.0;
		        self.0 = NULL_HWND;
		        hWnd
		    }

		    fn Attach(&mut self, hWndNew: HWND) {
		        debug_assert!(self.0 == NULL_HWND);
		        debug_assert!(hWndNew != NULL_HWND);
		        unsafe {
		            debug_assert!(user32::IsWindow(hWndNew) == TRUE);
		        }
		        self.0 = hWndNew;
		    }

			fn IsWindow(&self) -> bool {
		        unsafe {
		            user32::IsWindow(self.0) == TRUE
		        }
		    }
		}
	}
}


// macro_rules! do_call {
// 	($name:ident,$tp:ident,$id:expr) => {
// 		let $name = $tp($id as HWND);
// 		$name.display_hwnd();
// 	}
// }
pub fn make_call(){
	let d1 = dlg1(100 as HWND);
	d1.display_hwnd();
	//println!("{}", dlg1::display_hwnd as usize);

	let d2 = dlg2(102 as HWND);
	d2.display_hwnd();
	//println!("{}", dlg2::display_hwnd as usize);

	let d3 = dlg3(104 as HWND);
	d3.display_hwnd();
	
	let d4 = dlg4(105 as HWND);
	d4.display_hwnd();

	let d5 = dlg5(106 as HWND);
	d5.display_hwnd();

	let d6 = dlg6(107 as HWND);
	d6.display_hwnd();

	let d7 = dlg7(108 as HWND);
	d7.display_hwnd();

	let d8 = dlg8(109 as HWND);
	d8.display_hwnd();

	let d9 = dlg9(110 as HWND);
	d9.display_hwnd();

	let d10 = dlg10(111 as HWND);
	d10.display_hwnd();

	let d11 = dlg11(112 as HWND);
	d11.display_hwnd();

	let d12 = dlg12(113 as HWND);
	d12.display_hwnd();

	let d13 = dlg13(114 as HWND);
	d13.display_hwnd();

	//do_call!(d1,dlg1,100);
}

make_impl!(dlg1);
make_impl!(dlg2);
make_impl!(dlg3);
make_impl!(dlg4);
make_impl!(dlg5);
make_impl!(dlg6);
make_impl!(dlg7);
make_impl!(dlg8);
make_impl!(dlg9);
make_impl!(dlg10);
make_impl!(dlg11);
make_impl!(dlg12);
make_impl!(dlg13);
// make_impl!(dlg14);
// make_impl!(dlg15);
// make_impl!(dlg16);
// make_impl!(dlg17);
// make_impl!(dlg18);
// make_impl!(dlg19);
