#![allow(non_camel_case_types)]

pub use libc::c_char;
pub use libc::c_int;
pub use libc::c_uint;
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

#[repr(C)]
pub enum TickitMouseEventType
{
  TICKIT_MOUSEEV_PRESS = 1,
  TICKIT_MOUSEEV_DRAG,
  TICKIT_MOUSEEV_RELEASE,
  TICKIT_MOUSEEV_WHEEL,
}

#[repr(C)]
pub enum X_Tickit_MouseWheel
{
  TICKIT_MOUSEWHEEL_UP = 1,
  TICKIT_MOUSEWHEEL_DOWN,
}

bitset!(X_Tickit_Mod: c_uint
{
  TICKIT_MOD_SHIFT = 0x01,
  TICKIT_MOD_ALT   = 0x02,
  TICKIT_MOD_CTRL  = 0x04
})

#[repr(C)]
pub struct TickitEvent
{
  lines: c_int, cols: c_int,// RESIZE
  type_: c_int,             // KEY, MOUSE
  str_: *c_char,            // KEY
  button: c_int,            // MOUSE
  line: c_int, col: c_int,  // MOUSE
  mod_: c_int,              // KEY, MOUSE
}

pub struct TickitPen;

#[repr(C)]
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

#[repr(C)]
pub enum TickitPenAttrType
{
  TICKIT_PENTYPE_BOOL,
  TICKIT_PENTYPE_INT,
  TICKIT_PENTYPE_COLOUR,
}

extern
{
pub fn tickit_pen_new() -> *mut TickitPen;
pub fn tickit_pen_clone(orig: *mut TickitPen) -> *mut TickitPen;
pub fn tickit_pen_destroy(pen: *mut TickitPen);

pub fn tickit_pen_has_attr(pen: *TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_is_nonempty(pen: *TickitPen) -> c_int;
pub fn tickit_pen_is_nondefault(pen: *TickitPen) -> c_int;

pub fn tickit_pen_get_bool_attr(pen: *TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_set_bool_attr(pen: *mut TickitPen, attr: TickitPenAttr, val: c_int);

pub fn tickit_pen_get_int_attr(pen: *TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_set_int_attr(pen: *mut TickitPen, attr: TickitPenAttr, val: c_int);

pub fn tickit_pen_get_colour_attr(pen: *TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_set_colour_attr(pen: *mut TickitPen, attr: TickitPenAttr, value: c_int);
pub fn tickit_pen_set_colour_attr_desc(pen: *mut TickitPen, attr: TickitPenAttr, value: *c_char) -> c_int;

pub fn tickit_pen_clear_attr(pen: *mut TickitPen, attr: TickitPenAttr);
pub fn tickit_pen_clear(pen: *mut TickitPen);

pub fn tickit_pen_equiv_attr(a: *TickitPen, b: *TickitPen, attr: TickitPenAttr) -> c_int;
pub fn tickit_pen_equiv(a: *TickitPen, b: *TickitPen) -> c_int;

pub fn tickit_pen_copy_attr(dst: *mut TickitPen, src: *TickitPen, attr: TickitPenAttr);
pub fn tickit_pen_copy(dst: *mut TickitPen, src: *TickitPen, overwrite: c_int);
}

pub type TickitPenEventFn = extern fn(tt: *mut TickitPen, ev: TickitEventType, args: *mut TickitEvent, data: *mut c_void);

extern
{
pub fn tickit_pen_bind_event(tt: *mut TickitPen, ev: TickitEventType, fn_: TickitPenEventFn, data: *mut c_void) -> c_int;
pub fn tickit_pen_unbind_event_id(tt: *mut TickitPen, id: c_int);

pub fn tickit_pen_attrtype(attr: TickitPenAttr) -> TickitPenAttrType;
pub fn tickit_pen_attrname(attr: TickitPenAttr) -> *c_char;
pub fn tickit_pen_lookup_attr(name: *c_char) -> TickitPenAttr;
}


#[repr(C)]
pub struct TickitRect
{
  top: c_int,
  left: c_int,
  lines: c_int,
  cols: c_int,
}

extern
{
pub fn tickit_rect_init_sized(rect: *mut TickitRect, top: c_int, left: c_int, lines: c_int, cols: c_int);
pub fn tickit_rect_init_bounded(rect: *mut TickitRect, top: c_int, left: c_int, bottom: c_int, right: c_int);
}

#[inline]
pub unsafe fn tickit_rect_bottom(rect: *TickitRect) -> c_int
{
    return (*rect).top + (*rect).lines;
}

#[inline]
pub unsafe fn tickit_rect_right(rect: *TickitRect) -> c_int
{
    return (*rect).left + (*rect).cols;
}

extern
{
pub fn tickit_rect_intersect(dst: *mut TickitRect, a: *TickitRect, b: *TickitRect) -> c_int;

pub fn tickit_rect_intersects(a: *TickitRect, b: *TickitRect) -> c_int;
pub fn tickit_rect_contains(large: *TickitRect, small: *TickitRect) -> c_int;

pub fn tickit_rect_add(ret: *[TickitRect, ..3], a: *TickitRect, b: *TickitRect) -> c_int;
pub fn tickit_rect_subtract(ret: *[TickitRect, ..4], orig: *TickitRect, hole: *TickitRect) -> c_int;
}


pub struct TickitRectSet;

extern
{
pub fn tickit_rectset_new() -> *mut TickitRectSet;
pub fn tickit_rectset_destroy(trs: *mut TickitRectSet);

pub fn tickit_rectset_clear(trs: *mut TickitRectSet);

pub fn tickit_rectset_rects(trs: *TickitRectSet) -> size_t;
pub fn tickit_rectset_get_rects(trs: *TickitRectSet, rects: *TickitRect, n: size_t) -> size_t;

pub fn tickit_rectset_add(trs: *mut TickitRectSet, rect: *TickitRect);
pub fn tickit_rectset_subtract(trs: *mut TickitRectSet, rect: *TickitRect);

pub fn tickit_rectset_intersects(trs: *TickitRectSet, rect: *TickitRect) -> c_int;
pub fn tickit_rectset_contains(trs: *TickitRectSet, rect: *TickitRect) -> c_int;
}


pub struct TickitTerm;
pub type TickitTermOutputFunc = extern fn(tt: *mut TickitTerm, bytes: *c_char, len: size_t, user: *mut c_void);

extern
{
pub fn tickit_term_new() -> *mut TickitTerm;
pub fn tickit_term_new_for_termtype(termtype: *c_char) -> *mut TickitTerm;
pub fn tickit_term_destroy(tt: *mut TickitTerm);

pub fn tickit_term_get_termtype(tt: *mut TickitTerm) -> *c_char;

pub fn tickit_term_set_output_fd(tt: *mut TickitTerm, fd: c_int);
pub fn tickit_term_get_output_fd(tt: *TickitTerm) -> c_int;
pub fn tickit_term_set_output_func(tt: *mut TickitTerm, fn_: TickitTermOutputFunc, user: *mut c_void);
pub fn tickit_term_set_output_buffer(tt: *mut TickitTerm, len: size_t);

pub fn tickit_term_await_started(tt: *mut TickitTerm, timeout: *timeval);
pub fn tickit_term_flush(tt: *mut TickitTerm);

/* fd is allowed to be unset (-1); works abstractly */
pub fn tickit_term_set_input_fd(tt: *mut TickitTerm, fd: c_int);
pub fn tickit_term_get_input_fd(tt: *TickitTerm) -> c_int;

pub fn tickit_term_get_utf8(tt: *TickitTerm) -> c_int;
pub fn tickit_term_set_utf8(tt: *mut TickitTerm, utf8: c_int);

pub fn tickit_term_input_push_bytes(tt: *mut TickitTerm, bytes: *c_char, len: size_t);
pub fn tickit_term_input_readable(tt: *mut TickitTerm);
pub fn tickit_term_input_check_timeout(tt: *mut TickitTerm) -> c_int;
pub fn tickit_term_input_wait(tt: *mut TickitTerm, timeout: *timeval);

pub fn tickit_term_get_size(tt: *TickitTerm, lines: *mut c_int, cols: *mut c_int);
pub fn tickit_term_set_size(tt: *mut TickitTerm, lines: c_int, cols: c_int);
pub fn tickit_term_refresh_size(tt: *mut TickitTerm);
}

pub type TickitTermEventFn = extern fn (tt: *mut TickitTerm, ev: TickitEventType, args: *mut TickitEvent, data: *mut c_void);

extern
{
pub fn tickit_term_bind_event(tt: *mut TickitTerm, ev: TickitEventType, fn_: TickitTermEventFn, data: *mut c_void) -> c_int;
pub fn tickit_term_unbind_event_id(tt: *mut TickitTerm, id: c_int);

pub fn tickit_term_print(tt: *mut TickitTerm, str_: *c_char);
//pub fn tickit_term_printf(tt: *mut TickitTerm, fmt: *c_char, ...);
//pub fn tickit_term_vprintf(tt: *mut TickitTerm, fmt: *c_char, args: va_list);
pub fn tickit_term_goto(tt: *mut TickitTerm, line: c_int, col: c_int) -> c_int;
pub fn tickit_term_move(tt: *mut TickitTerm, downward: c_int, rightward: c_int);
pub fn tickit_term_scrollrect(tt: *mut TickitTerm, top: c_int, left: c_int, lines: c_int, cols: c_int, downward: c_int, rightward: c_int) -> c_int;

pub fn tickit_term_chpen(tt: *mut TickitTerm, pen: *TickitPen);
pub fn tickit_term_setpen(tt: *mut TickitTerm, pen: *TickitPen);

pub fn tickit_term_clear(tt: *mut TickitTerm);
pub fn tickit_term_erasech(tt: *mut TickitTerm, count: c_int, moveend: c_int);
}

#[repr(C)]
pub enum TickitTermCtl
{
  /* This is part of the API so additions must go at the end only */
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
pub fn tickit_term_setctl_str(tt: *mut TickitTerm, ctl: TickitTermCtl, value: *c_char) -> c_int;
}

#[repr(C)]
pub struct TickitStringPos
{
  bytes: size_t,
  codepoints: c_int,
  graphemes: c_int,
  columns: c_int,
}

extern
{
pub fn tickit_string_count(str_: *c_char, pos: *mut TickitStringPos, limit: *TickitStringPos) -> size_t;
pub fn tickit_string_countmore(str_: *c_char, pos: *mut TickitStringPos, limit: *TickitStringPos) -> size_t;
}

#[inline]
pub unsafe fn tickit_stringpos_zero(pos: *mut TickitStringPos)
{
  (*pos).bytes = 0;
  (*pos).codepoints = 0;
  (*pos).graphemes = 0;
  (*pos).columns = 0;
}

#[inline] #[allow(non_snake_case_functions)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_BYTES(v: size_t) -> TickitStringPos
{
    TickitStringPos{bytes: v, codepoints: -1, graphemes: -1, columns: -1}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_bytes(pos: *mut TickitStringPos, bytes: size_t)
{
  (*pos).codepoints = 0;
  (*pos).graphemes = 0;
  (*pos).columns = -1;
  (*pos).bytes = bytes;
}

#[inline] #[allow(non_snake_case_functions)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_CODEPOINTS(v: c_int) -> TickitStringPos
{
    TickitStringPos{bytes: -1, codepoints: v, graphemes : -1, columns : -1}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_codepoints(pos: *mut TickitStringPos, codepoints: c_int)
{
  (*pos).bytes = 0;
  (*pos).graphemes = 0;
  (*pos).columns = -1;
  (*pos).codepoints = codepoints;
}

#[inline] #[allow(non_snake_case_functions)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_GRAPHEMES(v: c_int) -> TickitStringPos
{
    TickitStringPos{bytes: -1, codepoints: -1, graphemes: v, columns: -1}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_graphemes(pos: *mut TickitStringPos, graphemes: c_int)
{
  (*pos).bytes = 0;
  (*pos).codepoints = 0;
  (*pos).columns = -1;
  (*pos).graphemes = graphemes;
}

#[inline] #[allow(non_snake_case_functions)]
pub unsafe fn INIT_TICKIT_STRINGPOS_LIMIT_COLUMNS(v: c_int) -> TickitStringPos
{
    TickitStringPos{bytes: -1, codepoints: -1, graphemes: -1, columns: v}
}
#[inline]
pub unsafe fn tickit_stringpos_limit_columns(pos: *mut TickitStringPos, columns: c_int)
{
  (*pos).bytes = 0;
  (*pos).codepoints = 0;
  (*pos).graphemes = -1;
  (*pos).columns = columns;
}

extern
{
pub fn tickit_string_mbswidth(str_: *char) -> c_int;
pub fn tickit_string_byte2col(str_: *char, byte: size_t) -> c_int;
pub fn tickit_string_col2byte(str_: *char, col: c_int) -> size_t;
}
