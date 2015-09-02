

use user32;
// use kernel32;
 use winapi::*;
 use wtl::atl::*;

//use about;

pub struct MainDlg {
    dlg: CDialogImpl,
}

// impl MsgMapTrait for MainDlg {
// 	fn message_handler(uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL)->LRESULT{
// 		0
// 	}	
// }

impl MainDlg {
	pub fn new()-> MainDlg{
		// let mut m = MainDlg{
		// 	dlg:CDialogImpl::new(129,Self::DialogProc)
		// };
		// let pself = &mut m as *mut MainDlg as  *mut c_void;
		// m.dlg.set_pself(pself);
		// //println!("real cb:{:p}", &Self::DialogProc);
		// m
		MainDlg{
			dlg:CDialogImpl::new(129,Self::DialogProc)
		}
	}

	pub fn do_modal(&mut self){
		// self.dlg.add_listener(WM_CLOSE, |uMsg:UINT,wParam:WPARAM,lParam:LPARAM,bHandled:&mut BOOL|{
		// 	println!("{}", uMsg);
		// 	0
		// });
		//self.dlg.DoModal(NULL_HWND, 0 as LPARAM);
		let pself = self as *mut Self as *mut c_void;
		self.dlg.set_pself(pself);
		self.dlg.DoModal2();
	}
	// unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> INT_PTR;
	//pself:*mut c_void,  ,lResult:&mut LRESULT,dwMsgMapID:DWORD
	//pub fn DialogProc(hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM ) -> INT_PTR{
	pub fn DialogProc(pself:*mut c_void,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL{
		if uMsg == WM_CLOSE{
		// if self.m_bModal{
		// 	self.EndDialog(0);
		// }else{
		// 	self.DestroyWindow();
		// }
			unsafe{user32::PostQuitMessage(0)};
		}
		0
	}
}	

// pub struct Dialog {
//     m_hWnd: HWND,
//     thk   : &'static mut thunk::Thunk
// }

// impl Dialog {
// 	pub fn new()->Dialog{
// 		Dialog{
// 			m_hWnd:NULL_HWND,
// 			thk   :thunk::get_thunk(),
// 		}
// 	}
// }
// //auto impl winTrait
// impl_hwnd_trait!(Dialog);

// impl CDialogImpl for Dialog {}
// impl CWindowImplRoot for Dialog{}
// impl CDialogImplBaseT for Dialog {}
// //after impl the two trait,Dialog auto impl CDialogImpl
// impl DialogTrait for Dialog {

// 	fn InitThunk(&mut self,h:HWND,dlg_proc:DLGPROC) -> DLGPROC { //convert &mut self to *const T in this method
// 		println!("init thunk,mhc proc,proc addr:{}",dlg_proc as usize);
// 		self.m_hWnd = h;
// 		dlg_proc
// 	}		

// 	fn ProcessWindowMessage(&self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL{
// 		//println!("proc msg in mhc:{}", uMsg);
// 		match uMsg {
// 			WM_CLOSE=>{
// 				unsafe{user32::PostQuitMessage(0)};
// 				//self.EndDialog(0);	//todo:this will cause an assertion error
// 				TRUE
// 			},
// 			WM_COMMAND=>{
// 				let id = LOWORD(wParam as DWORD);
// 				match id {
// 					1000=>{
// 						show_msg_dlg();
// 					},
// 					101=>{
// 						let mut a = about::Dialog::new();
// 						a.DoModal();
// 					},
// 					_=>{

// 					},
// 				}
// 				TRUE
// 			},
// 			_=>FALSE,
// 		}
		
// 	}

// 	fn State(&self)->DWORD {
// 		1
// 	}

// 	fn AddState(&self,s:DWORD) {

// 	}

// 	fn OnFinalMessage(&self){

// 	}

// 	fn IDD(&self)->WORD{
// 		//101
// 		129
// 	}
// }

// fn show_msg_dlg(){
// 	let hello = "hello大家好";

// 	unsafe{
// 		let out = [0u16,24];
// 		let wcsLen = kernel32::MultiByteToWideChar(CP_UTF8, 0, hello as *const str as LPCCH , hello.len() as c_int, out.as_ptr() as LPWSTR, 24);
// 		//println!("{}", wcsLen);
// 		user32::MessageBoxW(NULL_HWND, out.as_ptr() as LPCWSTR, out.as_ptr() as LPCWSTR, 0u32);
// 	}
// }