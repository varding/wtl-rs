#![allow(non_snake_case)]

#![allow(dead_code)]
use winapi::*;

pub struct COMBOBOXINFO {
	cbSize: DWORD,
    rcItem: RECT,
    rcButton: RECT,
    stateButton: DWORD,
    hwndCombo: HWND,
    hwndItem: HWND,
    hwndList: HWND,
}

pub type PCOMBOBOXINFO = *mut COMBOBOXINFO;


pub type EDITWORDBREAKPROCW = fn(lpch: LPWSTR, ichCurrent: c_int, cch: c_int, code: c_int)->c_int;