#![allow(non_snake_case,dead_code,unused_variables,unused_assignments)]

use std::{self, ptr};
use winapi::*;
use user32;
use kernel32;

use std::rc::Rc;
use std::cmp::Ordering;
use std::fmt;

use super::super::thunk;
use super::super::cwindow::*;
use super::consts::*;
use super::{Event,DlgMsg};

use ctrls::BtnMsg;

type DLGPROC2 = unsafe extern "system" fn(HWND, u32, u64, i64) -> i64;

pub struct Dialog<T>{
    cwin: CWindow, // basic operations for objects that have HWND
    thk: &'static mut thunk::Thunk, // thunk that convert static function call to
    idd: WORD, // resource id of the dlg
    state: DWORD, // destroy or not
    modal: bool, // is modal dialog

    // T can't be mut borrowed more than once,so dialog save raw pointer,
    // when all gui run in one thread this is safe
    root:*mut T, //raw pointer to the Root Dialogs
    //messages
    bin_search_cnt:u32,         //used for combine search,search step cnt for bin search
    pub handlers: Vec<Handler<T>>,
}

impl<T> fmt::Display for Dialog<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hwnd:0x{:x}", self.cwin.GetHwnd() as usize)
    }
}

fn MAKEINTRESOURCEW(id: WORD) -> LPCWSTR {
    id as usize as LPCWSTR
}

//frequently used
impl<T> Dialog<T> {
    fn InitThunk(&mut self, h: HWND) -> DLGPROC {
        let pself = self as *mut _ as *mut c_void;
        self.thk.init(Self::DialogProc as DWORD_PTR, pself);
        self.cwin.Attach(h);
        let p = self.thk.GetCodeAddress();
        unsafe {
            std::mem::transmute(p)
        }
    }

    //user can pass a dlg_proc to override the default DLGPROC of CDialogImpl,and take over every msg your self
    pub fn new(idd: WORD) -> Dialog<T> {
        Dialog {
            cwin: CWindow::new(NULL_HWND),
            thk: thunk::get_thunk(),
            idd: idd,
            state: 0,
            modal: false,
            root: 0 as *mut T,
            bin_search_cnt:0,
            handlers:vec![Handler::new(0xFFFF,0xFFFF, 0xFFFF , 0xFFFF, |_,_|{})],   //put a sentinel in the vec
        }
    }
}

//CDialogImplBaseT
impl<T> Dialog<T> {
    unsafe extern "system" fn StartDialogProc(hWnd: HWND,
                                              uMsg: UINT,
                                              wParam: WPARAM,
                                              lParam: LPARAM)
                                              -> INT_PTR {
        let p_this = thunk::get_this();
        //println!("4. get this:{:p}", p_this);
        let pself = p_this as *mut Self;
        //println!("5. start dialog proc,addr:0x{:x},DialogProc:0x{:x}",Self::StartDialogProc as usize,Self::DialogProc as usize);
        //println!("6. proc_msg before init thunk:0x{:x}", Self::DialogProc as usize);
        let dlg_proc_thunk = Self::InitThunk(&mut *pself, hWnd);

        let pthunk = dlg_proc_thunk.unwrap();

        //println!("7. start proc,thunk addr:0x{:x}", proc_msg as usize);
        // handler must be sorted here:before any message been processed
        Self::sort_handlers(&mut *pself);


        //DWLP_DLGPROC = sizeof(LRESULT) + DWLP_MSGRESULT
        user32::SetWindowLongPtrW(hWnd,
                                  (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int,
                                  pthunk as LONG_PTR);

        //it is actually the entry of the thunk
        pthunk(hWnd, uMsg, wParam, lParam)
    }

    //if bHandled return TRUE
    unsafe extern "system" fn DialogProc(hWnd: HWND,
                                         uMsg: UINT,
                                         wParam: WPARAM,
                                         lParam: LPARAM)
                                         -> INT_PTR {
        
        let mut_self = &mut *(hWnd as *mut Self);

        let mut lRes: LRESULT = 0;
        let h = mut_self.GetHwnd();
        let mut bRet = Self::ProcessWindowMessage(mut_self,h,uMsg,wParam,lParam,&mut lRes,0);//unsafe{};

        if bRet == TRUE {
            match uMsg {
                WM_COMPAREITEM |
                WM_VKEYTOITEM |
                WM_CHARTOITEM |
                WM_INITDIALOG |
                WM_QUERYDRAGICON |
                WM_CTLCOLORMSGBOX |
                WM_CTLCOLOREDIT |
                WM_CTLCOLORLISTBOX |
                WM_CTLCOLORBTN |
                WM_CTLCOLORDLG |
                WM_CTLCOLORSCROLLBAR |
                WM_CTLCOLORSTATIC => {
                    if lRes > 0 {
                        bRet = TRUE;
                    }
                }
                // return in DWL_MSGRESULT
                //Make sure the window was not destroyed before setting attributes.
                _ => {
                    if mut_self.state & WINSTATE_DESTROYED == 0 {
                        user32::SetWindowLongPtrW(mut_self.cwin.GetHwnd(),DWLP_MSGRESULT as c_int,lRes);
                    }
                }
            }
        } else if uMsg == WM_NCDESTROY {
            mut_self.state |= WINSTATE_DESTROYED;
        }

        if mut_self.state & WINSTATE_DESTROYED != 0 {
            let hWndThis = mut_self.cwin.Detach();
            mut_self.state &= !WINSTATE_DESTROYED;
            // clean up after dialog is destroyed
            //mut_self->OnFinalMessage(hWndThis);
        }
        bRet as INT_PTR
        //0
    }
}

impl<T> Dialog<T> {
    fn sort_handlers(&mut self) {
        //sort handlers for search algorithm
        self.handlers.sort_by(|f1,f2|{
            f1.cmp(&f2)
        });

        //calculate how many steps can bin search do
        self.bin_search_cnt = (self.handlers.len() as f32).log2() as u32;

        // bin_search_cnt - 4 will be a very big u32 value(equals max_u32 - bin_search_cnt) when bin_search_cnt < 4
        if self.bin_search_cnt > 4 {
            self.bin_search_cnt -= 4;
        }else{
            self.bin_search_cnt = 0;
        }

        // priority only used in sort algorithm
        for h in &mut self.handlers {
            h.priority = 0;
        }

        // for h in &self.handlers{
        //     println!("{}", h);
        // }
    }

    //messages
    pub fn ProcessWindowMessage(&mut self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL {
        let mut e = Event::new(hWnd,uMsg,wParam,lParam,lResult);
        
        // this should be called before TranslateMessage and DispathMessage
        // let mut i = 0;
        // // check if "PreTranslateMessage" exist
        // while self.handlers[i].key() == 0 {
        //     // call PreTranslateMessage
        //     unsafe{
        //         (self.handlers[i].call)(e,&mut *self.root);
        //     }

        //     // stop call PreTranslateMessage that has low priority,or the real message handlers
        //     if *lResult > 0{
        //         return TRUE;
        //     }

        //     i+=1;
        // }

        let k:HandleKey;
        match uMsg {
            WM_COMMAND=>{
                k = HandleKey::new(uMsg, LOWORD(wParam as DWORD), HIWORD(wParam as DWORD));
            },
            WM_NOTIFY=>{
                let p = unsafe{&*(lParam as LPNMHDR)};
                //id == ((LPNMHDR)lParam)->idFrom && cd == ((LPNMHDR)lParam)->code)
                // TODO:check if idFrom and code range is u16??
                k = HandleKey::new(uMsg, p.idFrom as u16,p.code as u16);
            },
            _=>{
                k = HandleKey::new_msg(uMsg);
            }
        }
        self.combine_search(k.key(),&mut e)
    }


    // https://en.wikipedia.org/wiki/Binary_search_algorithm
    // https://schani.wordpress.com/2010/04/30/linear-vs-binary-search/
    // according to the bench,the compiler already cmov optimized
    // return TRUE if message handled
    fn combine_search(&self,key:u64,e:&mut Event) -> BOOL {
        // bin search
        let mut left = 0;
        let mut right = self.handlers.len() - 1;
        let mut mid = 0;
        for i in(0..self.bin_search_cnt){
            mid = (left + right) >> 1;
            debug_assert!(mid < right);
            if self.handlers[mid].key() < key {
                left = mid + 1;
            }else{
                right = mid;
            }
        }

        // linear search ,we must put a sentinel at end
        let mut i = left;

        // find smallest key
        loop{
            if self.handlers[i].key() >= key {
                break;
            }
            i+=1;
        }

        let mut bFind = FALSE;
        // call
        while self.handlers[i].key() == key {
            unsafe{
                (self.handlers[i].call)(e,&mut *self.root);
            }
            bFind = TRUE;
            i+=1;
        }
        bFind
    }
}

//CDialogImpl
impl<T> Dialog<T> {
    pub fn DoModal2(&mut self,r:*mut T) {
        let hWndParent = unsafe {
            user32::GetActiveWindow()
        };
        self.DoModal(hWndParent, NULL_LPARAM,r);
    }

    pub fn DoModal(&mut self,hWndParent: HWND,dwInitParam: LPARAM,r:*mut T) -> INT_PTR {
        //ATLASSUME(m_hWnd == NULL);
        self.root = r;
        self.modal = true;
        //self.puser = puser;
        thunk::set_this(self as *mut Self as *mut c_void);

        unsafe {
            let hInst = kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE;
            let r = user32::DialogBoxParamW(hInst,
                                            MAKEINTRESOURCEW(self.idd),
                                            hWndParent,
                                            Some(Self::StartDialogProc),
                                            dwInitParam);
            //let e = kernel32::GetLastError();
            //println!("err:{}", e);
            r
        }
    }

    pub fn EndDialog(&self, nRetCode: c_int) -> BOOL {
        self.cwin.assert_window();
        assert!(self.modal);
        unsafe {
            user32::EndDialog(self.cwin.GetHwnd(), nRetCode as INT_PTR)
        }
    }

    // modeless dialogs
    pub fn Create3(&mut self,r:*mut T) ->HWND {
        self.Create(NULL_HWND, NULL_LPARAM,r)
    }

    pub fn Create2(&mut self, hWndParent: HWND,r:*mut T) -> HWND {
        self.Create(hWndParent, NULL_LPARAM,r)
    }

    pub fn Create(&mut self, hWndParent: HWND, dwInitParam: LPARAM,r:*mut T) -> HWND {
        //ATLASSUME(m_hWnd == NULL);
        self.root = r;
        thunk::set_this(self as *mut Self as *mut c_void);
        self.modal = false;

        unsafe {
            let hWnd = user32::CreateDialogParamW(0 as HINSTANCE,
                                                  MAKEINTRESOURCEW(self.idd),
                                                  hWndParent,
                                                  Some(Self::StartDialogProc),
                                                  dwInitParam);
            // let e = kernel32::GetLastError();
            // println!("err:{}", e);
            //self.cwin.Attach(hWnd);
            user32::ShowWindow(hWnd, SW_SHOW);
            //self.cwin.ShowWindow(SW_SHOW);
            //ATLASSUME(m_hWnd == hWnd);
            hWnd
        }
    }

    pub fn DestroyWindow(&mut self) -> BOOL {
        self.cwin.DestroyWindow()
        // self.cwin.assert_window();
        // assert!(self.modal == false);
        // unsafe {
        //     if user32::DestroyWindow(self.cwin.GetHwnd()) == FALSE {
        //         return FALSE;
        //     }
        // }
        // return TRUE;
    }
}

//CWindowImplRoot
impl<T> Dialog<T> {
    fn ForwardNotifications(&self,
                            uMsg: UINT,
                            wParam: WPARAM,
                            lParam: LPARAM,
                            bHandled: &mut BOOL)
                            -> LRESULT {
        let mut lResult: LRESULT = 0;
        match uMsg {
            WM_COMMAND |
            WM_NOTIFY |
            WM_PARENTNOTIFY |
            WM_DRAWITEM |
            WM_MEASUREITEM |
            WM_COMPAREITEM |
            WM_DELETEITEM |
            WM_VKEYTOITEM |
            WM_CHARTOITEM |
            WM_HSCROLL |
            WM_VSCROLL |
            WM_CTLCOLORBTN |
            WM_CTLCOLORDLG |
            WM_CTLCOLOREDIT |
            WM_CTLCOLORLISTBOX |
            WM_CTLCOLORMSGBOX |
            WM_CTLCOLORSCROLLBAR |
            WM_CTLCOLORSTATIC => {
                lResult = self.cwin.GetParent2().SendMessage(uMsg, wParam, lParam);
            }
            _ => *bHandled = FALSE,
        }
        lResult
    }

    fn ReflectNotifications(&self,
                            uMsg: UINT,
                            wParam: WPARAM,
                            lParam: LPARAM,
                            bHandled: &mut BOOL)
                            -> LRESULT {
        let mut hWndChild = NULL_HWND;
        unsafe {
            match uMsg {
                WM_COMMAND => {
                    if lParam != NULL_LPARAM {
                        hWndChild = lParam as HWND;
                    }
                }
                WM_NOTIFY => {
                    hWndChild = (*(lParam as LPNMHDR)).hwndFrom;
                }
                WM_PARENTNOTIFY => {
                    match LOWORD(wParam as DWORD) as DWORD {
                        WM_CREATE | WM_DESTROY => hWndChild = lParam as HWND,
                        _ => hWndChild =
                                 self.cwin.GetDlgItem2(HIWORD(wParam as DWORD) as c_int).GetHwnd(),
                    }
                }
                WM_DRAWITEM => {
                    if wParam != 0 {
                        hWndChild = (*(lParam as LPDRAWITEMSTRUCT)).hwndItem;
                    }
                }
                WM_MEASUREITEM => {
                    if wParam != 0 {
                        let id = (*(lParam as LPMEASUREITEMSTRUCT)).CtlID;
                        hWndChild = self.cwin.GetDlgItem2(id as c_int).GetHwnd();
                    }
                }
                WM_COMPAREITEM => {
                    if wParam != 0 {
                        hWndChild = (*(lParam as LPCOMPAREITEMSTRUCT)).hwndItem;
                    }
                }
                WM_DELETEITEM => {
                    if wParam != 0 {
                        hWndChild = (*(lParam as LPDELETEITEMSTRUCT)).hwndItem;
                    }
                }
                WM_VKEYTOITEM | WM_CHARTOITEM | WM_HSCROLL | WM_VSCROLL => hWndChild =
                                                                               lParam as HWND,
                WM_CTLCOLORBTN |
                WM_CTLCOLORDLG |
                WM_CTLCOLOREDIT |
                WM_CTLCOLORLISTBOX |
                WM_CTLCOLORMSGBOX |
                WM_CTLCOLORSCROLLBAR |
                WM_CTLCOLORSTATIC => hWndChild = lParam as HWND,
                _ => (),
            }

            if hWndChild == NULL_HWND {
                *bHandled = FALSE;
                return 1;
            }

            //ATLASSERT(::IsWindow(hWndChild));
            assert!(user32::IsWindow(hWndChild) == TRUE);
            user32::SendMessageW(hWndChild, OCM__BASE + uMsg, wParam, lParam)
        }
    }

    //static function
    fn DefaultReflectionHandler(hWnd: HWND,
                                uMsg: UINT,
                                wParam: WPARAM,
                                lParam: LPARAM,
                                lResult: &mut LRESULT)
                                -> BOOL {
        match uMsg {
            OCM_COMMAND |
            OCM_NOTIFY |
            OCM_PARENTNOTIFY |
            OCM_DRAWITEM |
            OCM_MEASUREITEM |
            OCM_COMPAREITEM |
            OCM_DELETEITEM |
            OCM_VKEYTOITEM |
            OCM_CHARTOITEM |
            OCM_HSCROLL |
            OCM_VSCROLL |
            OCM_CTLCOLORBTN |
            OCM_CTLCOLORDLG |
            OCM_CTLCOLOREDIT |
            OCM_CTLCOLORLISTBOX |
            OCM_CTLCOLORMSGBOX |
            OCM_CTLCOLORSCROLLBAR |
            OCM_CTLCOLORSTATIC => {
                unsafe {
                    *lResult = user32::DefWindowProcW(hWnd, uMsg - OCM__BASE, wParam, lParam);
                }
                return TRUE;
            }
            _ => (),
        }
        FALSE
    }

}

// all handlers
impl<T> Dialog<T> {
    pub fn msg_handler(&mut self)->DlgMsg<T>{
        DlgMsg::new(&mut self.handlers)
    }

    pub fn btn_handler(&mut self,id:WORD)->BtnMsg<T>{
        BtnMsg::new(id,&mut self.handlers)   
    }
}
////////////////////////////////////////////////////////////////////
// handler
// all structs who want to process the win message need to impl this
// and then called to put all the closure into Dialog.handlers
// all message process structs must live as long as MessageLoop
// all message process structs will be create and call the register function with MessageLoop as param
// dynamic generated code
// pub trait HandlerRegister {
//     fn register(&mut self,&MessageLoop);
// }

#[repr(C,packed)]
struct HandleKey {
    priority:u16,
    code:WORD,
    id  :WORD,
    msg :WORD,
}

impl HandleKey {
    #[inline(always)]
    fn new_msg(msg:UINT)->HandleKey{
        HandleKey{
            msg:msg as WORD,
            id:0,
            code:0,
            priority:0,
            //hwnd:h,
        }
    }

    #[inline(always)]
    fn new(msg:UINT,id:u16,code:u16)->HandleKey{
        HandleKey{
            msg:msg as WORD,
            id:id,
            code:code,
            priority:0,
            //hwnd:h,
        }
    }

    #[inline(always)]
    pub fn key(&self)->u64{
        unsafe{
            *(self as *const _ as *const u64)
        }
    }
}

/// Priority which is ranging from 0 ~ 65535. If more than one handler for the same message/command/notify,
/// then the wtl-rs will call them by priority,the smaller number the higher priority.
/// 0 ~ 99 been reserved for wtl-rs system,so user can use 100 ~ 65535
//this only use for sorting algorithm ,after that it will be set to zero for search algorithm
//so user can't set priority at runtime

/// the sort algorithm convert *self to u64, so only little endian machine can be sort correct
#[repr(C,packed)]
pub struct Handler<T> {
    priority:u16,
    code:WORD,
    id  :WORD,
    msg :WORD,
    
    call:Rc<Fn(&mut Event,&mut T)>,
}

impl<T> fmt::Display for Handler<T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handler:\nmsg:{}\nid:{}\ncode:{}\npriority:{}\n", self.msg,self.id,self.code,self.priority)
    }
}

impl<T> Handler<T> {
    #[inline(always)]
    pub fn new<F>(msg:UINT,id:u16,code:u16,priority:u16,f:F)->Handler<T> where F:Fn(&mut Event,&mut T) + 'static {
        Handler{
            msg:msg as WORD,
            id :id,
            code:code,
            priority:priority,
            call:Rc::new(f),
        }
    }

    #[inline(always)]
    fn key(&self)->u64{
        unsafe{
            *(self as *const _ as *const u64)
        }
    }

    // for sort
    fn cmp(&self,other:&Self)->Ordering {
        self.key().cmp(&other.key())
    }
}

/// only handle the priority setting method
pub struct HandlerPriority<'a,T:'a> {
    h: &'a mut Handler<T>
}

/// priority ranges from 0 ~ 65535
impl <'a,T> HandlerPriority<'a,T> {
    #[inline(always)]
    pub fn new(h:&'a mut Handler<T>)->HandlerPriority<'a,T>{
        HandlerPriority{
            h:h,
        }
    }

    /// low priority,p range from 0 ~ 32767(0x7FFF),and real priority range from 32768 ~ 65535
    pub fn set_user_priority(&mut self,p:u16){
        debug_assert!(p < 8000);
        self.h.priority = p + 8000;        //32767
    }

    /// high priority, p range from 0 ~ 32767,and real priority range from 0 ~ 32767
    pub fn set_system_priority(&mut self,p:u16){
        debug_assert!(p < 8000);
        self.h.priority = p;
    }
}
/////////////////////////////////////////////////////////
// expose all cwin methods

// currently racer not support macros,so add all functions manually
impl<T> Dialog<T> {
    #[inline(always)]
    pub fn GetHwnd(&self) -> HWND {
        self.cwin.GetHwnd()
    }

    #[inline(always)]
    pub fn Detach(&mut self)-> HWND {
        self.cwin.Detach()
    }

    #[inline(always)]
    pub fn Attach (&mut self,hWndNew:HWND){
        self.cwin.Attach(hWndNew)
    }

    #[inline(always)]
    pub fn assert_window(&self) {
        self.cwin.assert_window()
    }

    #[inline(always)]
    pub fn GetParent(&self) -> HWND {
        self.cwin.GetParent()
    }

    #[inline(always)]
    pub fn SetParent(&self,hWndNewParent:HWND) -> HWND {
        self.cwin.SetParent(hWndNewParent)
    }

    #[inline(always)]
    pub fn GetDlgItem(&self,nID:c_int) -> HWND {
        self.cwin.GetDlgItem(nID)
    }
    
    #[inline(always)]
    pub fn GetParent2 (&self) -> CWindow {
        self.cwin.GetParent2()
    }

    #[inline(always)]
    pub fn SetParent2 (&self,hWndNewParent:HWND) -> CWindow {
        self.cwin.SetParent2(hWndNewParent)
    }

    #[inline(always)]
    pub fn GetDlgItem2 (&self,nID:c_int) -> CWindow {
        self.cwin.GetDlgItem2(nID)
    }

    #[inline(always)]
    pub fn GetTopWindow (&self) -> CWindow {
        self.cwin.GetTopWindow()
    }

    #[inline(always)]
    pub fn GetWindow (&self,nCmd:UINT) -> CWindow {
        self.cwin.GetWindow(nCmd)
    }

    #[inline(always)]
    pub fn GetLastActivePopup (&self) -> CWindow {
        self.cwin.GetLastActivePopup()
    }

    #[inline(always)]
    pub fn ChildWindowFromPoint (&self,point:POINT) -> CWindow {
        self.cwin.ChildWindowFromPoint(point)
    }

    #[inline(always)]
    pub fn ChildWindowFromPointEx (&self,point:POINT,uFlags:UINT) -> CWindow {
        self.cwin.ChildWindowFromPointEx(point,uFlags)
    }

    #[inline(always)]
    pub fn GetNextDlgGroupItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
        self.cwin.GetNextDlgGroupItem(hWndCtl,bPrevious)
    }

    #[inline(always)]
    pub fn GetNextDlgTabItem (&self,hWndCtl:HWND,bPrevious:BOOL) -> CWindow {
        self.cwin.GetNextDlgTabItem(hWndCtl,bPrevious)
    }

    #[inline(always)]
    pub fn GetTopLevelParent (&self) -> CWindow {
        self.cwin.GetTopLevelParent()
    }

    #[inline(always)]
    pub fn GetTopLevelWindow (&self) -> CWindow {
        self.cwin.GetTopLevelWindow()
    }

    #[inline(always)]
    pub fn GetDescendantWindow (&self,nID:c_int) -> CWindow {
        self.cwin.GetDescendantWindow(nID)
    }

    // #[inline(always)]
    // pub fn Create(lpstrWndClass:LPCWSTR ,hWndParent:HWND ,rect:&RECT,szWindowName:LPCWSTR,dwStyle:DWORD,dwExStyle:DWORD,hMenu:HMENU,lpCreateParam:LPVOID) -> HWND {

    // }


    // #[inline(always)]
    // pub fn DestroyWindow (&mut self) -> bool {
    //     self.cwin.DestroyWindow()
    // }

    #[inline(always)]
    pub fn GetStyle (&self) -> DWORD {
        self.cwin.GetStyle()
    }

    #[inline(always)]
    pub fn GetExStyle (&self) -> DWORD {
        self.cwin.GetExStyle()
    }

    #[inline(always)]
    pub fn GetWindowLong (&self,nIndex:c_int) -> LONG {
        self.cwin.GetWindowLong(nIndex)
    }

    #[inline(always)]
    pub fn GetWindowLongPtr (&self,nIndex:c_int) -> LONG_PTR {
        self.cwin.GetWindowLongPtr(nIndex)
    }

    #[inline(always)]
    pub fn SetWindowLong (&self,nIndex:c_int,dwNewLong:LONG) -> LONG {
        self.cwin.SetWindowLong(nIndex,dwNewLong)
    }

    #[inline(always)]
    pub fn SetWindowLongPtr (&self,nIndex:c_int,dwNewLong:LONG_PTR) -> LONG_PTR {
        self.cwin.SetWindowLongPtr(nIndex,dwNewLong)
    }

    #[inline(always)]
    pub fn GetWindowWord (&self,nIndex:c_int) -> WORD {
        self.cwin.GetWindowWord(nIndex)
    }

    #[inline(always)]
    pub fn SetWindowWord (&self,nIndex:c_int,wNewWord:WORD) -> WORD {
        self.cwin.SetWindowWord(nIndex,wNewWord)
    }

    #[inline(always)]
    pub fn SendMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
        self.cwin.SendMessage(message,wParam,lParam)
    }

    #[inline(always)]
    pub fn PostMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
        self.cwin.PostMessage(message,wParam,lParam)
    }

    #[inline(always)]
    pub fn SendNotifyMessage (&self,message:UINT,wParam:WPARAM,lParam:LPARAM) -> bool {
        self.cwin.SendNotifyMessage(message,wParam,lParam)
    }

    #[inline(always)]
    pub fn GetWindowTextLength (&self) -> c_int {
        self.cwin.GetWindowTextLength()
    }

    #[inline(always)]
    pub fn SetFont (&self,hFont:HFONT,bRedraw:BOOL)  {
        self.cwin.SetFont(hFont,bRedraw)
    }

    #[inline(always)]
    pub fn GetFont (&self) -> HFONT {
        self.cwin.GetFont()
    }

    #[inline(always)]
    pub fn GetMenu (&self) -> HMENU {
        self.cwin.GetMenu()
    }

    #[inline(always)]
    pub fn SetMenu (&self,hMenu:HMENU) -> bool {
        self.cwin.SetMenu(hMenu)
    }

    #[inline(always)]
    pub fn DrawMenuBar (&self) -> bool {
        self.cwin.DrawMenuBar()
    }

    #[inline(always)]
    pub fn GetSystemMenu (&self,bRevert:BOOL) -> HMENU {
        self.cwin.GetSystemMenu(bRevert)
    }

    #[inline(always)]
    pub fn HiliteMenuItem (&self,hMenu:HMENU,uItemHilite:UINT,uHilite:UINT) -> bool {
        self.cwin.HiliteMenuItem(hMenu,uItemHilite,uHilite)
    }

    #[inline(always)]
    pub fn IsIconic (&self) -> bool {
        self.cwin.IsIconic()
    }

    #[inline(always)]
    pub fn IsZoomed (&self) -> bool {
        self.cwin.IsZoomed()
    }

    #[inline(always)]
    pub fn MoveWindow(&self,x:c_int,y:c_int,nWidth:c_int,nHeight:c_int,bRepaint:BOOL) -> bool{
        self.cwin.MoveWindow(x,y,nWidth,nHeight,bRepaint)
    }

    #[inline(always)]
    pub fn MoveWindow2 (&self,lpRect:&RECT,bRepaint:BOOL) -> bool {
        self.cwin.MoveWindow2(lpRect,bRepaint)
    }

    #[inline(always)]
    pub fn SetWindowPos(&self,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,nFlags:UINT) -> bool {
        self.cwin.SetWindowPos(hWndInsertAfter,x,y,cx,cy,nFlags)
    }

    #[inline(always)]
    pub fn SetWindowPos2 (&self,hWndInsertAfter:HWND,lpRect:&RECT,nFlags:UINT) -> bool {
        self.cwin.SetWindowPos2(hWndInsertAfter,lpRect,nFlags)
    }

    #[inline(always)]
    pub fn ArrangeIconicWindows (&self) -> UINT {
        self.cwin.ArrangeIconicWindows()
    }

    #[inline(always)]
    pub fn BringWindowToTop (&self) -> bool {
        self.cwin.BringWindowToTop()
    }

    #[inline(always)]
    pub fn GetWindowRect (&self,lpRect:LPRECT) -> bool {
        self.cwin.GetWindowRect(lpRect)
    }

    #[inline(always)]
    pub fn GetClientRect (&self,lpRect:&mut RECT) -> bool {
        self.cwin.GetClientRect(lpRect)
    }

    #[inline(always)]
    pub fn GetWindowPlacement(&self,lpwndpl:&mut WINDOWPLACEMENT) -> bool {
        self.cwin.GetWindowPlacement(lpwndpl)
    }

    #[inline(always)]
    pub fn SetWindowPlacement(&self,lpwndpl:&WINDOWPLACEMENT) -> bool {
        self.cwin.SetWindowPlacement(lpwndpl)
    }

    #[inline(always)]
    pub fn ClientToScreen (&self,lpPoint:LPPOINT) -> bool {
        self.cwin.ClientToScreen(lpPoint)
    }

    #[inline(always)]
    pub fn ClientToScreen2 (&self,lpRect:&mut RECT) -> bool {
        self.cwin.ClientToScreen2(lpRect)
    }

    #[inline(always)]
    pub fn ScreenToClient (&self,lpPoint:LPPOINT) -> bool {
        self.cwin.ScreenToClient(lpPoint)
    }

    #[inline(always)]
    pub fn ScreenToClient2 (&self,lpRect:&mut RECT) -> bool {
        self.cwin.ScreenToClient2(lpRect)
    }

    #[inline(always)]
    pub fn MapWindowPoints (&self,hWndTo:HWND,lpPoint:LPPOINT,nCount:UINT) -> c_int {
        self.cwin.MapWindowPoints(hWndTo,lpPoint,nCount)
    }

    #[inline(always)]
    pub fn MapWindowPoints2 (&self,hWndTo:HWND,lpRect:LPRECT) -> c_int {
        self.cwin.MapWindowPoints2(hWndTo,lpRect)
    }

    #[inline(always)]
    pub fn BeginPaint (&self,lpPaint:LPPAINTSTRUCT) -> HDC {
        self.cwin.BeginPaint(lpPaint)
    }

    #[inline(always)]
    pub fn EndPaint (&self,lpPaint:LPPAINTSTRUCT)  {
        self.cwin.EndPaint(lpPaint)
    }

    #[inline(always)]
    pub fn GetDC (&self) -> HDC {
        self.cwin.GetDC()
    }

    #[inline(always)]
    pub fn GetWindowDC (&self) -> HDC {
        self.cwin.GetWindowDC()
    }

    #[inline(always)]
    pub fn ReleaseDC (&self,hDC:HDC) -> c_int {
        self.cwin.ReleaseDC(hDC)
    }

    #[inline(always)]
    pub fn Print (&self,hDC:HDC,dwFlags:DWORD)  {
        self.cwin.Print(hDC,dwFlags)
    }

    #[inline(always)]
    pub fn PrintClient (&self,hDC:HDC,dwFlags:DWORD)  {
        self.cwin.PrintClient(hDC,dwFlags)
    }

    #[inline(always)]
    pub fn UpdateWindow (&self) -> bool {
        self.cwin.UpdateWindow()
    }

    #[inline(always)]
    pub fn SetRedraw (&self,bRedraw:BOOL)  {
        self.cwin.SetRedraw(bRedraw)
    }

    #[inline(always)]
    pub fn GetUpdateRect (&self,lpRect:LPRECT,bErase:BOOL) -> bool {
        self.cwin.GetUpdateRect(lpRect,bErase)
    }

    #[inline(always)]
    pub fn GetUpdateRgn (&self,hRgn:HRGN,bErase:BOOL) -> c_int {
        self.cwin.GetUpdateRgn(hRgn,bErase)
    }

    #[inline(always)]
    pub fn Invalidate (&self,bErase:BOOL) -> bool {
        self.cwin.Invalidate(bErase)
    }

    #[inline(always)]
    pub fn Invalidate2 (&self,lpRect:LPCRECT,bErase:BOOL) -> bool {
        self.cwin.Invalidate2(lpRect,bErase)
    }

    #[inline(always)]
    pub fn ValidateRect (&self,lpRect:LPCRECT) -> bool {
        self.cwin.ValidateRect(lpRect)
    }

    #[inline(always)]
    pub fn InvalidateRgn (&self,hRgn:HRGN,bErase:BOOL)  {
        self.cwin.InvalidateRgn(hRgn,bErase)
    }

    #[inline(always)]
    pub fn ValidateRgn (&self,hRgn:HRGN) -> bool {
        self.cwin.ValidateRgn(hRgn)
    }

    #[inline(always)]
    pub fn ShowWindow (&self,nCmdShow:c_int) -> bool {
        self.cwin.ShowWindow(nCmdShow)
    }

    #[inline(always)]
    pub fn IsWindowVisible (&self) -> bool {
        self.cwin.IsWindowVisible()
    }

    #[inline(always)]
    pub fn ShowOwnedPopups (&self,bShow:BOOL) -> bool {
        self.cwin.ShowOwnedPopups(bShow)
    }

    #[inline(always)]
    pub fn GetDCEx (&self,hRgnClip:HRGN,flags:DWORD) -> HDC {
        self.cwin.GetDCEx(hRgnClip,flags)
    }

    #[inline(always)]
    pub fn LockWindowUpdate (&self,bLock:bool) -> bool {
        self.cwin.LockWindowUpdate(bLock)
    }

    #[inline(always)]
    pub fn RedrawWindow2(&self) -> bool{
        self.cwin.RedrawWindow2()
    }

    #[inline(always)]
    pub fn RedrawWindow(&self,lpRectUpdate:LPCRECT,hRgnUpdate:HRGN,flags:UINT)->bool{
        self.cwin.RedrawWindow(lpRectUpdate,hRgnUpdate,flags)
    }

    #[inline(always)]
    pub fn SetTimer(&self,nIDEvent:UINT_PTR,nElapse:UINT) -> UINT_PTR {
        self.cwin.SetTimer(nIDEvent,nElapse)
    }

    #[inline(always)]
    pub fn SetTimer2(&self,nIDEvent:UINT_PTR,nElapse:UINT,lpfnTimer:TimerProc)->UINT_PTR{
        self.cwin.SetTimer2(nIDEvent,nElapse,lpfnTimer)
    }

    #[inline(always)]
    pub fn KillTimer (&self,nIDEvent:UINT_PTR) -> bool {
        self.cwin.KillTimer(nIDEvent)
    }

    #[inline(always)]
    pub fn IsWindowEnabled (&self) -> bool {
        self.cwin.IsWindowEnabled()
    }

    #[inline(always)]
    pub fn EnableWindow (&self,bEnable:BOOL) -> bool {
        self.cwin.EnableWindow(bEnable)
    }

    #[inline(always)]
    pub fn SetActiveWindow (&self) -> HWND {
        self.cwin.SetActiveWindow()
    }

    #[inline(always)]
    pub fn SetCapture (&self) -> HWND {
        self.cwin.SetCapture()
    }

    #[inline(always)]
    pub fn SetFocus (&self) -> HWND {
        self.cwin.SetFocus()
    }

    #[inline(always)]
    pub fn CheckDlgButton (&self,nIDButton:c_int,nCheck:UINT) -> bool {
        self.cwin.CheckDlgButton(nIDButton,nCheck)
    }

    #[inline(always)]
    pub fn CheckRadioButton (&self,nIDFirstButton:c_int,nIDLastButton:c_int,nIDCheckButton:c_int) -> bool {
        self.cwin.CheckRadioButton(nIDFirstButton,nIDLastButton,nIDCheckButton)
    }

    #[inline(always)]
    pub fn GetDlgItemInt(&self,nID:c_int) -> UINT {
        self.cwin.GetDlgItemInt(nID)
    }

    #[inline(always)]
    pub fn GetDlgItemInt2(&self,nID:c_int,lpTrans:&mut BOOL,bSigned:BOOL) -> UINT {
        self.cwin.GetDlgItemInt2(nID,lpTrans,bSigned)
    }

    #[inline(always)]
    pub fn IsDlgButtonChecked (&self,nIDButton:c_int) -> UINT {
        self.cwin.IsDlgButtonChecked(nIDButton)
    }

    #[inline(always)]
    pub fn SendDlgItemMessage (&self,nID:c_int,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
        self.cwin.SendDlgItemMessage(nID,message,wParam,lParam)
    }

    #[inline(always)]
    pub fn SetDlgItemInt (&self,nID:c_int,nValue:UINT,bSigned:BOOL) -> bool {
        self.cwin.SetDlgItemInt(nID,nValue,bSigned)
    }

    #[inline(always)]
    pub fn GetScrollPos (&self,nBar:c_int) -> c_int {
        self.cwin.GetScrollPos(nBar)
    }

    #[inline(always)]
    pub fn GetScrollRange (&self,nBar:c_int,lpMinPos:LPINT,lpMaxPos:LPINT) -> bool {
        self.cwin.GetScrollRange(nBar,lpMinPos,lpMaxPos)
    }

    #[inline(always)]
    pub fn ScrollWindow (&self,xAmount:c_int,yAmount:c_int,lpRect:LPCRECT,lpClipRect:LPCRECT) -> bool {
        self.cwin.ScrollWindow(xAmount,yAmount,lpRect,lpClipRect)
    }

    #[inline(always)]
    pub fn ScrollWindowEx(&self,dx:c_int,dy:c_int,lpRectScroll:LPCRECT ,lpRectClip:LPCRECT ,hRgnUpdate:HRGN ,lpRectUpdate:LPRECT ,uFlags:UINT ) -> c_int {
        self.cwin.ScrollWindowEx(dx,dy,lpRectScroll ,lpRectClip ,hRgnUpdate ,lpRectUpdate ,uFlags )
    }

    #[inline(always)]
    pub fn ScrollWindowExDefault(&self,dx:c_int,dy:c_int,uFlags:UINT)->c_int{
        self.cwin.ScrollWindowExDefault(dx,dy,uFlags)
    }

    #[inline(always)]
    pub fn SetScrollPos (&self,nBar:c_int,nPos:c_int,bRedraw:BOOL) -> c_int {
        self.cwin.SetScrollPos(nBar,nPos,bRedraw)
    }

    #[inline(always)]
    pub fn SetScrollRange (&self,nBar:c_int,nMinPos:c_int,nMaxPos:c_int,bRedraw:BOOL) -> bool {
        self.cwin.SetScrollRange(nBar,nMinPos,nMaxPos,bRedraw)
    }

    #[inline(always)]
    pub fn ShowScrollBar (&self,nBar:c_int,bShow:BOOL) -> bool {
        self.cwin.ShowScrollBar(nBar,bShow)
    }

    #[inline(always)]
    pub fn EnableScrollBar (&self,uSBFlags:UINT,uArrowFlags:UINT) -> bool {
        self.cwin.EnableScrollBar(uSBFlags,uArrowFlags)
    }

    #[inline(always)]
    pub fn IsChild (&self,hWnd:HWND) -> bool {
        self.cwin.IsChild(hWnd)
    }

    #[inline(always)]
    pub fn GetDlgCtrlID (&self) -> c_int {
        self.cwin.GetDlgCtrlID()
    }

    #[inline(always)]
    pub fn SetDlgCtrlID (&self,nID:c_int) -> c_int {
        self.cwin.SetDlgCtrlID(nID)
    }

    #[inline(always)]
    pub fn FlashWindow (&self,bInvert:BOOL) -> bool {
        self.cwin.FlashWindow(bInvert)
    }

    #[inline(always)]
    pub fn ChangeClipboardChain (&self,hWndNewNext:HWND) -> bool {
        self.cwin.ChangeClipboardChain(hWndNewNext)
    }

    #[inline(always)]
    pub fn SetClipboardViewer (&self) -> HWND {
        self.cwin.SetClipboardViewer()
    }

    #[inline(always)]
    pub fn OpenClipboard (&self) -> bool {
        self.cwin.OpenClipboard()
    }

    #[inline(always)]
    pub fn CreateCaret (&self,hBitmap:HBITMAP) -> bool {
        self.cwin.CreateCaret(hBitmap)
    }

    #[inline(always)]
    pub fn CreateSolidCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
        self.cwin.CreateSolidCaret(nWidth,nHeight)
    }

    #[inline(always)]
    pub fn CreateGrayCaret (&self,nWidth:c_int,nHeight:c_int) -> bool {
        self.cwin.CreateGrayCaret(nWidth,nHeight)
    }

    #[inline(always)]
    pub fn HideCaret (&self) -> bool {
        self.cwin.HideCaret()
    }

    #[inline(always)]
    pub fn ShowCaret (&self) -> bool {
        self.cwin.ShowCaret()
    }

    #[inline(always)]
    pub fn DragAcceptFiles (&self,bAccept:BOOL)  {
        self.cwin.DragAcceptFiles(bAccept)
    }

    #[inline(always)]
    pub fn SetIcon (&self,hIcon:HICON,bBigIcon:BOOL) -> HICON {
        self.cwin.SetIcon(hIcon,bBigIcon)
    }

    #[inline(always)]
    pub fn GetIcon (&self,bBigIcon:BOOL) -> HICON {
        self.cwin.GetIcon(bBigIcon)
    }

    #[inline(always)]
    pub fn SetWindowContextHelpId (&self,dwContextHelpId:DWORD) -> bool {
        self.cwin.SetWindowContextHelpId(dwContextHelpId)
    }

    #[inline(always)]
    pub fn GetWindowContextHelpId (&self) -> DWORD {
        self.cwin.GetWindowContextHelpId()
    }

    #[inline(always)]
    pub fn SetHotKey (&self,wVirtualKeyCode:WORD,wModifiers:WORD) -> c_int {
        self.cwin.SetHotKey(wVirtualKeyCode,wModifiers)
    }

    #[inline(always)]
    pub fn GetHotKey (&self) -> DWORD {
        self.cwin.GetHotKey()
    }

    #[inline(always)]
    pub fn GetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO) -> bool {
        self.cwin.GetScrollInfo(nBar,lpScrollInfo)
    }

    #[inline(always)]
    pub fn SetScrollInfo (&self,nBar:c_int,lpScrollInfo:LPSCROLLINFO,bRedraw:BOOL) -> c_int {
        self.cwin.SetScrollInfo(nBar,lpScrollInfo,bRedraw)
    }

    #[inline(always)]
    pub fn IsDialogMessage (&self,lpMsg:LPMSG) -> bool {
        self.cwin.IsDialogMessage(lpMsg)
    }

    #[inline(always)]
    pub fn NextDlgCtrl (&self)  {
        self.cwin.NextDlgCtrl()
    }

    #[inline(always)]
    pub fn PrevDlgCtrl (&self)  {
        self.cwin.PrevDlgCtrl()
    }

    #[inline(always)]
    pub fn GotoDlgCtrl (&self,hWndCtrl:HWND)  {
        self.cwin.GotoDlgCtrl(hWndCtrl)
    }

    #[inline(always)]
    pub fn ResizeClient (&self,nWidth:c_int,nHeight:c_int,bRedraw:BOOL) -> bool {
        self.cwin.ResizeClient(nWidth,nHeight,bRedraw)
    }

    #[inline(always)]
    pub fn GetWindowRgn (&self,hRgn:HRGN) -> c_int {
        self.cwin.GetWindowRgn(hRgn)
    }

    #[inline(always)]
    pub fn SetWindowRgn (&self,hRgn:HRGN,bRedraw:BOOL) -> c_int {
        self.cwin.SetWindowRgn(hRgn,bRedraw)
    }

    #[inline(always)]
    pub fn DeferWindowPos(&self,hWinPosInfo:HDWP,hWndInsertAfter:HWND,x:c_int,y:c_int,cx:c_int,cy:c_int,uFlags:UINT) -> HDWP {
        self.cwin.DeferWindowPos(hWinPosInfo,hWndInsertAfter,x,y,cx,cy,uFlags)
    }

    #[inline(always)]
    pub fn GetWindowThreadID (&self) -> DWORD {
        self.cwin.GetWindowThreadID()
    }

    #[inline(always)]
    pub fn GetWindowProcessID (&self) -> DWORD {
        self.cwin.GetWindowProcessID()
    }

    #[inline(always)]
    pub fn IsWindow (&self) -> bool {
        self.cwin.IsWindow()
    }

    #[inline(always)]
    pub fn IsWindowUnicode (&self) -> bool {
        self.cwin.IsWindowUnicode()
    }

    #[inline(always)]
    pub fn ShowWindowAsync (&self,nCmdShow:c_int) -> bool {
        self.cwin.ShowWindowAsync(nCmdShow)
    }

    #[inline(always)]
    pub fn CenterWindow (&self,hCenter:HWND) -> BOOL {
        self.cwin.CenterWindow(hCenter)
    }

    #[inline(always)]
    pub fn ModifyStyle (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
        self.cwin.ModifyStyle(dwRemove,dwAdd,nFlags)
    }

    #[inline(always)]
    pub fn ModifyStyleEx (&self,dwRemove:DWORD,dwAdd:DWORD,nFlags:UINT) -> bool {
        self.cwin.ModifyStyleEx(dwRemove,dwAdd,nFlags)
    }

    #[inline(always)]
    pub fn SetWindowText (&self, lpszString: &str) -> bool{
        self.cwin.SetWindowText(lpszString)
    }
}

