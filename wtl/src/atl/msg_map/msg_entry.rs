use winapi::*;
use std::collections::BTreeMap;
use super::super::CWindow;


// // pub\s+fn\s+(\w+).*(Fn\(.*\)).*$ => aa pub \1: Option<Box<\2>>,
/// only one call for message entry

pub struct MsgEntry<T> {
    pub on_create: Option<Box<Fn(&T,&CREATESTRUCTW)->c_int>>,
    pub on_init_dialog: Option<Box<Fn(&T,&CWindow,LPARAM)->BOOL>>,
    // pub on_copy_data: Option<Box<Fn(&T,&COPYDATASTRUCT)>>,
    // pub on_destroy: Option<Box<Fn(&T)>>,
    // pub on_move: Option<Box<Fn(&T,&CPoint)>>,
    // pub on_size: Option<Box<Fn(&T,UINT,&CSize)>>,
    // pub on_activate: Option<Box<Fn(&T,UINT,BOOL,&CWindow)>>,
    // pub on_set_focus: Option<Box<Fn(&T,&CWindow)>>,
    // pub on_kill_focus: Option<Box<Fn(&T,&CWindow)>>,
    // pub on_enable: Option<Box<Fn(&T,BOOL)>>,
    // pub on_paint: Option<Box<Fn(&T,&CDCHandle)>>,
    pub on_close: Option<Box<Fn(&T)>>,
    /*
    pub on_query_end_session: Option<Box<Fn(&T,UINT,UINT)>>,
    pub on_query_open: Option<Box<Fn(&T)>>,
    pub on_erase_bkgnd: Option<Box<Fn(&T,&CDCHandle)>>,
    pub on_sys_color_change: Option<Box<Fn(&T)>>,
    pub on_end_session: Option<Box<Fn(&T,BOOL,UINT)>>,
    pub on_show_window: Option<Box<Fn(&T,BOOL,UINT)>>,
    pub on_ctl_color_edit: Option<Box<Fn(&T,&CDCHandle,&CEdit)>>,
    pub on_ctl_color_list_box: Option<Box<Fn(&T,&CDCHandle,&CListBox)>>,
    pub on_ctl_color_btn: Option<Box<Fn(&T,&CDCHandle,&CButton)>>,
    pub on_ctl_color_dlg: Option<Box<Fn(&T,&CDCHandle,&CWindow)>>,
    pub on_ctl_color_scroll_bar: Option<Box<Fn(&T,&CDCHandle,&CScrollBar)>>,
    pub on_ctl_color_static: Option<Box<Fn(&T,&CDCHandle,&CStatic)>>,
    pub on_activate_app: Option<Box<Fn(&T,BOOL,DWORD)>>,
    pub on_font_change: Option<Box<Fn(&T)>>,
    pub on_time_change: Option<Box<Fn(&T)>>,
    pub on_cancel_mode: Option<Box<Fn(&T)>>,
    pub on_set_cursor: Option<Box<Fn(&T,&CWindow,UINT,UINT)>>,
    pub on_mouse_activate: Option<Box<Fn(&T,&CWindow,UINT,UINT)>>,
    pub on_child_activate: Option<Box<Fn(&T)>>,
    pub on_get_min_max_info: Option<Box<Fn(&T,&MINMAXINFO)>>,
    pub on_icon_erase_bkgnd: Option<Box<Fn(&T,&CDCHandle)>>,
    pub on_spooler_status: Option<Box<Fn(&T,UINT,UINT)>>,
    pub on_draw_item: Option<Box<Fn(&T,c_int,&DRAWITEMSTRUCT)>>,
    pub on_measure_item: Option<Box<Fn(&T,c_int,&MEASUREITEMSTRUCT)>>,
    pub on_delete_item: Option<Box<Fn(&T,c_int,&DELETEITEMSTRUCT)>>,
    pub on_char_to_item: Option<Box<Fn(&T,UINT,UINT,&CListBox)>>,
    pub on_v_key_to_item: Option<Box<Fn(&T,UINT,UINT,&CListBox)>>,
    pub on_query_drag_icon: Option<Box<Fn(&T)>>,
    pub on_compare_item: Option<Box<Fn(&T,c_int,&COMPAREITEMSTRUCT)>>,
    pub on_compacting: Option<Box<Fn(&T,UINT)>>,
    pub on_nc_create: Option<Box<Fn(&T,&CREATESTRUCT)>>,
    pub on_nc_destroy: Option<Box<Fn(&T)>>,
    pub on_nc_calc_size: Option<Box<Fn(&T,BOOL,LPARAM)>>,
    pub on_nc_hit_test: Option<Box<Fn(&T,&CPoint)>>,
    pub on_nc_paint: Option<Box<Fn(&T,CRgnHandle)>>,
    pub on_nc_activate: Option<Box<Fn(&T,BOOL)>>,
    pub on_get_dlg_code: Option<Box<Fn(&T,&MSG)>>,
    pub on_nc_mouse_move: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_l_button_down: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_l_button_up: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_l_button_db_clk: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_r_button_down: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_r_button_up: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_r_button_dbl_clk: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_m_button_down: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_m_button_up: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_nc_m_button_dbl_clk: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_key_down: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_key_up: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_char: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_dead_char: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_sys_key_down: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_sys_key_up: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_sys_char: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_sys_dead_char: Option<Box<Fn(&T,UINT,UINT,UINT)>>,
    pub on_sys_command: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_t_card: Option<Box<Fn(&T,UINT,DWORD)>>,
    pub on_timer: Option<Box<Fn(&T,UINT_PTR)>>,
    pub on_h_scroll: Option<Box<Fn(&T,UINT,UINT,&CScrollBar)>>,
    pub on_v_scroll: Option<Box<Fn(&T,UINT,UINT,&CScrollBar)>>,
    pub on_init_menu: Option<Box<Fn(&T,&CMenuHandle)>>,
    pub on_init_menu_popup: Option<Box<Fn(&T,&CMenuHandle,UINT,BOOL)>>,
    pub on_menu_select: Option<Box<Fn(&T,UINT,UINT,&CMenuHandle)>>,
    pub on_menu_char: Option<Box<Fn(&T,UINT,UINT,&CMenuHandle)>>,
    pub on_notify: Option<Box<Fn(&T,c_int,&NMHDR)>>,
    pub on_enter_idle: Option<Box<Fn(&T,UINT,&CWindow)>>,
    pub on_mouse_move: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_mouse_wheel: Option<Box<Fn(&T,UINT,short,&CPoint)>>,
    pub on_l_button_down: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_l_button_up: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_l_button_dbl_clk: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_r_button_down: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_r_button_up: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_r_button_dbl_clk: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_m_button_down: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_m_button_up: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_m_button_dbl_clk: Option<Box<Fn(&T,UINT,&CPoint)>>,
    pub on_parent_notify: Option<Box<Fn(&T,UINT,UINT,LPARAM)>>,
    pub on_mdi_activate: Option<Box<Fn(&T,&CWindow,&CWindow)>>,
    pub on_render_format: Option<Box<Fn(&T,UINT)>>,
    pub on_render_all_formats: Option<Box<Fn(&T)>>,
    pub on_destroy_clipboard: Option<Box<Fn(&T)>>,
    pub on_draw_clipboard: Option<Box<Fn(&T)>>,
    pub on_paint_clipboard: Option<Box<Fn(&T,&CWindow,&PAINTSTRUCT)>>,
    pub on_v_scroll_clipboard: Option<Box<Fn(&T,&CWindow,UINT,UINT)>>,
    pub on_context_menu: Option<Box<Fn(&T,&CWindow,&CPoint)>>,
    pub on_size_clipboard: Option<Box<Fn(&T,&CWindow,&RECT)>>,
    pub on_change_cb_chain: Option<Box<Fn(&T,&CWindow,&CWindow)>>,
    pub on_h_scroll_clipboard: Option<Box<Fn(&T,&CWindow,UINT,UINT)>>,
    pub on_query_new_palette: Option<Box<Fn(&T)>>,
    pub on_palette_changed: Option<Box<Fn(&T,&CWindow)>>,
    pub on_palette_is_changing: Option<Box<Fn(&T,&CWindow)>>,
    pub on_drop_files: Option<Box<Fn(&T,HDROP)>>,
    pub on_window_pos_changing: Option<Box<Fn(&T,&WINDOWPOS)>>,
    pub on_window_pos_changed: Option<Box<Fn(&T,&WINDOWPOS)>>,
    pub on_exit_menu_loop: Option<Box<Fn(&T,BOOL)>>,
    pub on_enter_menu_loop: Option<Box<Fn(&T,BOOL)>>,
    pub on_style_changed: Option<Box<Fn(&T,c_int,&STYLESTRUCT)>>,
    pub on_sytle_changing: Option<Box<Fn(&T,c_int,&STYLESTRUCT)>>,
    pub on_sizing: Option<Box<Fn(&T,UINT,&RECT)>>,
    pub on_moving: Option<Box<Fn(&T,UINT,&RECT)>>,
    pub on_capture_changed: Option<Box<Fn(&T,&CWindow)>>,
    pub on_device_change: Option<Box<Fn(&T,UINT,DWORD_PTR)>>,
    pub on_command: Option<Box<Fn(&T,UINT,c_int,&CWindow)>>,
    pub on_display_change: Option<Box<Fn(&T,UINT,&CSize)>>,
    pub on_enter_size_move: Option<Box<Fn(&T)>>,
    pub on_exit_size_move: Option<Box<Fn(&T)>>,
    pub on_get_font: Option<Box<Fn(&T)>>,
    pub on_get_hot_key: Option<Box<Fn(&T,)>>,
    pub on_get_icon: Option<Box<Fn(&T,)>>,
    pub on_get_text_length: Option<Box<Fn(&T)>>,
    pub on_help: Option<Box<Fn(&T,&HELPINFO)>>,
    pub on_hot_key: Option<Box<Fn(&T,c_int,UINT,UINT)>>,
    pub on_input_lang_change: Option<Box<Fn(&T,DWORD,HKL)>>,
    pub on_input_lang_change_request: Option<Box<Fn(&T,BOOL,HKL)>>,
    pub on_next_dlg_ctl: Option<Box<Fn(&T,BOOL,WPARAM)>>,
    pub on_next_menu: Option<Box<Fn(&T,c_int,&MDINEXTMENU)>>,
    pub on_notify_format: Option<Box<Fn(&T,&CWindow)>>,
    pub on_power_broadcast: Option<Box<Fn(&T,DWORD,DWORD_PTR)>>,
    pub on_print: Option<Box<Fn(&T,&CDCHandle,UINT)>>,
    pub on_print_client: Option<Box<Fn(&T,&CDCHandle,UINT)>>,
    pub on_ras_dial_event: Option<Box<Fn(&T,&RASCONNSTATE,DWORD)>>,
    pub on_set_font: Option<Box<Fn(&T,&CFontHandle,BOOL)>>,
    pub on_set_hot_key: Option<Box<Fn(&T,c_int,UINT)>>,
    pub on_set_icon: Option<Box<Fn(&T,UINT,HICON)>>,
    pub on_set_redraw: Option<Box<Fn(&T,BOOL)>>,
    pub on_user_changed: Option<Box<Fn(&T)>>,
    pub on_mouser_hove: Option<Box<Fn(&T,WPARAM,&CPoint)>>,
    pub on_mouse_leave: Option<Box<Fn(&T)>>,
    pub on_menu_r_button_up: Option<Box<Fn(&T,WPARAM,&CMenuHandle)>>,
    pub on_menu_drag: Option<Box<Fn(&T,WPARAM,&CMenuHandle)>>,
    pub on_menu_get_object: Option<Box<Fn(&T,&MENUGETOBJECTINFO)>>,
    pub on_un_init_menu_popup: Option<Box<Fn(&T,UINT,&CMenuHandle)>>,
    pub on_menu_command: Option<Box<Fn(&T,WAPARM,&CMenuHandle)>>,
    pub on_app_command: Option<Box<Fn(&T,&CWindow,c_short,WORD,c_int)>>,
    pub on_ncx_button_down: Option<Box<Fn(&T,c_int,c_short,&CPoint)>>,
    pub on_ncx_button_up: Option<Box<Fn(&T,c_int,c_short,&CPoint)>>,
    pub on_ncx_button_dbl_clk: Option<Box<Fn(&T,c_int,c_short,&CPoint)>>,
    pub on_x_button_down: Option<Box<Fn(&T,c_int,c_int,&CPoint)>>,
    pub on_x_button_up: Option<Box<Fn(&T,c_int,c_int,&CPoint)>>,
    pub on_x_button_dbl_clk: Option<Box<Fn(&T,c_int,c_int,&CPoint)>>,
    pub on_change_ui_state: Option<Box<Fn(&T,WORD,WORD)>>,
    pub on_update_ui_state: Option<Box<Fn(&T,WORD,WORD)>>,
    pub on_query_ui_state: Option<Box<Fn(&T)>>,
    pub on_input: Option<Box<Fn(&T,WPARAM,HRAWINPUT)>>,
    pub on_uni_char: Option<Box<Fn(&T,wchar_t,UINT,UINT)>>,
    pub on_wt_session_change: Option<Box<Fn(&T,WPARAM,&WTSSESSION_NOTIFICATION)>>,
    pub on_theme_changed: Option<Box<Fn(&T)>>,
    pub on_mouse_wheel: Option<Box<Fn(&T,UINT,c_short,&CPoint)>>,
    pub on_forward_msg: Option<Box<Fn(&T,&MSG,DWORD)>>,
    pub on_dm_get_def_id: Option<Box<Fn(&T)>>,
    pub on_dm_set_def_id: Option<Box<Fn(&T,UINT)>>,
    pub on_dm_reposition: Option<Box<Fn(&T)>>,
    pub on_reflected_command: Option<Box<Fn(&T,UINT,c_int,&CWindow)>>,
    pub on_reflected_notify: Option<Box<Fn(&T,c_int,&NMHDR)>>,
    pub on_reflected_parent_notify: Option<Box<Fn(&T,UINT,UINT,LPARAM)>>,
    pub on_reflected_draw_item: Option<Box<Fn(&T,c_int,&DRAWITEMSTRUCT)>>,
    pub on_reflected_measure_item: Option<Box<Fn(&T,c_int,&MEASUREITEMSTRUCT)>>,
    pub on_reflected_compare_item: Option<Box<Fn(&T,c_int,&COMPAREITEMSTRUCT)>>,
    pub on_reflected_delete_item: Option<Box<Fn(&T,c_int,&DELETEITEMSTRUCT)>>,
    pub on_refelected_v_key_to_item: Option<Box<Fn(&T,UINT,UINT,&CListBox)>>,
    pub on_reflected_char_to_item: Option<Box<Fn(&T,UINT,UINT,&CListBox)>>,
    pub on_reflected_h_scroll: Option<Box<Fn(&T,UINT,UINT,&CScrollBar)>>,
    pub on_refelected_v_scroll: Option<Box<Fn(&T,UINT,UINT,&CScrollBar)>>,
    pub on_reflected_ctl_color_edit: Option<Box<Fn(&T,&CDCHandle,&CEdit)>>,
    pub on_reflected_ctl_color_list_box: Option<Box<Fn(&T,&CDCHandle,&CListBox)>>,
    pub on_reflected_ctl_color_btn: Option<Box<Fn(&T,&CDCHandle,&CButton)>>,
    pub on_reflected_ctl_color_dlg: Option<Box<Fn(&T,&CDCHandle,&CWindow)>>,
    pub on_reflected_ctl_color_scroll_bar: Option<Box<Fn(&T,&CDCHandle,&CScrollBar)>>,
    pub on_reflected_ctl_color_static: Option<Box<Fn(&T,&CDCHandle,&CStatic)>>,
    pub on_message_handler_ex: Option<Box<Fn(&T,UINT,WPARAM,LPARAM)>>,
    pub on_message_range_handler_ex: Option<Box<Fn(&T,UINT,WPARAM,LPARAM)>>,
    */
}

// pub\s+fn\s+(\w+).*(Fn\(.*\)).*$ => aa \1: None,
impl<T> MsgEntry<T> {
    pub fn new()->MsgEntry<T>{
        MsgEntry{
            on_create: None,
            on_init_dialog: None,
            // on_copy_data: None,
            // on_destroy: None,
            // on_move: None,
            // on_size: None,
            // on_activate: None,
            // on_set_focus: None,
            // on_kill_focus: None,
            // on_enable: None,
            // on_paint: None,
            on_close: None,
            // on_query_end_session: None,
            // on_query_open: None,
            // on_erase_bkgnd: None,
            // on_sys_color_change: None,
            // on_end_session: None,
            // on_show_window: None,
            // on_ctl_color_edit: None,
            // on_ctl_color_list_box: None,
            // on_ctl_color_btn: None,
            // on_ctl_color_dlg: None,
            // on_ctl_color_scroll_bar: None,
            // on_ctl_color_static: None,
            // on_activate_app: None,
            // on_font_change: None,
            // on_time_change: None,
            // on_cancel_mode: None,
            // on_set_cursor: None,
            // on_mouse_activate: None,
            // on_child_activate: None,
            // on_get_min_max_info: None,
            // on_icon_erase_bkgnd: None,
            // on_spooler_status: None,
            // on_draw_item: None,
            // on_measure_item: None,
            // on_delete_item: None,
            // on_char_to_item: None,
            // on_v_key_to_item: None,
            // on_query_drag_icon: None,
            // on_compare_item: None,
            // on_compacting: None,
            // on_nc_create: None,
            // on_nc_destroy: None,
            // on_nc_calc_size: None,
            // on_nc_hit_test: None,
            // on_nc_paint: None,
            // on_nc_activate: None,
            // on_get_dlg_code: None,
            // on_nc_mouse_move: None,
            // on_nc_l_button_down: None,
            // on_nc_l_button_up: None,
            // on_nc_l_button_db_clk: None,
            // on_nc_r_button_down: None,
            // on_nc_r_button_up: None,
            // on_nc_r_button_dbl_clk: None,
            // on_nc_m_button_down: None,
            // on_nc_m_button_up: None,
            // on_nc_m_button_dbl_clk: None,
            // on_key_down: None,
            // on_key_up: None,
            // on_char: None,
            // on_dead_char: None,
            // on_sys_key_down: None,
            // on_sys_key_up: None,
            // on_sys_char: None,
            // on_sys_dead_char: None,
            // on_sys_command: None,
            // on_t_card: None,
            // on_timer: None,
            // on_h_scroll: None,
            // on_v_scroll: None,
            // on_init_menu: None,
            // on_init_menu_popup: None,
            // on_menu_select: None,
            // on_menu_char: None,
            // on_notify: None,
            // on_enter_idle: None,
            // on_mouse_move: None,
            // on_mouse_wheel: None,
            // on_l_button_down: None,
            // on_l_button_up: None,
            // on_l_button_dbl_clk: None,
            // on_r_button_down: None,
            // on_r_button_up: None,
            // on_r_button_dbl_clk: None,
            // on_m_button_down: None,
            // on_m_button_up: None,
            // on_m_button_dbl_clk: None,
            // on_parent_notify: None,
            // on_mdi_activate: None,
            // on_render_format: None,
            // on_render_all_formats: None,
            // on_destroy_clipboard: None,
            // on_draw_clipboard: None,
            // on_paint_clipboard: None,
            // on_v_scroll_clipboard: None,
            // on_context_menu: None,
            // on_size_clipboard: None,
            // on_change_cb_chain: None,
            // on_h_scroll_clipboard: None,
            // on_query_new_palette: None,
            // on_palette_changed: None,
            // on_palette_is_changing: None,
            // on_drop_files: None,
            // on_window_pos_changing: None,
            // on_window_pos_changed: None,
            // on_exit_menu_loop: None,
            // on_enter_menu_loop: None,
            // on_style_changed: None,
            // on_sytle_changing: None,
            // on_sizing: None,
            // on_moving: None,
            // on_capture_changed: None,
            // on_device_change: None,
            // on_command: None,
            // on_display_change: None,
            // on_enter_size_move: None,
            // on_exit_size_move: None,
            // on_get_font: None,
            // on_get_hot_key: None,
            // on_get_icon: None,
            // on_get_text_length: None,
            // on_help: None,
            // on_hot_key: None,
            // on_input_lang_change: None,
            // on_input_lang_change_request: None,
            // on_next_dlg_ctl: None,
            // on_next_menu: None,
            // on_notify_format: None,
            // on_power_broadcast: None,
            // on_print: None,
            // on_print_client: None,
            // on_ras_dial_event: None,
            // on_set_font: None,
            // on_set_hot_key: None,
            // on_set_icon: None,
            // on_set_redraw: None,
            // on_user_changed: None,
            // on_mouser_hove: None,
            // on_mouse_leave: None,
            // on_menu_r_button_up: None,
            // on_menu_drag: None,
            // on_menu_get_object: None,
            // on_un_init_menu_popup: None,
            // on_menu_command: None,
            // on_app_command: None,
            // on_ncx_button_down: None,
            // on_ncx_button_up: None,
            // on_ncx_button_dbl_clk: None,
            // on_x_button_down: None,
            // on_x_button_up: None,
            // on_x_button_dbl_clk: None,
            // on_change_ui_state: None,
            // on_update_ui_state: None,
            // on_query_ui_state: None,
            // on_input: None,
            // on_uni_char: None,
            // on_wt_session_change: None,
            // on_theme_changed: None,
            // on_mouse_wheel: None,
            // on_forward_msg: None,
            // on_dm_get_def_id: None,
            // on_dm_set_def_id: None,
            // on_dm_reposition: None,
            // on_reflected_command: None,
            // on_reflected_notify: None,
            // on_reflected_parent_notify: None,
            // on_reflected_draw_item: None,
            // on_reflected_measure_item: None,
            // on_reflected_compare_item: None,
            // on_reflected_delete_item: None,
            // on_refelected_v_key_to_item: None,
            // on_reflected_char_to_item: None,
            // on_reflected_h_scroll: None,
            // on_refelected_v_scroll: None,
            // on_reflected_ctl_color_edit: None,
            // on_reflected_ctl_color_list_box: None,
            // on_reflected_ctl_color_btn: None,
            // on_reflected_ctl_color_dlg: None,
            // on_reflected_ctl_color_scroll_bar: None,
            // on_reflected_ctl_color_static: None,
            // on_message_handler_ex: None,
            // on_message_range_handler_ex: None,
        }
    }

    // cc (.*)$\r\naa (\w+)\r\nbb\s*(.*)$\r\ndd (\w+)
    // =>
    //\2=>{\r\n                if let Some\(ref call\) = self.\4 {\r\n                    \1\r\n                  // \3\r\n                    call \(t,\);\r\n                    *bHandled = TRUE;\r\n                }\r\n            },
    pub fn on_message(&self,t: &T,uMsg: UINT,wParam: WPARAM,lParam: LPARAM,bHandled: &mut BOOL) -> LRESULT {
        let mut lResult:LRESULT = 0;
        match uMsg{
            // WM_CREATE=>{
            //     if let Some(ref optc) = self.on_create {
            //         let c = unsafe{*(lParam as LPCREATESTRUCTW)};
            //         optc(t,&c);
            //         *bHandled = TRUE;
            //     }
            // },
            // WM_INITDIALOG=>{
            //     if let Some(ref optc) = self.on_init_dialog {
            //         let c = CWindow::new(wParam as HWND);
            //         optc(t,&c,lParam);
            //         *bHandled = TRUE;
            //     }
            // },
            // WM_CLOSE=>{
            //     if let Some(ref optc) = self.on_close {
            //         optc(t);
            //         *bHandled = TRUE;
            //     }
            // },
            WM_CREATE=>{
                if let Some(ref call) = self.on_create {
                    // int OnCreate(LPCREATESTRUCT lpCreateStruct)
                    // lResult = (LRESULT)func((LPCREATESTRUCT)lParam);
                    let lpCreateStruct = unsafe{*(lParam as LPCREATESTRUCTW)};
                    lResult = call (t,&lpCreateStruct) as LRESULT;
                    *bHandled = TRUE;
                }
            },
            WM_INITDIALOG=>{
                if let Some(ref call) = self.on_init_dialog {
                    // BOOL OnInitDialog(CWindow wndFocus, LPARAM lInitParam)
                    // lResult = (LRESULT)func((HWND)wParam, lParam);
                    let wndFocus = CWindow::new(wParam as HWND);
                    lResult = call (t,&wndFocus,lParam) as LRESULT;
                    *bHandled = TRUE;
                }
            },
/*
            WM_COPYDATA=>{
                if let Some(ref call) = self.on_copy_data {
                    // BOOL OnCopyData(CWindow wnd, PCOPYDATASTRUCT pCopyDataStruct)
                    // lResult = (LRESULT)func((HWND)wParam, (PCOPYDATASTRUCT)lParam);
                    let wnd = CWindow::new(wParam as HWND);
                    let pCopyDataStruct   = unsafe{*(lParam as PCOPYDATASTRUCT)};
                    lResult = call (t,&wnd,&pCopyDataStruct);
                    *bHandled = TRUE;
                }
            },
            WM_DESTROY=>{
                if let Some(ref call) = self.on_destroy {
                    // void OnDestroy()
                    // func();
                    call (t);
                    *bHandled = TRUE;
                }
            },
            WM_MOVE=>{
                if let Some(ref call) = self.on_move {
                    // void OnMove(CPoint ptPos)
                    // func(_WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    let ptPos = CPoint{x:GET_X_LPARAM(lParam), y:GET_Y_LPARAM(lParam)};
                    call (t,&ptPos);
                    *bHandled = TRUE;
                }
            },
            WM_SIZE=>{
                if let Some(ref call) = self.on_size {
                    // void OnSize(UINT nType, CSize size)
                    // func((UINT)wParam, _WTYPES_NS::CSize(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    let nType = wParam as UINT;
                    let size = CSize{x:GET_X_LPARAM(lParam), y:GET_Y_LPARAM(lParam)};
                    call (t,nType,&size);
                    *bHandled = TRUE;
                }
            },
            WM_ACTIVATE=>{
                if let Some(ref call) = self.on_activate {
                    // void OnActivate(UINT nState, BOOL bMinimized, CWindow wndOther)
                    // func((UINT)LOWORD(wParam), (BOOL)HIWORD(wParam), (HWND)lParam);
                    let nState = LOWORD(wParam) as UINT;
                    let bMinimized = HIWORD(wParam) as BOOL;
                    let wndOther = CWindow::new(lParam as HWND);
                    call (t,nState,bMinimized,&wndOther);
                    *bHandled = TRUE;
                }
            },
  
            WM_SETFOCUS=>{
                if let Some(ref call) = self.on_set_focus {
                    // void OnSetFocus(CWindow wndOld)
                    // func((HWND)wParam);
                    let wndOld = CWindow::new(wParam as HWND);
                    call (t,&wndOld);
                    *bHandled = TRUE;
                }
            },
   
            WM_KILLFOCUS=>{
                if let Some(ref call) = self.on_kill_focus {
                    // void OnKillFocus(CWindow wndFocus)
                    // func((HWND)wParam);
                    let wndFocus = CWindow::new(wParam as HWND);
                    call (t,&wndFocus);
                    *bHandled = TRUE;
                }
            },
            WM_ENABLE=>{
                if let Some(ref call) = self.on_enable {
                    // void OnEnable(BOOL bEnable)
                    // func((BOOL)wParam);
                    let bEnable = wParam as BOOL;
                    call (t,bEnable);
                    *bHandled = TRUE;
                }
            },
            WM_PAINT=>{
                if let Some(ref call) = self.on_paint {
                    // void OnPaint(CDCHandle dc)
                    // func((HDC)wParam);
                    let dc = CDCHandle::new(wParam as HDC);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
*/
            WM_CLOSE=>{
                if let Some(ref call) = self.on_close {
                    // void OnClose()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
/*
            WM_QUERYENDSESSION=>{
                if let Some(ref call) = self.on_query_end_session {
                    // BOOL OnQueryEndSession(UINT nSource, UINT uLogOff)
                    // lResult = (LRESULT)func((UINT)wParam, (UINT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_QUERYOPEN=>{
                if let Some(ref call) = self.on_query_open {
                    // BOOL OnQueryOpen()
                    // lResult = (LRESULT)func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_ERASEBKGND=>{
                if let Some(ref call) = self.on_erase_bkgnd {
                    // BOOL OnEraseBkgnd(CDCHandle dc)
                    // lResult = (LRESULT)func((HDC)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SYSCOLORCHANGE=>{
                if let Some(ref call) = self.on_sys_color_change {
                    // void OnSysColorChange()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_ENDSESSION=>{
                if let Some(ref call) = self.on_end_session {
                    // void OnEndSession(BOOL bEnding, UINT uLogOff)
                    // func((BOOL)wParam, (UINT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SHOWWINDOW=>{
                if let Some(ref call) = self.on_show_window {
                    // void OnShowWindow(BOOL bShow, UINT nStatus)
                    // func((BOOL)wParam, (int)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CTLCOLOREDIT=>{
                if let Some(ref call) = self.on_ctl_color_edit {
                    // HBRUSH OnCtlColorEdit(CDCHandle dc, CEdit edit)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CTLCOLORLISTBOX=>{
                if let Some(ref call) = self.on_ctl_color_list_box {
                    // HBRUSH OnCtlColorListBox(CDCHandle dc, CListBox listBox)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CTLCOLORBTN=>{
                if let Some(ref call) = self.on_ctl_color_btn {
                    // HBRUSH OnCtlColorBtn(CDCHandle dc, CButton button)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CTLCOLORDLG=>{
                if let Some(ref call) = self.on_ctl_color_dlg {
                    // HBRUSH OnCtlColorDlg(CDCHandle dc, CWindow wnd)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CTLCOLORSCROLLBAR=>{
                if let Some(ref call) = self.on_ctl_color_scroll_bar {
                    // HBRUSH OnCtlColorScrollBar(CDCHandle dc, CScrollBar scrollBar)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CTLCOLORSTATIC=>{
                if let Some(ref call) = self.on_ctl_color_static {
                    // HBRUSH OnCtlColorStatic(CDCHandle dc, CStatic wndStatic)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            // cc // void OnSettingChange(UINT uFlags, LPCTSTR lpszSection)
            // aa WM_SETTINGCHANGE
            // bb          func((UINT)wParam, (LPCTSTR)lParam);

            // cc // void OnDevModeChange(LPCTSTR lpDeviceName)
            // aa WM_DEVMODECHANGE
            // bb          func((LPCTSTR)lParam);

            WM_ACTIVATEAPP=>{
                if let Some(ref call) = self.on_activate_app {
                    // void OnActivateApp(BOOL bActive, DWORD dwThreadID)
                    // func((BOOL)wParam, (DWORD)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_FONTCHANGE=>{
                if let Some(ref call) = self.on_font_change {
                    // void OnFontChange()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_TIMECHANGE=>{
                if let Some(ref call) = self.on_time_change {
                    // void OnTimeChange()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CANCELMODE=>{
                if let Some(ref call) = self.on_cancel_mode {
                    // void OnCancelMode()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SETCURSOR=>{
                if let Some(ref call) = self.on_set_cursor {
                    // BOOL OnSetCursor(CWindow wnd, UINT nHitTest, UINT message)
                    // lResult = (LRESULT)func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MOUSEACTIVATE=>{
                if let Some(ref call) = self.on_mouse_activate {
                    // int OnMouseActivate(CWindow wndTopLevel, UINT nHitTest, UINT message)
                    // lResult = (LRESULT)func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CHILDACTIVATE=>{
                if let Some(ref call) = self.on_child_activate {
                    // void OnChildActivate()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_GETMINMAXINFO=>{
                if let Some(ref call) = self.on_get_min_max_info {
                    // void OnGetMinMaxInfo(LPMINMAXINFO lpMMI)
                    // func((LPMINMAXINFO)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_ICONERASEBKGND=>{
                if let Some(ref call) = self.on_icon_erase_bkgnd {
                    // void OnIconEraseBkgnd(CDCHandle dc)
                    // func((HDC)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SPOOLERSTATUS=>{
                if let Some(ref call) = self.on_spooler_status {
                    // void OnSpoolerStatus(UINT nStatus, UINT nJobs)
                    // func((UINT)wParam, (UINT)LOWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DRAWITEM=>{
                if let Some(ref call) = self.on_draw_item {
                    // void OnDrawItem(int nIDCtl, LPDRAWITEMSTRUCT lpDrawItemStruct)
                    // func((UINT)wParam, (LPDRAWITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MEASUREITEM=>{
                if let Some(ref call) = self.on_measure_item {
                    // void OnMeasureItem(int nIDCtl, LPMEASUREITEMSTRUCT lpMeasureItemStruct)
                    // func((UINT)wParam, (LPMEASUREITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DELETEITEM=>{
                if let Some(ref call) = self.on_delete_item {
                    // void OnDeleteItem(int nIDCtl, LPDELETEITEMSTRUCT lpDeleteItemStruct)
                    // func((UINT)wParam, (LPDELETEITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CHARTOITEM=>{
                if let Some(ref call) = self.on_char_to_item {
                    //int OnCharToItem(UINT nChar, UINT nIndex, CListBox listBox)
                    // lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_VKEYTOITEM=>{
                if let Some(ref call) = self.on_v_key_to_item {
                    // int OnVKeyToItem(UINT nKey, UINT nIndex, CListBox listBox)
                    // lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_QUERYDRAGICON=>{
                if let Some(ref call) = self.on_query_drag_icon {
                    // HCURSOR OnQueryDragIcon()
                    // lResult = (LRESULT)func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_COMPAREITEM=>{
                if let Some(ref call) = self.on_compare_item {
                    // int OnCompareItem(int nIDCtl, LPCOMPAREITEMSTRUCT lpCompareItemStruct)
                    // lResult = (LRESULT)func((UINT)wParam, (LPCOMPAREITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_COMPACTING=>{
                if let Some(ref call) = self.on_compacting {
                    // void OnCompacting(UINT nCpuTime)
                    // func((UINT)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCCREATE=>{
                if let Some(ref call) = self.on_nc_create {
                    // BOOL OnNcCreate(LPCREATESTRUCT lpCreateStruct)
                    // lResult = (LRESULT)func((LPCREATESTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCDESTROY=>{
                if let Some(ref call) = self.on_nc_destroy {
                    // void OnNcDestroy()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCCALCSIZE=>{
                if let Some(ref call) = self.on_nc_calc_size {
                    // LRESULT OnNcCalcSize(BOOL bCalcValidRects, LPARAM lParam)
                    // lResult = func((BOOL)wParam, lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCHITTEST=>{
                if let Some(ref call) = self.on_nc_hit_test {
                    // UINT OnNcHitTest(CPoint point)
                    // lResult = (LRESULT)func(_WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCPAINT=>{
                if let Some(ref call) = self.on_nc_paint {
                    // void OnNcPaint(CRgnHandle rgn)
                    // func((HRGN)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCACTIVATE=>{
                if let Some(ref call) = self.on_nc_activate {
                    // BOOL OnNcActivate(BOOL bActive)
                    // lResult = (LRESULT)func((BOOL)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_GETDLGCODE=>{
                if let Some(ref call) = self.on_get_dlg_code {
                    // UINT OnGetDlgCode(LPMSG lpMsg)
                    // lResult = (LRESULT)func((LPMSG)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCMOUSEMOVE=>{
                if let Some(ref call) = self.on_nc_mouse_move {
                    // void OnNcMouseMove(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCLBUTTONDOWN=>{
                if let Some(ref call) = self.on_nc_l_button_down {
                    // void OnNcLButtonDown(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCLBUTTONUP=>{
                if let Some(ref call) = self.on_nc_l_button_up {
                    // void OnNcLButtonUp(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCLBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_nc_l_button_db_clk {
                    // void OnNcLButtonDblClk(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCRBUTTONDOWN=>{
                if let Some(ref call) = self.on_nc_r_button_down {
                    // void OnNcRButtonDown(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCRBUTTONUP=>{
                if let Some(ref call) = self.on_nc_r_button_up {
                    // void OnNcRButtonUp(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCRBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_nc_r_button_dbl_clk {
                    // void OnNcRButtonDblClk(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCMBUTTONDOWN=>{
                if let Some(ref call) = self.on_nc_m_button_down {
                    // void OnNcMButtonDown(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCMBUTTONUP=>{
                if let Some(ref call) = self.on_nc_m_button_up {
                    // void OnNcMButtonUp(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCMBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_nc_m_button_dbl_clk {
                    // void OnNcMButtonDblClk(UINT nHitTest, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_KEYDOWN=>{
                if let Some(ref call) = self.on_key_down {
                    // void OnKeyDown(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_KEYUP=>{
                if let Some(ref call) = self.on_key_up {
                    // void OnKeyUp(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CHAR=>{
                if let Some(ref call) = self.on_char {
                    // void OnChar(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DEADCHAR=>{
                if let Some(ref call) = self.on_dead_char {
                    // void OnDeadChar(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SYSKEYDOWN=>{
                if let Some(ref call) = self.on_sys_key_down {
                    // void OnSysKeyDown(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SYSKEYUP=>{
                if let Some(ref call) = self.on_sys_key_up {
                    // void OnSysKeyUp(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SYSCHAR=>{
                if let Some(ref call) = self.on_sys_char {
                    // void OnSysChar(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SYSDEADCHAR=>{
                if let Some(ref call) = self.on_sys_dead_char {
                    // void OnSysDeadChar(UINT nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SYSCOMMAND=>{
                if let Some(ref call) = self.on_sys_command {
                    // void OnSysCommand(UINT nID, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_TCARD=>{
                if let Some(ref call) = self.on_t_card {
                    // void OnTCard(UINT idAction, DWORD dwActionData)
                    // func((UINT)wParam, (DWORD)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_TIMER=>{
                if let Some(ref call) = self.on_timer {
                    // void OnTimer(UINT_PTR nIDEvent)
                    // func((UINT_PTR)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_HSCROLL=>{
                if let Some(ref call) = self.on_h_scroll {
                    // void OnHScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
                    // func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_VSCROLL=>{
                if let Some(ref call) = self.on_v_scroll {
                    // void OnVScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
                    // func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_INITMENU=>{
                if let Some(ref call) = self.on_init_menu {
                    // void OnInitMenu(CMenuHandle menu)
                    // func((HMENU)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_INITMENUPOPUP=>{
                if let Some(ref call) = self.on_init_menu_popup {
                    // void OnInitMenuPopup(CMenuHandle menuPopup, UINT nIndex, BOOL bSysMenu)
                    // func((HMENU)wParam, (UINT)LOWORD(lParam), (BOOL)HIWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MENUSELECT=>{
                if let Some(ref call) = self.on_menu_select {
                    // void OnMenuSelect(UINT nItemID, UINT nFlags, CMenuHandle menu)
                    // func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HMENU)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MENUCHAR=>{
                if let Some(ref call) = self.on_menu_char {
                    // LRESULT OnMenuChar(UINT nChar, UINT nFlags, CMenuHandle menu)
                    // lResult = func((TCHAR)LOWORD(wParam), (UINT)HIWORD(wParam), (HMENU)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NOTIFY=>{
                if let Some(ref call) = self.on_notify {
                    // LRESULT OnNotify(int idCtrl, LPNMHDR pnmh)
                    // lResult = func((int)wParam, (LPNMHDR)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_ENTERIDLE=>{
                if let Some(ref call) = self.on_enter_idle {
                    // void OnEnterIdle(UINT nWhy, CWindow wndWho)
                    // func((UINT)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MOUSEMOVE=>{
                if let Some(ref call) = self.on_mouse_move {
                    // void OnMouseMove(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MOUSEWHEEL=>{
                if let Some(ref call) = self.on_mouse_wheel {
                    // BOOL OnMouseWheel(UINT nFlags, short zDelta, CPoint pt)
                    // lResult = (LRESULT)func((UINT)LOWORD(wParam), (short)HIWORD(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_LBUTTONDOWN=>{
                if let Some(ref call) = self.on_l_button_down {
                    // void OnLButtonDown(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_LBUTTONUP=>{
                if let Some(ref call) = self.on_l_button_up {
                    // void OnLButtonUp(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_LBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_l_button_dbl_clk {
                    // void OnLButtonDblClk(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_RBUTTONDOWN=>{
                if let Some(ref call) = self.on_r_button_down {
                    // void OnRButtonDown(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_RBUTTONUP=>{
                if let Some(ref call) = self.on_r_button_up {
                    // void OnRButtonUp(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_RBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_r_button_dbl_clk {
                    // void OnRButtonDblClk(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MBUTTONDOWN=>{
                if let Some(ref call) = self.on_m_button_down {
                    // void OnMButtonDown(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MBUTTONUP=>{
                if let Some(ref call) = self.on_m_button_up {
                    // void OnMButtonUp(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_m_button_dbl_clk {
                    // void OnMButtonDblClk(UINT nFlags, CPoint point)
                    // func((UINT)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_PARENTNOTIFY=>{
                if let Some(ref call) = self.on_parent_notify {
                    // void OnParentNotify(UINT message, UINT nChildID, LPARAM lParam)
                    // func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MDIACTIVATE=>{
                if let Some(ref call) = self.on_mdi_activate {
                    // void OnMDIActivate(CWindow wndActivate, CWindow wndDeactivate)
                    // func((HWND)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_RENDERFORMAT=>{
                if let Some(ref call) = self.on_render_format {
                    // void OnRenderFormat(UINT nFormat)
                    // func((UINT)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_RENDERALLFORMATS=>{
                if let Some(ref call) = self.on_render_all_formats {
                    // void OnRenderAllFormats()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DESTROYCLIPBOARD=>{
                if let Some(ref call) = self.on_destroy_clipboard {
                    // void OnDestroyClipboard()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DRAWCLIPBOARD=>{
                if let Some(ref call) = self.on_draw_clipboard {
                    // void OnDrawClipboard()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_PAINTCLIPBOARD=>{
                if let Some(ref call) = self.on_paint_clipboard {
                    // void OnPaintClipboard(CWindow wndViewer, const LPPAINTSTRUCT lpPaintStruct)
                    // func((HWND)wParam, (const LPPAINTSTRUCT)::GlobalLock((HGLOBAL)lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_VSCROLLCLIPBOARD=>{
                if let Some(ref call) = self.on_v_scroll_clipboard {
                    // void OnVScrollClipboard(CWindow wndViewer, UINT nSBCode, UINT nPos)
                    // func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CONTEXTMENU=>{
                if let Some(ref call) = self.on_context_menu {
                    // void OnContextMenu(CWindow wnd, CPoint point)
                    // func((HWND)wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SIZECLIPBOARD=>{
                if let Some(ref call) = self.on_size_clipboard {
                    // void OnSizeClipboard(CWindow wndViewer, const LPRECT lpRect)
                    // func((HWND)wParam, (const LPRECT)::GlobalLock((HGLOBAL)lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            // cc // void OnAskCbFormatName(UINT nMaxCount, LPTSTR lpszString)
            // aa WM_ASKCBFORMATNAME
            // bb          func((UINT)wParam, (LPTSTR)lParam);

            WM_CHANGECBCHAIN=>{
                if let Some(ref call) = self.on_change_cb_chain {
                    // void OnChangeCbChain(CWindow wndRemove, CWindow wndAfter)
                    // func((HWND)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_HSCROLLCLIPBOARD=>{
                if let Some(ref call) = self.on_h_scroll_clipboard {
                    // void OnHScrollClipboard(CWindow wndViewer, UINT nSBCode, UINT nPos)
                    // func((HWND)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_QUERYNEWPALETTE=>{
                if let Some(ref call) = self.on_query_new_palette {
                    // BOOL OnQueryNewPalette()
                    // lResult = (LRESULT)func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_PALETTECHANGED=>{
                if let Some(ref call) = self.on_palette_changed {
                    // void OnPaletteChanged(CWindow wndFocus)
                    // func((HWND)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_PALETTEISCHANGING=>{
                if let Some(ref call) = self.on_palette_is_changing {
                    // void OnPaletteIsChanging(CWindow wndPalChg)
                    // func((HWND)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DROPFILES=>{
                if let Some(ref call) = self.on_drop_files {
                    // void OnDropFiles(HDROP hDropInfo)
                    // func((HDROP)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_WINDOWPOSCHANGING=>{
                if let Some(ref call) = self.on_window_pos_changing {
                    // void OnWindowPosChanging(LPWINDOWPOS lpWndPos)
                    // func((LPWINDOWPOS)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_WINDOWPOSCHANGED=>{
                if let Some(ref call) = self.on_window_pos_changed {
                    // void OnWindowPosChanged(LPWINDOWPOS lpWndPos)
                    // func((LPWINDOWPOS)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_EXITMENULOOP=>{
                if let Some(ref call) = self.on_exit_menu_loop {
                    // void OnExitMenuLoop(BOOL fIsTrackPopupMenu)
                    // func((BOOL)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_ENTERMENULOOP=>{
                if let Some(ref call) = self.on_enter_menu_loop {
                    // void OnEnterMenuLoop(BOOL fIsTrackPopupMenu)
                    // func((BOOL)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_STYLECHANGED=>{
                if let Some(ref call) = self.on_style_changed {
                    // void OnStyleChanged(int nStyleType, LPSTYLESTRUCT lpStyleStruct)
                    // func((UINT)wParam, (LPSTYLESTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_STYLECHANGING=>{
                if let Some(ref call) = self.on_sytle_changing {
                    // void OnStyleChanging(int nStyleType, LPSTYLESTRUCT lpStyleStruct)
                    // func((UINT)wParam, (LPSTYLESTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SIZING=>{
                if let Some(ref call) = self.on_sizing {
                    // void OnSizing(UINT fwSide, LPRECT pRect)
                    // func((UINT)wParam, (LPRECT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MOVING=>{
                if let Some(ref call) = self.on_moving {
                    // void OnMoving(UINT fwSide, LPRECT pRect)
                    // func((UINT)wParam, (LPRECT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CAPTURECHANGED=>{
                if let Some(ref call) = self.on_capture_changed {
                    // void OnCaptureChanged(CWindow wnd)
                    // func((HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DEVICECHANGE=>{
                if let Some(ref call) = self.on_device_change {
                    // BOOL OnDeviceChange(UINT nEventType, DWORD_PTR dwData)
                    // lResult = (LRESULT)func((UINT)wParam, (DWORD_PTR)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_COMMAND=>{
                if let Some(ref call) = self.on_command {
                    // void OnCommand(UINT uNotifyCode, int nID, CWindow wndCtl)
                    // func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_DISPLAYCHANGE=>{
                if let Some(ref call) = self.on_display_change {
                    // void OnDisplayChange(UINT uBitsPerPixel, CSize sizeScreen)
                    // func((UINT)wParam, _WTYPES_NS::CSize(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_ENTERSIZEMOVE=>{
                if let Some(ref call) = self.on_enter_size_move {
                    // void OnEnterSizeMove()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_EXITSIZEMOVE=>{
                if let Some(ref call) = self.on_exit_size_move {
                    // void OnExitSizeMove()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_GETFONT=>{
                if let Some(ref call) = self.on_get_font {
                    // HFONT OnGetFont()
                    // lResult = (LRESULT)func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_GETHOTKEY=>{
                if let Some(ref call) = self.on_get_hot_key {
                    // LRESULT OnGetHotKey()
                    // lResult = func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_GETICON=>{
                if let Some(ref call) = self.on_get_icon {
                    // HICON OnGetIcon()
                    // lResult = (LRESULT)func((UINT)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            // cc // int OnGetText(int cchTextMax, LPTSTR lpszText)
            // aa WM_GETTEXT
            // bb          lResult = (LRESULT)func((int)wParam, (LPTSTR)lParam);
            WM_GETTEXTLENGTH=>{
                if let Some(ref call) = self.on_get_text_length {
                    // int OnGetTextLength()
                    // lResult = (LRESULT)func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_HELP=>{
                if let Some(ref call) = self.on_help {
                    // void OnHelp(LPHELPINFO lpHelpInfo)
                    // func((LPHELPINFO)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_HOTKEY=>{
                if let Some(ref call) = self.on_hot_key {
                    // void OnHotKey(int nHotKeyID, UINT uModifiers, UINT uVirtKey)
                    // func((int)wParam, (UINT)LOWORD(lParam), (UINT)HIWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_INPUTLANGCHANGE=>{
                if let Some(ref call) = self.on_input_lang_change {
                    // void OnInputLangChange(DWORD dwCharSet, HKL hKbdLayout)
                    // func((DWORD)wParam, (HKL)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_INPUTLANGCHANGEREQUEST=>{
                if let Some(ref call) = self.on_input_lang_change_request {
                    // void OnInputLangChangeRequest(BOOL bSysCharSet, HKL hKbdLayout)
                    // func((BOOL)wParam, (HKL)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NEXTDLGCTL=>{
                if let Some(ref call) = self.on_next_dlg_ctl {
                    // void OnNextDlgCtl(BOOL bHandle, WPARAM wCtlFocus)
                    // func((BOOL)LOWORD(lParam), wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NEXTMENU=>{
                if let Some(ref call) = self.on_next_menu {
                    // void OnNextMenu(int nVirtKey, LPMDINEXTMENU lpMdiNextMenu)
                    // func((int)wParam, (LPMDINEXTMENU)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NOTIFYFORMAT=>{
                if let Some(ref call) = self.on_notify_format {
                    // int OnNotifyFormat(CWindow wndFrom, int nCommand)
                    // lResult = (LRESULT)func((HWND)wParam, (int)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_POWERBROADCAST=>{
                if let Some(ref call) = self.on_power_broadcast {
                    // BOOL OnPowerBroadcast(DWORD dwPowerEvent, DWORD_PTR dwData)
                    // lResult = (LRESULT)func((DWORD)wParam, (DWORD_PTR)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_PRINT=>{
                if let Some(ref call) = self.on_print {
                    // void OnPrint(CDCHandle dc, UINT uFlags)
                    // func((HDC)wParam, (UINT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_PRINTCLIENT=>{
                if let Some(ref call) = self.on_print_client {
                    // void OnPrintClient(CDCHandle dc, UINT uFlags)
                    // func((HDC)wParam, (UINT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_RASDIALEVENT=>{
                if let Some(ref call) = self.on_ras_dial_event {
                    // void OnRasDialEvent(RASCONNSTATE rasconnstate, DWORD dwError)
                    // func((RASCONNSTATE)wParam, (DWORD)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SETFONT=>{
                if let Some(ref call) = self.on_set_font {
                    // void OnSetFont(CFontHandle font, BOOL bRedraw)
                    // func((HFONT)wParam, (BOOL)LOWORD(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SETHOTKEY=>{
                if let Some(ref call) = self.on_set_hot_key {
                    // int OnSetHotKey(int nVirtKey, UINT uFlags)
                    // lResult = (LRESULT)func((int)LOBYTE(LOWORD(wParam)), (UINT)HIBYTE(LOWORD(wParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SETICON=>{
                if let Some(ref call) = self.on_set_icon {
                    // HICON OnSetIcon(UINT uType, HICON hIcon)
                    // lResult = (LRESULT)func((UINT)wParam, (HICON)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_SETREDRAW=>{
                if let Some(ref call) = self.on_set_redraw {
                    // void OnSetRedraw(BOOL bRedraw)
                    // func((BOOL)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            // cc // int OnSetText(LPCTSTR lpstrText)
            // aa WM_SETTEXT
            // bb          lResult = (LRESULT)func((LPCTSTR)lParam);
            WM_USERCHANGED=>{
                if let Some(ref call) = self.on_user_changed {
                    // void OnUserChanged()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MOUSEHOVER=>{
                if let Some(ref call) = self.on_mouser_hove {
                    // void OnMouseHover(WPARAM wParam, CPoint ptPos)
                    // func(wParam, _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MOUSELEAVE=>{
                if let Some(ref call) = self.on_mouse_leave {
                    // void OnMouseLeave()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MENURBUTTONUP=>{
                if let Some(ref call) = self.on_menu_r_button_up {
                    // void OnMenuRButtonUp(WPARAM wParam, CMenuHandle menu)
                    // func(wParam, (HMENU)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MENUDRAG=>{
                if let Some(ref call) = self.on_menu_drag {
                    // LRESULT OnMenuDrag(WPARAM wParam, CMenuHandle menu)
                    // lResult = func(wParam, (HMENU)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MENUGETOBJECT=>{
                if let Some(ref call) = self.on_menu_get_object {
                    // LRESULT OnMenuGetObject(PMENUGETOBJECTINFO info)
                    // lResult = func((PMENUGETOBJECTINFO)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_UNINITMENUPOPUP=>{
                if let Some(ref call) = self.on_un_init_menu_popup {
                    // void OnUnInitMenuPopup(UINT nID, CMenuHandle menu)
                    // func((UINT)HIWORD(lParam), (HMENU)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MENUCOMMAND=>{
                if let Some(ref call) = self.on_menu_command {
                    // void OnMenuCommand(WPARAM nIndex, CMenuHandle menu)
                    // func(wParam, (HMENU)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_APPCOMMAND=>{
                if let Some(ref call) = self.on_app_command {
                    // BOOL OnAppCommand(CWindow wndFocus, short cmd, WORD uDevice, int dwKeys)
                    // lResult = (LRESULT)func((HWND)wParam, GET_APPCOMMAND_LPARAM(lParam), GET_DEVICE_LPARAM(lParam), GET_KEYSTATE_LPARAM(lParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCXBUTTONDOWN=>{
                if let Some(ref call) = self.on_ncx_button_down {
                    // void OnNCXButtonDown(int fwButton, short nHittest, CPoint ptPos)
                    // func(GET_XBUTTON_WPARAM(wParam), GET_NCHITTEST_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCXBUTTONUP=>{
                if let Some(ref call) = self.on_ncx_button_up {
                    // void OnNCXButtonUp(int fwButton, short nHittest, CPoint ptPos)
                    // func(GET_XBUTTON_WPARAM(wParam), GET_NCHITTEST_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_NCXBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_ncx_button_dbl_clk {
                    // void OnNCXButtonDblClk(int fwButton, short nHittest, CPoint ptPos)
                    // func(GET_XBUTTON_WPARAM(wParam), GET_NCHITTEST_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_XBUTTONDOWN=>{
                if let Some(ref call) = self.on_x_button_down {
                    // void OnXButtonDown(int fwButton, int dwKeys, CPoint ptPos)
                    // func(GET_XBUTTON_WPARAM(wParam), GET_KEYSTATE_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_XBUTTONUP=>{
                if let Some(ref call) = self.on_x_button_up {
                    // void OnXButtonUp(int fwButton, int dwKeys, CPoint ptPos)
                    // func(GET_XBUTTON_WPARAM(wParam), GET_KEYSTATE_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_XBUTTONDBLCLK=>{
                if let Some(ref call) = self.on_x_button_dbl_clk {
                    // void OnXButtonDblClk(int fwButton, int dwKeys, CPoint ptPos)
                    // func(GET_XBUTTON_WPARAM(wParam), GET_KEYSTATE_WPARAM(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_CHANGEUISTATE=>{
                if let Some(ref call) = self.on_change_ui_state {
                    // void OnChangeUIState(WORD nAction, WORD nState)
                    // func(LOWORD(wParam), HIWORD(wParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_UPDATEUISTATE=>{
                if let Some(ref call) = self.on_update_ui_state {
                    // void OnUpdateUIState(WORD nAction, WORD nState)
                    // func(LOWORD(wParam), HIWORD(wParam));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_QUERYUISTATE=>{
                if let Some(ref call) = self.on_query_ui_state {
                    // LRESULT OnQueryUIState()
                    // lResult = func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_INPUT=>{
                if let Some(ref call) = self.on_input {
                    // void OnInput(WPARAM RawInputCode, HRAWINPUT hRawInput)
                    // func(GET_RAWINPUT_CODE_WPARAM(wParam), (HRAWINPUT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_UNICHAR=>{
                if let Some(ref call) = self.on_uni_char {
                    // void OnUniChar(TCHAR nChar, UINT nRepCnt, UINT nFlags)
                    // func((TCHAR)wParam, (UINT)lParam & 0xFFFF, (UINT)((lParam & 0xFFFF0000) >> 16));
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_WTSSESSION_CHANGE=>{
                if let Some(ref call) = self.on_wt_session_change {
                    // void OnWTSSessionChange(WPARAM nStatusCode, PWTSSESSION_NOTIFICATION nSessionID)
                    // func(wParam, (PWTSSESSION_NOTIFICATION)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_THEMECHANGED=>{
                if let Some(ref call) = self.on_theme_changed {
                    // void OnThemeChanged()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            WM_MOUSEHWHEEL=>{
                if let Some(ref call) = self.on_mouse_wheel {
                    // BOOL OnMouseHWheel(UINT nFlags, short zDelta, CPoint pt)
                    // lResult = (LRESULT)func((UINT)LOWORD(wParam), (short)HIWORD(wParam), _WTYPES_NS::CPoint(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)));
                    call (t,);
                    *bHandled = TRUE;
                }
            },

            ///////////////////////////////////////////////////////////////////////////////
            // ATL defined messages
            WM_FORWARDMSG=>{
                if let Some(ref call) = self.on_forward_msg {
                    // BOOL OnForwardMsg(LPMSG Msg, DWORD nUserData)
                    // lResult = (LRESULT)func((LPMSG)lParam, (DWORD)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            ///////////////////////////////////////////////////////////////////////////////
            // Dialog specific messages
            DM_GETDEFID=>{
                if let Some(ref call) = self.on_dm_get_def_id {
                    // LRESULT OnDMGetDefID()
                    // lResult = func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            DM_SETDEFID=>{
                if let Some(ref call) = self.on_dm_set_def_id {
                    // void OnDMSetDefID(UINT DefID)
                    // func((UINT)wParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            DM_REPOSITION=>{
                if let Some(ref call) = self.on_dm_reposition {
                    // void OnDMReposition()
                    // func();
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            ///////////////////////////////////////////////////////////////////////////////
            // Reflected messages
            OCM_COMMAND=>{
                if let Some(ref call) = self.on_reflected_command {
                    // void OnReflectedCommand(UINT uNotifyCode, int nID, CWindow wndCtl)
                    // func((UINT)HIWORD(wParam), (int)LOWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_NOTIFY=>{
                if let Some(ref call) = self.on_reflected_notify {
                    // LRESULT OnReflectedNotify(int idCtrl, LPNMHDR pnmh)
                    // lResult = func((int)wParam, (LPNMHDR)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_PARENTNOTIFY=>{
                if let Some(ref call) = self.on_reflected_parent_notify {
                    // void OnReflectedParentNotify(UINT message, UINT nChildID, LPARAM lParam)
                    // func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_DRAWITEM=>{
                if let Some(ref call) = self.on_reflected_draw_item {
                    // void OnReflectedDrawItem(int nIDCtl, LPDRAWITEMSTRUCT lpDrawItemStruct)
                    // func((UINT)wParam, (LPDRAWITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_MEASUREITEM=>{
                if let Some(ref call) = self.on_reflected_measure_item {
                    // void OnReflectedMeasureItem(int nIDCtl, LPMEASUREITEMSTRUCT lpMeasureItemStruct)
                    // func((UINT)wParam, (LPMEASUREITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_COMPAREITEM=>{
                if let Some(ref call) = self.on_reflected_compare_item {
                    // int OnReflectedCompareItem(int nIDCtl, LPCOMPAREITEMSTRUCT lpCompareItemStruct)
                    // lResult = (LRESULT)func((UINT)wParam, (LPCOMPAREITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_DELETEITEM=>{
                if let Some(ref call) = self.on_reflected_delete_item {
                    // void OnReflectedDeleteItem(int nIDCtl, LPDELETEITEMSTRUCT lpDeleteItemStruct)
                    // func((UINT)wParam, (LPDELETEITEMSTRUCT)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
  
            OCM_VKEYTOITEM=>{
                if let Some(ref call) = self.on_refelected_v_key_to_item {
                    // int OnReflectedVKeyToItem(UINT nKey, UINT nIndex, CListBox listBox)
                    // lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_CHARTOITEM=>{
                if let Some(ref call) = self.on_reflected_char_to_item {
                    //int OnReflectedCharToItem(UINT nChar, UINT nIndex, CListBox listBox)
                    // lResult = (LRESULT)func((UINT)LOWORD(wParam), (UINT)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_HSCROLL=>{
                if let Some(ref call) = self.on_reflected_h_scroll {
                    // void OnReflectedHScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
                    // func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_VSCROLL=>{
                if let Some(ref call) = self.on_refelected_v_scroll {
                    // void OnReflectedVScroll(UINT nSBCode, UINT nPos, CScrollBar pScrollBar)
                    // func((int)LOWORD(wParam), (short)HIWORD(wParam), (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_CTLCOLOREDIT=>{
                if let Some(ref call) = self.on_reflected_ctl_color_edit {
                    // HBRUSH OnReflectedCtlColorEdit(CDCHandle dc, CEdit edit)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_CTLCOLORLISTBOX=>{
                if let Some(ref call) = self.on_reflected_ctl_color_list_box {
                    // HBRUSH OnReflectedCtlColorListBox(CDCHandle dc, CListBox listBox)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_CTLCOLORBTN=>{
                if let Some(ref call) = self.on_reflected_ctl_color_btn {
                    // HBRUSH OnReflectedCtlColorBtn(CDCHandle dc, CButton button)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_CTLCOLORDLG=>{
                if let Some(ref call) = self.on_reflected_ctl_color_dlg {
                    // HBRUSH OnReflectedCtlColorDlg(CDCHandle dc, CWindow wnd)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_CTLCOLORSCROLLBAR=>{
                if let Some(ref call) = self.on_reflected_ctl_color_scroll_bar {
                    // HBRUSH OnReflectedCtlColorScrollBar(CDCHandle dc, CScrollBar scrollBar)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            OCM_CTLCOLORSTATIC=>{
                if let Some(ref call) = self.on_reflected_ctl_color_static {
                    // HBRUSH OnReflectedCtlColorStatic(CDCHandle dc, CStatic wndStatic)
                    // lResult = (LRESULT)func((HDC)wParam, (HWND)lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
            msg=>{
                if let Some(ref call) = self.on_message_handler_ex {
                    // LRESULT OnMessageHandlerEX(UINT uMsg, WPARAM wParam, LPARAM lParam)
                    // lResult = func(uMsg, wParam, lParam);
                    call (t,);
                    *bHandled = TRUE;
                }
            },
*/
            // cc // LRESULT OnMessageRangeHandlerEX(UINT uMsg, WPARAM wParam, LPARAM lParam)
            // bb          lResult = func(uMsg, wParam, lParam);
            // dd on_message_range_handler_ex

            _=>{

            },
        }
        lResult
    }   
}