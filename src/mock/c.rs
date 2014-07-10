#![allow(non_camel_case_types)]

use c::{TickitTerm,TickitRect,TickitPen};
use libc::{c_char,c_int,size_t};

pub enum X_TickitMockTermLogType
{

  LOG_GOTO = 1,
  LOG_PRINT,
  LOG_ERASECH,
  LOG_CLEAR,
  LOG_SCROLLRECT,
  LOG_SETPEN,
}

pub struct TickitMockTermLogEntry
{
  pub type_: X_TickitMockTermLogType,
  pub val1: c_int, pub val2: c_int,  // GOTO(line, col); ERASECH(count, moveend); SCROLLRECT(downward,rightward)
  pub str_: *const c_char, // PRINT
  pub rect: TickitRect, // SCROLLRECT
  pub pen: *mut TickitPen,  // SETPEN
}

/* A TickitMockTerm really is a TickitTerm */
pub type TickitMockTerm = TickitTerm;

extern
{
pub fn tickit_mockterm_new(lines: c_int, cols: c_int) -> *mut TickitMockTerm;
pub fn tickit_mockterm_destroy(mt: *mut TickitMockTerm);

pub fn tickit_mockterm_resize(mt: *mut TickitMockTerm, newlines: c_int, newcols: c_int);

pub fn tickit_mockterm_get_display_text(mt: *mut TickitMockTerm, buffer: *mut c_char, len: size_t, line: c_int, col: c_int, width: c_int) -> size_t;
pub fn tickit_mockterm_get_display_pen(mt: *mut TickitMockTerm, line: c_int, col: c_int) -> *mut TickitPen;

pub fn tickit_mockterm_loglen(mt: *mut TickitMockTerm) -> c_int;
pub fn tickit_mockterm_peeklog(mt: *mut TickitMockTerm, i: c_int) -> *mut TickitMockTermLogEntry;
pub fn tickit_mockterm_clearlog(mt: *mut TickitMockTerm);

pub fn tickit_mockterm_get_position(mt: *mut TickitMockTerm, line: *mut c_int, col: *mut c_int);
}
