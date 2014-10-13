#![experimental]

use std;
use termkey;

use libc::{c_char,c_int,c_void,size_t};

use TickitPen;
use TickitRect;
use c::TickitTermCtl;

use termkey::TermKey;
use termkey::TermKeyEvent;

pub mod c;

// TODO see if this can go away
pub struct CDriverRef
{
    driver: *mut c::TickitTermDriver,
}

impl CDriverRef
{
    pub fn write_str(self, str_: &str)
    {
        unsafe
        {
            let bytes: &[c_char] = std::mem::transmute(str_.as_bytes());
            c::tickit_termdrv_write_str(self.driver, bytes.as_ptr(), bytes.len() as size_t)
        }
    }
    pub fn current_pen(self) -> TickitPen
    {
        unsafe
        {
            let pen = c::tickit_termdrv_current_pen(self.driver);
            let pen: &TickitPen = std::mem::transmute(&pen);
            pen.clone()
        }
    }
}

#[allow(unused_variable)]
pub trait TickitTermDriverImpl
{
    fn attach(&mut self, cdr: CDriverRef, tt: &mut ::TickitTerm) {}
    fn start(&mut self, cdr: CDriverRef) {}
    fn started(&mut self, cdr: CDriverRef) -> bool { true }
    fn stop(&mut self, cdr: CDriverRef) {}
    fn print(&mut self, cdr: CDriverRef, str_: &str);
    fn goto_abs(&mut self, cdr: CDriverRef, line: int, col: int) -> bool;
    fn move_rel(&mut self, cdr: CDriverRef, downward: int, rightward: int);
    fn scrollrect(&mut self, cdr: CDriverRef, rect: &TickitRect, downward: int, rightward: int) -> bool;
    fn erasech(&mut self, cdr: CDriverRef, count: int, moveend: Option<bool>);
    fn clear(&mut self, cdr: CDriverRef);
    fn chpen(&mut self, cdr: CDriverRef, delta: &TickitPen, final_: &TickitPen);
    fn getctl_int(&mut self, cdr: CDriverRef, ctl: TickitTermCtl) -> Option<int>;
    fn setctl_int(&mut self, cdr: CDriverRef, ctl: TickitTermCtl, value: int) -> bool;
    fn setctl_str(&mut self, cdr: CDriverRef, ctl: TickitTermCtl, value: &str) -> bool;
    fn gotkey(&mut self, cdr: CDriverRef, tk: &mut TermKey, key: &TermKeyEvent) -> bool { false }
}

// TODO add lifetimes
struct RustTermDriver
{
    #[allow(dead_code)]
    driver: c::TickitTermDriver,
    vtable: Box<TickitTermDriverImpl+'static>,
}

impl ::TickitTerm
{
    pub fn new_for_driver<T: TickitTermDriverImpl + 'static>(driver_impl: T) -> ::TickitTerm
    {
        unsafe
        {
            let vtable_impl = box driver_impl;
            let c_driver = c::TickitTermDriver{tt: std::ptr::null_mut(), vtable: &RUST_VTABLE};
            let driver = box RustTermDriver{driver: c_driver, vtable: vtable_impl};
            let raw_driver: *mut c::TickitTermDriver = std::mem::transmute(driver);
            let tt = c::tickit_term_new_for_driver(raw_driver);
            assert!(tt.is_not_null());
            TickitTerm{tt: tt, output_hook: std::ptr::null_mut(), output_box: None}
        }
    }
}

extern fn rust_vtable_attach(ttd: *mut c::TickitTermDriver, tt: *mut super::c::TickitTerm)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        let termp: &mut (_, *mut c_void, *mut c_void) = &mut (tt, std::ptr::null_mut(), std::ptr::null_mut());
        let tt: &mut ::TickitTerm = std::mem::transmute(termp);
        (*ttd).vtable.attach(CDriverRef{driver: &mut (*ttd).driver}, tt);
    }
}

extern fn rust_vtable_destroy(ttd: *mut c::TickitTermDriver)
{
    unsafe
    {
        let _: Box<RustTermDriver> = std::mem::transmute(ttd);
    }
}

extern fn rust_vtable_start(ttd: *mut c::TickitTermDriver)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.start(CDriverRef{driver: &mut (*ttd).driver});
    }
}

extern fn rust_vtable_started(ttd: *mut c::TickitTermDriver) -> c_int
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.started(CDriverRef{driver: &mut (*ttd).driver}) as c_int
    }
}

extern fn rust_vtable_stop(ttd: *mut c::TickitTermDriver)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.stop(CDriverRef{driver: &mut (*ttd).driver});
    }
}

extern fn rust_vtable_print(ttd: *mut c::TickitTermDriver, str_: *const c_char, len: size_t)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        let str_: *const u8 = std::mem::transmute(str_);
        std::slice::raw::buf_as_slice(str_, len as uint, |arr| { (*ttd).vtable.print(CDriverRef{driver: &mut (*ttd).driver}, std::str::raw::from_utf8(arr)); });
    }
}

extern fn rust_vtable_goto_abs(ttd: *mut c::TickitTermDriver, line: c_int, col: c_int) -> c_int
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.goto_abs(CDriverRef{driver: &mut (*ttd).driver}, line as int, col as int) as c_int
    }
}

extern fn rust_vtable_move_rel(ttd: *mut c::TickitTermDriver, downward: c_int, rightward: c_int)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.move_rel(CDriverRef{driver: &mut (*ttd).driver}, downward as int, rightward as int);
    }
}

extern fn rust_vtable_scrollrect(ttd: *mut c::TickitTermDriver, rect: *const super::c::TickitRect, downward: c_int, rightward: c_int) -> c_int
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.scrollrect(CDriverRef{driver: &mut (*ttd).driver}, &TickitRect::from_c(*rect), downward as int, rightward as int) as c_int
    }
}

extern fn rust_vtable_erasech(ttd: *mut c::TickitTermDriver, count: c_int, moveend: c_int)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        let moveend = match moveend
        {
            -1 => None,
            0 => Some(false),
            1 => Some(true),
            _ => fail!(),
        };
        (*ttd).vtable.erasech(CDriverRef{driver: &mut (*ttd).driver}, count as int, moveend);
    }
}

extern fn rust_vtable_clear(ttd: *mut c::TickitTermDriver)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.clear(CDriverRef{driver: &mut (*ttd).driver});
    }
}

extern fn rust_vtable_chpen(ttd: *mut c::TickitTermDriver, mut delta: *const super::c::TickitPen, mut final_: *const super::c::TickitPen)
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        let delta: &mut TickitPen = std::mem::transmute(&mut delta);
        let final_: &mut TickitPen = std::mem::transmute(&mut final_);
        (*ttd).vtable.chpen(CDriverRef{driver: &mut (*ttd).driver}, delta, final_);
    }
}

extern fn rust_vtable_getctl_int(ttd: *mut c::TickitTermDriver, ctl: super::c::TickitTermCtl, value: *mut c_int) -> c_int
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        let tmp = (*ttd).vtable.getctl_int(CDriverRef{driver: &mut (*ttd).driver}, ctl);
        match tmp
        {
            Some(v) =>
            {
                *value = v as c_int;
                1
            }
            None =>
            {
                0
            }
        }
    }
}

extern fn rust_vtable_setctl_int(ttd: *mut c::TickitTermDriver, ctl: super::c::TickitTermCtl, value: c_int) -> c_int
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        (*ttd).vtable.setctl_int(CDriverRef{driver: &mut (*ttd).driver}, ctl, value as int) as c_int
    }
}

extern fn rust_vtable_setctl_str(ttd: *mut c::TickitTermDriver, ctl: super::c::TickitTermCtl, value: *const c_char) -> c_int
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);
        let s: &str = std::str::raw::c_str_to_static_slice(value);
        (*ttd).vtable.setctl_str(CDriverRef{driver: &mut (*ttd).driver}, ctl, s) as c_int
    }
}

extern fn rust_vtable_gotkey(ttd: *mut c::TickitTermDriver, mut tk: *mut termkey::c::TermKey, key: *const termkey::c::TermKeyKey) -> c_int
{
    unsafe
    {
        let ttd: *mut RustTermDriver = std::mem::transmute(ttd);;
        let key: &TermKeyEvent = &TermKeyEvent::from_c(tk, *key);
        let tk: &mut TermKey = std::mem::transmute(&mut tk);
        (*ttd).vtable.gotkey(CDriverRef{driver: &mut (*ttd).driver}, tk, key) as c_int
    }
}

static RUST_VTABLE: c::TickitTermDriverVTable = c::TickitTermDriverVTable
{
    attach: Some(rust_vtable_attach),
    destroy: Some(rust_vtable_destroy),
    start: Some(rust_vtable_start),
    started: Some(rust_vtable_started),
    stop: Some(rust_vtable_stop),
    print: Some(rust_vtable_print),
    goto_abs: Some(rust_vtable_goto_abs),
    move_rel: Some(rust_vtable_move_rel),
    scrollrect: Some(rust_vtable_scrollrect),
    erasech: Some(rust_vtable_erasech),
    clear: Some(rust_vtable_clear),
    chpen: Some(rust_vtable_chpen),
    getctl_int: Some(rust_vtable_getctl_int),
    setctl_int: Some(rust_vtable_setctl_int),
    setctl_str: Some(rust_vtable_setctl_str),
    gotkey: Some(rust_vtable_gotkey),
};
