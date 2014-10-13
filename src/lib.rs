#![feature(link_args)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(unsafe_destructor)]

extern crate collections;
extern crate libc;

extern crate termkey;

use libc::c_char;
use libc::c_int;
use libc::c_long;
use libc::c_void;
use libc::size_t;
use libc::timeval;

use c::TickitPenAttr;
pub use c::TickitPenAttrType;
use c::X_Tickit_Mod;
use c::TickitTermCtl;
use c::TickitLineStyle;
use c::TickitLineCaps;

mod bitset_macro;
pub mod c;
pub mod drv;
mod generated_link;
pub mod mock;

fn const_<T>(v: *mut T) -> *const T
{
    v as *const T
}

fn const_opt_pen(v: Option<&TickitPen>) -> *const c::TickitPen
{
    v.map(|p| const_(p.pen)).unwrap_or(std::ptr::null())
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
    // UnbindEvent,
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
            let cpen = c::tickit_pen_new();
            assert!(cpen.is_not_null());
            TickitPen{pen: cpen}
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
            c::tickit_pen_destroy(self.pen);
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
            c::tickit_pen_set_bool_attr(self.pen, attr, val as c_int);
        }
    }
    pub fn maybe_get_bool_attr(&self, attr: TickitPenAttr) -> Option<bool>
    {
        if self.has_attr(attr)
        {
            Some(self.get_bool_attr(attr))
        }
        else
        {
            None
        }
    }
    pub fn with_bool_attr(mut self, attr: TickitPenAttr, val: bool) -> TickitPen
    {
        self.set_bool_attr(attr, val);
        self
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
    pub fn maybe_get_int_attr(&self, attr: TickitPenAttr) -> Option<int>
    {
        if self.has_attr(attr)
        {
            Some(self.get_int_attr(attr))
        }
        else
        {
            None
        }
    }
    pub fn with_int_attr(mut self, attr: TickitPenAttr, val: int) -> TickitPen
    {
        self.set_int_attr(attr, val);
        self
    }

    pub fn get_colour_attr(&self, attr: TickitPenAttr) -> int
    {
        unsafe
        {
            c::tickit_pen_get_colour_attr(const_(self.pen), attr) as int
        }
    }
    pub fn maybe_get_colour_attr(&self, attr: TickitPenAttr) -> Option<int>
    {
        if self.has_attr(attr)
        {
            Some(self.get_colour_attr(attr))
        }
        else
        {
            None
        }
    }
    pub fn set_colour_attr(&mut self, attr: TickitPenAttr, value: int)
    {
        unsafe
        {
            c::tickit_pen_set_colour_attr(self.pen, attr, value as c_int);
        }
    }
    pub fn with_colour_attr(mut self, attr: TickitPenAttr, val: int) -> TickitPen
    {
        self.set_colour_attr(attr, val);
        self
    }
    pub fn set_colour_attr_desc(&mut self, attr: TickitPenAttr, value: &str) -> bool
    {
        unsafe
        {
            value.with_c_str(
                |v| { c::tickit_pen_set_colour_attr_desc(self.pen, attr, v) }
            ) != 0
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
            c::tickit_pen_copy(self.pen, const_(src.pen), overwrite as c_int);
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
                }
            )
        }
        x if x == c::TICKIT_EV_CHANGE =>
        {
            ChangeEvent
        }
        _ =>
        {
            UnknownEvent
        }
    }
}

extern fn pen_hacky_forever_bind_function(mut pen: *mut c::TickitPen, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
{
    if ev == c::TICKIT_EV_UNBIND
    {
        return;
    }
    unsafe
    {
        let penp = &mut pen;
        let pen_: &mut TickitPen = std::mem::transmute(penp);
        let args_ = event_args(ev, &mut *args);
        let cb: fn(&mut TickitPen, &TickitEvent) = std::mem::transmute(data);
        cb(pen_, &args_);
    }
}

struct LivelyPenData<'a>
{
    pen: *mut TickitPen,
    cb: |&mut TickitPen, &TickitEvent|: 'a,
}

#[must_use]
pub struct LivelyPenEvent<'a>
{
    id: c_int,
    data: Box<LivelyPenData<'a>>,
}

#[unsafe_destructor]
impl<'a> Drop for LivelyPenEvent<'a>
{
    fn drop(&mut self)
    {
        if self.data.pen.is_not_null()
        {
            unsafe
            {
                c::tickit_pen_unbind_event_id((*self.data.pen).pen, self.id);
            }
        }
    }
}

extern fn pen_lively_callback(mut pen: *mut c::TickitPen, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
{
    unsafe
    {
        let lively: *mut LivelyPenData = std::mem::transmute(data);
        if ev == c::TICKIT_EV_UNBIND
        {
            (*lively).pen = std::ptr::null_mut();
        }
        else
        {
            let penp = &mut pen;
            let pen_: &mut TickitPen = std::mem::transmute(penp);
            let args_ = event_args(ev, &mut *args);
            ((*lively).cb)(pen_, &args_);
        }
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

    pub fn bind_event_lively<'a>(&mut self, ev: c::TickitEventType, cb: |&mut TickitPen, &TickitEvent|: 'a) -> LivelyPenEvent<'a>
    {
        // why aren't we taking the lifetime of 'self' ?
        // First, because that would prevent anybody else from using it
        // Second, because we don't need it - we get an "unregister" event.
        unsafe
        {
            let fun = Some(pen_lively_callback);
            let mut data = box LivelyPenData::<'a>{pen: self as *mut _, cb: cb};
            let raw_data: *mut c_void = &mut *data as *mut _ as *mut c_void;
            let ev = ev | c::TICKIT_EV_UNBIND;
            let id = c::tickit_pen_bind_event(self.pen, ev, fun, raw_data);
            LivelyPenEvent{id: id, data: data}
        }
    }

    // pub fn bind_event_split<T>(&mut self, ev: c::TickitEventType, cb: fn(&mut TickitPen, &TickitEvent, T), data: T)
}

// RIP cross-mod impl c::TickitPenAttr

#[deriving(PartialEq)]
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
        TickitRect{top: top, left: left, lines: bottom - top, cols: right - left}
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
            let n2 = c::tickit_rectset_get_rects(const_(self.set), tmp.as_mut_ptr(), n);
            assert!(n == n2);
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
    output_hook: *mut c_void, // really LivelyTermOutData<'?>
    output_box: Option<Box<TermOutputDataWrapper>>
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
                Ok(TickitTerm{tt: tt, output_hook: std::ptr::null_mut(), output_box: None})
            }
            else
            {
                Err(std::os::errno() as c_int)
            }
        }
    }
    pub fn new_for_termtype(name: &str) -> Result<TickitTerm, c_int>
    {
        unsafe
        {
            let tt = name.with_c_str(|n| {
                c::tickit_term_new_for_termtype(n)
            });
            if tt.is_not_null()
            {
                Ok(TickitTerm{tt: tt, output_hook: std::ptr::null_mut(), output_box: None})
            }
            else
            {
                Err(std::os::errno() as c_int)
            }
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
            let x = self.output_hook as *mut LivelyTermOutData<'static>;
            if x.is_not_null()
            {
                (*x).tt = std::ptr::null_mut();
            }
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

extern fn term_hacky_forever_output_function(term: *mut c::TickitTerm, bytes: *const c_char, len: size_t, data: *mut c_void)
{
    unsafe
    {
        let termp: &mut (_, *mut c_void, *mut c_void) = &mut (term, std::ptr::null_mut(), std::ptr::null_mut());
        let term_: &mut TickitTerm = std::mem::transmute(termp);
        let cb: fn(&mut TickitTerm, &[u8]) = std::mem::transmute(data);
        let bytes: *const u8 = std::mem::transmute(bytes);
        std::slice::raw::buf_as_slice(bytes, len as uint, |arr| { cb(term_, arr) });
    }
}

struct LivelyTermOutData<'a>
{
    tt: *mut TickitTerm,
    cb: |&mut TickitTerm, &[u8]|: 'a,
}

#[must_use]
pub struct LivelyTermOutEvent<'a>
{
    data: Box<LivelyTermOutData<'a>>,
}

#[unsafe_destructor]
impl<'a> Drop for LivelyTermOutEvent<'a>
{
    fn drop(&mut self)
    {
        if self.data.tt.is_not_null()
        {
            unsafe
            {
                c::tickit_term_set_output_func((*self.data.tt).tt, None, std::ptr::null_mut());
                (*self.data.tt).output_hook = std::ptr::null_mut();
            }
        }
    }
}

extern fn term_out_lively_callback(term: *mut c::TickitTerm, bytes: *const c_char, len: size_t, data: *mut c_void)
{
    unsafe
    {
        let lively: *mut LivelyTermOutData = std::mem::transmute(data);
        {
            let termp: &mut (_, *mut c_void, *mut c_void) = &mut (term, std::ptr::null_mut(), std::ptr::null_mut());
            let term_: &mut TickitTerm = std::mem::transmute(termp);
            let bytes: *const u8 = std::mem::transmute(bytes);
            std::slice::raw::buf_as_slice(bytes, len as uint, |arr| { ((*lively).cb)(term_, arr) });
        }
    }
}

struct TermOutputData<T>
{
    cb: fn(&mut TickitTerm, &[u8], &mut T),
    data: T,
}

// https://github.com/rust-lang/rust/issues/15513
/* extern */ fn term_output_callback<T>(term: *mut c::TickitTerm, bytes: *const c_char, len: size_t, data: *mut c_void)
{
    unsafe
    {
        let termp: &mut (_, *mut c_void, *mut c_void) = &mut (term, std::ptr::null_mut(), std::ptr::null_mut());
        let term_: &mut TickitTerm = std::mem::transmute(termp);
        let bytes: *const u8 = std::mem::transmute(bytes);
        let data: &mut TermOutputData<T> = std::mem::transmute(data);
        std::slice::raw::buf_as_slice(bytes, len as uint, |arr| { (data.cb)(term_, arr, &mut data.data) });
    }
}

struct TermOutputDataWrapper
{
    rust_cb: Option<fn(*mut c::TickitTerm, *const c_char, size_t, *mut c_void)>,
    rust_drop: Option<fn(*mut c_void)>,
    rust_data: *mut c_void,
}

impl Drop for TermOutputDataWrapper
{
    fn drop(&mut self)
    {
        self.rust_drop.unwrap()(self.rust_data);
    }
}

extern fn term_output_wrapper(term: *mut c::TickitTerm, bytes: *const c_char, len: size_t, data: *mut c_void)
{
    unsafe
    {
        let data = data as *mut TermOutputDataWrapper;
        ((*data).rust_cb.unwrap())(term, bytes, len, (*data).rust_data);
    }
}

fn term_output_drop<T>(data: *mut c_void)
{
    unsafe
    {
        let _: Box<TermOutputData<T>> = std::mem::transmute(data);
    }
}

impl TickitTerm
{
    pub fn x_set_output_func_forever(&mut self, cb: fn(&mut TickitTerm, bytes: &[u8]))
    {
        unsafe
        {
            let fun = Some(term_hacky_forever_output_function);
            let data: *mut c_void = std::mem::transmute(cb);
            c::tickit_term_set_output_func(self.tt, fun, data);
        }
    }

    pub fn set_output_lively<'a>(&mut self, cb: |&mut TickitTerm, &[u8]|: 'a) -> LivelyTermOutEvent<'a>
    {
        // why aren't we taking the lifetime of 'self' ?
        // First, because that would prevent anybody else from using it
        // Second, because we don't need it - even though there is no
        // 'unregister' event, there's only one so we can drop it.
        unsafe
        {
            let fun = Some(term_out_lively_callback);
            let mut data = box LivelyTermOutData::<'a>{tt: self, cb: cb};
            let raw_data: *mut c_void = &mut *data as *mut _ as *mut c_void;
            c::tickit_term_set_output_func(self.tt, fun, raw_data);
            self.output_hook = raw_data;
            LivelyTermOutEvent{data: data}
        }
    }

    pub fn set_output_func<T>(&mut self, cb: fn(&mut TickitTerm, &[u8], &mut T), data: T)
    {
        unsafe
        {
            let fun = Some(term_output_callback::<T>);
            let raw_data: *mut c_void = std::mem::transmute(box TermOutputData::<T>{cb: cb, data: data});
            let wrap_fun = Some(term_output_wrapper);
            self.output_box = Some(box TermOutputDataWrapper{rust_cb: fun, rust_drop: Some(term_output_drop::<T>), rust_data: raw_data});
            let wrap_data: &TermOutputDataWrapper = &**self.output_box.as_ref().unwrap();
            let wrap_data = wrap_data as *const _ as *mut c_void;
            c::tickit_term_set_output_func(self.tt, wrap_fun, wrap_data);
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
            c::tickit_term_set_utf8(self.tt, utf8 as c_int);
        }
    }

    pub fn input_push_bytes(&mut self, bytes: &[u8])
    {
        unsafe
        {
            let b: &[c_char] = std::mem::transmute(bytes);
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
    pub fn input_check_timeout(&mut self) -> Option<uint>
    {
        unsafe
        {
            let t = c::tickit_term_input_check_timeout(self.tt);
            if t != -1
            {
                Some(t as uint)
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

    pub fn get_size(&self) -> (uint, uint)
    {
        unsafe
        {
            let mut lines: c_int = std::mem::uninitialized();
            let mut cols: c_int = std::mem::uninitialized();
            c::tickit_term_get_size(const_(self.tt), &mut lines, &mut cols);
            (lines as uint, cols as uint)
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

extern fn term_hacky_forever_bind_function(term: *mut c::TickitTerm, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
{
    if ev == c::TICKIT_EV_UNBIND
    {
        return;
    }
    unsafe
    {
        let termp: &mut (_, *mut c_void, *mut c_void) = &mut (term, std::ptr::null_mut(), std::ptr::null_mut());
        let term_: &mut TickitTerm = std::mem::transmute(termp);
        let args_ = event_args(ev, &mut *args);
        let cb: fn(&mut TickitTerm, &TickitEvent) = std::mem::transmute(data);
        cb(term_, &args_);
    }
}

struct LivelyTermData<'a>
{
    term: *mut TickitTerm,
    cb: |&mut TickitTerm, &TickitEvent|: 'a,
}

#[must_use]
pub struct LivelyTermEvent<'a>
{
    id: c_int,
    data: Box<LivelyTermData<'a>>,
}

#[unsafe_destructor]
impl<'a> Drop for LivelyTermEvent<'a>
{
    fn drop(&mut self)
    {
        if self.data.term.is_not_null()
        {
            unsafe
            {
                c::tickit_term_unbind_event_id((*self.data.term).tt, self.id);
            }
        }
    }
}

extern fn term_lively_callback(term: *mut c::TickitTerm, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
{
    unsafe
    {
        let lively: *mut LivelyTermData = std::mem::transmute(data);
        if ev == c::TICKIT_EV_UNBIND
        {
            (*lively).term = std::ptr::null_mut();
        }
        else
        {
            let termp: &mut (_, *mut c_void, *mut c_void) = &mut (term, std::ptr::null_mut(), std::ptr::null_mut());
            let term_: &mut TickitTerm = std::mem::transmute(termp);
            let args_ = event_args(ev, &mut *args);
            ((*lively).cb)(term_, &args_);
        }
    }
}

struct SplitTermData<T>
{
    cb: fn(&mut TickitTerm, &TickitEvent, &mut T),
    data: T,
}

// https://github.com/rust-lang/rust/issues/15513
/* extern */ fn term_split_callback<T>(term: *mut c::TickitTerm, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
-> bool
{
    unsafe
    {
        if ev == c::TICKIT_EV_UNBIND
        {
            let _: Box<SplitTermData<T>> = std::mem::transmute(data);
            true
        }
        else
        {
            let termp: &mut (_, *mut c_void, *mut c_void) = &mut (term, std::ptr::null_mut(), std::ptr::null_mut());
            let term_: &mut TickitTerm = std::mem::transmute(termp);
            let args_ = event_args(ev, &mut *args);
            let data: &mut SplitTermData<T> = std::mem::transmute(data);
            (data.cb)(term_, &args_, &mut data.data);
            false
        }
    }
}

struct SplitTermDataWrapper
{
    rust_cb: Option<fn(*mut c::TickitTerm, c::TickitEventType, *mut c::TickitEvent, *mut c_void)
                 -> bool>,
    rust_data: *mut c_void,
}

extern fn term_split_callback_wrapper(term: *mut c::TickitTerm, ev: c::TickitEventType, args: *mut c::TickitEvent, data: *mut c_void)
{
    unsafe
    {
        let data = data as *mut SplitTermDataWrapper;
        let del = ((*data).rust_cb.unwrap())(term, ev, args, (*data).rust_data);
        if del
        {
            let _: Box<SplitTermDataWrapper> = std::mem::transmute(data);
        }
    }
}

// TODO possibly store a pointer to this in the 'data' payload, to avoid
// the need for a runtime assert!() that the term is the same.
// Or we could just assume that the user won't be a *complete* idiot.
pub struct CancellableTermEvent
{
    tt: *mut c::TickitTerm,
    id: c_int,
    nocopy: std::kinds::marker::NoCopy,
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

    pub fn bind_event_lively<'a>(&mut self, ev: c::TickitEventType, cb: |&mut TickitTerm, &TickitEvent|: 'a) -> LivelyTermEvent<'a>
    {
        // why aren't we taking the lifetime of 'self' ?
        // First, because that would prevent anybody else from using it
        // Second, because we don't need it - we get an "unregister" event.
        unsafe
        {
            let fun = Some(term_lively_callback);
            let mut data = box LivelyTermData::<'a>{term: self as *mut _, cb: cb};
            let raw_data: *mut c_void = &mut *data as *mut _ as *mut c_void;
            let ev = ev | c::TICKIT_EV_UNBIND;
            let id = c::tickit_term_bind_event(self.tt, ev, fun, raw_data);
            LivelyTermEvent{id: id, data: data}
        }
    }

    pub fn bind_event<T>(&mut self, ev: c::TickitEventType, cb: fn(&mut TickitTerm, &TickitEvent, &mut T), data: T) -> CancellableTermEvent
    {
        unsafe
        {
            let fun = Some(term_split_callback::<T>);
            let raw_data: *mut c_void = std::mem::transmute(box SplitTermData::<T>{cb: cb, data: data});
            let ev = ev | c::TICKIT_EV_UNBIND;
            let wrap_fun = Some(term_split_callback_wrapper);
            let wrap_data: *mut c_void = std::mem::transmute(box SplitTermDataWrapper{rust_cb: fun, rust_data: raw_data});
            let id = c::tickit_term_bind_event(self.tt, ev, wrap_fun, wrap_data);
            CancellableTermEvent{tt: self.tt, id: id, nocopy: std::kinds::marker::NoCopy}
        }
    }

    pub fn unbind_event_id(&mut self, can: CancellableTermEvent)
    {
        assert!(self.tt == can.tt);
        unsafe
        {
            c::tickit_term_unbind_event_id(can.tt, can.id);
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
    pub fn move_(&mut self, downward: int, rightward: int)
    {
        unsafe
        {
            c::tickit_term_move(self.tt, downward as c_int, rightward as c_int);
        }
    }
    pub fn scrollrect(&mut self, rect: TickitRect, downward: int, rightward: int) -> bool
    {
        unsafe
        {
            c::tickit_term_scrollrect(self.tt, rect.top as c_int, rect.left as c_int, rect.lines as c_int, rect.cols as c_int, downward as c_int, rightward as c_int) != 0
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
    pub fn erasech(&mut self, count: int, moveend: Option<bool>)
    {
        let moveend = moveend.map(|x| x as c_int).unwrap_or(-1);
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
    // these are public just to let unit tests check failure correctly
    pub fn x_bcount(str_: &[u8], pos: &mut StringPos, limit: Option<StringPos>) -> uint
    {
        unsafe
        {
            let s: &[c_char] = std::mem::transmute(str_);
            let mut cpos = pos.to_c();
            let rv = c::tickit_string_ncount(s.as_ptr(), s.len() as size_t, &mut cpos, match limit { Some(l) => { &l.to_c() as *const _ } None => { std::ptr::null() } });
            *pos = StringPos::from_c(cpos);
            rv as uint
        }
    }
    pub fn x_bcountmore(str_: &[u8], pos: &mut StringPos, limit: Option<StringPos>) -> uint
    {
        unsafe
        {
            let s: &[c_char] = std::mem::transmute(str_);
            let mut cpos = pos.to_c();
            let rv = c::tickit_string_ncountmore(s.as_ptr(), s.len() as size_t, &mut cpos, match limit { Some(l) => { &l.to_c() as *const _ } None => { std::ptr::null() } });
            *pos = StringPos::from_c(cpos);
            rv as uint
        }
    }
    pub fn count(str_: &str, pos: &mut StringPos, limit: Option<StringPos>) -> uint
    {
        StringPos::x_bcount(str_.as_bytes(), pos, limit)
    }
    pub fn countmore(str_: &str, pos: &mut StringPos, limit: Option<StringPos>) -> uint
    {
        StringPos::x_bcountmore(str_.as_bytes(), pos, limit)
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
            let rb = c::tickit_renderbuffer_new(lines as c_int, cols as c_int);
            TickitRenderBuffer{ rb: rb }
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
    pub fn text_at(&mut self, line: int, col: int, text: &str, pen: Option<&TickitPen>) -> int
    {
        unsafe
        {
            text.with_c_str(
                |t| { c::tickit_renderbuffer_text_at(self.rb, line as c_int, col as c_int, t, const_opt_pen(pen)) as int }
            )
        }
    }
    pub fn text(&mut self, text: &str, pen: Option<&TickitPen>) -> int
    {
        unsafe
        {
            text.with_c_str(
                |t| { c::tickit_renderbuffer_text(self.rb, t, const_opt_pen(pen)) as int }
            )
        }
    }
    pub fn erase_at(&mut self, line: int, col: int, len: int, pen: Option<&TickitPen>)
    {
        unsafe
        {
            c::tickit_renderbuffer_erase_at(self.rb, line as c_int, col as c_int, len as c_int, const_opt_pen(pen));
        }
    }
    pub fn erase(&mut self, len: int, pen: Option<&TickitPen>)
    {
        unsafe
        {
            c::tickit_renderbuffer_erase(self.rb, len as c_int, const_opt_pen(pen));
        }
    }
    pub fn erase_to(&mut self, col: int, pen: Option<&TickitPen>)
    {
        unsafe
        {
            c::tickit_renderbuffer_erase_to(self.rb, col as c_int, const_opt_pen(pen));
        }
    }
    pub fn eraserect(&mut self, rect: &TickitRect, pen: Option<&TickitPen>)
    {
        unsafe
        {
            c::tickit_renderbuffer_eraserect(self.rb, &rect.to_c(), const_opt_pen(pen));
        }
    }
    pub fn clear(&mut self, pen: Option<&TickitPen>)
    {
        unsafe
        {
            c::tickit_renderbuffer_clear(self.rb, const_opt_pen(pen));
        }
    }
    pub fn char_at(&mut self, line: int, col: int, codepoint: char, pen: Option<&TickitPen>)
    {
        unsafe
        {
            c::tickit_renderbuffer_char_at(self.rb, line as c_int, col as c_int, codepoint as c_long, const_opt_pen(pen));
        }
    }
    pub fn char(&mut self, codepoint: char, pen: Option<&TickitPen>)
    {
        unsafe
        {
            c::tickit_renderbuffer_char(self.rb, codepoint as c_long, const_opt_pen(pen));
        }
    }
}

impl TickitRenderBuffer
{
    pub fn hline_at(&mut self, line: int, startcol: int, endcol: int, style: TickitLineStyle, pen: Option<&TickitPen>, caps: TickitLineCaps)
    {
        unsafe
        {
            c::tickit_renderbuffer_hline_at(self.rb, line as c_int, startcol as c_int, endcol as c_int, style, const_opt_pen(pen), caps);
        }
    }
    pub fn vline_at(&mut self, startline: int, endline: int, col: int, style: TickitLineStyle, pen: Option<&TickitPen>, caps: TickitLineCaps)
    {
        unsafe
        {
            c::tickit_renderbuffer_vline_at(self.rb, startline as c_int, endline as c_int, col as c_int, style, const_opt_pen(pen), caps);
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

#[experimental]
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

#[experimental]
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
            let n = c::tickit_renderbuffer_get_cell_text(self.rb, line as c_int, col as c_int, std::ptr::null_mut(), 0);
            if n > 0
            {
                let buf: Vec<u8> = Vec::from_fn(n as uint, |_| { std::mem::uninitialized() });
                c::tickit_renderbuffer_get_cell_text(self.rb, line as c_int, col as c_int, buf.as_ptr() as *mut c_char, n);
                collections::string::raw::from_utf8(buf)
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

#[experimental]
pub enum TickitRenderBufferSpanInfo
{
    SkipSpan{pub n_columns: int},
    TextSpan{pub pen: TickitPen, pub text: String},
}

#[experimental]
impl TickitRenderBuffer
{
// returns the text length or -1 on error
    pub fn get_span(&mut self, line: int, startcol: int) -> TickitRenderBufferSpanInfo
    {
        unsafe
        {
            let mut span_info: c::TickitRenderBufferSpanInfo = std::mem::uninitialized();
            span_info.pen = std::ptr::null_mut();
            // The return value is supposed to be the same as span_info.len,
            // but due to a bug, it is instead the length you pass in.
            // Thus, we need to pass in the span_info even the first time.
            // But this works out just as well for non-text inspections.
            let badlen = c::tickit_renderbuffer_get_span(self.rb, line as c_int, startcol as c_int, &mut span_info, std::ptr::null_mut(), 0);
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
                TextSpan{pen: TickitPen{pen: c::tickit_pen_clone(const_(span_info.pen))}, text: collections::string::raw::from_utf8(buf)}
            }
        }
    }
}
