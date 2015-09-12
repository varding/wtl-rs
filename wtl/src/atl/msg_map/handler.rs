
use user32;
use kernel32;
use winapi::*;
use std::collections::BTreeMap;
use super::CWindow;
use super::OptCall;
use super::MsgEntry;
use super::CmdEntry;

pub struct Handler<T> {
    msg_entry: MsgEntry<T>,
    cmd_entry: CmdEntry<T>,
    //edit_msg : EditMsg,
}

impl<T> Handler<T> {
    pub fn new() -> Handler<T> {
        Handler {
            msg_entry: MsgEntry::new(),
            cmd_entry: CmdEntry::new(), 
        }
    }
}

/// atlcrack.h
impl<T> Handler<T> {
    ///////////////////////////////////////////////////////////////////////////////
    // Standard Windows message macros

    // int OnCreate(LPCREATESTRUCT lpCreateStruct)
    // #define MSG_WM_CREATE(func) \
    //     if (uMsg == WM_CREATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((LPCREATESTRUCT)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_create<F>(&mut self,f:F) where F:Fn(&T,&CREATESTRUCTW)->c_int + 'static {
        self.msg_entry.on_create = Some(Box::new(f));
    }

    // // BOOL OnInitDialog(CWindow wndFocus, LPARAM lInitParam)
    // #define MSG_WM_INITDIALOG(func) \
    //     if (uMsg == WM_INITDIALOG) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HWND)wParam, lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_init_dialog<F>(&mut self,f:F) where F:Fn(&T,&CWindow,LPARAM)->BOOL + 'static {
        self.msg_entry.on_init_dialog = Some(Box::new(f));
    }
/*
    // // BOOL OnCopyData(CWindow wnd, PCOPYDATASTRUCT pCopyDataStruct)
    // #define MSG_WM_COPYDATA(func) \
    //     if (uMsg == WM_COPYDATA) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HWND)wParam, (PCOPYDATASTRUCT)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_copy_data<F>(&mut self,f:F) where F:Fn(&T,&COPYDATASTRUCT) + 'static {
        self.msg_entry.on_copy_data = Some(Box::new(f));
    }

    // // void OnDestroy()
    // #define MSG_WM_DESTROY(func) \
    //     if (uMsg == WM_DESTROY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_destroy<F>(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_destroy = Some(Box::new(f));
    }

    // // void OnMove(CPoint ptPos)
    // #define MSG_WM_MOVE(func) \
    //     if (uMsg == WM_MOVE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(_WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_move<F>(&mut self,f:F) where F:Fn(&T,&CPoint) + 'static {
        self.msg_entry.on_move = Some(Box::new(f));
    }


    // // void OnSize(UINT nType, CSize size)
    // #define MSG_WM_SIZE(func) \
    //     if (uMsg == WM_SIZE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CSize(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_size<F>(&mut self,f:F) where F:Fn(&T,UINT,&CSize) + 'static {
        self.msg_entry.on_size = Some(Box::new(f));
    }



    // // void OnActivate(UINT nState, BOOL bMinimized, CWindow wndOther)
    // #define MSG_WM_ACTIVATE(func) \
    //     if (uMsg == WM_ACTIVATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)LOWORD(wParam), (BOOL)HIWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_activate<F>(&mut self,f:F) where F:Fn(&T,UINT,BOOL,&CWindow) + 'static {
        self.msg_entry.on_activate = Some(Box::new(f));
    }  


    // // void OnSetFocus(CWindow wndOld)
    // #define MSG_WM_SETFOCUS(func) \
    //     if (uMsg == WM_SETFOCUS) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_set_focus<F>(&mut self,f:F) where F:Fn(&T,&CWindow) + 'static {
        self.msg_entry.on_set_focus = Some(Box::new(f));
    }   


    // // void OnKillFocus(CWindow wndFocus)
    // #define MSG_WM_KILLFOCUS(func) \
    //     if (uMsg == WM_KILLFOCUS) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_kill_focus(&mut self,f:F) where F:Fn(&T,&CWindow) + 'static {
        self.msg_entry.on_kill_focus = Some(Box::new(f));
    }


    // // void OnEnable(BOOL bEnable)
    // #define MSG_WM_ENABLE(func) \
    //     if (uMsg == WM_ENABLE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_enable(&mut self,f:F) where F:Fn(&T,BOOL) + 'static {
        self.msg_entry.on_enable = Some(Box::new(f));
    }


    // // void OnPaint(CDCHandle dc)
    // #define MSG_WM_PAINT(func) \
    //     if (uMsg == WM_PAINT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HDC)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_paint(&mut self,f:F) where F:Fn(&T,&CDCHandle) + 'static {
        self.msg_entry.on_paint = Some(Box::new(f));
    }
*/
    // // void OnClose()
    // #define MSG_WM_CLOSE(func) \
    //     if (uMsg == WM_CLOSE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_close<F>(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_close = Some(Box::new(f));
    }

/*
    // // BOOL OnQueryEndSession(UINT nSource, UINT uLogOff)
    // #define MSG_WM_QUERYENDSESSION(func) \
    //     if (uMsg == WM_QUERYENDSESSION) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)wParam, (UINT)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_query_end_session(&mut self,f:F) where F:Fn(&T,UINT,UINT) + 'static {
        self.msg_entry.on_query_end_session = Some(Box::new(f));
    }

    // // BOOL OnQueryOpen()
    // #define MSG_WM_QUERYOPEN(func) \
    //     if (uMsg == WM_QUERYOPEN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_query_open(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_query_open = Some(Box::new(f));
    }

    // // BOOL OnEraseBkgnd(CDCHandle dc)
    // #define MSG_WM_ERASEBKGND(func) \
    //     if (uMsg == WM_ERASEBKGND) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_erase_bkgnd(&mut self,f:F) where F:Fn(&T,&CDCHandle) + 'static {
        self.msg_entry.on_erase_bkgnd = Some(Box::new(f));
    }

    // // void OnSysColorChange()
    // #define MSG_WM_SYSCOLORCHANGE(func) \
    //     if (uMsg == WM_SYSCOLORCHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sys_color_change(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_sys_color_change = Some(Box::new(f));
    }

    // // void OnEndSession(BOOL bEnding, UINT uLogOff)
    // #define MSG_WM_ENDSESSION(func) \
    //     if (uMsg == WM_ENDSESSION) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam, (UINT)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_end_session(&mut self,f:F) where F:Fn(&T,BOOL,UINT) + 'static {
        self.msg_entry.on_end_session = Some(Box::new(f));
    }


    // // void OnShowWindow(BOOL bShow, UINT nStatus)
    // #define MSG_WM_SHOWWINDOW(func) \
    //     if (uMsg == WM_SHOWWINDOW) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam, (int)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_show_window(&mut self,f:F) where F:Fn(&T,BOOL,UINT) + 'static {
        self.msg_entry.on_show_window = Some(Box::new(f));
    }


    // // HBRUSH OnCtlColorEdit(CDCHandle dc, CEdit edit)
    // #define MSG_WM_CTLCOLOREDIT(func) \
    //     if (uMsg == WM_CTLCOLOREDIT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ctl_color_edit(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CEdit) + 'static {
        self.msg_entry.on_ctl_color_edit = Some(Box::new(f));
    }

    // // HBRUSH OnCtlColorListBox(CDCHandle dc, CListBox listBox)
    // #define MSG_WM_CTLCOLORLISTBOX(func) \
    //     if (uMsg == WM_CTLCOLORLISTBOX) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ctl_color_list_box(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CListBox) + 'static {
        self.msg_entry.on_ctl_color_list_box = Some(Box::new(f));
    }

    // // HBRUSH OnCtlColorBtn(CDCHandle dc, CButton button)
    // #define MSG_WM_CTLCOLORBTN(func) \
    //     if (uMsg == WM_CTLCOLORBTN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ctl_color_btn(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CButton) + 'static {
        self.msg_entry.on_ctl_color_btn = Some(Box::new(f));
    }

    // // HBRUSH OnCtlColorDlg(CDCHandle dc, CWindow wnd)
    // #define MSG_WM_CTLCOLORDLG(func) \
    //     if (uMsg == WM_CTLCOLORDLG) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ctl_color_dlg(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CWindow) + 'static {
        self.msg_entry.on_ctl_color_dlg = Some(Box::new(f));
    }

    // // HBRUSH OnCtlColorScrollBar(CDCHandle dc, CScrollBar scrollBar)
    // #define MSG_WM_CTLCOLORSCROLLBAR(func) \
    //     if (uMsg == WM_CTLCOLORSCROLLBAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ctl_color_scroll_bar(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CScrollBar) + 'static {
        self.msg_entry.on_ctl_color_scroll_bar = Some(Box::new(f));
    }

    // // HBRUSH OnCtlColorStatic(CDCHandle dc, CStatic wndStatic)
    // #define MSG_WM_CTLCOLORSTATIC(func) \
    //     if (uMsg == WM_CTLCOLORSTATIC) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ctl_color_static(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CStatic) + 'static {
        self.msg_entry.on_ctl_color_static = Some(Box::new(f));
    }


    // // void OnSettingChange(UINT uFlags, LPCTSTR lpszSection)
    // #define MSG_WM_SETTINGCHANGE(func) \
    //     if (uMsg == WM_SETTINGCHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPCTSTR)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // void OnDevModeChange(LPCTSTR lpDeviceName)
    // #define MSG_WM_DEVMODECHANGE(func) \
    //     if (uMsg == WM_DEVMODECHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((LPCTSTR)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // void OnActivateApp(BOOL bActive, DWORD dwThreadID)
    // #define MSG_WM_ACTIVATEAPP(func) \
    //     if (uMsg == WM_ACTIVATEAPP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam, (DWORD)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_activate_app(&mut self,f:F) where F:Fn(&T,BOOL,DWORD) + 'static {
        self.msg_entry.on_activate_app = Some(Box::new(f));
    }

    // // void OnFontChange()
    // #define MSG_WM_FONTCHANGE(func) \
    //     if (uMsg == WM_FONTCHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_font_change(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_font_change = Some(Box::new(f));
    }

    // // void OnTimeChange()
    // #define MSG_WM_TIMECHANGE(func) \
    //     if (uMsg == WM_TIMECHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_time_change(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_time_change = Some(Box::new(f));
    }

    // // void OnCancelMode()
    // #define MSG_WM_CANCELMODE(func) \
    //     if (uMsg == WM_CANCELMODE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_cancel_mode(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_cancel_mode = Some(Box::new(f));
    }


    // // BOOL OnSetCursor(CWindow wnd, UINT nHitTest, UINT message)
    // #define MSG_WM_SETCURSOR(func) \
    //     if (uMsg == WM_SETCURSOR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam)); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_set_cursor(&mut self,f:F) where F:Fn(&T,&CWindow,UINT,UINT) + 'static {
        self.msg_entry.on_set_cursor = Some(Box::new(f));
    }


    // // int OnMouseActivate(CWindow wndTopLevel, UINT nHitTest, UINT message)
    // #define MSG_WM_MOUSEACTIVATE(func) \
    //     if (uMsg == WM_MOUSEACTIVATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam)); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_mouse_activate(&mut self,f:F) where F:Fn(&T,&CWindow,UINT,UINT) + 'static {
        self.msg_entry.on_mouse_activate = Some(Box::new(f));
    }


    // // void OnChildActivate()
    // #define MSG_WM_CHILDACTIVATE(func) \
    //     if (uMsg == WM_CHILDACTIVATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_child_activate(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_child_activate = Some(Box::new(f));
    }


    // // void OnGetMinMaxInfo(LPMINMAXINFO lpMMI)
    // #define MSG_WM_GETMINMAXINFO(func) \
    //     if (uMsg == WM_GETMINMAXINFO) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((LPMINMAXINFO)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_get_min_max_info(&mut self,f:F) where F:Fn(&T,&MINMAXINFO) + 'static {
        self.msg_entry.on_get_min_max_info = Some(Box::new(f));
    }

    // // void OnIconEraseBkgnd(CDCHandle dc)
    // #define MSG_WM_ICONERASEBKGND(func) \
    //     if (uMsg == WM_ICONERASEBKGND) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HDC)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_icon_erase_bkgnd(&mut self,f:F) where F:Fn(&T,&CDCHandle) + 'static {
        self.msg_entry.on_icon_erase_bkgnd = Some(Box::new(f));
    }


    // // void OnSpoolerStatus(UINT nStatus, UINT nJobs)
    // #define MSG_WM_SPOOLERSTATUS(func) \
    //     if (uMsg == WM_SPOOLERSTATUS) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (UINT)LOWORD(lParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_spooler_status(&mut self,f:F) where F:Fn(&T,UINT,UINT) + 'static {
        self.msg_entry.on_spooler_status = Some(Box::new(f));
    }

    // // void OnDrawItem(int nIDCtl, LPDRAWITEMSTRUCT lpDrawItemStruct)
    // #define MSG_WM_DRAWITEM(func) \
    //     if (uMsg == WM_DRAWITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPDRAWITEMSTRUCT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_draw_item(&mut self,f:F) where F:Fn(&T,c_int,&DRAWITEMSTRUCT) + 'static {
        self.msg_entry.on_draw_item = Some(Box::new(f));
    }

    // // void OnMeasureItem(int nIDCtl, LPMEASUREITEMSTRUCT lpMeasureItemStruct)
    // #define MSG_WM_MEASUREITEM(func) \
    //     if (uMsg == WM_MEASUREITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPMEASUREITEMSTRUCT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_measure_item(&mut self,f:F) where F:Fn(&T,c_int,&MEASUREITEMSTRUCT) + 'static {
        self.msg_entry.on_measure_item = Some(Box::new(f));
    }

    // // void OnDeleteItem(int nIDCtl, LPDELETEITEMSTRUCT lpDeleteItemStruct)
    // #define MSG_WM_DELETEITEM(func) \
    //     if (uMsg == WM_DELETEITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPDELETEITEMSTRUCT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_delete_item(&mut self,f:F) where F:Fn(&T,c_int,&DELETEITEMSTRUCT) + 'static {
        self.msg_entry.on_delete_item = Some(Box::new(f));
    }

    // //int OnCharToItem(UINT nChar, UINT nIndex, CListBox listBox)
    // #define MSG_WM_CHARTOITEM(func) \
    //     if (uMsg == WM_CHARTOITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_char_to_item(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CListBox) + 'static {
        self.msg_entry.on_char_to_item = Some(Box::new(f));
    }

    // // int OnVKeyToItem(UINT nKey, UINT nIndex, CListBox listBox)
    // #define MSG_WM_VKEYTOITEM(func) \
    //     if (uMsg == WM_VKEYTOITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_v_key_to_item(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CListBox) + 'static {
        self.msg_entry.on_v_key_to_item = Some(Box::new(f));
    }

    // // HCURSOR OnQueryDragIcon()
    // #define MSG_WM_QUERYDRAGICON(func) \
    //     if (uMsg == WM_QUERYDRAGICON) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_query_drag_icon(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_query_drag_icon = Some(Box::new(f));
    }

    // // int OnCompareItem(int nIDCtl, LPCOMPAREITEMSTRUCT lpCompareItemStruct)
    // #define MSG_WM_COMPAREITEM(func) \
    //     if (uMsg == WM_COMPAREITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)wParam, (LPCOMPAREITEMSTRUCT)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_compare_item(&mut self,f:F) where F:Fn(&T,c_int,&COMPAREITEMSTRUCT) + 'static {
        self.msg_entry.on_compare_item = Some(Box::new(f));
    }

    // // void OnCompacting(UINT nCpuTime)
    // #define MSG_WM_COMPACTING(func) \
    //     if (uMsg == WM_COMPACTING) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_compacting(&mut self,f:F) where F:Fn(&T,UINT) + 'static {
        self.msg_entry.on_compacting = Some(Box::new(f));
    }

    // // BOOL OnNcCreate(LPCREATESTRUCT lpCreateStruct)
    // #define MSG_WM_NCCREATE(func) \
    //     if (uMsg == WM_NCCREATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((LPCREATESTRUCT)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_create(&mut self,f:F) where F:Fn(&T,&CREATESTRUCT) + 'static {
        self.msg_entry.on_nc_create = Some(Box::new(f));
    }

    // // void OnNcDestroy()
    // #define MSG_WM_NCDESTROY(func) \
    //     if (uMsg == WM_NCDESTROY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_destroy(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_nc_destroy = Some(Box::new(f));
    }

    // // LRESULT OnNcCalcSize(BOOL bCalcValidRects, LPARAM lParam)
    // #define MSG_WM_NCCALCSIZE(func) \
    //     if (uMsg == WM_NCCALCSIZE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((BOOL)wParam, lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_calc_size(&mut self,f:F) where F:Fn(&T,BOOL,LPARAM) + 'static {
        self.msg_entry.on_nc_calc_size = Some(Box::new(f));
    }

    // // UINT OnNcHitTest(CPoint point)
    // #define MSG_WM_NCHITTEST(func) \
    //     if (uMsg == WM_NCHITTEST) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func(_WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_hit_test(&mut self,f:F) where F:Fn(&T,&CPoint) + 'static {
        self.msg_entry.on_nc_hit_test = Some(Box::new(f));
    }

    // // void OnNcPaint(CRgnHandle rgn)
    // #define MSG_WM_NCPAINT(func) \
    //     if (uMsg == WM_NCPAINT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HRGN)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_paint(&mut self,f:F) where F:Fn(&T,CRgnHandle) + 'static {
        self.msg_entry.on_nc_paint = Some(Box::new(f));
    }

    // // BOOL OnNcActivate(BOOL bActive)
    // #define MSG_WM_NCACTIVATE(func) \
    //     if (uMsg == WM_NCACTIVATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((BOOL)wParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_activate(&mut self,f:F) where F:Fn(&T,BOOL) + 'static {
        self.msg_entry.on_nc_activate = Some(Box::new(f));
    }

    // // UINT OnGetDlgCode(LPMSG lpMsg)
    // #define MSG_WM_GETDLGCODE(func) \
    //     if (uMsg == WM_GETDLGCODE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((LPMSG)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_get_dlg_code(&mut self,f:F) where F:Fn(&T,&MSG) + 'static {
        self.msg_entry.on_get_dlg_code = Some(Box::new(f));
    }


    // // void OnNcMouseMove(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCMOUSEMOVE(func) \
    //     if (uMsg == WM_NCMOUSEMOVE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_mouse_move(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_mouse_move = Some(Box::new(f));
    }


    // // void OnNcLButtonDown(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCLBUTTONDOWN(func) \
    //     if (uMsg == WM_NCLBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_l_button_down(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_l_button_down = Some(Box::new(f));
    }


    // // void OnNcLButtonUp(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCLBUTTONUP(func) \
    //     if (uMsg == WM_NCLBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_l_button_up(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_l_button_up = Some(Box::new(f));
    }


    // // void OnNcLButtonDblClk(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCLBUTTONDBLCLK(func) \
    //     if (uMsg == WM_NCLBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_l_button_db_clk(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_l_button_db_clk = Some(Box::new(f));
    }

    // // void OnNcRButtonDown(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCRBUTTONDOWN(func) \
    //     if (uMsg == WM_NCRBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_r_button_down(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_r_button_down = Some(Box::new(f));
    }

    // // void OnNcRButtonUp(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCRBUTTONUP(func) \
    //     if (uMsg == WM_NCRBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_r_button_up(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_r_button_up = Some(Box::new(f));
    }


    // // void OnNcRButtonDblClk(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCRBUTTONDBLCLK(func) \
    //     if (uMsg == WM_NCRBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_r_button_dbl_clk(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_r_button_dbl_clk = Some(Box::new(f));
    }


    // // void OnNcMButtonDown(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCMBUTTONDOWN(func) \
    //     if (uMsg == WM_NCMBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_m_button_down(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_m_button_down = Some(Box::new(f));
    }


    // // void OnNcMButtonUp(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCMBUTTONUP(func) \
    //     if (uMsg == WM_NCMBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_m_button_up(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_m_button_up = Some(Box::new(f));
    }


    // // void OnNcMButtonDblClk(UINT nHitTest, CPoint point)
    // #define MSG_WM_NCMBUTTONDBLCLK(func) \
    //     if (uMsg == WM_NCMBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_nc_m_button_dbl_clk(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_nc_m_button_dbl_clk = Some(Box::new(f));
    }


    // // void OnKeyDown(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_KEYDOWN(func) \
    //     if (uMsg == WM_KEYDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_key_down(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_key_down = Some(Box::new(f));
    }


    // // void OnKeyUp(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_KEYUP(func) \
    //     if (uMsg == WM_KEYUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_key_up(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_key_up = Some(Box::new(f));
    }


    // // void OnChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_CHAR(func) \
    //     if (uMsg == WM_CHAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_char(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_char = Some(Box::new(f));
    }


    // // void OnDeadChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_DEADCHAR(func) \
    //     if (uMsg == WM_DEADCHAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_dead_char(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_dead_char = Some(Box::new(f));
    }


    // // void OnSysKeyDown(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_SYSKEYDOWN(func) \
    //     if (uMsg == WM_SYSKEYDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sys_key_down(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_sys_key_down = Some(Box::new(f));
    }


    // // void OnSysKeyUp(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_SYSKEYUP(func) \
    //     if (uMsg == WM_SYSKEYUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sys_key_up(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_sys_key_up = Some(Box::new(f));
    }


    // // void OnSysChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_SYSCHAR(func) \
    //     if (uMsg == WM_SYSCHAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sys_char(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_sys_char = Some(Box::new(f));
    }


    // // void OnSysDeadChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_SYSDEADCHAR(func) \
    //     if (uMsg == WM_SYSDEADCHAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sys_dead_char(&mut self,f:F) where F:Fn(&T,UINT,UINT,UINT) + 'static {
        self.msg_entry.on_sys_dead_char = Some(Box::new(f));
    }


    // // void OnSysCommand(UINT nID, CPoint point)
    // #define MSG_WM_SYSCOMMAND(func) \
    //     if (uMsg == WM_SYSCOMMAND) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sys_command(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_sys_command = Some(Box::new(f));
    }


    // // void OnTCard(UINT idAction, DWORD dwActionData)
    // #define MSG_WM_TCARD(func) \
    //     if (uMsg == WM_TCARD) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (DWORD)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_t_card(&mut self,f:F) where F:Fn(&T,UINT,DWORD) + 'static {
        self.msg_entry.on_t_card = Some(Box::new(f));
    }


    // // void OnTimer(UINT_PTR nIDEvent)
    // #define MSG_WM_TIMER(func) \
    //     if (uMsg == WM_TIMER) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT_PTR)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_timer(&mut self,f:F) where F:Fn(&T,UINT_PTR) + 'static {
        self.msg_entry.on_timer = Some(Box::new(f));
    }


    // // void OnHScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    // #define MSG_WM_HSCROLL(func) \
    //     if (uMsg == WM_HSCROLL) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_h_scroll(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CScrollBar) + 'static {
        self.msg_entry.on_h_scroll = Some(Box::new(f));
    }

    // // void OnVScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    // #define MSG_WM_VSCROLL(func) \
    //     if (uMsg == WM_VSCROLL) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_v_scroll(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CScrollBar) + 'static {
        self.msg_entry.on_v_scroll = Some(Box::new(f));
    }


    // // void OnInitMenu(CMenuHandle menu)
    // #define MSG_WM_INITMENU(func) \
    //     if (uMsg == WM_INITMENU) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HMENU)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_init_menu(&mut self,f:F) where F:Fn(&T,&CMenuHandle) + 'static {
        self.msg_entry.on_init_menu = Some(Box::new(f));
    }


    // // void OnInitMenuPopup(CMenuHandle menuPopup, UINT nIndex, BOOL bSysMenu)
    // #define MSG_WM_INITMENUPOPUP(func) \
    //     if (uMsg == WM_INITMENUPOPUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HMENU)wParam, (UINT)LOWORD(lParam), (BOOL)HIWORD(lParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_init_menu_popup(&mut self,f:F) where F:Fn(&T,&CMenuHandle,UINT,BOOL) + 'static {
        self.msg_entry.on_init_menu_popup = Some(Box::new(f));
    }



    // // void OnMenuSelect(UINT nItemID, UINT nFlags, CMenuHandle menu)
    // #define MSG_WM_MENUSELECT(func) \
    //     if (uMsg == WM_MENUSELECT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HMENU)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_menu_select(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CMenuHandle) + 'static {
        self.msg_entry.on_menu_select = Some(Box::new(f));
    }


    // // LRESULT OnMenuChar(UINT nChar, UINT nFlags, CMenuHandle menu)
    // #define MSG_WM_MENUCHAR(func) \
    //     if (uMsg == WM_MENUCHAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((TCHAR)LOWORD(wParam), (UINT)HIWORD(wParam), (HMENU)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_menu_char(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CMenuHandle) -> LRESULT + 'static {
        self.msg_entry.on_menu_char = Some(Box::new(f));
    }


    // // LRESULT OnNotify(int idCtrl, LPNMHDR pnmh)
    // #define MSG_WM_NOTIFY(func) \
    //     if (uMsg == WM_NOTIFY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((int)wParam, (LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_notify(&mut self,f:F) where F:Fn(&T,c_int,&NMHDR) -> LRESULT + 'static {
        self.msg_entry.on_notify = Some(Box::new(f));
    }


    // // void OnEnterIdle(UINT nWhy, CWindow wndWho)
    // #define MSG_WM_ENTERIDLE(func) \
    //     if (uMsg == WM_ENTERIDLE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_enter_idle(&mut self,f:F) where F:Fn(&T,UINT,&CWindow) + 'static {
        self.msg_entry.on_enter_idle = Some(Box::new(f));
    }


    // // void OnMouseMove(UINT nFlags, CPoint point)
    // #define MSG_WM_MOUSEMOVE(func) \
    //     if (uMsg == WM_MOUSEMOVE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_mouse_move(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_mouse_move = Some(Box::new(f));
    }


    // // BOOL OnMouseWheel(UINT nFlags, short zDelta, CPoint pt)
    // #define MSG_WM_MOUSEWHEEL(func) \
    //     if (uMsg == WM_MOUSEWHEEL) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)LOWORD(wParam), (short)HIWORD(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_mouse_wheel(&mut self,f:F) where F:Fn(&T,UINT,short,&CPoint) + 'static {
        self.msg_entry.on_mouse_wheel = Some(Box::new(f));
    }


    // // void OnLButtonDown(UINT nFlags, CPoint point)
    // #define MSG_WM_LBUTTONDOWN(func) \
    //     if (uMsg == WM_LBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_l_button_down(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_l_button_down = Some(Box::new(f));
    }


    // // void OnLButtonUp(UINT nFlags, CPoint point)
    // #define MSG_WM_LBUTTONUP(func) \
    //     if (uMsg == WM_LBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_l_button_up(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_l_button_up = Some(Box::new(f));
    }


    // // void OnLButtonDblClk(UINT nFlags, CPoint point)
    // #define MSG_WM_LBUTTONDBLCLK(func) \
    //     if (uMsg == WM_LBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_l_button_dbl_clk(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_l_button_dbl_clk = Some(Box::new(f));
    }


    // // void OnRButtonDown(UINT nFlags, CPoint point)
    // #define MSG_WM_RBUTTONDOWN(func) \
    //     if (uMsg == WM_RBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_r_button_down(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_r_button_down = Some(Box::new(f));
    }


    // // void OnRButtonUp(UINT nFlags, CPoint point)
    // #define MSG_WM_RBUTTONUP(func) \
    //     if (uMsg == WM_RBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_r_button_up(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_r_button_up = Some(Box::new(f));
    }


    // // void OnRButtonDblClk(UINT nFlags, CPoint point)
    // #define MSG_WM_RBUTTONDBLCLK(func) \
    //     if (uMsg == WM_RBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_r_button_dbl_clk(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_r_button_dbl_clk = Some(Box::new(f));
    }


    // // void OnMButtonDown(UINT nFlags, CPoint point)
    // #define MSG_WM_MBUTTONDOWN(func) \
    //     if (uMsg == WM_MBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_m_button_down(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_m_button_down = Some(Box::new(f));
    }


    // // void OnMButtonUp(UINT nFlags, CPoint point)
    // #define MSG_WM_MBUTTONUP(func) \
    //     if (uMsg == WM_MBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_m_button_up(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_m_button_up = Some(Box::new(f));
    }


    // // void OnMButtonDblClk(UINT nFlags, CPoint point)
    // #define MSG_WM_MBUTTONDBLCLK(func) \
    //     if (uMsg == WM_MBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_m_button_dbl_clk(&mut self,f:F) where F:Fn(&T,UINT,&CPoint) + 'static {
        self.msg_entry.on_m_button_dbl_clk = Some(Box::new(f));
    }


    // // void OnParentNotify(UINT message, UINT nChildID, LPARAM lParam)
    // #define MSG_WM_PARENTNOTIFY(func) \
    //     if (uMsg == WM_PARENTNOTIFY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_parent_notify(&mut self,f:F) where F:Fn(&T,UINT,UINT,LPARAM) + 'static {
        self.msg_entry.on_parent_notify = Some(Box::new(f));
    }


    // // void OnMDIActivate(CWindow wndActivate, CWindow wndDeactivate)
    // #define MSG_WM_MDIACTIVATE(func) \
    //     if (uMsg == WM_MDIACTIVATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam, (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_mdi_activate(&mut self,f:F) where F:Fn(&T,&CWindow,&CWindow) + 'static {
        self.msg_entry.on_mdi_activate = Some(Box::new(f));
    }


    // // void OnRenderFormat(UINT nFormat)
    // #define MSG_WM_RENDERFORMAT(func) \
    //     if (uMsg == WM_RENDERFORMAT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_render_format(&mut self,f:F) where F:Fn(&T,UINT) + 'static {
        self.msg_entry.on_render_format = Some(Box::new(f));
    }


    // // void OnRenderAllFormats()
    // #define MSG_WM_RENDERALLFORMATS(func) \
    //     if (uMsg == WM_RENDERALLFORMATS) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_render_all_formats(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_render_all_formats = Some(Box::new(f));
    }


    // // void OnDestroyClipboard()
    // #define MSG_WM_DESTROYCLIPBOARD(func) \
    //     if (uMsg == WM_DESTROYCLIPBOARD) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_destroy_clipboard(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_destroy_clipboard = Some(Box::new(f));
    }


    // // void OnDrawClipboard()
    // #define MSG_WM_DRAWCLIPBOARD(func) \
    //     if (uMsg == WM_DRAWCLIPBOARD) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_draw_clipboard(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_draw_clipboard = Some(Box::new(f));
    }

    // // void OnPaintClipboard(CWindow wndViewer, const LPPAINTSTRUCT lpPaintStruct)
    // #define MSG_WM_PAINTCLIPBOARD(func) \
    //     if (uMsg == WM_PAINTCLIPBOARD) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam, (const LPPAINTSTRUCT)::GlobalLock((HGLOBAL)lParam)); \
    //         ::GlobalUnlock((HGLOBAL)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_paint_clipboard(&mut self,f:F) where F:Fn(&T,&CWindow,&PAINTSTRUCT) + 'static {
        self.msg_entry.on_paint_clipboard = Some(Box::new(f));
    }


    // // void OnVScrollClipboard(CWindow wndViewer, UINT nSBCode, UINT nPos)
    // #define MSG_WM_VSCROLLCLIPBOARD(func) \
    //     if (uMsg == WM_VSCROLLCLIPBOARD) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_v_scroll_clipboard(&mut self,f:F) where F:Fn(&T,&CWindow,UINT,UINT) + 'static {
        self.msg_entry.on_v_scroll_clipboard = Some(Box::new(f));
    }


    // // void OnContextMenu(CWindow wnd, CPoint point)
    // #define MSG_WM_CONTEXTMENU(func) \
    //     if (uMsg == WM_CONTEXTMENU) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_context_menu(&mut self,f:F) where F:Fn(&T,&CWindow,&CPoint) + 'static {
        self.msg_entry.on_context_menu = Some(Box::new(f));
    }


    // // void OnSizeClipboard(CWindow wndViewer, const LPRECT lpRect)
    // #define MSG_WM_SIZECLIPBOARD(func) \
    //     if (uMsg == WM_SIZECLIPBOARD) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam, (const LPRECT)::GlobalLock((HGLOBAL)lParam)); \
    //         ::GlobalUnlock((HGLOBAL)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_size_clipboard(&mut self,f:F) where F:Fn(&T,&CWindow,&RECT) + 'static {
        self.msg_entry.on_size_clipboard = Some(Box::new(f));
    }


    // // void OnAskCbFormatName(UINT nMaxCount, LPTSTR lpszString)
    // #define MSG_WM_ASKCBFORMATNAME(func) \
    //     if (uMsg == WM_ASKCBFORMATNAME) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPTSTR)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // void OnChangeCbChain(CWindow wndRemove, CWindow wndAfter)
    // #define MSG_WM_CHANGECBCHAIN(func) \
    //     if (uMsg == WM_CHANGECBCHAIN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam, (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_change_cb_chain(&mut self,f:F) where F:Fn(&T,&CWindow,&CWindow) + 'static {
        self.msg_entry.on_change_cb_chain = Some(Box::new(f));
    }


    // // void OnHScrollClipboard(CWindow wndViewer, UINT nSBCode, UINT nPos)
    // #define MSG_WM_HSCROLLCLIPBOARD(func) \
    //     if (uMsg == WM_HSCROLLCLIPBOARD) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_h_scroll_clipboard(&mut self,f:F) where F:Fn(&T,&CWindow,UINT,UINT) + 'static {
        self.msg_entry.on_h_scroll_clipboard = Some(Box::new(f));
    }


    // // BOOL OnQueryNewPalette()
    // #define MSG_WM_QUERYNEWPALETTE(func) \
    //     if (uMsg == WM_QUERYNEWPALETTE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_query_new_palette(&mut self,f:F) where F:Fn(&T)->BOOL + 'static {
        self.msg_entry.on_query_new_palette = Some(Box::new(f));
    }


    // // void OnPaletteChanged(CWindow wndFocus)
    // #define MSG_WM_PALETTECHANGED(func) \
    //     if (uMsg == WM_PALETTECHANGED) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_palette_changed(&mut self,f:F) where F:Fn(&T,&CWindow) + 'static {
        self.msg_entry.on_palette_changed = Some(Box::new(f));
    }


    // // void OnPaletteIsChanging(CWindow wndPalChg)
    // #define MSG_WM_PALETTEISCHANGING(func) \
    //     if (uMsg == WM_PALETTEISCHANGING) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_palette_is_changing(&mut self,f:F) where F:Fn(&T,&CWindow) + 'static {
        self.msg_entry.on_palette_is_changing = Some(Box::new(f));
    }


    // // void OnDropFiles(HDROP hDropInfo)
    // #define MSG_WM_DROPFILES(func) \
    //     if (uMsg == WM_DROPFILES) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HDROP)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_drop_files(&mut self,f:F) where F:Fn(&T,HDROP) + 'static {
        self.msg_entry.on_drop_files = Some(Box::new(f));
    }


    // // void OnWindowPosChanging(LPWINDOWPOS lpWndPos)
    // #define MSG_WM_WINDOWPOSCHANGING(func) \
    //     if (uMsg == WM_WINDOWPOSCHANGING) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((LPWINDOWPOS)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_window_pos_changing(&mut self,f:F) where F:Fn(&T,&WINDOWPOS) + 'static {
        self.msg_entry.on_window_pos_changing = Some(Box::new(f));
    }


    // // void OnWindowPosChanged(LPWINDOWPOS lpWndPos)
    // #define MSG_WM_WINDOWPOSCHANGED(func) \
    //     if (uMsg == WM_WINDOWPOSCHANGED) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((LPWINDOWPOS)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_window_pos_changed(&mut self,f:F) where F:Fn(&T,&WINDOWPOS) + 'static {
        self.msg_entry.on_window_pos_changed = Some(Box::new(f));
    }


    // // void OnExitMenuLoop(BOOL fIsTrackPopupMenu)
    // #define MSG_WM_EXITMENULOOP(func) \
    //     if (uMsg == WM_EXITMENULOOP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_exit_menu_loop(&mut self,f:F) where F:Fn(&T,BOOL) + 'static {
        self.msg_entry.on_exit_menu_loop = Some(Box::new(f));
    }


    // // void OnEnterMenuLoop(BOOL fIsTrackPopupMenu)
    // #define MSG_WM_ENTERMENULOOP(func) \
    //     if (uMsg == WM_ENTERMENULOOP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_enter_menu_loop(&mut self,f:F) where F:Fn(&T,BOOL) + 'static {
        self.msg_entry.on_enter_menu_loop = Some(Box::new(f));
    }


    // // void OnStyleChanged(int nStyleType, LPSTYLESTRUCT lpStyleStruct)
    // #define MSG_WM_STYLECHANGED(func) \
    //     if (uMsg == WM_STYLECHANGED) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPSTYLESTRUCT)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_style_changed(&mut self,f:F) where F:Fn(&T,c_int,&STYLESTRUCT) + 'static {
        self.msg_entry.on_style_changed = Some(Box::new(f));
    }


    // // void OnStyleChanging(int nStyleType, LPSTYLESTRUCT lpStyleStruct)
    // #define MSG_WM_STYLECHANGING(func) \
    //     if (uMsg == WM_STYLECHANGING) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPSTYLESTRUCT)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sytle_changing(&mut self,f:F) where F:Fn(&T,c_int,&STYLESTRUCT) + 'static {
        self.msg_entry.on_sytle_changing = Some(Box::new(f));
    }


    // // void OnSizing(UINT fwSide, LPRECT pRect)
    // #define MSG_WM_SIZING(func) \
    //     if (uMsg == WM_SIZING) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPRECT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_sizing(&mut self,f:F) where F:Fn(&T,UINT,&RECT) + 'static {
        self.msg_entry.on_sizing = Some(Box::new(f));
    }


    // // void OnMoving(UINT fwSide, LPRECT pRect)
    // #define MSG_WM_MOVING(func) \
    //     if (uMsg == WM_MOVING) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPRECT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_moving(&mut self,f:F) where F:Fn(&T,UINT,&RECT) + 'static {
        self.msg_entry.on_moving = Some(Box::new(f));
    }


    // // void OnCaptureChanged(CWindow wnd)
    // #define MSG_WM_CAPTURECHANGED(func) \
    //     if (uMsg == WM_CAPTURECHANGED) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_capture_changed(&mut self,f:F) where F:Fn(&T,&CWindow) + 'static {
        self.msg_entry.on_capture_changed = Some(Box::new(f));
    }


    // // BOOL OnDeviceChange(UINT nEventType, DWORD_PTR dwData)
    // #define MSG_WM_DEVICECHANGE(func) \
    //     if (uMsg == WM_DEVICECHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)wParam, (DWORD_PTR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_device_change(&mut self,f:F) where F:Fn(&T,UINT,DWORD_PTR) + 'static {
        self.msg_entry.on_device_change = Some(Box::new(f));
    }


    // // void OnCommand(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define MSG_WM_COMMAND(func) \
    //     if (uMsg == WM_COMMAND) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_command(&mut self,f:F) where F:Fn(&T,UINT,c_int,&CWindow) + 'static {
        self.msg_entry.on_command = Some(Box::new(f));
    }


    // // void OnDisplayChange(UINT uBitsPerPixel, CSize sizeScreen)
    // #define MSG_WM_DISPLAYCHANGE(func) \
    //     if (uMsg == WM_DISPLAYCHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, _WTYPES_NS::CSize(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_display_change(&mut self,f:F) where F:Fn(&T,UINT,&CSize) + 'static {
        self.msg_entry.on_display_change = Some(Box::new(f));
    }


    // // void OnEnterSizeMove()
    // #define MSG_WM_ENTERSIZEMOVE(func) \
    //     if (uMsg == WM_ENTERSIZEMOVE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_enter_size_move(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_enter_size_move = Some(Box::new(f));
    }


    // // void OnExitSizeMove()
    // #define MSG_WM_EXITSIZEMOVE(func) \
    //     if (uMsg == WM_EXITSIZEMOVE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_exit_size_move(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_exit_size_move = Some(Box::new(f));
    }


    // // HFONT OnGetFont()
    // #define MSG_WM_GETFONT(func) \
    //     if (uMsg == WM_GETFONT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_get_font(&mut self,f:F) where F:Fn(&T)->HFONT + 'static {
        self.msg_entry.on_get_font = Some(Box::new(f));
    }


    // // LRESULT OnGetHotKey()
    // #define MSG_WM_GETHOTKEY(func) \
    //     if (uMsg == WM_GETHOTKEY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_get_hot_key(&mut self,f:F) where F:Fn(&T,)->LRESULT + 'static {
        self.msg_entry.on_get_hot_key = Some(Box::new(f));
    }


    // // HICON OnGetIcon()
    // #define MSG_WM_GETICON(func) \
    //     if (uMsg == WM_GETICON) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)wParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_get_icon(&mut self,f:F) where F:Fn(&T,)->HICON + 'static {
        self.msg_entry.on_get_icon = Some(Box::new(f));
    }


    // // int OnGetText(int cchTextMax, LPTSTR lpszText)
    // #define MSG_WM_GETTEXT(func) \
    //     if (uMsg == WM_GETTEXT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((int)wParam, (LPTSTR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }



    // // int OnGetTextLength()
    // #define MSG_WM_GETTEXTLENGTH(func) \
    //     if (uMsg == WM_GETTEXTLENGTH) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_get_text_length(&mut self,f:F) where F:Fn(&T)->c_int + 'static {
        self.msg_entry.on_get_text_length = Some(Box::new(f));
    }

    // // void OnHelp(LPHELPINFO lpHelpInfo)
    // #define MSG_WM_HELP(func) \
    //     if (uMsg == WM_HELP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((LPHELPINFO)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_help(&mut self,f:F) where F:Fn(&T,&HELPINFO) + 'static {
        self.msg_entry.on_help = Some(Box::new(f));
    }


    // // void OnHotKey(int nHotKeyID, UINT uModifiers, UINT uVirtKey)
    // #define MSG_WM_HOTKEY(func) \
    //     if (uMsg == WM_HOTKEY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((int)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_hot_key(&mut self,f:F) where F:Fn(&T,c_int,UINT,UINT) + 'static {
        self.msg_entry.on_hot_key = Some(Box::new(f));
    }


    // // void OnInputLangChange(DWORD dwCharSet, HKL hKbdLayout)
    // #define MSG_WM_INPUTLANGCHANGE(func) \
    //     if (uMsg == WM_INPUTLANGCHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((DWORD)wParam, (HKL)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_input_lang_change(&mut self,f:F) where F:Fn(&T,DWORD,HKL) + 'static {
        self.msg_entry.on_input_lang_change = Some(Box::new(f));
    }


    // // void OnInputLangChangeRequest(BOOL bSysCharSet, HKL hKbdLayout)
    // #define MSG_WM_INPUTLANGCHANGEREQUEST(func) \
    //     if (uMsg == WM_INPUTLANGCHANGEREQUEST) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam, (HKL)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_input_lang_change_request(&mut self,f:F) where F:Fn(&T,BOOL,HKL) + 'static {
        self.msg_entry.on_input_lang_change_request = Some(Box::new(f));
    }


    // // void OnNextDlgCtl(BOOL bHandle, WPARAM wCtlFocus)
    // #define MSG_WM_NEXTDLGCTL(func) \
    //     if (uMsg == WM_NEXTDLGCTL) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)LOWORD(lParam), wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_next_dlg_ctl(&mut self,f:F) where F:Fn(&T,BOOL,WPARAM) + 'static {
        self.msg_entry.on_next_dlg_ctl = Some(Box::new(f));
    }


    // // void OnNextMenu(int nVirtKey, LPMDINEXTMENU lpMdiNextMenu)
    // #define MSG_WM_NEXTMENU(func) \
    //     if (uMsg == WM_NEXTMENU) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((int)wParam, (LPMDINEXTMENU)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_next_menu(&mut self,f:F) where F:Fn(&T,c_int,&MDINEXTMENU) + 'static {
        self.msg_entry.on_next_menu = Some(Box::new(f));
    }


    // // int OnNotifyFormat(CWindow wndFrom, int nCommand)
    // #define MSG_WM_NOTIFYFORMAT(func) \
    //     if (uMsg == WM_NOTIFYFORMAT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HWND)wParam, (int)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_notify_format(&mut self,f:F) where F:Fn(&T,&CWindow) + 'static {
        self.msg_entry.on_notify_format = Some(Box::new(f));
    }


    // // BOOL OnPowerBroadcast(DWORD dwPowerEvent, DWORD_PTR dwData)
    // #define MSG_WM_POWERBROADCAST(func) \
    //     if (uMsg == WM_POWERBROADCAST) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((DWORD)wParam, (DWORD_PTR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_power_broadcast(&mut self,f:F) where F:Fn(&T,DWORD,DWORD_PTR) + 'static {
        self.msg_entry.on_power_broadcast = Some(Box::new(f));
    }

    // // void OnPrint(CDCHandle dc, UINT uFlags)
    // #define MSG_WM_PRINT(func) \
    //     if (uMsg == WM_PRINT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HDC)wParam, (UINT)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_print(&mut self,f:F) where F:Fn(&T,&CDCHandle,UINT) + 'static {
        self.msg_entry.on_print = Some(Box::new(f));
    }
    // // void OnPrintClient(CDCHandle dc, UINT uFlags)
    // #define MSG_WM_PRINTCLIENT(func) \
    //     if (uMsg == WM_PRINTCLIENT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HDC)wParam, (UINT)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_print_client(&mut self,f:F) where F:Fn(&T,&CDCHandle,UINT) + 'static {
        self.msg_entry.on_print_client = Some(Box::new(f));
    }

    // // void OnRasDialEvent(RASCONNSTATE rasconnstate, DWORD dwError)
    // #define MSG_WM_RASDIALEVENT(func) \
    //     if (uMsg == WM_RASDIALEVENT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((RASCONNSTATE)wParam, (DWORD)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ras_dial_event(&mut self,f:F) where F:Fn(&T,&RASCONNSTATE,DWORD) + 'static {
        self.msg_entry.on_ras_dial_event = Some(Box::new(f));
    }


    // // void OnSetFont(CFontHandle font, BOOL bRedraw)
    // #define MSG_WM_SETFONT(func) \
    //     if (uMsg == WM_SETFONT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((HFONT)wParam, (BOOL)LOWORD(lParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_set_font(&mut self,f:F) where F:Fn(&T,&CFontHandle,BOOL) + 'static {
        self.msg_entry.on_set_font = Some(Box::new(f));
    }


    // // int OnSetHotKey(int nVirtKey, UINT uFlags)
    // #define MSG_WM_SETHOTKEY(func) \
    //     if (uMsg == WM_SETHOTKEY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((int)LOBYTE(LOWORD(wParam)), (UINT)HIBYTE(LOWORD(wParam))); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_set_hot_key(&mut self,f:F) where F:Fn(&T,c_int,UINT) + 'static {
        self.msg_entry.on_set_hot_key = Some(Box::new(f));
    }

    // // HICON OnSetIcon(UINT uType, HICON hIcon)
    // #define MSG_WM_SETICON(func) \
    //     if (uMsg == WM_SETICON) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)wParam, (HICON)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_set_icon(&mut self,f:F) where F:Fn(&T,UINT,HICON) + 'static {
        self.msg_entry.on_set_icon = Some(Box::new(f));
    }

    // // void OnSetRedraw(BOOL bRedraw)
    // #define MSG_WM_SETREDRAW(func) \
    //     if (uMsg == WM_SETREDRAW) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((BOOL)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_set_redraw(&mut self,f:F) where F:Fn(&T,BOOL) + 'static {
        self.msg_entry.on_set_redraw = Some(Box::new(f));
    }


    // // int OnSetText(LPCTSTR lpstrText)
    // #define MSG_WM_SETTEXT(func) \
    //     if (uMsg == WM_SETTEXT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((LPCTSTR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    // pub fn on_set_text(&mut self,f:F) where F:Fn(&T,) + 'static {

    // }

    // // void OnUserChanged()
    // #define MSG_WM_USERCHANGED(func) \
    //     if (uMsg == WM_USERCHANGED) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_user_changed(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_user_changed = Some(Box::new(f));
    }


    // ///////////////////////////////////////////////////////////////////////////////
    // // New NT4 & NT5 messages

    // #if (_WIN32_WINNT >= 0x0400)

    // // void OnMouseHover(WPARAM wParam, CPoint ptPos)
    // #define MSG_WM_MOUSEHOVER(func) \
    //     if (uMsg == WM_MOUSEHOVER) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_mouser_hove(&mut self,f:F) where F:Fn(&T,WPARAM,&CPoint) + 'static {
        self.msg_entry.on_mouser_hove = Some(Box::new(f));
    }

    // // void OnMouseLeave()
    // #define MSG_WM_MOUSELEAVE(func) \
    //     if (uMsg == WM_MOUSELEAVE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_mouse_leave(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_mouse_leave = Some(Box::new(f));
    }


    // #endif // _WIN32_WINNT >= 0x0400

    // #if (WINVER >= 0x0500)

    // // void OnMenuRButtonUp(WPARAM wParam, CMenuHandle menu)
    // #define MSG_WM_MENURBUTTONUP(func) \
    //     if (uMsg == WM_MENURBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(wParam, (HMENU)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_menu_r_button_up(&mut self,f:F) where F:Fn(&T,WPARAM,&CMenuHandle) + 'static {
        self.msg_entry.on_menu_r_button_up = Some(Box::new(f));
    }


    // // LRESULT OnMenuDrag(WPARAM wParam, CMenuHandle menu)
    // #define MSG_WM_MENUDRAG(func) \
    //     if (uMsg == WM_MENUDRAG) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func(wParam, (HMENU)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_menu_drag(&mut self,f:F) where F:Fn(&T,WPARAM,&CMenuHandle) + 'static {
        self.msg_entry.on_menu_drag = Some(Box::new(f));
    }

    // // LRESULT OnMenuGetObject(PMENUGETOBJECTINFO info)
    // #define MSG_WM_MENUGETOBJECT(func) \
    //     if (uMsg == WM_MENUGETOBJECT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((PMENUGETOBJECTINFO)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_menu_get_object(&mut self,f:F) where F:Fn(&T,&MENUGETOBJECTINFO)->LRESULT + 'static {
        self.msg_entry.on_menu_get_object = Some(Box::new(f));
    }


    // // void OnUnInitMenuPopup(UINT nID, CMenuHandle menu)
    // #define MSG_WM_UNINITMENUPOPUP(func) \
    //     if (uMsg == WM_UNINITMENUPOPUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(lParam), (HMENU)wParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_un_init_menu_popup(&mut self,f:F) where F:Fn(&T,UINT,&CMenuHandle) + 'static {
        self.msg_entry.on_un_init_menu_popup = Some(Box::new(f));
    }


    // // void OnMenuCommand(WPARAM nIndex, CMenuHandle menu)
    // #define MSG_WM_MENUCOMMAND(func) \
    //     if (uMsg == WM_MENUCOMMAND) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(wParam, (HMENU)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_menu_command(&mut self,f:F) where F:Fn(&T,WAPARM,&CMenuHandle) + 'static {
        self.msg_entry.on_menu_command = Some(Box::new(f));
    }


    // #endif // WINVER >= 0x0500

    // #if (_WIN32_WINNT >= 0x0500)

    // // BOOL OnAppCommand(CWindow wndFocus, short cmd, WORD uDevice, int dwKeys)
    // #define MSG_WM_APPCOMMAND(func) \
    //     if (uMsg == WM_APPCOMMAND) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HWND)wParam, GET_APPCOMMAND_LPARAM(lParam), GET_DEVICE_LPARAM(lParam), GET_KEYSTATE_LPARAM(lParam)); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_app_command(&mut self,f:F) where F:Fn(&T,&CWindow,c_short,WORD,c_int) + 'static {
        self.msg_entry.on_app_command = Some(Box::new(f));
    }


    // // void OnNCXButtonDown(int fwButton, short nHittest, CPoint ptPos)
    // #define MSG_WM_NCXBUTTONDOWN(func) \
    //     if (uMsg == WM_NCXBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(GET_XBUTTON_WPARAM(wParam), GET_NCHITTEST_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ncx_button_down(&mut self,f:F) where F:Fn(&T,c_int,c_short,&CPoint) + 'static {
        self.msg_entry.on_ncx_button_down = Some(Box::new(f));
    }


    // // void OnNCXButtonUp(int fwButton, short nHittest, CPoint ptPos)
    // #define MSG_WM_NCXBUTTONUP(func) \
    //     if (uMsg == WM_NCXBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(GET_XBUTTON_WPARAM(wParam), GET_NCHITTEST_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ncx_button_up(&mut self,f:F) where F:Fn(&T,c_int,c_short,&CPoint) + 'static {
        self.msg_entry.on_ncx_button_up = Some(Box::new(f));
    }


    // // void OnNCXButtonDblClk(int fwButton, short nHittest, CPoint ptPos)
    // #define MSG_WM_NCXBUTTONDBLCLK(func) \
    //     if (uMsg == WM_NCXBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(GET_XBUTTON_WPARAM(wParam), GET_NCHITTEST_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_ncx_button_dbl_clk(&mut self,f:F) where F:Fn(&T,c_int,c_short,&CPoint) + 'static {
        self.msg_entry.on_ncx_button_dbl_clk = Some(Box::new(f));
    }


    // // void OnXButtonDown(int fwButton, int dwKeys, CPoint ptPos)
    // #define MSG_WM_XBUTTONDOWN(func) \
    //     if (uMsg == WM_XBUTTONDOWN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(GET_XBUTTON_WPARAM(wParam), GET_KEYSTATE_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_x_button_down(&mut self,f:F) where F:Fn(&T,c_int,c_int,&CPoint) + 'static {
        self.msg_entry.on_x_button_down = Some(Box::new(f));
    }


    // // void OnXButtonUp(int fwButton, int dwKeys, CPoint ptPos)
    // #define MSG_WM_XBUTTONUP(func) \
    //     if (uMsg == WM_XBUTTONUP) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(GET_XBUTTON_WPARAM(wParam), GET_KEYSTATE_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_x_button_up(&mut self,f:F) where F:Fn(&T,c_int,c_int,&CPoint) + 'static {
        self.msg_entry.on_x_button_up = Some(Box::new(f));
    }

    // // void OnXButtonDblClk(int fwButton, int dwKeys, CPoint ptPos)
    // #define MSG_WM_XBUTTONDBLCLK(func) \
    //     if (uMsg == WM_XBUTTONDBLCLK) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(GET_XBUTTON_WPARAM(wParam), GET_KEYSTATE_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_x_button_dbl_clk(&mut self,f:F) where F:Fn(&T,c_int,c_int,&CPoint) + 'static {
        self.msg_entry.on_x_button_dbl_clk = Some(Box::new(f));
    }

    // // void OnChangeUIState(WORD nAction, WORD nState)
    // #define MSG_WM_CHANGEUISTATE(func) \
    //     if (uMsg == WM_CHANGEUISTATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(LOWORD(wParam), HIWORD(wParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_change_ui_state(&mut self,f:F) where F:Fn(&T,WORD,WORD) + 'static {
        self.msg_entry.on_change_ui_state = Some(Box::new(f));
    }

    // // void OnUpdateUIState(WORD nAction, WORD nState)
    // #define MSG_WM_UPDATEUISTATE(func) \
    //     if (uMsg == WM_UPDATEUISTATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(LOWORD(wParam), HIWORD(wParam)); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_update_ui_state(&mut self,f:F) where F:Fn(&T,WORD,WORD) + 'static {
        self.msg_entry.on_update_ui_state = Some(Box::new(f));
    }


    // // LRESULT OnQueryUIState()
    // #define MSG_WM_QUERYUISTATE(func) \
    //     if (uMsg == WM_QUERYUISTATE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_query_ui_state(&mut self,f:F) where F:Fn(&T)->LRESULT + 'static {
        self.msg_entry.on_query_ui_state = Some(Box::new(f));
    }


    // #endif // (_WIN32_WINNT >= 0x0500)

    // #if(_WIN32_WINNT >= 0x0501)

    // // void OnInput(WPARAM RawInputCode, HRAWINPUT hRawInput)
    // #define MSG_WM_INPUT(func) \
    //     if (uMsg == WM_INPUT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(GET_RAWINPUT_CODE_WPARAM(wParam), (HRAWINPUT)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_input(&mut self,f:F) where F:Fn(&T,WPARAM,HRAWINPUT) + 'static {
        self.msg_entry.on_input = Some(Box::new(f));
    }


    // // void OnUniChar(TCHAR nChar, UINT nRepCnt, UINT nFlags)
    // #define MSG_WM_UNICHAR(func) \
    //     if (uMsg == WM_UNICHAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16)); \
    //         if(IsMsgHandled()) \
    //         { \
    //             lResult = (wParam == UNICODE_NOCHAR) ? TRUE : FALSE; \
    //             return TRUE; \
    //         } \
    //     }
    pub fn on_uni_char(&mut self,f:F) where F:Fn(&T,wchar_t,UINT,UINT) + 'static {
        self.msg_entry.on_uni_char = Some(Box::new(f));
    }


    // // void OnWTSSessionChange(WPARAM nStatusCode, PWTSSESSION_NOTIFICATION nSessionID)
    // #define MSG_WM_WTSSESSION_CHANGE(func) \
    //     if (uMsg == WM_WTSSESSION_CHANGE) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(wParam, (PWTSSESSION_NOTIFICATION)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_wt_session_change(&mut self,f:F) where F:Fn(&T,WPARAM,&WTSSESSION_NOTIFICATION) + 'static {
        self.msg_entry.on_wt_session_change = Some(Box::new(f));
    }


    // // void OnThemeChanged()
    // #define MSG_WM_THEMECHANGED(func) \
    //     if (uMsg == WM_THEMECHANGED) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_theme_changed(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_theme_changed = Some(Box::new(f));
    }


    // #endif // _WIN32_WINNT >= 0x0501

    // #if (_WIN32_WINNT >= 0x0600)

    // // BOOL OnMouseHWheel(UINT nFlags, short zDelta, CPoint pt)
    // #define MSG_WM_MOUSEHWHEEL(func) \
    //     if (uMsg == WM_MOUSEHWHEEL) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)LOWORD(wParam), (short)HIWORD(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam))); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_mouse_wheel(&mut self,f:F) where F:Fn(&T,UINT,c_short,&CPoint)->BOOL + 'static {
        self.msg_entry.on_mouse_wheel = Some(Box::new(f));
    }


    // #endif // (_WIN32_WINNT >= 0x0600)

    // ///////////////////////////////////////////////////////////////////////////////
    // // ATL defined messages

    // // BOOL OnForwardMsg(LPMSG Msg, DWORD nUserData)
    // #define MSG_WM_FORWARDMSG(func) \
    //     if (uMsg == WM_FORWARDMSG) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((LPMSG)lParam, (DWORD)wParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_forward_msg(&mut self,f:F) where F:Fn(&T,&MSG,DWORD)->BOOL + 'static {
        self.msg_entry.on_forward_msg = Some(Box::new(f));
    }


    // ///////////////////////////////////////////////////////////////////////////////
    // // Dialog specific messages

    // // LRESULT OnDMGetDefID()
    // #define MSG_DM_GETDEFID(func) \
    //     if (uMsg == DM_GETDEFID) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func(); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_dm_get_def_id(&mut self,f:F) where F:Fn(&T)->LRESULT + 'static {
        self.msg_entry.on_dm_get_def_id = Some(Box::new(f));
    }


    // // void OnDMSetDefID(UINT DefID)
    // #define MSG_DM_SETDEFID(func) \
    //     if (uMsg == DM_SETDEFID) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_dm_set_def_id(&mut self,f:F) where F:Fn(&T,UINT) + 'static {
        self.msg_entry.on_dm_set_def_id = Some(Box::new(f));
    }


    // // void OnDMReposition()
    // #define MSG_DM_REPOSITION(func) \
    //     if (uMsg == DM_REPOSITION) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func(); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_dm_reposition(&mut self,f:F) where F:Fn(&T) + 'static {
        self.msg_entry.on_dm_reposition = Some(Box::new(f));
    }


    // ///////////////////////////////////////////////////////////////////////////////
    // // Reflected messages

    // // void OnReflectedCommand(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define MSG_OCM_COMMAND(func) \
    //     if (uMsg == OCM_COMMAND) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_command(&mut self,f:F) where F:Fn(&T,UINT,c_int,&CWindow) + 'static {
        self.msg_entry.on_reflected_command = Some(Box::new(f));
    }


    // // LRESULT OnReflectedNotify(int idCtrl, LPNMHDR pnmh)
    // #define MSG_OCM_NOTIFY(func) \
    //     if (uMsg == OCM_NOTIFY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((int)wParam, (LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_notify(&mut self,f:F) where F:Fn(&T,c_int,&NMHDR)->LRESULT + 'static {
        self.msg_entry.on_reflected_notify = Some(Box::new(f));
    }


    // // void OnReflectedParentNotify(UINT message, UINT nChildID, LPARAM lParam)
    // #define MSG_OCM_PARENTNOTIFY(func) \
    //     if (uMsg == OCM_PARENTNOTIFY) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_parent_notify(&mut self,f:F) where F:Fn(&T,UINT,UINT,LPARAM) + 'static {
        self.msg_entry.on_reflected_parent_notify = Some(Box::new(f));
    }


    // // void OnReflectedDrawItem(int nIDCtl, LPDRAWITEMSTRUCT lpDrawItemStruct)
    // #define MSG_OCM_DRAWITEM(func) \
    //     if (uMsg == OCM_DRAWITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPDRAWITEMSTRUCT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_draw_item(&mut self,f:F) where F:Fn(&T,c_int,&DRAWITEMSTRUCT) + 'static {
        self.msg_entry.on_reflected_draw_item = Some(Box::new(f));
    }


    // // void OnReflectedMeasureItem(int nIDCtl, LPMEASUREITEMSTRUCT lpMeasureItemStruct)
    // #define MSG_OCM_MEASUREITEM(func) \
    //     if (uMsg == OCM_MEASUREITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPMEASUREITEMSTRUCT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_measure_item(&mut self,f:F) where F:Fn(&T,c_int,&MEASUREITEMSTRUCT) + 'static {
        self.msg_entry.on_reflected_measure_item = Some(Box::new(f));
    }


    // // int OnReflectedCompareItem(int nIDCtl, LPCOMPAREITEMSTRUCT lpCompareItemStruct)
    // #define MSG_OCM_COMPAREITEM(func) \
    //     if (uMsg == OCM_COMPAREITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)wParam, (LPCOMPAREITEMSTRUCT)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_compare_item(&mut self,f:F) where F:Fn(&T,c_int,&COMPAREITEMSTRUCT) + 'static {
        self.msg_entry.on_reflected_compare_item = Some(Box::new(f));
    }


    // // void OnReflectedDeleteItem(int nIDCtl, LPDELETEITEMSTRUCT lpDeleteItemStruct)
    // #define MSG_OCM_DELETEITEM(func) \
    //     if (uMsg == OCM_DELETEITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)wParam, (LPDELETEITEMSTRUCT)lParam); \
    //         lResult = TRUE; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_delete_item(&mut self,f:F) where F:Fn(&T,c_int,&DELETEITEMSTRUCT) + 'static {
        self.msg_entry.on_reflected_delete_item = Some(Box::new(f));
    }  

    // // int OnReflectedVKeyToItem(UINT nKey, UINT nIndex, CListBox listBox)
    // #define MSG_OCM_VKEYTOITEM(func) \
    //     if (uMsg == OCM_VKEYTOITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_refelected_v_key_to_item(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CListBox)->c_int + 'static {
        self.msg_entry.on_refelected_v_key_to_item = Some(Box::new(f));
    }


    // //int OnReflectedCharToItem(UINT nChar, UINT nIndex, CListBox listBox)
    // #define MSG_OCM_CHARTOITEM(func) \
    //     if (uMsg == OCM_CHARTOITEM) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_char_to_item(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CListBox)->c_int + 'static {
        self.msg_entry.on_reflected_char_to_item = Some(Box::new(f));
    }


    // // void OnReflectedHScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    // #define MSG_OCM_HSCROLL(func) \
    //     if (uMsg == OCM_HSCROLL) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_h_scroll(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CScrollBar) + 'static {
        self.msg_entry.on_reflected_h_scroll = Some(Box::new(f));
    }


    // // void OnReflectedVScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    // #define MSG_OCM_VSCROLL(func) \
    //     if (uMsg == OCM_VSCROLL) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_refelected_v_scroll(&mut self,f:F) where F:Fn(&T,UINT,UINT,&CScrollBar) + 'static {
        self.msg_entry.on_refelected_v_scroll = Some(Box::new(f));
    }


    // // HBRUSH OnReflectedCtlColorEdit(CDCHandle dc, CEdit edit)
    // #define MSG_OCM_CTLCOLOREDIT(func) \
    //     if (uMsg == OCM_CTLCOLOREDIT) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_ctl_color_edit(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CEdit)->HBRUSH + 'static {
        self.msg_entry.on_reflected_ctl_color_edit = Some(Box::new(f));
    }


    // // HBRUSH OnReflectedCtlColorListBox(CDCHandle dc, CListBox listBox)
    // #define MSG_OCM_CTLCOLORLISTBOX(func) \
    //     if (uMsg == OCM_CTLCOLORLISTBOX) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_ctl_color_list_box(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CListBox)->HBRUSH + 'static {
        self.msg_entry.on_reflected_ctl_color_list_box = Some(Box::new(f));
    }


    // // HBRUSH OnReflectedCtlColorBtn(CDCHandle dc, CButton button)
    // #define MSG_OCM_CTLCOLORBTN(func) \
    //     if (uMsg == OCM_CTLCOLORBTN) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_ctl_color_btn(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CButton)->HBRUSH + 'static {
        self.msg_entry.on_reflected_ctl_color_btn = Some(Box::new(f));
    }


    // // HBRUSH OnReflectedCtlColorDlg(CDCHandle dc, CWindow wnd)
    // #define MSG_OCM_CTLCOLORDLG(func) \
    //     if (uMsg == OCM_CTLCOLORDLG) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_ctl_color_dlg(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CWindow)->HBRUSH + 'static {
        self.msg_entry.on_reflected_ctl_color_dlg = Some(Box::new(f));
    }


    // // HBRUSH OnReflectedCtlColorScrollBar(CDCHandle dc, CScrollBar scrollBar)
    // #define MSG_OCM_CTLCOLORSCROLLBAR(func) \
    //     if (uMsg == OCM_CTLCOLORSCROLLBAR) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_ctl_color_scroll_bar(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CScrollBar)->HBRUSH + 'static {
        self.msg_entry.on_reflected_ctl_color_scroll_bar = Some(Box::new(f));
    }


    // // HBRUSH OnReflectedCtlColorStatic(CDCHandle dc, CStatic wndStatic)
    // #define MSG_OCM_CTLCOLORSTATIC(func) \
    //     if (uMsg == OCM_CTLCOLORSTATIC) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = (LRESULT)func((HDC)wParam, (HWND)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_reflected_ctl_color_static(&mut self,f:F) where F:Fn(&T,&CDCHandle,&CStatic)->HBRUSH + 'static {
        self.msg_entry.on_reflected_ctl_color_static = Some(Box::new(f));
    }


    // ///////////////////////////////////////////////////////////////////////////////
    // // Generic message handlers

    // // LRESULT OnMessageHandlerEX(UINT uMsg, WPARAM wParam, LPARAM lParam)
    // #define MESSAGE_HANDLER_EX(msg, func) \
    //     if(uMsg == msg) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func(uMsg, wParam, lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_message_handler_ex(&mut self,f:F) where F:Fn(&T,UINT,WPARAM,LPARAM)->LRESULT + 'static {
        self.msg_entry.on_message_handler_ex = Some(Box::new(f));
    }


    // // LRESULT OnMessageRangeHandlerEX(UINT uMsg, WPARAM wParam, LPARAM lParam)
    // #define MESSAGE_RANGE_HANDLER_EX(msgFirst, msgLast, func) \
    //     if(uMsg >= msgFirst && uMsg <= msgLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func(uMsg, wParam, lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    pub fn on_message_range_handler_ex(&mut self,msgFirst:UINT,msgLast:UINT,f:F) where F:Fn(&T,UINT,WPARAM,LPARAM)->LRESULT + 'static {
        self.msg_entry.on_message_range_handler_ex = Some(Box::new(f));
    }
*/
    pub fn on_message(&self,
                      t: &T,
                      uMsg: UINT,
                      wParam: WPARAM,
                      lParam: LPARAM,
                      bHandled: &mut BOOL)
                      -> LRESULT {
        self.msg_entry.on_message(t, uMsg, wParam, lParam,bHandled)
    }

}


// command
impl<T> Handler<T> {
	/// commmand handler
    pub fn add_cmd_listener_before<F>(&mut self, id: WORD, f: F)
        where F: Fn(&T, WORD, WORD, LPARAM) -> LRESULT + 'static
    {
        //self.cmd_entry.insert(id, Box::new(f));
    }

    pub fn add_cmd_listener<F>(&mut self, id: WORD, f: F)
        where F: Fn(&T, WORD, WORD, LPARAM) -> LRESULT + 'static
    {
        //self.cmd_entry.insert(id, Box::new(f));
    }

    pub fn add_cmd_listener_after<F>(&mut self, id: WORD, f: F)
        where F: Fn(&T, WORD, WORD, LPARAM) -> LRESULT + 'static
    {
        //self.cmd_entry.insert(id, Box::new(f));
    }

    pub fn on_command(&self,
                      t: &T,
                      code: WORD,
                      id: WORD,
                      lParam: LPARAM,
                      bHandled: &mut BOOL)
                      -> LRESULT {
        let mut lRes = 0;
        lRes = self.cmd_entry.on_command(t, code, id, lParam, bHandled);
        lRes
    }

    /// on_btn_click:&T,id,Fn
    pub fn on_btn_click_before<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,&CWindow)+'static {
        let opt_call = self.cmd_entry.on_btn_click.entry(id).or_insert(OptCall::new());
        opt_call.before = Some(Box::new(f));
    }

    pub fn on_btn_click<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,&CWindow)+'static{
        let opt_call = self.cmd_entry.on_btn_click.entry(id).or_insert(OptCall::new());
        opt_call.around = Some(Box::new(f));
    }

    pub fn on_btn_click_after<F>(&mut self,id:WORD,f:F) where F:Fn(&T,WORD,&CWindow)+'static{
        let opt_call = self.cmd_entry.on_btn_click.entry(id).or_insert(OptCall::new());
        opt_call.after = Some(Box::new(f));
    }

}

// dispatch
impl<T> Handler<T> {
	/// return TRUE if processed
    pub fn disptach_msg(&mut self,
                        t: &T,
                        hWnd: HWND,
                        uMsg: UINT,
                        wParam: WPARAM,
                        lParam: LPARAM,
                        lResult: &mut LRESULT,
                        dwMsgMapID: DWORD)
                        -> BOOL {
		//debug_assert!(self.handler.is_none() == false);
        let mut bHandled = FALSE;
		//let mut bHandled:BOOL = TRUE;
        match uMsg {
            WM_NOTIFY => {
				//*lResult = self.on_notify(wParam, lParam);
            }
            WM_COMMAND => {
                let id = LOWORD(wParam as DWORD);
                let code = HIWORD(wParam as DWORD);
				//*lResult = self.on_command(code, id, lParam);
                *lResult = self.on_command(t,code, id, lParam,&mut bHandled);
            }
            _ => {
				//*lResult = self.on_message(uMsg, wParam, lParam);
                *lResult = self.on_message(t,uMsg, wParam, lParam,&mut bHandled);
				//return FALSE;
            }
        }
        bHandled
    }
}


/*
pub fn on_(&mut self,f:F) where F:Fn(&T,) + 'static {

    }

    */


    /*
    // ///////////////////////////////////////////////////////////////////////////////
    // // Commands and notifications

    // // void OnCommandHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define COMMAND_HANDLER_EX(id, code, func) \
    //     if (uMsg == WM_COMMAND && code == HIWORD(wParam) && id == LOWORD(wParam)) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_command_handler_ex(&mut self,f:F) where F:Fn(&T,UINT,c_int,&CWindow) + 'static {

    }

    // // void OnCommandIDHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define COMMAND_ID_HANDLER_EX(id, func) \
    //     if (uMsg == WM_COMMAND && id == LOWORD(wParam)) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_command_id_handler_ex(&mut self,f:F) where F:Fn(&T,UINT,c_int,&CWindow) + 'static {

    }


    // // void OnCommandCodeHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define COMMAND_CODE_HANDLER_EX(code, func) \
    //     if (uMsg == WM_COMMAND && code == HIWORD(wParam)) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_command_code_handler_ex(&mut self,f:F) where F:Fn(&T,UINT,c_int,&CWindow) + 'static {

    }


    // // LRESULT OnNotifyHandlerEX(LPNMHDR pnmh)
    // #define NOTIFY_HANDLER_EX(id, cd, func) \
    //     if (uMsg == WM_NOTIFY && cd == ((LPNMHDR)lParam)->code && id == ((LPNMHDR)lParam)->idFrom) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_notify_handler_ex(&mut self,f:F) where F:Fn(&T,&NMHDR)->LRESULT + 'static {

    }


    // // LRESULT OnNotifyIDHandlerEX(LPNMHDR pnmh)
    // #define NOTIFY_ID_HANDLER_EX(id, func) \
    //     if (uMsg == WM_NOTIFY && id == ((LPNMHDR)lParam)->idFrom) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_notify_id_handler_ex(&mut self,f:F) where F:Fn(&T,&NMHDR)->LRESULT + 'static {

    }


    // // LRESULT OnNotifyCodeHandlerEX(LPNMHDR pnmh)
    // #define NOTIFY_CODE_HANDLER_EX(cd, func) \
    //     if (uMsg == WM_NOTIFY && cd == ((LPNMHDR)lParam)->code) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_notify_code_handler_ex(&mut self,f:F) where F:Fn(&T,&NMHDR)->LRESULT + 'static {

    }


    // // void OnCommandRangeHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define COMMAND_RANGE_HANDLER_EX(idFirst, idLast, func) \
    //     if(uMsg == WM_COMMAND && LOWORD(wParam) >= idFirst && LOWORD(wParam) <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_command_range_handler_ex(&mut self,f:F) where F:Fn(&T,UINT,c_int,&CWindow) + 'static {

    }


    // // void OnCommandRangeCodeHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define COMMAND_RANGE_CODE_HANDLER_EX(idFirst, idLast, code, func) \
    //     if(uMsg == WM_COMMAND && code == HIWORD(wParam) && LOWORD(wParam) >= idFirst && LOWORD(wParam) <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_command_range_code_handler_ex(&mut self,f:F) where F:Fn(&T,UINT,c_int,&CWindow) + 'static {

    }


    // // LRESULT OnNotifyRangeHandlerEX(LPNMHDR pnmh)
    // #define NOTIFY_RANGE_HANDLER_EX(idFirst, idLast, func) \
    //     if(uMsg == WM_NOTIFY && ((LPNMHDR)lParam)->idFrom >= idFirst && ((LPNMHDR)lParam)->idFrom <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }
    pub fn on_notify_range_handler_ex(&mut self,f:F) where F:Fn(&T,&NMHDR)->LRESULT + 'static {

    }


    // // LRESULT OnNotifyRangeCodeHandlerEX(LPNMHDR pnmh)
    // #define NOTIFY_RANGE_CODE_HANDLER_EX(idFirst, idLast, cd, func) \
    //     if(uMsg == WM_NOTIFY && cd == ((LPNMHDR)lParam)->code && ((LPNMHDR)lParam)->idFrom >= idFirst && ((LPNMHDR)lParam)->idFrom <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedCommandHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define REFLECTED_COMMAND_HANDLER_EX(id, code, func) \
    //     if (uMsg == OCM_COMMAND && code == HIWORD(wParam) && id == LOWORD(wParam)) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedCommandIDHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define REFLECTED_COMMAND_ID_HANDLER_EX(id, func) \
    //     if (uMsg == OCM_COMMAND && id == LOWORD(wParam)) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedCommandCodeHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define REFLECTED_COMMAND_CODE_HANDLER_EX(code, func) \
    //     if (uMsg == OCM_COMMAND && code == HIWORD(wParam)) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedNotifyHandlerEX(LPNMHDR pnmh)
    // #define REFLECTED_NOTIFY_HANDLER_EX(id, cd, func) \
    //     if (uMsg == OCM_NOTIFY && cd == ((LPNMHDR)lParam)->code && id == ((LPNMHDR)lParam)->idFrom) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedNotifyIDHandlerEX(LPNMHDR pnmh)
    // #define REFLECTED_NOTIFY_ID_HANDLER_EX(id, func) \
    //     if (uMsg == OCM_NOTIFY && id == ((LPNMHDR)lParam)->idFrom) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedNotifyCodeHandlerEX(LPNMHDR pnmh)
    // #define REFLECTED_NOTIFY_CODE_HANDLER_EX(cd, func) \
    //     if (uMsg == OCM_NOTIFY && cd == ((LPNMHDR)lParam)->code) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // void OnReflectedCommandRangeHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define REFLECTED_COMMAND_RANGE_HANDLER_EX(idFirst, idLast, func) \
    //     if(uMsg == OCM_COMMAND && LOWORD(wParam) >= idFirst && LOWORD(wParam) <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // void OnReflectedCommandRangeCodeHandlerEX(UINT uNotifyCode, int nID, CWindow wndCtl)
    // #define REFLECTED_COMMAND_RANGE_CODE_HANDLER_EX(idFirst, idLast, code, func) \
    //     if(uMsg == OCM_COMMAND && code == HIWORD(wParam) && LOWORD(wParam) >= idFirst && LOWORD(wParam) <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam); \
    //         lResult = 0; \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedNotifyRangeHandlerEX(LPNMHDR pnmh)
    // #define REFLECTED_NOTIFY_RANGE_HANDLER_EX(idFirst, idLast, func) \
    //     if(uMsg == OCM_NOTIFY && ((LPNMHDR)lParam)->idFrom >= idFirst && ((LPNMHDR)lParam)->idFrom <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }

    // // LRESULT OnReflectedNotifyRangeCodeHandlerEX(LPNMHDR pnmh)
    // #define REFLECTED_NOTIFY_RANGE_CODE_HANDLER_EX(idFirst, idLast, cd, func) \
    //     if(uMsg == OCM_NOTIFY && cd == ((LPNMHDR)lParam)->code && ((LPNMHDR)lParam)->idFrom >= idFirst && ((LPNMHDR)lParam)->idFrom <= idLast) \
    //     { \
    //         SetMsgHandled(TRUE); \
    //         lResult = func((LPNMHDR)lParam); \
    //         if(IsMsgHandled()) \
    //             return TRUE; \
    //     }


    */