use user32;
// use kernel32;
use winapi::*;
use wtl::atl::*;

use about;

static mut depth: u32 = 0;

pub struct MainDlg {
    dlg: CDialogImpl,
    handler: Handler<MainDlg>,
    about_dlg: about::AboutDlg,
}

impl MainDlg {
    pub fn new() -> MainDlg {
        MainDlg {
            dlg: CDialogImpl::new(129, Self::ProcessWindowMessage, None),
            about_dlg: about::AboutDlg::new(),
            handler: Handler::new(),
        }
    }

    #[trace]
    pub fn create(&mut self) {
  //       self.handler.add_msg_listener(WM_CLOSE, |pself:&Self,uMsg:UINT,wParam:WPARAM,lParam:LPARAM|->LRESULT{
		// 	println!("close main dlg");
		// 	unsafe{user32::PostQuitMessage(0)};
		// 	0
		// });
        self.handler.on_close(|pself|{
            println!("close main dlg");
            unsafe{user32::PostQuitMessage(0)};
        });

        self.handler.on_btn_click(101, |pself,id,cwin|{
            pself.about_dlg.dlg.ShowWindow(SW_SHOW);
        });

  //       self.handler.add_cmd_listener(101, |pself:&Self,code:WORD,id:WORD,lParam:LPARAM|->LRESULT {
		// 	pself.about_dlg.dlg.ShowWindow(SW_SHOW);
		// 	0
		// });

        // Binder::on_btn_click(101, |id,code,cwin|{
        //     self.about_dlg.dlg.ShowWindow(SW_SHOW);
        // });

        // Binder::on_btn_clicks(&[100,101], |id,code,cwin|{
        //     self.about_dlg.dlg.ShowWindow(SW_SHOW);
        // });

        let p = self as *mut Self as *mut c_void;
        self.dlg.DoModal2(p);
    }

    
    pub fn OnInitDialog(&mut self) {
        self.dlg.CenterWindow(NULL_HWND);
        self.about_dlg.Create(self.dlg.GetHwnd());
    }

	//pass OnInitDialog as param

	impl_proc_msg!(OnInitDialog);
}



struct Binder;

//place holders
impl Binder{
    //id,code,CWindow 
    //Todo use CButton instead of cwindow
    fn on_btn_click<F>(id: WORD,f:F) where F:Fn(WORD,WORD,CWindow) {
        // add code here
    }

    fn on_btn_clicks<F>(ids:&[WORD],f:F) where F:Fn(WORD,WORD,CWindow) {
        // add code here
    }
}