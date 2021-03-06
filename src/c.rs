#![allow(non_camel_case_types)]

pub use libc::c_char;
pub use libc::c_int;
pub use libc::c_uint;
pub use libc::c_long;
pub use libc::c_void;
pub use libc::size_t;
pub use libc::timeval;

bitset!(TickitEventType: c_uint
{
  TICKIT_EV_RESIZE = 0x01, // Term = lines, cols
  TICKIT_EV_KEY    = 0x02, // Term = type(TickitKeyEventType), str
  TICKIT_EV_MOUSE  = 0x04, // Term = type(TickitMouseEventType), button, line, col
  TICKIT_EV_CHANGE = 0x08, // Pen = {none}

  TICKIT_EV_UNBIND = 0x80000000 // event handler is being unbound
})

#[repr(C)]
pub enum TickitKeyEventType
{
  TICKIT_KEYEV_KEY = 1,
  TICKIT_KEYEV_TEXT,
}

#[repr(C)] #[deriving(PartialEq)]
pub enum TickitMouseEventType
{
  TICKIT_MOUSEEV_PRESS = 1,
  TICKIT_MOUSEEV_DRAG,
  TICKIT_MOUSEEV_RELEASE,
  TICKIT_MOUSEEV_WHEEL,
}

#[repr(C)] #[deriving(PartialEq, Show)]
pub enum X_Tickit_MouseWheel
{
  TICKIT_MOUSEWHEEL_UP = 1,
  TICKIT_MOUSEWHEEL_DOWN,
}

bitset!(X_Tickit_Mod: c_int
{
  TICKIT_MOD_SHIFT = 0x01,
  TICKIT_MOD_ALT   = 0x02,
  TICKIT_MOD_CTRL  = 0x04
})

#[repr(C)]
pub struct TickitEvent
{
  pub lines: c_int, pub cols: c_int,// RESIZE
  pub type_: c_int,                 // KEY, MOUSE
  pub str_: *const c_char,          // KEY
  pub button: c_int,                // MOUSE
  pub line: c_int, pub col: c_int,  // MOUSE
  pub mod_: X_Tickit_Mod,           // KEY, MOUSE
}

pub enum TickitPen {}

#[repr(C)] #[deriving(PartialEq, Show)]
pub enum TickitPenAttr
{
  TICKIT_PEN_FG,         /* colour */
  TICKIT_PEN_BG,         /* colour */
  TICKIT_PEN_BOLD,       /* bool */
  TICKIT_PEN_UNDER,      /* bool: TODO - number? */
  TICKIT_PEN_ITALIC,     /* bool */
  TICKIT_PEN_REVERSE,    /* bool */
  TICKIT_PEN_STRIKE,     /* bool */
  TICKIT_PEN_ALTFONT,    /* number */

  TICKIT_N_PEN_ATTRS
}

impl TickitPenAttr
{
    pub fn attrtype(self) -> TickitPenAttrType
    {
        unsafe
        {
            tickit_pen_attrtype(self)
        }
    }
    pub fn attrname_opt(self) -> Option<&'static str>
    {
        unsafe
        {
            let cstr = tickit_pen_attrname(self);
            if cstr.is_not_null()
            {
                Some(::std::str::raw::c_str_to_static_slice(cstr))
            }
            else
            {
                None
            }
        }
    }
    pub fn attrname(self) -> &'static str
    {
        self.attrname_opt().unwrap()
    }
    pub fn lookup_attr(name: &str) -> TickitPenAttr
    {
        unsafe
        {
            name.with_c_str(
                |n| { tickit_pen_lookup_attr(n) }
            )
        }
    }
}


#[repr(C)] #[deriving(PartialEq, Show)]
pub enum TickitPenAttrType
{
  TICKIT_PENTYPE_BOOL,
  TICKIT_PENTYPE_INT,
  TICKIT_PENTYPE_COLOUR,
}

extern
{
pub fn tickit_pen_new() -> *mut TickitPen;
// pub fn tickit_pen_newattrs(repeat (TickitPenAttr, c_int), then -1) -> *mut TickitPen;
pub fn tickit_pen_clone(orig: *const TickitPen) -> *mut TickitPen;
pub fn tickit_pen_destroy(pen: *mut TickitPen);

pub fn tickit_pen_has_attr(pen: *const TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_is_nonempty(pen: *const TickitPen) -> c_int;
pub fn tickit_pen_nondefault_attr(pen: *const TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_is_nondefault(pen: *const TickitPen) -> c_int;

pub fn tickit_pen_get_bool_attr(pen: *const TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_set_bool_attr(pen: *mut TickitPen, attr: TickitPenAttr, val: c_int);

pub fn tickit_pen_get_int_attr(pen: *const TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_set_int_attr(pen: *mut TickitPen, attr: TickitPenAttr, val: c_int);

pub fn tickit_pen_get_colour_attr(pen: *const TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_set_colour_attr(pen: *mut TickitPen, attr: TickitPenAttr, value: c_int);
pub fn tickit_pen_set_colour_attr_desc(pen: *mut TickitPen, attr: TickitPenAttr, value: *const c_char) -> c_int;

pub fn tickit_pen_clear_attr(pen: *mut TickitPen, attr: TickitPenAttr);
pub fn tickit_pen_clear(pen: *mut TickitPen);

pub fn tickit_pen_equiv_attr(a: *const TickitPen, b: *const TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_equiv(a: *const TickitPen, b: *const TickitPen) -> c_int;

pub fn tickit_pen_copy_attr(dst: *mut TickitPen, src: *const TickitPen, attr: TickitPenAttr);
pub fn tickit_pen_copy(dst: *mut TickitPen, src: *const TickitPen, overwrite: c_int);
}

pub type TickitPenEventFn = Option<extern fn(pen: *mut TickitPen, ev: TickitEventType, args: *mut TickitEvent, data: *mut c_void)>;

// https://github.com/rust-lang/rust/issues/15533
mod hack1
{
use super::{TickitPen,TickitEventType,TickitPenEventFn};
use super::{c_void,c_int};
extern
{
pub fn tickit_pen_bind_event(tt: *mut TickitPen, ev: TickitEventType, fn_: TickitPenEventFn, data: *mut c_void) -> c_int;
}
}
pub unsafe fn tickit_pen_bind_event(tt: *mut TickitPen, ev: TickitEventType, fn_: TickitPenEventFn, data: *mut c_void) -> c_int
{
    hack1::tickit_pen_bind_event(tt, ev, fn_, data)
}
extern
{
pub fn tickit_pen_unbind_event_id(tt: *mut TickitPen, id: c_int);

pub fn tickit_pen_attrtype(attr: TickitPenAttr) -> TickitPenAttrType;
pub fn tickit_pen_attrname(attr: TickitPenAttr) -> *const c_char;
pub fn tickit_pen_lookup_attr(name: *const c_char) -> TickitPenAttr;
}


#[repr(C)]
pub struct TickitRect
{
  pub top: c_int,
  pub left: c_int,
  pub lines: c_int,
  pub cols: c_int,
}

extern
{
pub fn tickit_rect_init_sized(rect: *mut TickitRect, top: c_int, left: c_int, lines: c_int, cols: c_int);
pub fn tickit_rect_init_bounded(rect: *mut TickitRect, top: c_int, left: c_int, bottom: c_int, right: c_int);
}

#[inline]
pub unsafe fn tickit_rect_bottom(rect: *const TickitRect) -> c_int
{
    return (*rect).top + (*rect).lines;
}

#[inline]
pub unsafe fn tickit_rect_right(rect: *const TickitRect) -> c_int
{
    return (*rect).left + (*rect).cols;
}

extern
{
pub fn tickit_rect_intersect(dst: *mut TickitRect, a: *const TickitRect, b: *const TickitRect) -> c_int;

pub fn tickit_rect_intersects(a: *const TickitRect, b: *const TickitRect) -> c_int;
pub fn tickit_rect_contains(large: *const TickitRect, small: *const TickitRect) -> c_int;

pub fn tickit_rect_add(ret: *mut [TickitRect, ..3], a: *const TickitRect, b: *const TickitRect) -> c_int;
pub fn tickit_rect_subtract(ret: *mut [TickitRect, ..4], orig: *const TickitRect, hole: *const TickitRect) -> c_int;
}


pub enum TickitRectSet {}

extern
{
pub fn tickit_rectset_new() -> *mut TickitRectSet;
pub fn tickit_rectset_destroy(trs: *mut TickitRectSet);

pub fn tickit_rectset_clear(trs: *mut TickitRectSet);

pub fn tickit_rectset_rects(trs: *const TickitRectSet) -> size_t;
pub fn tickit_rectset_get_rects(trs: *const TickitRectSet, rects: *mut TickitRect, n: size_t) -> size_t;

pub fn tickit_rectset_add(trs: *mut TickitRectSet, rect: *const TickitRect);
pub fn tickit_rectset_subtract(trs: *mut TickitRectSet, rect: *const TickitRect);

pub fn tickit_rectset_intersects(trs: *const TickitRectSet, rect: *const TickitRect) -> c_int;
pub fn tickit_rectset_contains(trs: *const TickitRectSet, rect: *const TickitRect) -> c_int;
}


pub enum TickitTerm {}
pub type TickitTermOutputFunc = Option<extern fn(tt: *mut TickitTerm, bytes: *const c_char, len: size_t, user: *mut c_void)>;

extern
{
pub fn tickit_term_new() -> *mut TickitTerm;
pub fn tickit_term_new_for_termtype(termtype: *const c_char) -> *mut TickitTerm;
pub fn tickit_term_destroy(tt: *mut TickitTerm);

pub fn tickit_term_get_termtype(tt: *mut TickitTerm) -> *const c_char;

pub fn tickit_term_set_output_fd(tt: *mut TickitTerm, fd: c_int);
pub fn tickit_term_get_output_fd(tt: *const TickitTerm) -> c_int;
}
mod hack3
{
use super::{TickitTerm,TickitTermOutputFunc};
use super::{c_void};
extern
{
pub fn tickit_term_set_output_func(tt: *mut TickitTerm, fn_: TickitTermOutputFunc, user: *mut c_void);
}
}
pub unsafe fn tickit_term_set_output_func(tt: *mut TickitTerm, fn_: TickitTermOutputFunc, user: *mut c_void)
{
    hack3::tickit_term_set_output_func(tt, fn_, user);
}
extern
{
pub fn tickit_term_set_output_buffer(tt: *mut TickitTerm, len: size_t);

pub fn tickit_term_await_started(tt: *mut TickitTerm, timeout: *const timeval);
pub fn tickit_term_flush(tt: *mut TickitTerm);

/* fd is allowed to be unset (-1); works abstractly */
pub fn tickit_term_set_input_fd(tt: *mut TickitTerm, fd: c_int);
pub fn tickit_term_get_input_fd(tt: *const TickitTerm) -> c_int;

pub fn tickit_term_get_utf8(tt: *const TickitTerm) -> c_int;
pub fn tickit_term_set_utf8(tt: *mut TickitTerm, utf8: c_int);

pub fn tickit_term_input_push_bytes(tt: *mut TickitTerm, bytes: *const c_char, len: size_t);
pub fn tickit_term_input_readable(tt: *mut TickitTerm);
pub fn tickit_term_input_check_timeout(tt: *mut TickitTerm) -> c_int;
pub fn tickit_term_input_wait(tt: *mut TickitTerm, timeout: *const timeval);

pub fn tickit_term_get_size(tt: *const TickitTerm, lines: *mut c_int, cols: *mut c_int);
pub fn tickit_term_set_size(tt: *mut TickitTerm, lines: c_int, cols: c_int);
pub fn tickit_term_refresh_size(tt: *mut TickitTerm);
}

pub type TickitTermEventFn = Option<extern fn (tt: *mut TickitTerm, ev: TickitEventType, args: *mut TickitEvent, data: *mut c_void)>;

// https://github.com/rust-lang/rust/issues/15533
mod hack2
{
use super::{TickitTerm,TickitEventType,TickitTermEventFn};
use super::{c_void,c_int};
extern
{
pub fn tickit_term_bind_event(tt: *mut TickitTerm, ev: TickitEventType, fn_: TickitTermEventFn, data: *mut c_void) -> c_int;
}
}
pub unsafe fn tickit_term_bind_event(tt: *mut TickitTerm, ev: TickitEventType, fn_: TickitTermEventFn, data: *mut c_void) -> c_int
{
    hack2::tickit_term_bind_event(tt, ev, fn_, data)
}
extern
{
pub fn tickit_term_unbind_event_id(tt: *mut TickitTerm, id: c_int);

pub fn tickit_term_print(tt: *mut TickitTerm, str_: *const c_char);
pub fn tickit_term_printn(tt: *mut TickitTerm, str_: *const c_char, len: size_t);
//pub fn tickit_term_printf(tt: *mut TickitTerm, fmt: *const c_char, ...);
//pub fn tickit_term_vprintf(tt: *mut TickitTerm, fmt: *const c_char, args: va_list);
pub fn tickit_term_goto(tt: *mut TickitTerm, line: c_int, col: c_int) -> c_int;
pub fn tickit_term_move(tt: *mut TickitTerm, downward: c_int, rightward: c_int);
pub fn tickit_term_scrollrect(tt: *mut TickitTerm, top: c_int, left: c_int, lines: c_int, cols: c_int, downward: c_int, rightward: c_int) -> c_int;

pub fn tickit_term_chpen(tt: *mut TickitTerm, pen: *const TickitPen);
pub fn tickit_term_setpen(tt: *mut TickitTerm, pen: *const TickitPen);

pub fn tickit_term_clear(tt: *mut TickitTerm);
pub fn tickit_term_erasech(tt: *mut TickitTerm, count: c_int, moveend: c_int);
}

#[repr(C)]
pub enum TickitTermCtl
{
  TICKIT_TERMCTL_ALTSCREEN = 1,
  TICKIT_TERMCTL_CURSORVIS,
  TICKIT_TERMCTL_MOUSE,
  TICKIT_TERMCTL_CURSORBLINK,
  TICKIT_TERMCTL_CURSORSHAPE,
  TICKIT_TERMCTL_ICON_TEXT,
  TICKIT_TERMCTL_TITLE_TEXT,
  TICKIT_TERMCTL_ICONTITLE_TEXT,
  TICKIT_TERMCTL_KEYPAD_APP,
  TICKIT_TERMCTL_COLORS, // read-only
}

#[repr(C)]
pub enum TickitTermMouseMode
{
  TICKIT_TERM_MOUSEMODE_OFF,
  TICKIT_TERM_MOUSEMODE_CLICK,
  TICKIT_TERM_MOUSEMODE_DRAG,
  TICKIT_TERM_MOUSEMODE_MOVE,
}

#[repr(C)]
pub enum TickitTermCursorShape
{
  TICKIT_TERM_CURSORSHAPE_BLOCK = 1,
  TICKIT_TERM_CURSORSHAPE_UNDER,
  TICKIT_TERM_CURSORSHAPE_LEFT_BAR,
}

extern
{
pub fn tickit_term_getctl_int(tt: *mut TickitTerm, ctl: TickitTermCtl, value: *mut c_int) -> c_int;
pub fn tickit_term_setctl_int(tt: *mut TickitTerm, ctl: TickitTermCtl, value: c_int) -> c_int;
pub fn tickit_term_setctl_str(tt: *mut TickitTerm, ctl: TickitTermCtl, value: *const c_char) -> c_int;
}

#[repr(C)]
pub struct TickitStringPos
{
  pub bytes: size_t,
  pub codepoints: c_int,
  pub graphemes: c_int,
  pub columns: c_int,
}

extern
{
pub fn tickit_string_count(str_: *const c_char, pos: *mut TickitStringPos, limit: *const TickitStringPos) -> size_t;
pub fn tickit_string_countmore(str_: *const c_char, pos: *mut TickitStringPos, limit: *const TickitStringPos) -> size_t;
pub fn tickit_string_ncount(str_: *const c_char, len: size_t, pos: *mut TickitStringPos, limit: *const TickitStringPos) -> size_t;
pub fn tickit_string_ncountmore(str_: *const c_char, len: size_t, pos: *mut TickitStringPos, limit: *const TickitStringPos) -> size_t;
}

#[inline]
pub unsafe fn tickit_stringpos_zero(pos: *mut TickitStringPos)
{
  (*pos).bytes = 0;
  (*pos).codepoints = 0;
  (*pos).graphemes = 0;
  (*pos).columns = 0;
}

pub static INIT_TICKIT_STRINGPOS_LIMIT_NONE: TickitStringPos =
{
    TickitStringPos{bytes: -1, codepoints: -1, graphemes: -1, columns: -1}
};
#[inline]
pub unsafe fn tickit_stringpos_limit_none(pos: *mut TickitStringPos)
{
  (*pos).bytes = -1;
  (*pos).codepoints = -1;
  (*pos).graphemes = -1;
  (*pos).columns = -1;
}

#[inline] #[allow(non_snake_case)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_BYTES(v: size_t) -> TickitStringPos
{
    TickitStringPos{bytes: v, codepoints: -1, graphemes: -1, columns: -1}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_bytes(pos: *mut TickitStringPos, bytes: size_t)
{
  (*pos).bytes = bytes;
  (*pos).codepoints = -1;
  (*pos).graphemes = -1;
  (*pos).columns = -1;
}

#[inline] #[allow(non_snake_case)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_CODEPOINTS(v: c_int) -> TickitStringPos
{
    TickitStringPos{bytes: -1, codepoints: v, graphemes : -1, columns : -1}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_codepoints(pos: *mut TickitStringPos, codepoints: c_int)
{
  (*pos).bytes = -1;
  (*pos).codepoints = codepoints;
  (*pos).graphemes = -1;
  (*pos).columns = -1;
}

#[inline] #[allow(non_snake_case)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_GRAPHEMES(v: c_int) -> TickitStringPos
{
    TickitStringPos{bytes: -1, codepoints: -1, graphemes: v, columns: -1}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_graphemes(pos: *mut TickitStringPos, graphemes: c_int)
{
  (*pos).bytes = -1;
  (*pos).codepoints = -1;
  (*pos).graphemes = graphemes;
  (*pos).columns = -1;
}

#[inline] #[allow(non_snake_case)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_COLUMNS(v: c_int) -> TickitStringPos
{
    TickitStringPos{bytes: -1, codepoints: -1, graphemes: -1, columns: v}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_columns(pos: *mut TickitStringPos, columns: c_int)
{
  (*pos).bytes = -1;
  (*pos).codepoints = -1;
  (*pos).graphemes = -1;
  (*pos).columns = columns;
}

extern
{
pub fn tickit_string_mbswidth(str_: *const c_char) -> c_int;
pub fn tickit_string_byte2col(str_: *const c_char, byte: size_t) -> c_int;
pub fn tickit_string_col2byte(str_: *const c_char, col: c_int) -> size_t;
}


pub enum TickitRenderBuffer {}

extern
{
pub fn tickit_renderbuffer_new(lines: c_int, cols: c_int) -> *mut TickitRenderBuffer;
pub fn tickit_renderbuffer_destroy(rb: *mut TickitRenderBuffer);

pub fn tickit_renderbuffer_get_size(rb: *const TickitRenderBuffer, lines: *mut c_int, cols: *mut c_int);

pub fn tickit_renderbuffer_translate(rb: *mut TickitRenderBuffer, downward: c_int, rightward: c_int);
pub fn tickit_renderbuffer_clip(rb: *mut TickitRenderBuffer, rect: *const TickitRect);
pub fn tickit_renderbuffer_mask(rb: *mut TickitRenderBuffer, mask: *const TickitRect);

pub fn tickit_renderbuffer_has_cursorpos(rb: *const TickitRenderBuffer) -> c_int;
pub fn tickit_renderbuffer_get_cursorpos(rb: *const TickitRenderBuffer, line: *mut c_int, col: *mut c_int);
pub fn tickit_renderbuffer_goto(rb: *mut TickitRenderBuffer, line: c_int, col: c_int);
pub fn tickit_renderbuffer_ungoto(rb: *mut TickitRenderBuffer);

pub fn tickit_renderbuffer_setpen(rb: *mut TickitRenderBuffer, pen: *const TickitPen);

pub fn tickit_renderbuffer_reset(rb: *mut TickitRenderBuffer);

pub fn tickit_renderbuffer_save(rb: *mut TickitRenderBuffer);
pub fn tickit_renderbuffer_savepen(rb: *mut TickitRenderBuffer);
pub fn tickit_renderbuffer_restore(rb: *mut TickitRenderBuffer);

pub fn tickit_renderbuffer_skip_at(rb: *mut TickitRenderBuffer, line: c_int, col: c_int, len: c_int);
pub fn tickit_renderbuffer_skip(rb: *mut TickitRenderBuffer, len: c_int);
pub fn tickit_renderbuffer_skip_to(rb: *mut TickitRenderBuffer, col: c_int);
pub fn tickit_renderbuffer_text_at(rb: *mut TickitRenderBuffer, line: c_int, col: c_int, text: *const c_char, pen: *const TickitPen) -> c_int;
pub fn tickit_renderbuffer_text(rb: *mut TickitRenderBuffer, text: *const c_char, pen: *const TickitPen) -> c_int;
pub fn tickit_renderbuffer_erase_at(rb: *mut TickitRenderBuffer, line: c_int, col: c_int, len: c_int, pen: *const TickitPen);
pub fn tickit_renderbuffer_erase(rb: *mut TickitRenderBuffer, len: c_int, pen: *const TickitPen);
pub fn tickit_renderbuffer_erase_to(rb: *mut TickitRenderBuffer, col: c_int, pen: *const TickitPen);
pub fn tickit_renderbuffer_eraserect(rb: *mut TickitRenderBuffer, rect: *const TickitRect, pen: *const TickitPen);
pub fn tickit_renderbuffer_clear(rb: *mut TickitRenderBuffer, pen: *const TickitPen);
pub fn tickit_renderbuffer_char_at(rb: *mut TickitRenderBuffer, line: c_int, col: c_int, codepoint: c_long, pen: *const TickitPen);
pub fn tickit_renderbuffer_char(rb: *mut TickitRenderBuffer, codepoint: c_long, pen: *const TickitPen);
}

#[repr(C)] #[deriving(PartialEq,Show)]
pub enum TickitLineStyle
{
  X_TICKIT_LINE_NONE = 0,
  TICKIT_LINE_SINGLE = 1,
  TICKIT_LINE_DOUBLE = 2,
  TICKIT_LINE_THICK  = 3,
}

bitset!(TickitLineCaps: c_int
{
  X_TICKIT_LINECAP_NEITHER = 0x00,
  TICKIT_LINECAP_START = 0x01,
  TICKIT_LINECAP_END   = 0x02,
  TICKIT_LINECAP_BOTH  = 0x03
})

extern
{
pub fn tickit_renderbuffer_hline_at(rb: *mut TickitRenderBuffer, line: c_int, startcol: c_int, endcol: c_int, style: TickitLineStyle, pen: *const TickitPen, caps: TickitLineCaps);
pub fn tickit_renderbuffer_vline_at(rb: *mut TickitRenderBuffer, startline: c_int, endline: c_int, col: c_int, style: TickitLineStyle, pen: *const TickitPen, caps: TickitLineCaps);

pub fn tickit_renderbuffer_flush_to_term(rb: *mut TickitRenderBuffer, tt: *mut TickitTerm);
}

// This API is still somewhat experimental

#[repr(C)]
pub struct TickitRenderBufferLineMask
{
  pub north: c_char,
  pub south: c_char,
  pub east: c_char,
  pub west: c_char,
}

extern
{
pub fn tickit_renderbuffer_get_cell_active(rb: *mut TickitRenderBuffer, line: c_int, col: c_int) -> c_int;
pub fn tickit_renderbuffer_get_cell_text(rb: *mut TickitRenderBuffer, line: c_int, col: c_int, buffer: *mut c_char, len: size_t) -> size_t;
pub fn tickit_renderbuffer_get_cell_linemask(rb: *mut TickitRenderBuffer, line: c_int, col: c_int) -> TickitRenderBufferLineMask;

pub fn tickit_renderbuffer_get_cell_pen(rb: *mut TickitRenderBuffer, line: c_int, col: c_int) -> *const TickitPen;
}

#[repr(C)]
pub struct TickitRenderBufferSpanInfo
{
  pub is_active: c_int,
  pub n_columns: c_int,
  pub text: *mut c_char,
  pub len: size_t,
  pub pen: *mut TickitPen,
}

extern
{
// returns the text length or -1 on error
pub fn tickit_renderbuffer_get_span(rb: *mut TickitRenderBuffer, line: c_int, startcol: c_int, info: *mut TickitRenderBufferSpanInfo, buffer: *mut c_char, len: size_t) -> size_t;
}
