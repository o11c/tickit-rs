use c::{TickitTerm,TickitRect,TickitPen,TickitTermCtl};
use termkey::c::{TermKey,TermKeyKey};
use libc::{c_char,c_int,c_void,size_t};

pub struct TickitTermDriverVTable
{
  pub attach: Option<extern fn(ttd: *mut TickitTermDriver, tt: *mut TickitTerm)>,
  pub destroy: Option<extern fn(ttd: *mut TickitTermDriver)>,
  pub start: Option<extern fn(ttd: *mut TickitTermDriver)>,
  pub started: Option<extern fn(ttd: *mut TickitTermDriver) -> c_int>,
  pub stop: Option<extern fn(ttd: *mut TickitTermDriver)>,
  pub print: Option<extern fn(ttd: *mut TickitTermDriver, str_: *const c_char, len: size_t)>,
  pub goto_abs: Option<extern fn(ttd: *mut TickitTermDriver, line: c_int, col: c_int) -> c_int>,
  pub move_rel: Option<extern fn(ttd: *mut TickitTermDriver, downward: c_int, rightward: c_int)>,
  pub scrollrect: Option<extern fn(ttd: *mut TickitTermDriver, rect: *const TickitRect, downward: c_int, rightward: c_int) -> c_int>,
  pub erasech: Option<extern fn(ttd: *mut TickitTermDriver, count: c_int, moveend: c_int)>,
  pub clear: Option<extern fn(ttd: *mut TickitTermDriver)>,
  pub chpen: Option<extern fn(ttd: *mut TickitTermDriver, delta: *const TickitPen, final: *const TickitPen)>,
  pub getctl_int: Option<extern fn(ttd: *mut TickitTermDriver, ctl: TickitTermCtl, value: *mut c_int) -> c_int>,
  pub setctl_int: Option<extern fn(ttd: *mut TickitTermDriver, ctl: TickitTermCtl, value: c_int) -> c_int>,
  pub setctl_str: Option<extern fn(ttd: *mut TickitTermDriver, ctl: TickitTermCtl, value: *const c_char) -> c_int>,
  pub gotkey: Option<extern fn(ttd: *mut TickitTermDriver, tk: *mut TermKey, key: *const TermKeyKey) -> c_int>,
}

#[repr(C)]
pub struct TickitTermDriver
{
  pub tt: *mut TickitTerm,
  pub vtable: *const TickitTermDriverVTable,
}

extern
{
pub fn tickit_termdrv_get_tmpbuffer(ttd: *mut TickitTermDriver, len: size_t) -> *mut c_void;
pub fn tickit_termdrv_write_str(ttd: *mut TickitTermDriver, str: *const c_char, len: size_t);
// pub fn tickit_termdrv_write_strf(ttd: *mut TickitTermDriver, fmt: *const c_char, ...);
pub fn tickit_termdrv_current_pen(ttd: *mut TickitTermDriver) -> *mut TickitPen;
}

mod hack4
{
use super::TickitTermDriver;
use c::TickitTerm;
extern
{
pub fn tickit_term_new_for_driver(ttd: *mut TickitTermDriver) -> *mut TickitTerm;
}
}
pub unsafe fn tickit_term_new_for_driver(ttd: *mut TickitTermDriver) -> *mut TickitTerm
{
    hack4::tickit_term_new_for_driver(ttd)
}

extern
{
pub fn tickit_term_get_driver(tt: *mut TickitTerm) -> *mut TickitTermDriver;
}
