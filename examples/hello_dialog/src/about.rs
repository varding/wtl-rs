
use kernel32;
use user32;
use winapi::*;
use wtl::atl::*;

use super::ui;

//100
// pub struct AboutDlg {
//     pub dlg: CDialogImpl,
//     handler: Handler<AboutDlg>,
// }

// impl AboutDlg {
//     pub fn new() -> AboutDlg {
//         AboutDlg {
//             dlg: CDialogImpl::new(100, Self::ProcessWindowMessage, None),
//             handler: Handler::new(),
//         }
//     }

//     pub fn Create(&mut self, h: HWND) {
//   //       self.handler.add_msg_listener(WM_CLOSE, |pself:&Self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM|->LRESULT{
// 		// 	println!("close main dlg");
// 		// 	pself.dlg.ShowWindow(SW_HIDE);
// 		// 	0
// 		// });
//         self.handler.on_close(|pself|{
//             println!("close main dlg");
//             pself.dlg.ShowWindow(SW_HIDE);
//         });

// 		//IDOK
//         self.handler.on_btn_click(1, |pself,id,cwin|{
//             pself.show_msg_dlg(pself.dlg.GetHwnd());
//         });
//   //       self.handler.add_cmd_listener(1, |pself:&Self,code:WORD,id:WORD,lParam:LPARAM|->LRESULT {
// 		// 	pself.show_msg_dlg(pself.dlg.GetHwnd());
// 		// 	0
// 		// });

//         let pself = self as *mut Self as *mut c_void;
//         self.dlg.Create2(pself,h);
//     }

//     pub fn show_msg_dlg(&self, h: HWND) {
//         let hello = "hello\u{5927}\u{5bb6}\u{597d}";

//         unsafe {
//             let out = [0u16,24];
//             let wcsLen = kernel32::MultiByteToWideChar(CP_UTF8,
//                                                        0,
//                                                        hello as *const str as LPCCH,
//                                                        hello.len() as c_int,
//                                                        out.as_ptr() as LPWSTR,
//                                                        24);
// 			//println!("{}", wcsLen);
//             user32::MessageBoxW(h, out.as_ptr() as LPCWSTR, out.as_ptr() as LPCWSTR, 0u32);
//         }
//     }

//     pub fn OnInitDialog(&mut self) {
//         self.dlg.CenterWindow(NULL_HWND);
//     }
// 	//pass OnInitDialog as param, if no init_dialog,left params empty
// 	impl_proc_msg!(OnInitDialog);
// }


pub struct AboutDialogHandler;

impl ui::DialogHandler for AboutDialogHandler {
    fn register_handler(&self,r:&mut ui::Root){
        r.main_dlg.about_dialog.this_msg().on_init_dialog(0, |e,_|{
            println!("hello about");
            1
        });

        r.main_dlg.about_dialog.this_msg().on_close(0, |e:&Event,t:&mut ui::Root|{
            println!("bye about dlg");
            //unsafe{user32::PostQuitMessage(0)};
            t.main_dlg.about_dialog.this.DestroyWindow();
            1
        });
    }
}