
use winapi::*;
//use user32;

//use super::super::cwindow::*;
use super::consts::*;
use super::Event;

use super::Handler;

pub struct DlgMsg <'a,T:'a> {
    h:&'a mut Vec<Handler<T>>,
}

impl<'a,T> DlgMsg<'a,T> {
	pub fn new(h:&'a mut Vec<Handler<T>>)->DlgMsg<'a,T>{
		DlgMsg{
			h:h,
		}
	}
}
/// add message handlers,priority is a u16 type
/// 0 is highest priority,and 0xFFFF is the lowest priority.
/// higher priority will be called first when there are more than one listeners for a message
impl<'a,T> DlgMsg<'a,T> {
    // pub fn on_create<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(WM_CREATE, 0, 0, priority, f));
    // }
    // int OnCreate(LPCREATESTRUCT lpCreateStruct)
    pub fn on_create<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CREATE, 0, 0, priority, f));
    }


    // BOOL OnInitDialog(CWindow wndFocus, LPARAM lInitParam)
    pub fn on_init_dialog<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_INITDIALOG, 0, 0, priority, f));
    }


    // BOOL OnCopyData(CWindow wnd, PCOPYDATASTRUCT pCopyDataStruct)
    pub fn on_copy_data<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COPYDATA, 0, 0, priority, f));
    }


    // void OnDestroy()
    pub fn on_destroy<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DESTROY, 0, 0, priority, f));
    }


    // void OnMove(CPoint ptPos)
    pub fn on_move<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOVE, 0, 0, priority, f));
    }


    // void OnSize(UINT nType, CSize size)
    pub fn on_size<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SIZE, 0, 0, priority, f));
    }


    // void OnActivate(UINT nState, BOOL bMinimized, CWindow wndOther)
    pub fn on_activate<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ACTIVATE, 0, 0, priority, f));
    }

  
    // void OnSetFocus(CWindow wndOld)
    pub fn on_set_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETFOCUS, 0, 0, priority, f));
    }

   
    // void OnKillFocus(CWindow wndFocus)
    pub fn on_kill_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_KILLFOCUS, 0, 0, priority, f));
    }


    // void OnEnable(BOOL bEnable)
    pub fn on_enable<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ENABLE, 0, 0, priority, f));
    }


    // void OnPaint(CDCHandle dc)
    pub fn on_paint<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_PAINT, 0, 0, priority, f));
    }


    // void OnClose()
    pub fn on_close<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CLOSE, 0, 0, priority, f));
    }


    // BOOL OnQueryEndSession(UINT nSource, UINT uLogOff)
    pub fn on_query_end_session<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_QUERYENDSESSION, 0, 0, priority, f));
    }


    // BOOL OnQueryOpen()
    pub fn on_query_open<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_QUERYOPEN, 0, 0, priority, f));
    }


    // BOOL OnEraseBkgnd(CDCHandle dc)
    pub fn on_erase_bkgnd<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ERASEBKGND, 0, 0, priority, f));
    }


    // void OnSysColorChange()
    pub fn on_sys_color_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SYSCOLORCHANGE, 0, 0, priority, f));
    }


    // void OnEndSession(BOOL bEnding, UINT uLogOff)
    pub fn on_end_session<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ENDSESSION, 0, 0, priority, f));
    }


    // void OnShowWindow(BOOL bShow, UINT nStatus)
    pub fn on_show_window<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SHOWWINDOW, 0, 0, priority, f));
    }


    // HBRUSH OnCtlColorEdit(CDCHandle dc, CEdit edit)
    pub fn on_ctl_color_edit<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CTLCOLOREDIT, 0, 0, priority, f));
    }


    // HBRUSH OnCtlColorListBox(CDCHandle dc, CListBox listBox)
    pub fn on_ctl_color_list_box<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CTLCOLORLISTBOX, 0, 0, priority, f));
    }


    // HBRUSH OnCtlColorBtn(CDCHandle dc, CButton button)
    pub fn on_ctl_color_btn<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CTLCOLORBTN, 0, 0, priority, f));
    }


    // HBRUSH OnCtlColorDlg(CDCHandle dc, CWindow wnd)
    pub fn on_ctl_color_dlg<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CTLCOLORDLG, 0, 0, priority, f));
    }


    // HBRUSH OnCtlColorScrollBar(CDCHandle dc, CScrollBar scrollBar)
    pub fn on_ctl_color_scroll_bar<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CTLCOLORSCROLLBAR, 0, 0, priority, f));
    }


    // HBRUSH OnCtlColorStatic(CDCHandle dc, CStatic wndStatic)
    pub fn on_ctl_color_static<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CTLCOLORSTATIC, 0, 0, priority, f));
    }


    // void OnSettingChange(UINT uFlags, LPCTSTR lpszSection)
    // aa WM_SETTINGCHANGE
    // bb          func((UINT)wParam, (LPCTSTR)lParam);
    pub fn on_setting_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETTINGCHANGE, 0, 0, priority, f));
    }

    // void OnDevModeChange(LPCTSTR lpDeviceName)
    // aa WM_DEVMODECHANGE
    // bb          func((LPCTSTR)lParam);

    pub fn on_dev_mode_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DEVMODECHANGE, 0, 0, priority, f));
    }

    // void OnActivateApp(BOOL bActive, DWORD dwThreadID)
    pub fn on_activate_app<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ACTIVATEAPP, 0, 0, priority, f));
    }


    // void OnFontChange()
    pub fn on_font_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_FONTCHANGE, 0, 0, priority, f));
    }


    // void OnTimeChange()
    pub fn on_time_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_TIMECHANGE, 0, 0, priority, f));
    }


    // void OnCancelMode()
    pub fn on_cancel_mode<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CANCELMODE, 0, 0, priority, f));
    }


    // BOOL OnSetCursor(CWindow wnd, UINT nHitTest, UINT message)
    pub fn on_set_cursor<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETCURSOR, 0, 0, priority, f));
    }


    // int OnMouseActivate(CWindow wndTopLevel, UINT nHitTest, UINT message)
    pub fn on_mouse_activate<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOUSEACTIVATE, 0, 0, priority, f));
    }


    // void OnChildActivate()
    pub fn on_child_activate<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CHILDACTIVATE, 0, 0, priority, f));
    }


    // void OnGetMinMaxInfo(LPMINMAXINFO lpMMI)
    pub fn on_get_min_max_info<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_GETMINMAXINFO, 0, 0, priority, f));
    }


    // void OnIconEraseBkgnd(CDCHandle dc)
    pub fn on_icon_erase_bkgnd<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ICONERASEBKGND, 0, 0, priority, f));
    }


    // void OnSpoolerStatus(UINT nStatus, UINT nJobs)
    pub fn on_spooler_status<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SPOOLERSTATUS, 0, 0, priority, f));
    }


    // void OnDrawItem(int nIDCtl, LPDRAWITEMSTRUCT lpDrawItemStruct)
    pub fn on_draw_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DRAWITEM, 0, 0, priority, f));
    }


    // void OnMeasureItem(int nIDCtl, LPMEASUREITEMSTRUCT lpMeasureItemStruct)
    pub fn on_measure_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MEASUREITEM, 0, 0, priority, f));
    }


    // void OnDeleteItem(int nIDCtl, LPDELETEITEMSTRUCT lpDeleteItemStruct)
    pub fn on_delete_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DELETEITEM, 0, 0, priority, f));
    }


    //int OnCharToItem(UINT nChar, UINT nIndex, CListBox listBox)
    pub fn on_char_to_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CHARTOITEM, 0, 0, priority, f));
    }


    // int OnVKeyToItem(UINT nKey, UINT nIndex, CListBox listBox)
    pub fn on_v_key_to_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_VKEYTOITEM, 0, 0, priority, f));
    }


    // HCURSOR OnQueryDragIcon()
    pub fn on_query_drag_icon<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_QUERYDRAGICON, 0, 0, priority, f));
    }


    // int OnCompareItem(int nIDCtl, LPCOMPAREITEMSTRUCT lpCompareItemStruct)
    pub fn on_compare_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMPAREITEM, 0, 0, priority, f));
    }


    // void OnCompacting(UINT nCpuTime)
    pub fn on_compacting<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMPACTING, 0, 0, priority, f));
    }


    // BOOL OnNcCreate(LPCREATESTRUCT lpCreateStruct)
    pub fn on_nc_create<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCCREATE, 0, 0, priority, f));
    }


    // void OnNcDestroy()
    pub fn on_nc_destroy<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCDESTROY, 0, 0, priority, f));
    }


    // LRESULT OnNcCalcSize(BOOL bCalcValidRects, LPARAM lParam)
    pub fn on_nc_calc_size<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCCALCSIZE, 0, 0, priority, f));
    }


    // UINT OnNcHitTest(CPoint point)
    pub fn on_nc_hit_test<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCHITTEST, 0, 0, priority, f));
    }


    // void OnNcPaint(CRgnHandle rgn)
    pub fn on_nc_paint<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCPAINT, 0, 0, priority, f));
    }


    // BOOL OnNcActivate(BOOL bActive)
    pub fn on_nc_activate<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCACTIVATE, 0, 0, priority, f));
    }


    // UINT OnGetDlgCode(LPMSG lpMsg)
    pub fn on_get_dlg_code<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_GETDLGCODE, 0, 0, priority, f));
    }


    // void OnNcMouseMove(UINT nHitTest, CPoint point)
    pub fn on_nc_mouse_move<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCMOUSEMOVE, 0, 0, priority, f));
    }


    // void OnNcLButtonDown(UINT nHitTest, CPoint point)
    pub fn on_nc_l_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCLBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnNcLButtonUp(UINT nHitTest, CPoint point)
    pub fn on_nc_l_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCLBUTTONUP, 0, 0, priority, f));
    }


    // void OnNcLButtonDblClk(UINT nHitTest, CPoint point)
    pub fn on_nc_l_button_db_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCLBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnNcRButtonDown(UINT nHitTest, CPoint point)
    pub fn on_nc_r_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCRBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnNcRButtonUp(UINT nHitTest, CPoint point)
    pub fn on_nc_r_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCRBUTTONUP, 0, 0, priority, f));
    }


    // void OnNcRButtonDblClk(UINT nHitTest, CPoint point)
    pub fn on_nc_r_button_dbl_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCRBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnNcMButtonDown(UINT nHitTest, CPoint point)
    pub fn on_nc_m_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCMBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnNcMButtonUp(UINT nHitTest, CPoint point)
    pub fn on_nc_m_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCMBUTTONUP, 0, 0, priority, f));
    }


    // void OnNcMButtonDblClk(UINT nHitTest, CPoint point)
    pub fn on_nc_m_button_dbl_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCMBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnKeyDown(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_key_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_KEYDOWN, 0, 0, priority, f));
    }


    // void OnKeyUp(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_key_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_KEYUP, 0, 0, priority, f));
    }


    // void OnChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_char<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CHAR, 0, 0, priority, f));
    }


    // void OnDeadChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_dead_char<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DEADCHAR, 0, 0, priority, f));
    }


    // void OnSysKeyDown(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_sys_key_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SYSKEYDOWN, 0, 0, priority, f));
    }


    // void OnSysKeyUp(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_sys_key_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SYSKEYUP, 0, 0, priority, f));
    }


    // void OnSysChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_sys_char<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SYSCHAR, 0, 0, priority, f));
    }


    // void OnSysDeadChar(UINT nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_sys_dead_char<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SYSDEADCHAR, 0, 0, priority, f));
    }


    // void OnSysCommand(UINT nID, CPoint point)
    pub fn on_sys_command<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SYSCOMMAND, 0, 0, priority, f));
    }


    // void OnTCard(UINT idAction, DWORD dwActionData)
    pub fn on_t_card<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_TCARD, 0, 0, priority, f));
    }


    // void OnTimer(UINT_PTR nIDEvent)
    pub fn on_timer<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_TIMER, 0, 0, priority, f));
    }


    // void OnHScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    pub fn on_h_scroll<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_HSCROLL, 0, 0, priority, f));
    }


    // void OnVScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    pub fn on_v_scroll<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_VSCROLL, 0, 0, priority, f));
    }


    // void OnInitMenu(CMenuHandle menu)
    pub fn on_init_menu<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_INITMENU, 0, 0, priority, f));
    }


    // void OnInitMenuPopup(CMenuHandle menuPopup, UINT nIndex, BOOL bSysMenu)
    pub fn on_init_menu_popup<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_INITMENUPOPUP, 0, 0, priority, f));
    }


    // void OnMenuSelect(UINT nItemID, UINT nFlags, CMenuHandle menu)
    pub fn on_menu_select<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MENUSELECT, 0, 0, priority, f));
    }


    // LRESULT OnMenuChar(UINT nChar, UINT nFlags, CMenuHandle menu)
    pub fn on_menu_char<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MENUCHAR, 0, 0, priority, f));
    }


    // LRESULT OnNotify(int idCtrl, LPNMHDR pnmh)
    pub fn on_notify<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NOTIFY, 0, 0, priority, f));
    }


    // void OnEnterIdle(UINT nWhy, CWindow wndWho)
    pub fn on_enter_idle<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ENTERIDLE, 0, 0, priority, f));
    }


    // void OnMouseMove(UINT nFlags, CPoint point)
    pub fn on_mouse_move<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOUSEMOVE, 0, 0, priority, f));
    }


    // BOOL OnMouseWheel(UINT nFlags, short zDelta, CPoint pt)
    pub fn on_mouse_wheel<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOUSEWHEEL, 0, 0, priority, f));
    }


    // void OnLButtonDown(UINT nFlags, CPoint point)
    pub fn on_l_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_LBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnLButtonUp(UINT nFlags, CPoint point)
    pub fn on_l_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_LBUTTONUP, 0, 0, priority, f));
    }


    // void OnLButtonDblClk(UINT nFlags, CPoint point)
    pub fn on_l_button_dbl_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_LBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnRButtonDown(UINT nFlags, CPoint point)
    pub fn on_r_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_RBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnRButtonUp(UINT nFlags, CPoint point)
    pub fn on_r_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_RBUTTONUP, 0, 0, priority, f));
    }


    // void OnRButtonDblClk(UINT nFlags, CPoint point)
    pub fn on_r_button_dbl_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_RBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnMButtonDown(UINT nFlags, CPoint point)
    pub fn on_m_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnMButtonUp(UINT nFlags, CPoint point)
    pub fn on_m_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MBUTTONUP, 0, 0, priority, f));
    }


    // void OnMButtonDblClk(UINT nFlags, CPoint point)
    pub fn on_m_button_dbl_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnParentNotify(UINT message, UINT nChildID, LPARAM lParam)
    pub fn on_parent_notify<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_PARENTNOTIFY, 0, 0, priority, f));
    }


    // void OnMDIActivate(CWindow wndActivate, CWindow wndDeactivate)
    pub fn on_mdi_activate<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MDIACTIVATE, 0, 0, priority, f));
    }


    // void OnRenderFormat(UINT nFormat)
    pub fn on_render_format<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_RENDERFORMAT, 0, 0, priority, f));
    }


    // void OnRenderAllFormats()
    pub fn on_render_all_formats<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_RENDERALLFORMATS, 0, 0, priority, f));
    }


    // void OnDestroyClipboard()
    pub fn on_destroy_clipboard<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DESTROYCLIPBOARD, 0, 0, priority, f));
    }


    // void OnDrawClipboard()
    pub fn on_draw_clipboard<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DRAWCLIPBOARD, 0, 0, priority, f));
    }


    // void OnPaintClipboard(CWindow wndViewer, const LPPAINTSTRUCT lpPaintStruct)
    pub fn on_paint_clipboard<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_PAINTCLIPBOARD, 0, 0, priority, f));
    }


    // void OnVScrollClipboard(CWindow wndViewer, UINT nSBCode, UINT nPos)
    pub fn on_v_scroll_clipboard<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_VSCROLLCLIPBOARD, 0, 0, priority, f));
    }


    // void OnContextMenu(CWindow wnd, CPoint point)
    pub fn on_context_menu<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CONTEXTMENU, 0, 0, priority, f));
    }


    // void OnSizeClipboard(CWindow wndViewer, const LPRECT lpRect)
    pub fn on_size_clipboard<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SIZECLIPBOARD, 0, 0, priority, f));
    }


    // void OnAskCbFormatName(UINT nMaxCount, LPTSTR lpszString)
    // aa WM_ASKCBFORMATNAME
    // bb          func((UINT)wParam, (LPTSTR)lParam);
    pub fn on_ask_cb_fromat_name<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ASKCBFORMATNAME, 0, 0, priority, f));
    }

    // void OnChangeCbChain(CWindow wndRemove, CWindow wndAfter)
    pub fn on_change_cb_chain<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CHANGECBCHAIN, 0, 0, priority, f));
    }


    // void OnHScrollClipboard(CWindow wndViewer, UINT nSBCode, UINT nPos)
    pub fn on_h_scroll_clipboard<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_HSCROLLCLIPBOARD, 0, 0, priority, f));
    }


    // BOOL OnQueryNewPalette()
    pub fn on_query_new_palette<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_QUERYNEWPALETTE, 0, 0, priority, f));
    }


    // void OnPaletteChanged(CWindow wndFocus)
    pub fn on_palette_changed<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_PALETTECHANGED, 0, 0, priority, f));
    }


    // void OnPaletteIsChanging(CWindow wndPalChg)
    pub fn on_palette_is_changing<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_PALETTEISCHANGING, 0, 0, priority, f));
    }


    // void OnDropFiles(HDROP hDropInfo)
    pub fn on_drop_files<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DROPFILES, 0, 0, priority, f));
    }


    // void OnWindowPosChanging(LPWINDOWPOS lpWndPos)
    pub fn on_window_pos_changing<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_WINDOWPOSCHANGING, 0, 0, priority, f));
    }


    // void OnWindowPosChanged(LPWINDOWPOS lpWndPos)
    pub fn on_window_pos_changed<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_WINDOWPOSCHANGED, 0, 0, priority, f));
    }


    // void OnExitMenuLoop(BOOL fIsTrackPopupMenu)
    pub fn on_exit_menu_loop<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_EXITMENULOOP, 0, 0, priority, f));
    }


    // void OnEnterMenuLoop(BOOL fIsTrackPopupMenu)
    pub fn on_enter_menu_loop<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ENTERMENULOOP, 0, 0, priority, f));
    }


    // void OnStyleChanged(int nStyleType, LPSTYLESTRUCT lpStyleStruct)
    pub fn on_style_changed<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_STYLECHANGED, 0, 0, priority, f));
    }


    // void OnStyleChanging(int nStyleType, LPSTYLESTRUCT lpStyleStruct)
    pub fn on_sytle_changing<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_STYLECHANGING, 0, 0, priority, f));
    }


    // void OnSizing(UINT fwSide, LPRECT pRect)
    pub fn on_sizing<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SIZING, 0, 0, priority, f));
    }


    // void OnMoving(UINT fwSide, LPRECT pRect)
    pub fn on_moving<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOVING, 0, 0, priority, f));
    }


    // void OnCaptureChanged(CWindow wnd)
    pub fn on_capture_changed<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CAPTURECHANGED, 0, 0, priority, f));
    }


    // BOOL OnDeviceChange(UINT nEventType, DWORD_PTR dwData)
    pub fn on_device_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DEVICECHANGE, 0, 0, priority, f));
    }


    // void OnCommand(UINT uNotifyCode, int nID, CWindow wndCtl)
    pub fn on_command<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, 0, 0, priority, f));
    }


    // void OnDisplayChange(UINT uBitsPerPixel, CSize sizeScreen)
    pub fn on_display_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_DISPLAYCHANGE, 0, 0, priority, f));
    }


    // void OnEnterSizeMove()
    pub fn on_enter_size_move<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_ENTERSIZEMOVE, 0, 0, priority, f));
    }


    // void OnExitSizeMove()
    pub fn on_exit_size_move<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_EXITSIZEMOVE, 0, 0, priority, f));
    }


    // HFONT OnGetFont()
    pub fn on_get_font<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_GETFONT, 0, 0, priority, f));
    }


    // LRESULT OnGetHotKey()
    pub fn on_get_hot_key<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_GETHOTKEY, 0, 0, priority, f));
    }


    // HICON OnGetIcon()
    pub fn on_get_icon<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_GETICON, 0, 0, priority, f));
    }


    // int OnGetText(int cchTextMax, LPTSTR lpszText)
    // aa WM_GETTEXT
    // bb          lResult = (LRESULT)func((int)wParam, (LPTSTR)lParam);
    pub fn on_get_text<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_GETTEXT, 0, 0, priority, f));
    }

    // int OnGetTextLength()
    pub fn on_get_text_length<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_GETTEXTLENGTH, 0, 0, priority, f));
    }


    // void OnHelp(LPHELPINFO lpHelpInfo)
    pub fn on_help<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_HELP, 0, 0, priority, f));
    }


    // void OnHotKey(int nHotKeyID, UINT uModifiers, UINT uVirtKey)
    pub fn on_hot_key<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_HOTKEY, 0, 0, priority, f));
    }


    // void OnInputLangChange(DWORD dwCharSet, HKL hKbdLayout)
    pub fn on_input_lang_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_INPUTLANGCHANGE, 0, 0, priority, f));
    }


    // void OnInputLangChangeRequest(BOOL bSysCharSet, HKL hKbdLayout)
    pub fn on_input_lang_change_request<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_INPUTLANGCHANGEREQUEST, 0, 0, priority, f));
    }


    // void OnNextDlgCtl(BOOL bHandle, WPARAM wCtlFocus)
    pub fn on_next_dlg_ctl<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NEXTDLGCTL, 0, 0, priority, f));
    }


    // void OnNextMenu(int nVirtKey, LPMDINEXTMENU lpMdiNextMenu)
    pub fn on_next_menu<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NEXTMENU, 0, 0, priority, f));
    }


    // int OnNotifyFormat(CWindow wndFrom, int nCommand)
    pub fn on_notify_format<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NOTIFYFORMAT, 0, 0, priority, f));
    }


    // BOOL OnPowerBroadcast(DWORD dwPowerEvent, DWORD_PTR dwData)
    pub fn on_power_broadcast<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_POWERBROADCAST, 0, 0, priority, f));
    }


    // void OnPrint(CDCHandle dc, UINT uFlags)
    pub fn on_print<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_PRINT, 0, 0, priority, f));
    }


    // void OnPrintClient(CDCHandle dc, UINT uFlags)
    pub fn on_print_client<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_PRINTCLIENT, 0, 0, priority, f));
    }


    // void OnRasDialEvent(RASCONNSTATE rasconnstate, DWORD dwError)
    // pub fn on_ras_dial_event<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(WM_RASDIALEVENT, 0, 0, priority, f));
    // }


    // void OnSetFont(CFontHandle font, BOOL bRedraw)
    pub fn on_set_font<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETFONT, 0, 0, priority, f));
    }


    // int OnSetHotKey(int nVirtKey, UINT uFlags)
    pub fn on_set_hot_key<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETHOTKEY, 0, 0, priority, f));
    }


    // HICON OnSetIcon(UINT uType, HICON hIcon)
    pub fn on_set_icon<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETICON, 0, 0, priority, f));
    }


    // void OnSetRedraw(BOOL bRedraw)
    pub fn on_set_redraw<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETREDRAW, 0, 0, priority, f));
    }


    // int OnSetText(LPCTSTR lpstrText)
    // aa WM_SETTEXT
    // bb          lResult = (LRESULT)func((LPCTSTR)lParam);
    pub fn on_set_text<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_SETTEXT, 0, 0, priority, f));
    }

    // void OnUserChanged()
    pub fn on_user_changed<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_USERCHANGED, 0, 0, priority, f));
    }



    // void OnMouseHover(WPARAM wParam, CPoint ptPos)
    pub fn on_mouser_hove<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOUSEHOVER, 0, 0, priority, f));
    }


    // void OnMouseLeave()
    pub fn on_mouse_leave<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOUSELEAVE, 0, 0, priority, f));
    }


    // void OnMenuRButtonUp(WPARAM wParam, CMenuHandle menu)
    pub fn on_menu_r_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MENURBUTTONUP, 0, 0, priority, f));
    }


    // LRESULT OnMenuDrag(WPARAM wParam, CMenuHandle menu)
    pub fn on_menu_drag<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MENUDRAG, 0, 0, priority, f));
    }


    // LRESULT OnMenuGetObject(PMENUGETOBJECTINFO info)
    pub fn on_menu_get_object<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MENUGETOBJECT, 0, 0, priority, f));
    }


    // void OnUnInitMenuPopup(UINT nID, CMenuHandle menu)
    pub fn on_un_init_menu_popup<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_UNINITMENUPOPUP, 0, 0, priority, f));
    }


    // void OnMenuCommand(WPARAM nIndex, CMenuHandle menu)
    pub fn on_menu_command<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MENUCOMMAND, 0, 0, priority, f));
    }


    // BOOL OnAppCommand(CWindow wndFocus, short cmd, WORD uDevice, int dwKeys)
    pub fn on_app_command<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_APPCOMMAND, 0, 0, priority, f));
    }


    // void OnNCXButtonDown(int fwButton, short nHittest, CPoint ptPos)
    pub fn on_ncx_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCXBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnNCXButtonUp(int fwButton, short nHittest, CPoint ptPos)
    pub fn on_ncx_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCXBUTTONUP, 0, 0, priority, f));
    }


    // void OnNCXButtonDblClk(int fwButton, short nHittest, CPoint ptPos)
    pub fn on_ncx_button_dbl_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_NCXBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnXButtonDown(int fwButton, int dwKeys, CPoint ptPos)
    pub fn on_x_button_down<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_XBUTTONDOWN, 0, 0, priority, f));
    }


    // void OnXButtonUp(int fwButton, int dwKeys, CPoint ptPos)
    pub fn on_x_button_up<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_XBUTTONUP, 0, 0, priority, f));
    }


    // void OnXButtonDblClk(int fwButton, int dwKeys, CPoint ptPos)
    pub fn on_x_button_dbl_clk<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_XBUTTONDBLCLK, 0, 0, priority, f));
    }


    // void OnChangeUIState(WORD nAction, WORD nState)
    pub fn on_change_ui_state<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_CHANGEUISTATE, 0, 0, priority, f));
    }


    // void OnUpdateUIState(WORD nAction, WORD nState)
    pub fn on_update_ui_state<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_UPDATEUISTATE, 0, 0, priority, f));
    }


    // LRESULT OnQueryUIState()
    pub fn on_query_ui_state<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_QUERYUISTATE, 0, 0, priority, f));
    }


    // void OnInput(WPARAM RawInputCode, HRAWINPUT hRawInput)
    pub fn on_input<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_INPUT, 0, 0, priority, f));
    }


    // void OnUniChar(TCHAR nChar, UINT nRepCnt, UINT nFlags)
    pub fn on_uni_char<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_UNICHAR, 0, 0, priority, f));
    }


    // void OnWTSSessionChange(WPARAM nStatusCode, PWTSSESSION_NOTIFICATION nSessionID)
    pub fn on_wt_session_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_WTSSESSION_CHANGE, 0, 0, priority, f));
    }


    // void OnThemeChanged()
    pub fn on_theme_changed<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_THEMECHANGED, 0, 0, priority, f));
    }


    // BOOL OnMouseHWheel(UINT nFlags, short zDelta, CPoint pt)
    pub fn on_mouse_h_wheel<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_MOUSEHWHEEL, 0, 0, priority, f));
    }


    ///////////////////////////////////////////////////////////////////////////////
    // ATL defined messages
    // BOOL OnForwardMsg(LPMSG Msg, DWORD nUserData)
    // pub fn on_forward_msg<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(WM_FORWARDMSG, 0, 0, priority, f));
    // }


    ///////////////////////////////////////////////////////////////////////////////
    // Dialog specific messages
    // LRESULT OnDMGetDefID()
    // pub fn on_dm_get_def_id<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(DM_GETDEFID, 0, 0, priority, f));
    // }


    // void OnDMSetDefID(UINT DefID)
    // pub fn on_dm_set_def_id<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(DM_SETDEFID, 0, 0, priority, f));
    // }


    // void OnDMReposition()
    // pub fn on_dm_reposition<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(DM_REPOSITION, 0, 0, priority, f));
    // }


    ///////////////////////////////////////////////////////////////////////////////
    // Reflected messages
    // void OnReflectedCommand(UINT uNotifyCode, int nID, CWindow wndCtl)
    pub fn on_reflected_command<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_COMMAND, 0, 0, priority, f));
    }


    // LRESULT OnReflectedNotify(int idCtrl, LPNMHDR pnmh)
    pub fn on_reflected_notify<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_NOTIFY, 0, 0, priority, f));
    }


    // void OnReflectedParentNotify(UINT message, UINT nChildID, LPARAM lParam)
    pub fn on_reflected_parent_notify<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_PARENTNOTIFY, 0, 0, priority, f));
    }


    // void OnReflectedDrawItem(int nIDCtl, LPDRAWITEMSTRUCT lpDrawItemStruct)
    pub fn on_reflected_draw_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_DRAWITEM, 0, 0, priority, f));
    }


    // void OnReflectedMeasureItem(int nIDCtl, LPMEASUREITEMSTRUCT lpMeasureItemStruct)
    pub fn on_reflected_measure_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_MEASUREITEM, 0, 0, priority, f));
    }


    // int OnReflectedCompareItem(int nIDCtl, LPCOMPAREITEMSTRUCT lpCompareItemStruct)
    pub fn on_reflected_compare_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_COMPAREITEM, 0, 0, priority, f));
    }


    // void OnReflectedDeleteItem(int nIDCtl, LPDELETEITEMSTRUCT lpDeleteItemStruct)
    pub fn on_reflected_delete_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_DELETEITEM, 0, 0, priority, f));
    }

  
    // int OnReflectedVKeyToItem(UINT nKey, UINT nIndex, CListBox listBox)
    pub fn on_refelected_v_key_to_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_VKEYTOITEM, 0, 0, priority, f));
    }


    //int OnReflectedCharToItem(UINT nChar, UINT nIndex, CListBox listBox)
    pub fn on_reflected_char_to_item<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_CHARTOITEM, 0, 0, priority, f));
    }


    // void OnReflectedHScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    pub fn on_reflected_h_scroll<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_HSCROLL, 0, 0, priority, f));
    }


    // void OnReflectedVScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
    pub fn on_refelected_v_scroll<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_VSCROLL, 0, 0, priority, f));
    }


    // HBRUSH OnReflectedCtlColorEdit(CDCHandle dc, CEdit edit)
    pub fn on_reflected_ctl_color_edit<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_CTLCOLOREDIT, 0, 0, priority, f));
    }


    // HBRUSH OnReflectedCtlColorListBox(CDCHandle dc, CListBox listBox)
    pub fn on_reflected_ctl_color_list_box<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_CTLCOLORLISTBOX, 0, 0, priority, f));
    }


    // HBRUSH OnReflectedCtlColorBtn(CDCHandle dc, CButton button)
    pub fn on_reflected_ctl_color_btn<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_CTLCOLORBTN, 0, 0, priority, f));
    }


    // HBRUSH OnReflectedCtlColorDlg(CDCHandle dc, CWindow wnd)
    pub fn on_reflected_ctl_color_dlg<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_CTLCOLORDLG, 0, 0, priority, f));
    }


    // HBRUSH OnReflectedCtlColorScrollBar(CDCHandle dc, CScrollBar scrollBar)
    pub fn on_reflected_ctl_color_scroll_bar<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_CTLCOLORSCROLLBAR, 0, 0, priority, f));
    }


    // HBRUSH OnReflectedCtlColorStatic(CDCHandle dc, CStatic wndStatic)
    pub fn on_reflected_ctl_color_static<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(OCM_CTLCOLORSTATIC, 0, 0, priority, f));
    }


    // LRESULT OnMessageHandlerEX(UINT uMsg, WPARAM wParam, LPARAM lParam)
    // pub fn on_message_handler_ex<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(msg, 0, 0, priority, f));
    // }


    // LRESULT OnMessageRangeHandlerEX(UINT uMsg, WPARAM wParam, LPARAM lParam)
    // bb          lResult = func(uMsg, wParam, lParam);
    // dd on_message_range_handler_ex
    // pub fn on_message_handler_ex<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
    //     self.h.push(Handler::new(msg, 0, 0, priority, f));
    // }

}