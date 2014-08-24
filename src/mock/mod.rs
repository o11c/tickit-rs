#![experimental]

use std;
use collections;

use libc::{c_char,c_int};

use TickitPen;
use TickitRect;
use TickitTerm;

pub mod c;

pub enum LogEntry<'a>
{
    Goto{pub line: int, pub col: int},
    Print{pub str_: &'a str},
    EraseCh{pub count: int, pub moveend: Option<bool>},
    Clear,
    ScrollRect{pub downward: int, pub rightward: int, pub rect: TickitRect},
    SetPen{pub pen: TickitPen},
}

impl<'a> LogEntry<'a>
{
    fn from_c(e: &c::TickitMockTermLogEntry) -> LogEntry<'a>
    {
        match e.type_
        {
            c::LOG_GOTO =>
            {
                Goto{line: e.val1 as int, col: e.val2 as int}
            }
            c::LOG_PRINT =>
            {
                Print{str_: unsafe { std::str::raw::c_str_to_static_slice(e.str_) }}
            }
            c::LOG_ERASECH =>
            {
                let moveend = match e.val2
                {
                    -1 => None,
                    0 => Some(false),
                    1 => Some(true),
                    _ => fail!(),
                };
                EraseCh{count: e.val1 as int, moveend: moveend}
            }
            c::LOG_CLEAR =>
            {
                Clear
            }
            c::LOG_SCROLLRECT =>
            {
                ScrollRect{downward: e.val1 as int, rightward: e.val2 as int, rect: TickitRect::from_c(e.rect)}
            }
            c::LOG_SETPEN =>
            {
                let pen: &TickitPen = unsafe { std::mem::transmute(&e.pen) };
                SetPen{pen: pen.clone()}
            }
        }
    }
}

pub struct MockTerm
{
    pub tt: TickitTerm,
}

impl MockTerm
{
    pub fn new(lines: int, cols: int) -> MockTerm
    {
        unsafe
        {
            let tt = c::tickit_mockterm_new(lines as c_int, cols as c_int);
            MockTerm{tt: TickitTerm{tt: tt, output_hook: std::ptr::mut_null(), output_box: None}}
        }
    }
}

/*
impl Drop for MockTerm
{
    fn drop(&mut self)
    {
        unsafe
        {
            // this is the same as tickit_term_destroy
            c::tickit_mockterm_destroy(self.mt);
        }
    }
}
*/

impl MockTerm
{
    fn mt(&mut self) -> *mut c::TickitMockTerm
    {
        self.tt.tt
    }

    pub fn resize(&mut self, newlines: uint, newcols: uint)
    {
        unsafe
        {
            c::tickit_mockterm_resize(self.mt(), newlines as c_int, newcols as c_int);
        }
    }

    pub fn get_display_text(&mut self, line: uint, col: uint, width: uint) -> String
    {
        unsafe
        {
            let line = line as c_int;
            let col = col as c_int;
            let width = width as c_int;
            let n = c::tickit_mockterm_get_display_text(self.mt(), std::ptr::mut_null(), 0, line, col, width);
            let mut buf: Vec<u8> = Vec::from_fn(n as uint, |_| std::mem::uninitialized());
            let ptr: *mut c_char = std::mem::transmute(buf.as_mut_ptr());
            c::tickit_mockterm_get_display_text(self.mt(), ptr, n, line, col, width);
            collections::string::raw::from_utf8(buf)
        }
    }
    pub fn get_display_pen(&mut self, line: int, col: int) -> TickitPen
    {
        unsafe
        {
            let pen = c::tickit_mockterm_get_display_pen(self.mt(), line as c_int, col as c_int);
            let pen: &TickitPen = std::mem::transmute(&pen);
            pen.clone()
        }
    }

    pub fn loglen(&mut self) -> uint
    {
        unsafe
        {
            c::tickit_mockterm_loglen(self.mt()) as uint
        }
    }
    pub fn peeklog<'a>(&'a mut self, i: uint) -> LogEntry<'a>
    {
        unsafe
        {
            LogEntry::from_c(&*c::tickit_mockterm_peeklog(self.mt(), i as c_int))
        }
    }
    pub fn clearlog(&mut self)
    {
        unsafe
        {
            c::tickit_mockterm_clearlog(self.mt());
        }
    }

    pub fn get_position(&mut self) -> (uint, uint)
    {
        unsafe
        {
            let mut line: c_int = std::mem::uninitialized();
            let mut col: c_int = std::mem::uninitialized();
            c::tickit_mockterm_get_position(self.mt(), &mut line, &mut col);
            (line as uint, col as uint)
        }
    }
}
