// versioning: upstream unreleased: 0.0, snapshot on march 1st
// last number is for changes in this rust binding
#![crate_id = "tickit#0.0.2014.5.2.1"]
#![crate_type = "dylib"]

#![feature(macro_rules)]
#![feature(struct_variant)]

extern crate green;
extern crate libc;
extern crate rustuv;

use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_void;
use libc::size_t;
use libc::timeval;

use c::TickitPenAttr;
use c::TickitPenAttrType;
use c::X_Tickit_Mod;
use c::TickitTermCtl;
use c::TickitLineStyle;
use c::TickitLineCaps;

mod bitset_macro;
pub mod c;

fn const_<T>(v: *mut T) -> *const T
{
    v as *const T
}


pub enum TickitKeyEvent<'a>
{
    KeyKeyEvent{pub key: &'a str, pub mod_: X_Tickit_Mod},
    KeyTextEvent{pub text: &'a str, pub mod_: X_Tickit_Mod},
}

pub enum TickitMouseEvent
{
    MousePressEvent{pub button: int, pub line: int, pub col: int, pub mod_: X_Tickit_Mod},
    MouseDragEvent{pub button: int, pub line: int, pub col: int, pub mod_: X_Tickit_Mod},
    MouseReleaseEvent{pub button: int, pub line: int, pub col: int, pub mod_: X_Tickit_Mod},
    MouseWheelEvent{pub dir: c::X_Tickit_MouseWheel, pub line: int, pub col: int, pub mod_: X_Tickit_Mod},
}

pub enum TickitEvent<'a>
{
    ResizeEvent{pub lines: int, pub cols: int},
    KeyEvent(TickitKeyEvent<'a>),
    MouseEvent(TickitMouseEvent),
    ChangeEvent,
    UnbindEvent,
    UnknownEvent,
}


pub struct TickitPen
{
    pen: *mut c::TickitPen,
}

impl TickitPen
{
    pub fn new() -> TickitPen
    {
        unsafe
        {
            TickitPen{pen: c::tickit_pen_new()}
        }
    }
}

impl Clone for TickitPen
{
    fn clone(&self) -> TickitPen
    {
        unsafe
        {
            TickitPen{pen: c::tickit_pen_clone(const_(self.pen))}
        }
    }
    fn clone_from(&mut self, other: &TickitPen)
    {
        unsafe
        {
            c::tickit_pen_copy(self.pen, const_(other.pen), 1)
        }
    }
}

impl Drop for TickitPen
{
    fn drop(&mut self)
    {
        unsafe
        {
            c::tickit_pen_destroy(self.pen)
        }
    }
}

impl TickitPen
{
    pub fn has_attr(&self, attr: TickitPenAttr) -> bool
    {
        unsafe
        {
            c::tickit_pen_has_attr(const_(self.pen), attr) != 0
        }
    }
    pub fn is_nonempty(&self) -> bool
    {
        unsafe
        {
            c::tickit_pen_is_nonempty(const_(self.pen)) != 0
        }
    }
    pub fn nondefault_attr(&self, attr: TickitPenAttr) -> bool
    {
        unsafe
        {
            c::tickit_pen_nondefault_attr(const_(self.pen), attr) != 0
        }
    }
    pub fn is_nondefault(&self) -> bool
    {
        unsafe
        {
            c::tickit_pen_is_nondefault(const_(self.pen)) != 0
        }
    }

    pub fn get_bool_attr(&self, attr: TickitPenAttr) -> bool
    {
        unsafe
        {
            c::tickit_pen_get_bool_attr(const_(self.pen), attr) != 0
        }
    }
    pub fn set_bool_attr(&mut self, attr: TickitPenAttr, val: bool)
    {
        unsafe
        {
            c::tickit_pen_set_bool_attr(self.pen, attr, if val { 1 } else { 0 });
        }
    }

    pub fn get_int_attr(&self, attr: TickitPenAttr) -> int
    {
        unsafe
        {
            c::tickit_pen_get_int_attr(const_(self.pen), attr) as int
        }
    }
    pub fn set_int_attr(&mut self, attr: TickitPenAttr, val: int)
    {
        unsafe
        {
            c::tickit_pen_set_int_attr(self.pen, attr, val as c_int);
        }
    }

    pub fn get_colour_attr(&self, attr: TickitPenAttr) -> int
    {
        unsafe
        {
            c::tickit_pen_get_colour_attr(const_(self.pen), attr) as int
        }
    }
    pub fn set_colour_attr(&mut self, attr: TickitPenAttr, value: int)
    {
        unsafe
        {
            c::tickit_pen_set_colour_attr(self.pen, attr, value as c_int);
        }
    }
    pub fn set_colour_attr_desc(&mut self, attr: TickitPenAttr, value: &str) -> int
    {
        unsafe
        {
            value.with_c_str(
                |v| { c::tickit_pen_set_colour_attr_desc(self.pen, attr, v) }
            ) as int
        }
    }

    pub fn clear_attr(&mut self, attr: TickitPenAttr)
    {
        unsafe
        {
            c::tickit_pen_clear_attr(self.pen, attr);
        }
    }
    pub fn clear(&mut self)
    {
        unsafe
        {
            c::tickit_pen_clear(self.pen);
        }
    }

    pub fn equiv_attr(&self, b: &TickitPen, attr: TickitPenAttr) -> bool
    {
        unsafe
        {
            c::tickit_pen_equiv_attr(const_(self.pen), const_(b.pen), attr) != 0
        }
    }
    pub fn equiv(&self, b: &TickitPen) -> bool
    {
        unsafe
        {
            c::tickit_pen_equiv(const_(self.pen), const_(b.pen)) != 0
        }
    }

    pub fn copy_attr(&mut self, src: &TickitPen, attr: TickitPenAttr)
    {
        unsafe
        {
            c::tickit_pen_copy_attr(self.pen, const_(src.pen), attr);
        }
    }
    pub fn copy(&mut self, src: &TickitPen, overwrite: bool)
    {
        unsafe
        {
            c::tickit_pen_copy(self.pen, const_(src.pen), if overwrite { 1 } else { 0 });
        }
    }
}

fn event_args<'a>(ty: c::TickitEventType, ar: &'a mut c::TickitEvent) -> TickitEvent<'a>
{
    match ty
    {
        x if x == c::TICKIT_EV_RESIZE =>
        {
            ResizeEvent{lines: ar.lines as int, cols: ar.cols as int}
        }
        x if x == c::TICKIT_EV_KEY =>
        {
            let type_: c::TickitKeyEventType = unsafe { std::mem::transmute(ar.type_) };
            let ev_str: &'a str = unsafe { std::str::raw::c_str_to_static_slice(ar.str_) };
            KeyEvent(
                match type_
                {
                    c::TICKIT_KEYEV_KEY =>
                    {
                        KeyKeyEvent{key: ev_str, mod_: ar.mod_}
                    }
                    c::TICKIT_KEYEV_TEXT =>
                    {
                        KeyTextEvent{text: ev_str, mod_: ar.mod_}
                    }
                    _ => { fail!() }
                }
            )
        }
        x if x == c::TICKIT_EV_MOUSE =>
        {
            let type_: c::TickitMouseEventType = unsafe { std::mem::transmute(ar.type_) };
            MouseEvent(
                match type_
                {
                    c::TICKIT_MOUSEEV_PRESS =>
                    {
                        MousePressEvent{button: ar.button as int, line: ar.line as int, col: ar.col as int, mod_: ar.mod_}
                    }
                    c::TICKIT_MOUSEEV_DRAG =>
                    {
                        MouseDragEvent{button: ar.button as int, line: ar.line as int, col: ar.col as int, mod_: ar.mod_}
                    }
                    c::TICKIT_MOUSEEV_RELEASE =>
                    {
                        MouseReleaseEvent{button: ar.button as int, line: ar.line as int, col: ar.col as int, mod_: ar.mod_}
                    }
                    c::TICKIT_MOUSEEV_WHEEL =>
                    {
                        let dir: c::X_Tickit_MouseWheel = unsafe { std::mem::transmute(ar.button) };
                        MouseWheelEvent{dir: dir, line: ar.line as int, col: ar.col as int, mod_: ar.mod_}
                    }
                    _ => { fail!() }
                }
            )
        }
        x if x == c::TICKIT_EV_CHANGE =>
        {
            ChangeEvent
        }
        x if x == c::TICKIT_EV_UNBIND =>
        {
            UnbindEvent
        }
        _ =>
        {
            UnknownEvent
        }
    }
}

extern fn pen_hacky_forever_bind_function(mut pen: *mut c::TickitPen, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
{
    unsafe
    {
        let penp = &mut pen;
        let pen_: &mut TickitPen = std::mem::transmute(penp);
        let args_ = event_args(ev, &mut *args);
        let cb: fn(&mut TickitPen, &TickitEvent) = std::mem::transmute(data);
        cb(pen_, &args_);
    }
}

impl TickitPen
{
    pub fn x_bind_event_forever(&mut self, ev: c::TickitEventType, cb: fn(&mut TickitPen, &TickitEvent))
    {
        unsafe
        {
            let fun = Some(pen_hacky_forever_bind_function);
            let data: *mut c_void = std::mem::transmute(cb);
            c::tickit_pen_bind_event(self.pen, ev, fun, data);
        }
    }
}

impl TickitPen
{
    pub fn attrtype(attr: TickitPenAttr) -> TickitPenAttrType
    {
        unsafe
        {
            c::tickit_pen_attrtype(attr)
        }
    }
    pub fn attrname(attr: TickitPenAttr) -> Option<&'static str>
    {
        unsafe
        {
            let cstr = c::tickit_pen_attrname(attr);
            if cstr.is_not_null()
            {
                Some(std::str::raw::c_str_to_static_slice(cstr))
            }
            else
            {
                None
            }
        }
    }
    pub fn lookup_attr(name: &str) -> TickitPenAttr
    {
        unsafe
        {
            name.with_c_str(
                |n| { c::tickit_pen_lookup_attr(n) }
            )
        }
    }
}


pub struct TickitRect
{
    pub top: int,
    pub left: int,
    pub lines: int,
    pub cols: int,
}

impl TickitRect
{
    pub fn init_sized(top: int, left: int, lines: int, cols: int) -> TickitRect
    {
        TickitRect{top: top, left: left, lines: lines, cols: cols}
    }
    pub fn init_bounded(top: int, left: int, bottom: int, right: int) -> TickitRect
    {
        TickitRect{top: top, left: left, lines: bottom - top, cols: right -- left}
    }
}

impl TickitRect
{
    #[inline]
    pub fn bottom(&self) -> int
    {
        return self.top + self.lines;
    }

    #[inline]
    pub fn right(&self) -> int
    {
        return self.left + self.cols;
    }
}

impl TickitRect
{
    fn to_c(self) -> c::TickitRect
    {
        c::TickitRect{top: self.top as c_int, left: self.left as c_int, lines: self.lines as c_int, cols: self.cols as c_int}
    }
    fn from_c(lo: c::TickitRect) -> TickitRect
    {
        TickitRect{top: lo.top as int, left: lo.left as int, lines: lo.lines as int, cols: lo.cols as int}
    }
}

impl TickitRect
{
    pub fn intersect(&self, b: &TickitRect) -> Option<TickitRect>
    {
        let mut tmp: c::TickitRect = unsafe { std::mem::uninitialized() };
        if unsafe { c::tickit_rect_intersect(&mut tmp, &self.to_c(), &b.to_c()) } != 0
        {
            Some(TickitRect::from_c(tmp))
        }
        else
        {
            None
        }
    }

    pub fn intersects(&self, b: &TickitRect) -> bool
    {
        unsafe { c::tickit_rect_intersects(&self.to_c(), &b.to_c()) != 0 }
    }
    pub fn contains(&self, small: &TickitRect) -> bool
    {
        unsafe { c::tickit_rect_contains(&self.to_c(), &small.to_c()) != 0 }
    }

    pub fn add(&self, b: &TickitRect) -> Vec<TickitRect>
    {
        let mut tmp: [c::TickitRect, ..3] = unsafe { std::mem::uninitialized() };
        let n = unsafe { c::tickit_rect_add(&mut tmp, &self.to_c(), &b.to_c()) };
        tmp.slice_to(n as uint).iter().map(|&r| { TickitRect::from_c(r) }).collect()
    }
    pub fn subtract(&self, hole: &TickitRect) -> Vec<TickitRect>
    {
        let mut tmp: [c::TickitRect, ..4] = unsafe { std::mem::uninitialized() };
        let n = unsafe { c::tickit_rect_subtract(&mut tmp, &self.to_c(), &hole.to_c()) };
        tmp.slice_to(n as uint).iter().map(|&r| { TickitRect::from_c(r) }).collect()
    }
}


pub struct TickitRectSet
{
    set: *mut c::TickitRectSet,
}

impl TickitRectSet
{
    pub fn new() -> TickitRectSet
    {
        unsafe
        {
            TickitRectSet{set: c::tickit_rectset_new()}
        }
    }
}

impl Drop for TickitRectSet
{
    fn drop(&mut self)
    {
        unsafe
        {
            c::tickit_rectset_destroy(self.set)
        }
    }
}
impl TickitRectSet
{
    pub fn clear(&mut self)
    {
        unsafe
        {
            c::tickit_rectset_clear(self.set);
        }
    }

    pub fn get_rects(&self) -> Vec<TickitRect>
    {
        let n = unsafe { c::tickit_rectset_rects(const_(self.set)) };
        let mut tmp = Vec::<c::TickitRect>::from_fn(n as uint, |_| { unsafe { std::mem::uninitialized() } } );
        unsafe
        {
            c::tickit_rectset_get_rects(const_(self.set), tmp.as_mut_ptr(), n);
        }

        tmp.iter().map(|&r| { TickitRect::from_c(r) }).collect()
    }

    pub fn add(&mut self, rect: &TickitRect)
    {
        unsafe
        {
            c::tickit_rectset_add(self.set, &rect.to_c());
        }
    }
    pub fn subtract(&mut self, rect: &TickitRect)
    {
        unsafe
        {
            c::tickit_rectset_subtract(self.set, &rect.to_c())
        }
    }

    pub fn intersects(&self, rect: &TickitRect) -> bool
    {
        unsafe
        {
            c::tickit_rectset_intersects(const_(self.set), &rect.to_c()) != 0
        }
    }
    pub fn contains(&self, rect: &TickitRect) -> bool
    {
        unsafe
        {
            c::tickit_rectset_contains(const_(self.set), &rect.to_c()) != 0
        }
    }
}



pub struct TickitTerm
{
    tt: *mut c::TickitTerm,
}

impl TickitTerm
{
    pub fn new() -> Result<TickitTerm, c_int>
    {
        unsafe
        {
            let tt = c::tickit_term_new();
            if tt.is_not_null()
            {
                Ok(TickitTerm{tt: c::tickit_term_new()})
            }
            else
            {
                Err(std::os::errno() as c::c_int)
            }
        }
    }
    pub fn new_for_termtype(name: &str) -> TickitTerm
    {
        unsafe
        {
            name.with_c_str(|n| {
                TickitTerm{tt: c::tickit_term_new_for_termtype(n)}
            })
        }
    }
}

impl Drop for TickitTerm
{
    fn drop(&mut self)
    {
        unsafe
        {
            c::tickit_term_destroy(self.tt);
        }
    }
}

impl TickitTerm
{
    pub fn get_termtype<'a>(&'a self) -> &'a str
    {
        unsafe
        {
            std::str::raw::c_str_to_static_slice(c::tickit_term_get_termtype(self.tt))
        }
    }

    pub fn set_output_fd(&mut self, fd: c_int)
    {
        unsafe
        {
            c::tickit_term_set_output_fd(self.tt, fd);
        }
    }
    pub fn get_output_fd(&self) -> c_int
    {
        unsafe
        {
            c::tickit_term_get_output_fd(const_(self.tt))
        }
    }
}

extern fn term_hacky_forever_output_function(mut term: *mut c::TickitTerm, bytes: *const c_char, len: size_t, data: *mut c_void)
{
    unsafe
    {
        let termp = &mut term;
        let term_: &mut TickitTerm = std::mem::transmute(termp);
        let cb: fn(&mut TickitTerm, &[u8]) = std::mem::transmute(data);
        let bytes: *const u8 = std::mem::transmute(bytes);
        std::slice::raw::buf_as_slice(bytes, len as uint, |arr| { cb(term_, arr) });
    }
}

impl TickitTerm
{
    pub fn x_set_output_func(&mut self, cb: fn(&mut TickitTerm, bytes: &[u8]))
    {
        unsafe
        {
            let fun = Some(term_hacky_forever_output_function);
            let data: *mut c_void = std::mem::transmute(cb);
            c::tickit_term_set_output_func(self.tt, fun, data);
        }
    }
}

impl TickitTerm
{
    pub fn set_output_buffer(&mut self, len: uint)
    {
        unsafe
        {
            c::tickit_term_set_output_buffer(self.tt, len as size_t);
        }
    }

    pub fn await_started(&mut self, timeout: Option<timeval>)
    {
        unsafe
        {
            c::tickit_term_await_started(self.tt, match timeout { Some(ref x) => x as *const _, None => std::ptr::null() });
        }
    }
    pub fn flush(&mut self)
    {
        unsafe
        {
            c::tickit_term_flush(self.tt);
        }
    }

    /* fd is allowed to be unset (-1); works abstractly */
    pub fn set_input_fd(&mut self, fd: c_int)
    {
        unsafe
        {
            c::tickit_term_set_input_fd(self.tt, fd);
        }
    }
    pub fn get_input_fd(&self) -> c_int
    {
        unsafe
        {
            c::tickit_term_get_input_fd(const_(self.tt))
        }
    }

    pub fn get_utf8(&self) -> bool
    {
        unsafe
        {
            c::tickit_term_get_utf8(const_(self.tt)) != 0
        }
    }
    pub fn set_utf8(&mut self, utf8: bool)
    {
        unsafe
        {
            c::tickit_term_set_utf8(self.tt, if utf8 { 1 } else { 0 });
        }
    }

    pub fn input_push_bytes(&mut self, bytes: &str)
    {
        unsafe
        {
            let b: &[c_char] = std::mem::transmute(bytes.as_bytes());
            c::tickit_term_input_push_bytes(self.tt, b.as_ptr(), b.len() as size_t);
        }
    }
    pub fn input_readable(&mut self)
    {
        unsafe
        {
            c::tickit_term_input_readable(self.tt);
        }
    }
    pub fn input_check_timeout(&mut self) -> Option<int>
    {
        unsafe
        {
            let t = c::tickit_term_input_check_timeout(self.tt);
            if t != -1
            {
                Some(t as int)
            }
            else
            {
                None
            }
        }
    }
    pub fn input_wait(&mut self, timeout: Option<timeval>)
    {
        unsafe
        {
            c::tickit_term_input_wait(self.tt, match timeout { Some(ref x) => x as *const _, None => std::ptr::null() });
        }
    }

    pub fn get_size(&self) -> (int, int)
    {
        unsafe
        {
            let mut lines: c_int = std::mem::uninitialized();
            let mut cols: c_int = std::mem::uninitialized();
            c::tickit_term_get_size(const_(self.tt), &mut lines, &mut cols);
            (lines as int, cols as int)
        }
    }
    pub fn set_size(&mut self, lines: int, cols: int)
    {
        unsafe
        {
            c::tickit_term_set_size(self.tt, lines as c_int, cols as c_int);
        }
    }
    pub fn refresh_size(&mut self)
    {
        unsafe
        {
            c::tickit_term_refresh_size(self.tt);
        }
    }
}

extern fn term_hacky_forever_bind_function(mut term: *mut c::TickitTerm, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
{
    unsafe
    {
        let termp = &mut term;
        let term_: &mut TickitTerm = std::mem::transmute(termp);
        let args_ = event_args(ev, &mut *args);
        let cb: fn(&mut TickitTerm, &TickitEvent) = std::mem::transmute(data);
        cb(term_, &args_);
    }
}

impl TickitTerm
{
    pub fn x_bind_event_forever(&mut self, ev: c::TickitEventType, cb: fn(&mut TickitTerm, &TickitEvent)) -> int
    {
        unsafe
        {
            let fun = Some(term_hacky_forever_bind_function);
            let data: *mut c_void = std::mem::transmute(cb);
            c::tickit_term_bind_event(self.tt, ev, fun, data) as int
        }
    }
}

impl TickitTerm
{
    pub fn print(&mut self, str_: &str)
    {
        unsafe
        {
            let s: &[c_char] = std::mem::transmute(str_.as_bytes());
            c::tickit_term_printn(self.tt, s.as_ptr(), s.len() as size_t);
        }
    }
    pub fn goto(&mut self, line: int, col: int) -> bool
    {
        unsafe
        {
            c::tickit_term_goto(self.tt, line as c_int, col as c_int) != 0
        }
    }
    pub fn move(&mut self, downward: int, rightward: int)
    {
        unsafe
        {
            c::tickit_term_move(self.tt, downward as c_int, rightward as c_int);
        }
    }
    pub fn scrollrect(&mut self, top: int, left: int, lines: int, cols: int, downward: int, rightward: int) -> bool
    {
        unsafe
        {
            c::tickit_term_scrollrect(self.tt, top as c_int, left as c_int, lines as c_int, cols as c_int, downward as c_int, rightward as c_int) != 0
        }
    }

    pub fn chpen(&mut self, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_term_chpen(self.tt, const_(pen.pen));
        }
    }
    pub fn setpen(&mut self, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_term_setpen(self.tt, const_(pen.pen));
        }
    }

    pub fn clear(&mut self)
    {
        unsafe
        {
            c::tickit_term_clear(self.tt);
        }
    }
    pub fn erasech(&mut self, count: int, moveend: int)
    {
        unsafe
        {
            c::tickit_term_erasech(self.tt, count as c_int, moveend as c_int);
        }
    }
}


impl TickitTerm
{
    pub fn getctl_int(&mut self, ctl: TickitTermCtl) -> Option<int>
    {
        let mut tmp: c_int = unsafe { std::mem::uninitialized() };
        let ok = unsafe { c::tickit_term_getctl_int(self.tt, ctl, &mut tmp) != 0 };
        if ok
        {
            Some(tmp as int)
        }
        else
        {
            None
        }
    }
    pub fn setctl_int(&mut self, ctl: TickitTermCtl, value: int) -> bool
    {
        unsafe
        {
            c::tickit_term_setctl_int(self.tt, ctl, value as c_int) != 0
        }
    }
    pub fn setctl_str(&mut self, ctl: TickitTermCtl, value: &str) -> bool
    {
        unsafe
        {
            value.with_c_str(
                |v| { c::tickit_term_setctl_str(self.tt, ctl, v) != 0 }
            )
        }
    }
}


pub struct StringPos
{
    pub bytes: uint,
    pub codepoints: uint,
    pub graphemes: uint,
    pub columns: uint,
}

impl StringPos
{
    fn to_c(self) -> c::TickitStringPos
    {
        c::TickitStringPos
        {
            bytes: self.bytes as size_t,
            codepoints: self.codepoints as c_int,
            graphemes: self.graphemes as c_int,
            columns: self.columns as c_int,
        }
    }

    fn from_c(lo: c::TickitStringPos) -> StringPos
    {
        StringPos
        {
            bytes: lo.bytes as uint,
            codepoints: lo.codepoints as uint,
            graphemes: lo.graphemes as uint,
            columns: lo.columns as uint,
        }
    }
}

impl StringPos
{
    pub fn count(str_: &str, pos: &mut StringPos, limit: Option<StringPos>) -> uint
    {
        unsafe
        {
            let s: &[c_char] = std::mem::transmute(str_.as_bytes());
            let mut cpos = pos.to_c();
            let rv = c::tickit_string_ncount(s.as_ptr(), s.len() as size_t, &mut cpos, match limit { Some(l) => { &l.to_c() as *const _ } None => { std::ptr::null() } });
            *pos = StringPos::from_c(cpos);
            rv as uint
        }
    }
    pub fn count_more(str_: &str, pos: &mut StringPos, limit: Option<StringPos>) -> uint
    {
        unsafe
        {
            let s: &[c_char] = std::mem::transmute(str_.as_bytes());
            let mut cpos = pos.to_c();
            let rv = c::tickit_string_ncountmore(s.as_ptr(), s.len() as size_t, &mut cpos, match limit { Some(l) => { &l.to_c() as *const _ } None => { std::ptr::null() } });
            *pos = StringPos::from_c(cpos);
            rv as uint
        }
    }
}

impl StringPos
{
    pub fn zero() -> StringPos
    {
        StringPos
        {
            bytes: 0,
            codepoints: 0,
            graphemes: 0,
            columns: 0,
        }
    }

    pub fn limit_none() -> StringPos
    {
        StringPos
        {
            bytes: -1,
            codepoints: -1,
            graphemes: -1,
            columns: -1,
        }
    }

    pub fn limit_bytes(v: uint) -> StringPos
    {
        StringPos
        {
            bytes: v,
            codepoints: -1,
            graphemes: -1,
            columns: -1,
        }
    }

    pub fn limit_codepoints(v: uint) -> StringPos
    {
        StringPos
        {
            bytes: -1,
            codepoints: v,
            graphemes: -1,
            columns: -1,
        }
    }

    pub fn limit_graphemes(v: uint) -> StringPos
    {
        StringPos
        {
            bytes: -1,
            codepoints: -1,
            graphemes: v,
            columns: -1,
        }
    }

    pub fn limit_columns(v: uint) -> StringPos
    {
        StringPos
        {
            bytes: -1,
            codepoints: -1,
            graphemes: -1,
            columns: v,
        }
    }
}

pub fn mbswidth(str_: &str) -> uint
{
    unsafe
    {
        str_.with_c_str(
            |s| { c::tickit_string_mbswidth(s) as uint }
        )
    }
}

pub fn byte2col(str_: &str, byte: uint) -> uint
{
    unsafe
    {
        str_.with_c_str(
            |s| { c::tickit_string_byte2col(s, byte as size_t) as uint }
        )
    }
}

pub fn col2byte(str_: &str, col: uint) -> uint
{
    unsafe
    {
        str_.with_c_str(
            |s| { c::tickit_string_col2byte(s, col as c_int) as uint }
        )
    }
}


pub struct TickitRenderBuffer
{
    rb: *mut c::TickitRenderBuffer,
}

impl TickitRenderBuffer
{
    pub fn new(lines: int, cols: int) -> TickitRenderBuffer
    {
        unsafe
        {
            TickitRenderBuffer{ rb: c::tickit_renderbuffer_new(lines as c_int, cols as c_int) }
        }
    }
}

impl Drop for TickitRenderBuffer
{
    fn drop(&mut self)
    {
        unsafe
        {
            c::tickit_renderbuffer_destroy(self.rb);
        }
    }
}

impl TickitRenderBuffer
{
    pub fn get_size(&self) -> (int, int)
    {
        unsafe
        {
            let mut lines: c_int = std::mem::uninitialized();
            let mut cols: c_int = std::mem::uninitialized();
            c::tickit_renderbuffer_get_size(const_(self.rb), &mut lines, &mut cols);
            (lines as int, cols as int)
        }
    }

    pub fn translate(&mut self, downward: int, rightward: int)
    {
        unsafe
        {
            c::tickit_renderbuffer_translate(self.rb, downward as c_int, rightward as c_int);
        }
    }
    pub fn clip(&mut self, rect: &TickitRect)
    {
        unsafe
        {
            c::tickit_renderbuffer_clip(self.rb, &rect.to_c());
        }
    }
    pub fn mask(&mut self, mask: &TickitRect)
    {
        unsafe
        {
            c::tickit_renderbuffer_mask(self.rb, &mask.to_c());
        }
    }

    pub fn get_cursorpos(&self) -> Option<(int, int)>
    {
        unsafe
        {
            if c::tickit_renderbuffer_has_cursorpos(const_(self.rb)) != 0
            {
                let mut line = std::mem::uninitialized();
                let mut col = std::mem::uninitialized();
                c::tickit_renderbuffer_get_cursorpos(const_(self.rb), &mut line, &mut col);
                Some((line as int, col as int))
            }
            else
            {
                None
            }
        }
    }
    pub fn goto(&mut self, line: int, col: int)
    {
        unsafe
        {
            c::tickit_renderbuffer_goto(self.rb, line as c_int, col as c_int);
        }
    }
    pub fn ungoto(&mut self)
    {
        unsafe
        {
            c::tickit_renderbuffer_ungoto(self.rb);
        }
    }

    pub fn setpen(&mut self, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_setpen(self.rb, const_(pen.pen));
        }
    }

    pub fn reset(&mut self)
    {
        unsafe
        {
            c::tickit_renderbuffer_reset(self.rb);
        }
    }

    pub fn save(&mut self)
    {
        unsafe
        {
            c::tickit_renderbuffer_save(self.rb);
        }
    }
    pub fn savepen(&mut self)
    {
        unsafe
        {
            c::tickit_renderbuffer_savepen(self.rb);
        }
    }
    pub fn restore(&mut self)
    {
        unsafe
        {
            c::tickit_renderbuffer_restore(self.rb);
        }
    }

    pub fn skip_at(&mut self, line: int, col: int, len: int)
    {
        unsafe
        {
            c::tickit_renderbuffer_skip_at(self.rb, line as c_int, col as c_int, len as c_int);
        }
    }
    pub fn skip(&mut self, len: int)
    {
        unsafe
        {
            c::tickit_renderbuffer_skip(self.rb, len as c_int);
        }
    }
    pub fn skip_to(&mut self, col: int)
    {
        unsafe
        {
            c::tickit_renderbuffer_skip_to(self.rb, col as c_int);
        }
    }
    pub fn text_at(&mut self, line: int, col: int, text: &str, pen: &TickitPen) -> int
    {
        unsafe
        {
            text.with_c_str(
                |t| { c::tickit_renderbuffer_text_at(self.rb, line as c_int, col as c_int, t, const_(pen.pen)) as int }
            )
        }
    }
    pub fn text(&mut self, text: &str, pen: &TickitPen) -> int
    {
        unsafe
        {
            text.with_c_str(
                |t| { c::tickit_renderbuffer_text(self.rb, t, const_(pen.pen)) as int }
            )
        }
    }
    pub fn erase_at(&mut self, line: int, col: int, len: int, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_erase_at(self.rb, line as c_int, col as c_int, len as c_int, const_(pen.pen));
        }
    }
    pub fn erase(&mut self, len: int, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_erase(self.rb, len as c_int, const_(pen.pen));
        }
    }
    pub fn erase_to(&mut self, col: int, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_erase_to(self.rb, col as c_int, const_(pen.pen));
        }
    }
    pub fn eraserect(&mut self, rect: &TickitRect, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_eraserect(self.rb, &rect.to_c(), const_(pen.pen));
        }
    }
    pub fn clear(&mut self, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_clear(self.rb, const_(pen.pen));
        }
    }
    pub fn char_at(&mut self, line: int, col: int, codepoint: char, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_char_at(self.rb, line as c_int, col as c_int, codepoint as c_long, const_(pen.pen));
        }
    }
    pub fn char(&mut self, codepoint: char, pen: &TickitPen)
    {
        unsafe
        {
            c::tickit_renderbuffer_char(self.rb, codepoint as c_long, const_(pen.pen));
        }
    }
}

impl TickitRenderBuffer
{
    pub fn hline_at(&mut self, line: int, startcol: int, endcol: int, style: TickitLineStyle, pen: &TickitPen, caps: TickitLineCaps)
    {
        unsafe
        {
            c::tickit_renderbuffer_hline_at(self.rb, line as c_int, startcol as c_int, endcol as c_int, style, const_(pen.pen), caps);
        }
    }
    pub fn vline_at(&mut self, startline: int, endline: int, col: int, style: TickitLineStyle, pen: &TickitPen, caps: TickitLineCaps)
    {
        unsafe
        {
            c::tickit_renderbuffer_vline_at(self.rb, startline as c_int, endline as c_int, col as c_int, style, const_(pen.pen), caps);
        }
    }

    pub fn flush_to_term(&mut self, tt: &mut TickitTerm)
    {
        unsafe
        {
            c::tickit_renderbuffer_flush_to_term(self.rb, tt.tt);
        }
    }
}

pub struct TickitRenderBufferLineMask
{
    pub north: TickitLineStyle,
    pub south: TickitLineStyle,
    pub east: TickitLineStyle,
    pub west: TickitLineStyle,
}

impl TickitRenderBufferLineMask
{
    fn from_c(lo: c::TickitRenderBufferLineMask) -> TickitRenderBufferLineMask
    {
        unsafe
        {
            TickitRenderBufferLineMask{north: std::mem::transmute(lo.north as c_int), south: std::mem::transmute(lo.south as c_int), east: std::mem::transmute(lo.east as c_int), west: std::mem::transmute(lo.west as c_int)}
        }
    }
    #[allow(dead_code)]
    fn to_c(self) -> c::TickitRenderBufferLineMask
    {
        c::TickitRenderBufferLineMask{north: self.north as c_char, south: self.south as c_char, east: self.east as c_char, west: self.west as c_char}
    }
}

impl TickitRenderBuffer
{
    pub fn get_cell_active(&mut self, line: int, col: int) -> bool
    {
        unsafe
        {
            c::tickit_renderbuffer_get_cell_active(self.rb, line as c_int, col as c_int) != 0
        }
    }
    pub fn get_cell_text(&mut self, line: int, col: int) -> String
    {
        unsafe
        {
            let n = c::tickit_renderbuffer_get_cell_text(self.rb, line as c_int, col as c_int, std::ptr::mut_null(), 0);
            if n > 0
            {
                let buf: Vec<u8> = Vec::from_fn(n as uint, |_| { std::mem::uninitialized() });
                c::tickit_renderbuffer_get_cell_text(self.rb, line as c_int, col as c_int, buf.as_ptr() as *mut c_char, n);
                std::str::raw::from_utf8_owned(buf)
            }
            else
            {
                "".to_string()
            }
        }
    }
    pub fn get_cell_linemask(&mut self, line: int, col: int) -> TickitRenderBufferLineMask
    {
        unsafe
        {
            let clm = c::tickit_renderbuffer_get_cell_linemask(self.rb, line as c_int, col as c_int);
            TickitRenderBufferLineMask::from_c(clm)
        }
    }

    pub fn get_cell_pen(&mut self, line: int, col: int) -> TickitPen
    {
        unsafe
        {
            let cpen = c::tickit_renderbuffer_get_cell_pen(self.rb, line as c_int, col as c_int);
            TickitPen{pen: c::tickit_pen_clone(cpen)}
        }
    }
}

pub enum TickitRenderBufferSpanInfo
{
    SkipSpan{pub n_columns: int},
    TextSpan{pub pen: TickitPen, pub text: String},
}

impl TickitRenderBuffer
{
// returns the text length or -1 on error
    pub fn get_span(&mut self, line: int, startcol: int) -> TickitRenderBufferSpanInfo
    {
        unsafe
        {
            let mut span_info: c::TickitRenderBufferSpanInfo = std::mem::uninitialized();
            span_info.pen = std::ptr::mut_null();
            // The return value is supposed to be the same as span_info.len,
            // but due to a bug, it is instead the length you pass in.
            // Thus, we need to pass in the span_info even the first time.
            // But this works out just as well for non-text inspections.
            let badlen = c::tickit_renderbuffer_get_span(self.rb, line as c_int, startcol as c_int, &mut span_info, std::ptr::mut_null(), 0);
            if badlen == -1
            {
                fail!();
            }
            if !(span_info.is_active != 0)
            {
                SkipSpan{n_columns: span_info.n_columns as int}
            }
            else
            {
                span_info.pen = c::tickit_pen_new();
                let goodlen = span_info.len;
                let buf: Vec<u8> = Vec::from_fn(goodlen as uint, |_| { std::mem::uninitialized() });
                c::tickit_renderbuffer_get_span(self.rb, line as c_int, startcol as c_int, &mut span_info, buf.as_ptr() as *mut c_char, goodlen);
                TextSpan{pen: TickitPen{pen: c::tickit_pen_clone(const_(span_info.pen))}, text: std::str::raw::from_utf8_owned(buf)}
            }
        }
    }
}


pub mod signal_hacks
{
    use std::task::TaskBuilder;
    use green::{SchedPool, PoolConfig, GreenTaskBuilder};
    use std::io::signal::{Interrupt, Listener, Signum};
    use std::comm::channel;
    use rustuv;

    // signals only work on green threads right now
    // http://doc.rust-lang.org/green/index.html#using-a-scheduler-pool
    // http://doc.rust-lang.org/std/io/signal/struct.Listener.html#example
    pub struct RemoteGreenSignalListener
    {
        // pool is never None except briefly during drop, to avoid unsafe.
        pool: Option<SchedPool>,
        pub rx: Receiver<Signum>,
    }
    impl RemoteGreenSignalListener
    {
        pub fn new() -> RemoteGreenSignalListener
        {
            let mut config = PoolConfig::new();
            config.threads = 1;
            config.event_loop_factory = rustuv::event_loop;
            let config = config;
            let mut pool = SchedPool::new(config);
            let (tx, rx) = channel::<Signum>();
            TaskBuilder::new().green(&mut pool).spawn(
                proc()
                {
                    let mut listener = Listener::new();
                    listener.register(Interrupt).unwrap();

                    loop
                    {
                        let sig = listener.rx.recv();
                        print!("\r\nInterrupt signal received. Press any key to flush\r\n");
                        tx.send(sig);
                        break;
                    }
                }
            );

            RemoteGreenSignalListener{pool: Some(pool), rx: rx}
        }
    }
    impl Drop for RemoteGreenSignalListener
    {
        fn drop(&mut self)
        {
            // apparently this would work without the Option
            // mem::replace(&mut self.pool, mem::zeroed()).shutdown();
            self.pool.take_unwrap().shutdown();
        }
    }
}
