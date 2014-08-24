#![feature(macro_rules)]
#![feature(unsafe_destructor)]
#![feature(struct_variant)]
#![allow(unused_variable)]
#![allow(unused_mut)]

extern crate libc;

extern crate tickit;

use tickit::TickitRect;

macro_rules! diag(
    ($($arg:tt)*) => ({
        let dst: &mut ::std::io::Writer = &mut ::std::io::stderr();
        let _ = write!(dst, "# ");
        let _ = writeln!(dst, $($arg)*);
    })
)

mod taplib
{
    pub struct Tap
    {
        nexttest: uint,
        total: uint,
        _fail: bool,
    }

    impl Tap
    {
        pub fn new() -> Tap
        {
            Tap{nexttest: 1, total: 0, _fail: false}
        }
    }

    impl Drop for Tap
    {
        fn drop(&mut self)
        {
            if self.total == 0
            {
                let t = self.nexttest - 1;
                self.plan_tests(t)
            }

            if self.total != self.nexttest - 1
            {
                diag!("Expected {} tests, got {}", self.total, self.nexttest - 1);
                self._fail = true;
            }
            if self._fail
            {
                if !::std::task::failing()
                {
                    fail!()
                }
                else
                {
                    diag!("avoiding double-fail!() ...")
                }
            }
        }
    }

    #[allow(dead_code)]
    impl Tap
    {
        pub fn plan_tests(&mut self, n: uint)
        {
            self.total = n;
            println!("1..{}", n);
        }
        pub fn finish(&mut self)
        {
            self.total = self.nexttest - 1;
        }

        pub fn pass(&mut self, name: &str)
        {
            println!("ok {} - {}", self.nexttest, name);
            self.nexttest += 1;
        }

        pub fn fail(&mut self, name: &str)
        {
            println!("not ok {} - {}", self.nexttest, name);
            self.nexttest += 1;
            self._fail = true;
        }

        pub fn ok(&mut self, cmp: bool, name: &str)
        {
            if cmp
            {
                self.pass(name);
            }
            else
            {
                self.fail(name);
            }
        }

        pub fn bypass(&mut self, count: uint, name: &str)
        {
            self.fail(name);
            self.nexttest -= 1;
            self.nexttest += count;
        }

        pub fn is_int<T: PartialEq + ::std::fmt::Show>(&mut self, got: T, expect: T, name: &str)
        {
            if got == expect
            {
                self.ok(true, name);
            }
            else
            {
                self.ok(false, name);
                diag!("got {} expected {} in: {}", got, expect, name);
            }
        }
        pub fn is_str<T: Str, U: Str>(&mut self, got: T, expect: U, name: &str)
        {
            let got = got.as_slice();
            let expect = expect.as_slice();

            if got == expect
            {
                self.ok(true, name);
            }
            else
            {
                self.ok(false, name);
                // differs from generic version by the ''s.
                diag!("got '{}' expected '{}' in: {}", got, expect, name);
            }
        }

        pub fn is_str_escape<T: Str, U: Str>(&mut self, got: T, expect: U, name: &str)
        {
            let got = got.as_slice();
            let expect = expect.as_slice();

            if got == expect
            {
                self.ok(true, name);
            }
            else
            {
                self.ok(false, name);
                diag!("got \"{}\" expected \"{}\" in: {}", got.escape_default(), expect.escape_default(), name);
            }
        }
    }
}

#[test]
fn test_01string()
{
    let mut tap = taplib::Tap::new();

    let mut pos = tickit::StringPos::zero();

    tap.is_int(tickit::StringPos::count("hello", &mut pos, None), 5, "tickit::StringPos::count ASCII");
    tap.is_int(pos.bytes,      5, "tickit::StringPos::count ASCII bytes");
    tap.is_int(pos.codepoints, 5, "tickit::StringPos::count ASCII codepoints");
    tap.is_int(pos.graphemes,  5, "tickit::StringPos::count ASCII graphemes");
    tap.is_int(pos.columns,    5, "tickit::StringPos::count ASCII columns");

    /* U+00E9 - LATIN SMALL LETTER E WITH ACUTE
    * 0xc3 0xa9
    */
    tap.is_int(tickit::StringPos::count("caf\u00e9", &mut pos, None), 5, "tickit::StringPos::count UTF-8");
    tap.is_int(pos.bytes,      5, "tickit::StringPos::count UTF-8 bytes");
    tap.is_int(pos.codepoints, 4, "tickit::StringPos::count UTF-8 codepoints");
    tap.is_int(pos.graphemes,  4, "tickit::StringPos::count UTF-8 graphemes");
    tap.is_int(pos.columns,    4, "tickit::StringPos::count UTF-8 columns");

    /* U+0301 - COMBINING ACUTE ACCENT
    * 0xcc 0x81
    */
    tap.is_int(tickit::StringPos::count("cafe\u0301", &mut pos, None), 6, "tickit::StringPos::count UTF-8 combining");
    tap.is_int(pos.bytes,      6, "tickit::StringPos::count UTF-8 combining bytes");
    tap.is_int(pos.codepoints, 5, "tickit::StringPos::count UTF-8 combining codepoints");
    tap.is_int(pos.graphemes,  4, "tickit::StringPos::count UTF-8 combining graphemes");
    tap.is_int(pos.columns,    4, "tickit::StringPos::count UTF-8 combining columns");

    /* U+5F61 - CJK UNIFIED IDEOGRAPH-5F61
    * 0xe5 0xbd 0xa1
    */
    tap.is_int(tickit::StringPos::count("\u5f61", &mut pos, None), 3, "tickit::StringPos::count UTF-8 CJK");
    tap.is_int(pos.bytes,      3, "tickit::StringPos::count UTF-8 CJK bytes");
    tap.is_int(pos.codepoints, 1, "tickit::StringPos::count UTF-8 CJK codepoints");
    tap.is_int(pos.graphemes,  1, "tickit::StringPos::count UTF-8 CJK graphemes");
    tap.is_int(pos.columns,    2, "tickit::StringPos::count UTF-8 CJK columns");

    /* U+FF21 - FULLWIDTH LATIN CAPITAL LETTER A
    * 0xef 0xbc 0xa1
    */
    tap.is_int(tickit::StringPos::count("\uff21", &mut pos, None), 3, "tickit::StringPos::count UTF-8 fullwidth");
    tap.is_int(pos.bytes,      3, "tickit::StringPos::count UTF-8 fullwidth bytes");
    tap.is_int(pos.codepoints, 1, "tickit::StringPos::count UTF-8 fullwidth codepoints");
    tap.is_int(pos.graphemes,  1, "tickit::StringPos::count UTF-8 fullwidth graphemes");
    tap.is_int(pos.columns,    2, "tickit::StringPos::count UTF-8 fullwidth columns");

    /* And now a nice long string */
    tap.is_int(tickit::StringPos::count("(\u30ce\u0ca0\u7ca0)\u30ce\u5f61\u253b\u2501\u253b", &mut pos, None),
      26, "tickit::StringPos::count UTF-8 string");
    tap.is_int(pos.bytes,      26, "tickit::StringPos::count UTF-8 string bytes");
    tap.is_int(pos.codepoints, 10, "tickit::StringPos::count UTF-8 string codepoints");
    tap.is_int(pos.graphemes,  10, "tickit::StringPos::count UTF-8 string graphemes");
    tap.is_int(pos.columns,    14, "tickit::StringPos::count UTF-8 string columns");

    /* Now with some limits */

    let mut limit = tickit::StringPos::limit_bytes(5);

    tap.is_int(tickit::StringPos::count("hello world", &mut pos, Some(limit)), 5, "tickit::StringPos::count byte-limit");
    tap.is_int(pos.bytes,      5, "tickit::StringPos::count byte-limit bytes");
    tap.is_int(pos.codepoints, 5, "tickit::StringPos::count byte-limit codepoints");
    tap.is_int(pos.graphemes,  5, "tickit::StringPos::count byte-limit graphemes");
    tap.is_int(pos.columns,    5, "tickit::StringPos::count byte-limit columns");

    /* check byte limit never chops UTF-8 codepoints */
    limit.bytes = 4;
    tap.is_int(tickit::StringPos::count("caf\u00e9", &mut pos, Some(limit)), 3, "tickit::StringPos::count byte-limit split");
    tap.is_int(pos.bytes,      3, "tickit::StringPos::count byte-limit split bytes");

    let mut limit = tickit::StringPos::limit_codepoints(3);

    tap.is_int(tickit::StringPos::count("hello world", &mut pos, Some(limit)), 3, "tickit::StringPos::count char-limit");
    tap.is_int(pos.bytes,      3, "tickit::StringPos::count char-limit bytes");
    tap.is_int(pos.codepoints, 3, "tickit::StringPos::count char-limit codepoints");
    tap.is_int(pos.graphemes,  3, "tickit::StringPos::count char-limit graphemes");
    tap.is_int(pos.columns,    3, "tickit::StringPos::count char-limit columns");

    /* check char limit never chops graphemes */
    limit.codepoints = 4;
    tap.is_int(tickit::StringPos::count("cafe\u0301", &mut pos, Some(limit)), 3, "tickit::StringPos::count char-limit split");
    tap.is_int(pos.codepoints, 3, "tickit::StringPos::count char-limit split codepoints");

    let limit = tickit::StringPos::limit_graphemes(4);

    tap.is_int(tickit::StringPos::count("hello world", &mut pos, Some(limit)), 4, "tickit::StringPos::count grapheme-limit");
    tap.is_int(pos.bytes,      4, "tickit::StringPos::count grapheme-limit bytes");
    tap.is_int(pos.codepoints, 4, "tickit::StringPos::count grapheme-limit codepoints");
    tap.is_int(pos.graphemes,  4, "tickit::StringPos::count grapheme-limit graphemes");
    tap.is_int(pos.columns,    4, "tickit::StringPos::count grapheme-limit columns");

    let mut limit = tickit::StringPos::limit_columns(6);

    tap.is_int(tickit::StringPos::count("hello world", &mut pos, Some(limit)), 6, "tickit::StringPos::count column-limit");
    tap.is_int(pos.bytes,      6, "tickit::StringPos::count column-limit bytes");
    tap.is_int(pos.codepoints, 6, "tickit::StringPos::count column-limit codepoints");
    tap.is_int(pos.graphemes,  6, "tickit::StringPos::count column-limit graphemes");
    tap.is_int(pos.columns,    6, "tickit::StringPos::count column-limit columns");

    /* check column limit never chops graphemes */
    limit.columns = 2;
    tap.is_int(tickit::StringPos::count("A\uff21", &mut pos, Some(limit)), 1, "tickit::StringPos::count column-limit split");
    tap.is_int(pos.columns,    1, "tickit::StringPos::count column-limit split grapheme");

    /* countmore should continue where count left off */
    let mut limit = tickit::StringPos::limit_columns(3);
    tickit::StringPos::count("cafe\u0301", &mut pos, Some(limit));
    limit.columns += 1;
    tickit::StringPos::countmore("cafe\u0301", &mut pos, Some(limit));
    tap.is_int(pos.bytes, 6, "tickit::StringPos::countmore continues after count");

    /* ncount */
    // The Rust wrapper uses these exclusively.
    // If they didn't work, *everything* would break.
    tap.pass("skip ncount test 1");
    tap.pass("skip ncount test 2");

    /* C0 and C1 controls and ASCII DEL are errors */
    let limit = tickit::StringPos::limit_none();

    tap.is_int(tickit::StringPos::count("\u001b", &mut pos, Some(limit)), -1, "tickit::StringPos::count -1 for C0");
    tap.is_int(tickit::StringPos::x_bcount(b"\x9b", &mut pos, Some(limit)), -1, "tickit::StringPos::count -1 for bare C1");
    tap.is_int(tickit::StringPos::count("\u009b", &mut pos, Some(limit)), -1, "tickit::StringPos::count -1 for encoded C1");
    tap.is_int(tickit::StringPos::count("\u007f", &mut pos, Some(limit)), -1, "tickit::StringPos::count -1 for DEL");

    /* convenience utilities */
    tap.is_int(tickit::mbswidth("caf\u00e9 time"), 9, "tickit_string_mbswidth");
    tap.is_int(tickit::byte2col("caf\u00e9 time", 7), 6, "tickit_string_byte2col");
    tap.is_int(tickit::col2byte("caf\u00e9 time", 6), 7, "tickit_string_col2byte");
}

#[test]
fn test_02pen()
{
    static mut changed: int = 0;

    fn on_changed(_: &mut tickit::TickitPen, _: &tickit::TickitEvent)
    {
        unsafe { changed += 1; }
    }

    let mut tap = taplib::Tap::new();

    {
        let mut pen = tickit::TickitPen::new();

        tap.pass("tickit::TickitPen::new");

        pen.x_bind_event_forever(tickit::c::TICKIT_EV_CHANGE, on_changed);

        tap.is_int(tickit::c::TICKIT_PEN_BOLD.attrtype(), tickit::c::TICKIT_PENTYPE_BOOL, "bold is a boolean attribute");

        tap.is_int(tickit::c::TickitPenAttr::lookup_attr("b"), tickit::c::TICKIT_PEN_BOLD, "lookup_attr \"b\" gives bold");
        tap.is_str(tickit::c::TICKIT_PEN_BOLD.attrname(), "b", "pen_attrname bold gives \"b\"");

        tap.is_int(unsafe { changed }, 0, "change counter 0 initially");

        tap.ok(!pen.has_attr(tickit::c::TICKIT_PEN_BOLD), "pen lacks bold initially");
        tap.ok(!pen.nondefault_attr(tickit::c::TICKIT_PEN_BOLD), "pen bold attr is default initially");
        tap.is_int(pen.get_bool_attr(tickit::c::TICKIT_PEN_BOLD), false, "bold 0 initially");

        tap.ok(!pen.is_nonempty(), "pen initially empty");
        tap.ok(!pen.is_nondefault(), "pen initially default");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, true);

        tap.ok(pen.has_attr(tickit::c::TICKIT_PEN_BOLD), "pen has bold after set");
        tap.ok(pen.nondefault_attr(tickit::c::TICKIT_PEN_BOLD), "pen bold attr is nondefault after set");
        tap.is_int(pen.get_bool_attr(tickit::c::TICKIT_PEN_BOLD), true, "bold 1 after set");

        tap.ok(pen.is_nonempty(), "pen non-empty after set bold on");
        tap.ok(pen.is_nondefault(), "pen non-default after set bold on");

        tap.is_int(unsafe { changed }, 1, "change counter 1 after set bold on");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, false);

        tap.ok(!pen.nondefault_attr(tickit::c::TICKIT_PEN_BOLD), "pen bold attr is default after set bold off");

        tap.ok(pen.is_nonempty(), "pen non-empty after set bold off");
        tap.ok(!pen.is_nondefault(), "pen default after set bold off");

        tap.is_int(unsafe { changed }, 2, "change counter 2 after set bold off");

        pen.clear_attr(tickit::c::TICKIT_PEN_BOLD);

        tap.ok(!pen.has_attr(tickit::c::TICKIT_PEN_BOLD), "pen lacks bold after clear");
        tap.is_int(pen.get_bool_attr(tickit::c::TICKIT_PEN_BOLD), false, "bold 0 after clear");

        tap.is_int(unsafe { changed }, 3, "change counter 3 after clear bold");

        tap.is_int(tickit::c::TICKIT_PEN_FG.attrtype(), tickit::c::TICKIT_PENTYPE_COLOUR, "foreground is a colour attribute");

        tap.ok(!pen.has_attr(tickit::c::TICKIT_PEN_FG), "pen lacks foreground initially");
        tap.is_int(pen.get_colour_attr(tickit::c::TICKIT_PEN_FG), -1, "foreground -1 initially");

        pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 4);

        tap.ok(pen.has_attr(tickit::c::TICKIT_PEN_FG), "pen has foreground after set");
        tap.is_int(pen.get_colour_attr(tickit::c::TICKIT_PEN_FG), 4, "foreground 4 after set");

        tap.ok(pen.set_colour_attr_desc(tickit::c::TICKIT_PEN_FG, "12"), "pen set foreground '12'");
        tap.is_int(pen.get_colour_attr(tickit::c::TICKIT_PEN_FG), 12, "foreground 12 after set '12'");

        tap.ok(pen.set_colour_attr_desc(tickit::c::TICKIT_PEN_FG, "green"), "pen set foreground 'green'");
        tap.is_int(pen.get_colour_attr(tickit::c::TICKIT_PEN_FG), 2, "foreground 2 after set 'green'");

        tap.ok(pen.set_colour_attr_desc(tickit::c::TICKIT_PEN_FG, "hi-red"), "pen set foreground 'hi-red'");
        tap.is_int(pen.get_colour_attr(tickit::c::TICKIT_PEN_FG), 8+1, "foreground 8+1 after set 'hi-red'");

        pen.clear_attr(tickit::c::TICKIT_PEN_FG);

        tap.ok(!pen.has_attr(tickit::c::TICKIT_PEN_FG), "pen lacks foreground after clear");
        tap.is_int(pen.get_colour_attr(tickit::c::TICKIT_PEN_FG), -1, "foreground -1 after clear");

        let mut pen2 = tickit::TickitPen::new();

        tap.ok(pen.equiv_attr(&pen2, tickit::c::TICKIT_PEN_BOLD), "pens have equiv bold attribute initially");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, true);

        tap.ok(!pen.equiv_attr(&pen2, tickit::c::TICKIT_PEN_BOLD), "pens have unequiv bold attribute after set");

        tap.ok(pen.equiv_attr(&pen2, tickit::c::TICKIT_PEN_ITALIC), "pens have equiv italic attribute");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_ITALIC, false);
        tap.ok(pen.equiv_attr(&pen2, tickit::c::TICKIT_PEN_ITALIC), "pens have equiv italic attribute after set 0");

        pen2.copy_attr(&pen, tickit::c::TICKIT_PEN_BOLD);
        tap.ok(pen.equiv_attr(&pen2, tickit::c::TICKIT_PEN_BOLD), "pens have equiv bold attribute after copy attr");

        pen2.clear_attr(tickit::c::TICKIT_PEN_BOLD);
        pen2.copy(&pen, true);

        tap.ok(pen.equiv_attr(&pen2, tickit::c::TICKIT_PEN_BOLD), "pens have equiv bold attribute after copy");

        pen2.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, false);
        pen2.copy(&pen, false);

        tap.ok(!pen.equiv_attr(&pen2, tickit::c::TICKIT_PEN_BOLD), "pens have non-equiv bold attribute after copy no overwrite");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_UNDER, false);
        pen2.clear_attr(tickit::c::TICKIT_PEN_UNDER);
        pen2.copy(&pen, true);

        tap.ok(pen2.has_attr(tickit::c::TICKIT_PEN_UNDER), "pen copy still copies present but default-value attributes");
    }

    tap.pass("tickit_pen_destroy");

    let mut pen = tickit::TickitPen::new().with_bool_attr(tickit::c::TICKIT_PEN_BOLD, true).with_colour_attr(tickit::c::TICKIT_PEN_FG, 3);
    tap.is_int(pen.get_bool_attr(tickit::c::TICKIT_PEN_BOLD), true, "pen bold attr for new_attrs");
    tap.is_int(pen.get_colour_attr(tickit::c::TICKIT_PEN_FG), 3, "pen fg attr for new_attrs");
}

fn rect_init_strp(str_: &str) -> tickit::TickitRect
{
    if str_.contains("..")
    {
        let ((left, top), (right, bottom)) =
        {
            let mut outer = str_.split_str("..").map(
                |x|
                {
                    let mut inner = x.split(',').map(
                        |y|
                        {
                            from_str::<int>(y).unwrap()
                        }
                    );
                    (inner.next().unwrap(), inner.next().unwrap())
                }
            );
            (outer.next().unwrap(), outer.next().unwrap())
        };
        tickit::TickitRect::init_bounded(top, left, bottom, right)
    }
    else
    {
        let ((left, top), (cols, lines)) =
        {
            let mut outer = str_.split('+').map(
                |x|
                {
                    let mut inner = x.split(',').map(
                        |y|
                        {
                            from_str::<int>(y).unwrap()
                        }
                    );
                    (inner.next().unwrap(), inner.next().unwrap())
                }
            );
            (outer.next().unwrap(), outer.next().unwrap())
        };
        tickit::TickitRect::init_sized(top, left, lines, cols)
    }
}

trait Show2
{
    fn s(self) -> String;
}
impl Show2 for tickit::TickitRect
{
    fn s(self) -> String
    {
        format!("{},{}..{},{}", self.left, self.top, self.right(), self.bottom())
    }
}

impl taplib::Tap
{
    pub fn is_rect(&mut self, got: tickit::TickitRect, expect: &str, name: &str)
    {
        self.is_int(got.s(), rect_init_strp(expect).s(), name)
    }
}

#[test]
fn test_03rect()
{
    let mut tap = taplib::Tap::new();

    let rect1 = tickit::TickitRect::init_sized(5, 10, 7, 20);

    tap.is_int(rect1.top,    5, "rect1.top");
    tap.is_int(rect1.left,  10, "rect1.left");
    tap.is_int(rect1.lines,  7, "rect1.lines");
    tap.is_int(rect1.cols,  20, "rect1.cols");
    tap.is_int(rect1.bottom(), 12, "rect1.bottom()");
    tap.is_int(rect1.right(),  30, "rect1.right()");

    let rect2 = tickit::TickitRect::init_sized(0, 0, 25, 80);
    let rectOut = rect1.intersect(&rect2).unwrap();

    tap.is_int(rectOut.top,    5, "rectOut.top from intersect wholescreen");
    tap.is_int(rectOut.left,  10, "rectOut.left from intersect wholescreen");
    tap.is_int(rectOut.lines,  7, "rectOut.lines from intersect wholescreen");
    tap.is_int(rectOut.cols,  20, "rectOut.cols from intersect wholescreen");
    tap.is_int(rectOut.bottom(), 12, "rectOut.bottom() from intersect wholescreen");
    tap.is_int(rectOut.right(),  30, "rectOut.right() from intersect wholescreen");

    let rect2 = tickit::TickitRect::init_sized(10, 20, 15, 60);
    let rectOut = rect1.intersect(&rect2).unwrap();

    tap.is_int(rectOut.top,   10, "rectOut.top from intersect partial");
    tap.is_int(rectOut.left,  20, "rectOut.left from intersect partial");
    tap.is_int(rectOut.lines,  2, "rectOut.lines from intersect partial");
    tap.is_int(rectOut.cols,  10, "rectOut.cols from intersect partial");
    tap.is_int(rectOut.bottom(), 12, "rectOut.bottom() from intersect partial");
    tap.is_int(rectOut.right(),  30, "rectOut.right() from intersect partial");

    let rect2 = tickit::TickitRect::init_sized(20, 20, 5, 60);
    tap.ok(rect1.intersect(&rect2).is_none(), "false from intersect outside");

    let rect2 = tickit::TickitRect::init_sized(7, 12, 3, 10);
    tap.ok(rect1.contains(&rect2), "tickit_rect_contains() for smaller");

    let rect2 = tickit::TickitRect::init_sized(3, 10, 5, 12);
    tap.ok(!rect1.contains(&rect2), "tickit_rect_contains() for overlap");

    let rect2 = tickit::TickitRect::init_sized(3, 10, 5, 12);
    tap.ok(rect1.intersects(&rect2), "tickit_rect_intersects() with overlap");

    let rect2 = tickit::TickitRect::init_sized(14, 10, 3, 20);
    tap.ok(!rect1.intersects(&rect2), "tickit_rect_intersects() with other");

    let rect2 = tickit::TickitRect::init_sized(12, 10, 3, 20);
    tap.ok(!rect1.intersects(&rect2), "tickit_rect_intersects() with abutting");

    let rect1 = tickit::TickitRect::init_bounded(3, 8, 9, 22);

    tap.is_int(rect1.top,    3, "rect1.top from init_bounded");
    tap.is_int(rect1.left,   8, "rect1.left from init_bounded");
    tap.is_int(rect1.lines,  6, "rect1.lines from init_bounded");
    tap.is_int(rect1.cols,  14, "rect1.cols from init_bounded");
    tap.is_int(rect1.bottom(),  9, "rect1.bottom() from init_bounded");
    tap.is_int(rect1.right(),  22, "rect1.right() from init_bounded");


    // Rectangle addition

    let rect1 = rect_init_strp("10,10..20,20");

    let rects = rect1.add(&rect_init_strp("10,10..20,20"));
    tap.is_int(rects.len(), 1, "rect_add same");
    tap.is_rect(rects[0], "10,10..20,20", "rects[0] for rect_add same");

    let rects = rect1.add(&rect_init_strp("5,10..10,20"));
    tap.is_int(rects.len(), 1, "rect_add left");
    tap.is_rect(rects[0], "5,10..20,20", "rects[0] for rect_add left");

    let rects = rect1.add(&rect_init_strp("20,10..25,20"));
    tap.is_int(rects.len(), 1, "rect_add right");
    tap.is_rect(rects[0], "10,10..25,20", "rects[0] for rect_add right");

    let rects = rect1.add(&rect_init_strp("10,5..20,10"));
    tap.is_int(rects.len(), 1, "rect_add top");
    tap.is_rect(rects[0], "10,5..20,20", "rects[0] for rect_add top");

    let rects = rect1.add(&rect_init_strp("10,20..20,25"));
    tap.is_int(rects.len(), 1, "rect_add bottom");
    tap.is_rect(rects[0], "10,10..20,25", "rects[0] for rect_add bottom");

    let rects = rect1.add(&rect_init_strp("12,5..18,10"));
    tap.is_int(rects.len(), 2, "rect_add T above");
    tap.is_rect(rects[0], "12,5..18,10", "rects[0] for rect_add T above");
    tap.is_rect(rects[1], "10,10..20,20", "rects[1] for rect_add T above");

    let rects = rect1.add(&rect_init_strp("12,20..18,30"));
    tap.is_int(rects.len(), 2, "rect_add T below");
    tap.is_rect(rects[0], "10,10..20,20", "rects[0] for rect_add T below");
    tap.is_rect(rects[1], "12,20..18,30", "rects[1] for rect_add T below");

    let rects = rect1.add(&rect_init_strp("5,12..10,18"));
    tap.is_int(rects.len(), 3, "rect_add T left");
    tap.is_rect(rects[0], "10,10..20,12", "rects[0] for rect_add T left");
    tap.is_rect(rects[1], "5,12..20,18", "rects[1] for rect_add T left");
    tap.is_rect(rects[2], "10,18..20,20", "rects[2] for rect_add T left");

    let rects = rect1.add(&rect_init_strp("20,12..25,18"));
    tap.is_int(rects.len(), 3, "rect_add T right");
    tap.is_rect(rects[0], "10,10..20,12", "rects[0] for rect_add T right");
    tap.is_rect(rects[1], "10,12..25,18", "rects[1] for rect_add T right");
    tap.is_rect(rects[2], "10,18..20,20", "rects[2] for rect_add T right");

    let rects = rect1.add(&rect_init_strp("15,15..25,25"));
    tap.is_int(rects.len(), 3, "rect_add diagonal");
    tap.is_rect(rects[0], "10,10..20,15", "rects[0] for rect_add diagonal");
    tap.is_rect(rects[1], "10,15..25,20", "rects[1] for rect_add diagonal");
    tap.is_rect(rects[2], "15,20..25,25", "rects[2] for rect_add diagonal");

    let rects = rect1.add(&rect_init_strp("12,8..18,22"));
    tap.is_int(rects.len(), 3, "rect_add cross");
    tap.is_rect(rects[0], "12,8..18,10", "rects[0] for rect_add cross");
    tap.is_rect(rects[1], "10,10..20,20", "rects[1] for rect_add cross");
    tap.is_rect(rects[2], "12,20..18,22", "rects[2] for rect_add cross");

    let rects = rect1.add(&rect_init_strp("10,30..20,40"));
    tap.is_int(rects.len(), 2, "rect_add non-overlap horizontal");
    tap.is_rect(rects[0], "10,10..20,20", "rects[0] for rect_add non-overlap horizontal");
    tap.is_rect(rects[1], "10,30..20,40", "rects[1] for rect_add non-overlap horizontal");

    let rects = rect1.add(&rect_init_strp("30,10..40,20"));
    tap.is_int(rects.len(), 2, "rect_add non-overlap vertical");
    tap.is_rect(rects[0], "10,10..20,20", "rects[0] for rect_add non-overlap vertical");
    tap.is_rect(rects[1], "30,10..40,20", "rects[1] for rect_add non-overlap vertical");

    // Rectangle subtraction

    let rects = rect1.subtract(&rect_init_strp("10,10..20,20"));
    tap.is_int(rects.len(), 0, "rect_subtract self");

    let rects = rect1.subtract(&rect_init_strp("5,10..15,20"));
    tap.is_int(rects.len(), 1, "rect_subtract truncate left");
    tap.is_rect(rects[0], "15,10..20,20", "rects[0] for rect_subtract truncate left");

    let rects = rect1.subtract(&rect_init_strp("15,10..25,20"));
    tap.is_int(rects.len(), 1, "rect_subtract truncate right");
    tap.is_rect(rects[0], "10,10..15,20", "rects[0] for rect_subtract truncate right");

    let rects = rect1.subtract(&rect_init_strp("10,5..20,15"));
    tap.is_int(rects.len(), 1, "rect_subtract truncate top");
    tap.is_rect(rects[0], "10,15..20,20", "rects[0] for rect_subtract truncate top");

    let rects = rect1.subtract(&rect_init_strp("10,15..20,25"));
    tap.is_int(rects.len(), 1, "rect_subtract truncate bottom");
    tap.is_rect(rects[0], "10,10..20,15", "rects[0] for rect_subtract truncate bottom");

    let rects = rect1.subtract(&rect_init_strp("5,12..25,18"));
    tap.is_int(rects.len(), 2, "rect_subtract slice horizontal");
    tap.is_rect(rects[0], "10,10..20,12", "rects[0] for rect_subtract slice horizontal");
    tap.is_rect(rects[1], "10,18..20,20", "rects[1] for rect_subtract slice horizontal");

    let rects = rect1.subtract(&rect_init_strp("12,5..18,25"));
    tap.is_int(rects.len(), 2, "rect_subtract slice vertical");
    tap.is_rect(rects[0], "10,10..12,20", "rects[0] for rect_subtract slice vertical");
    tap.is_rect(rects[1], "18,10..20,20", "rects[1] for rect_subtract slice vertical");

    let rects = rect1.subtract(&rect_init_strp("5,12..15,18"));
    tap.is_int(rects.len(), 3, "rect_subtract U left");
    tap.is_rect(rects[0], "10,10..20,12", "rects[0] for rect_subtract U left");
    tap.is_rect(rects[1], "15,12..20,18", "rects[1] for rect_subtract U left");
    tap.is_rect(rects[2], "10,18..20,20", "rects[2] for rect_subtract U left");

    let rects = rect1.subtract(&rect_init_strp("15,12..25,18"));
    tap.is_int(rects.len(), 3, "rect_subtract U right");
    tap.is_rect(rects[0], "10,10..20,12", "rects[0] for rect_subtract U right");
    tap.is_rect(rects[1], "10,12..15,18", "rects[1] for rect_subtract U right");
    tap.is_rect(rects[2], "10,18..20,20", "rects[2] for rect_subtract U right");

    let rects = rect1.subtract(&rect_init_strp("12,5..18,15"));
    tap.is_int(rects.len(), 3, "rect_subtract U top");
    tap.is_rect(rects[0], "10,10..12,15", "rects[0] for rect_subtract U top");
    tap.is_rect(rects[1], "18,10..20,15", "rects[1] for rect_subtract U top");
    tap.is_rect(rects[2], "10,15..20,20", "rects[2] for rect_subtract U top");

    let rects = rect1.subtract(&rect_init_strp("12,15..18,25"));
    tap.is_int(rects.len(), 3, "rect_subtract U bottom");
    tap.is_rect(rects[0], "10,10..20,15", "rects[0] for rect_subtract U bottom");
    tap.is_rect(rects[1], "10,15..12,20", "rects[1] for rect_subtract U bottom");
    tap.is_rect(rects[2], "18,15..20,20", "rects[2] for rect_subtract U bottom");

    let rects = rect1.subtract(&rect_init_strp("12,12..18,18"));
    tap.is_int(rects.len(), 4, "rect_subtract hole");
    tap.is_rect(rects[0], "10,10..20,12", "rects[0] for rect_subtract hole");
    tap.is_rect(rects[1], "10,12..12,18", "rects[1] for rect_subtract hole");
    tap.is_rect(rects[2], "18,12..20,18", "rects[2] for rect_subtract hole");
    tap.is_rect(rects[3], "10,18..20,20", "rects[3] for rect_subtract hole");
}

fn rects_init_strp(str_: &str) -> Vec<tickit::TickitRect>
{
    str_.split(' ').filter(|s| !s.is_empty()).map(|s| rect_init_strp(s)).collect()
}

fn diag_eq(got: &tickit::TickitRect, expect: &tickit::TickitRect) -> bool
{
    if got != expect
    {
        diag!("got {} expected {}", got.s(), expect.s());
    }
    got == expect
}

#[test]
fn test_04rectset()
{
    let mut tap = taplib::Tap::new();

    {
        let mut trs = tickit::TickitRectSet::new();

        tap.pass("tickit::TickitRectSet::new");

        let rects = trs.get_rects();
        tap.is_int(rects.len(), 0, "tickit_rectset_get_rects initially");

        // Distinct regions
        trs.add(&rect_init_strp("10,10+20,5"));

        let rects = trs.get_rects();
        tap.is_int(rects.len(), 1, "tickit_rectset_get_rects after tickit_rectset_add");
        tap.is_rect(rects[0], "10,10+20,5", "rects[0] after tickit_rectset_add");

        trs.add(&rect_init_strp("10,20+20,2"));

        let rects = trs.get_rects();
        tap.is_int(rects.len(), 2, "tickit_rectset_get_rects after second tickit_rectset_add");
        tap.is_rect(rects[0], "10,10+20,5", "rects[0] after second tickit_rectset_add");
        tap.is_rect(rects[1], "10,20+20,2", "rects[1] after second tickit_rectset_add");

        trs.clear();

        let rects = trs.get_rects();
        tap.is_int(rects.len(), 0, "tickit_rectset_rects after clear");

        // Intersect and containment
        trs.add(&rect_init_strp("1,1..20,5"));
        trs.add(&rect_init_strp("1,5..10,10"));

        tap.ok( trs.intersects(&rect_init_strp("0,0..5,5")), "intersects overlap");
        tap.ok(!trs.intersects(&rect_init_strp("15,6..25,9")), "doesn't intersect no overlap");

        tap.ok( trs.contains(&rect_init_strp("5,1..15,4")), "contains simple");
        tap.ok( trs.contains(&rect_init_strp("5,2..8,9")), "contains split");
        tap.ok(!trs.contains(&rect_init_strp("5,2..12,9")), "doesn't contain split");
        tap.ok(!trs.contains(&rect_init_strp("15,6..25,9")), "doesn't contain non-intersect");
    }


    {
        let test_add = |input: &str, output: &str|
        {
            let mut trs = tickit::TickitRectSet::new();

            let inputs = rects_init_strp(input);

            let exp_outputs = rects_init_strp(output);

            for &reverse in [false, true].iter()
            {
                trs.clear();

                if reverse
                {
                    for i in inputs.iter().rev()
                    {
                        trs.add(i);
                    }
                }
                else
                {
                    for i in inputs.iter()
                    {
                        trs.add(i);
                    }
                }

                let got_outputs = trs.get_rects();

                let c1 = got_outputs.iter().zip(exp_outputs.iter()).all(|(got, exp)| diag_eq(got, exp));

                if exp_outputs.len() > got_outputs.len()
                {
                    diag!("Received too many rects");
                }
                if exp_outputs.len() < got_outputs.len()
                {
                    diag!("Received insufficient rects");
                }
                let c2 = exp_outputs.len() == got_outputs.len();

                tap.ok(c1 && c2, if reverse { "tickit_rectset_add reversed" } else { "tickit_rectset_add" });
            }
        };

        // Distinct regions
        test_add("10,10..30,15 40,10..60,15", "10,10..30,15 40,10..60,15");
        test_add("10,10..30,15 10,20..30,25", "10,10..30,15 10,20..30,25");

        // Ignorable regions
        test_add("10,10..30,15 10,10..30,15", "10,10..30,15");
        test_add("10,10..30,15 10,10..20,12", "10,10..30,15");
        test_add("10,10..30,15 20,13..30,15", "10,10..30,15");
        test_add("10,10..30,15 15,11..25,14", "10,10..30,15");

        // Overlapping extension top
        test_add("10,10..30,15 10,8..30,12", "10,8..30,15");
        test_add("10,10..30,15 10,8..30,10", "10,8..30,15");
        test_add("10,10..30,12 10,15..30,17 10,12..30,15", "10,10..30,17");

        // Overlapping extension bottom
        test_add("10,10..30,15 10,12..30,17", "10,10..30,17");
        test_add("10,10..30,15 10,15..30,17", "10,10..30,17");

        // Overlapping extension left
        test_add("10,10..30,15 5,10..25,15", "5,10..30,15");
        test_add("10,10..30,15 5,10..10,15", "5,10..30,15");

        // Overlapping extension right
        test_add("10,10..30,15 20,10..35,15", "10,10..35,15");
        test_add("10,10..30,15 30,10..35,15", "10,10..35,15");

        // L/T shape top abutting
        test_add("10,10..30,15 10,8..20,10", "10,8..20,10 10,10..30,15");
        test_add("10,10..30,15 15,8..25,10", "15,8..25,10 10,10..30,15");
        test_add("10,10..30,15 20,8..30,10", "20,8..30,10 10,10..30,15");

        // L/T shape top overlapping
        test_add("10,10..30,15 10,8..20,12", "10,8..20,10 10,10..30,15");
        test_add("10,10..30,15 15,8..25,12", "15,8..25,10 10,10..30,15");
        test_add("10,10..30,15 20,8..30,12", "20,8..30,10 10,10..30,15");

        // L/T shape bottom abutting
        test_add("10,10..30,15 10,15..20,17", "10,10..30,15 10,15..20,17");
        test_add("10,10..30,15 15,15..25,17", "10,10..30,15 15,15..25,17");
        test_add("10,10..30,15 20,15..30,17", "10,10..30,15 20,15..30,17");

        // L/T shape bottom overlapping
        test_add("10,10..30,15 10,13..20,17", "10,10..30,15 10,15..20,17");
        test_add("10,10..30,15 15,13..25,17", "10,10..30,15 15,15..25,17");
        test_add("10,10..30,15 20,13..30,17", "10,10..30,15 20,15..30,17");

        // L/T shape left abutting
        test_add("10,10..30,15 5,10..10,12", "5,10..30,12 10,12..30,15");
        test_add("10,10..30,15 5,11..10,14", "10,10..30,11 5,11..30,14 10,14..30,15");
        test_add("10,10..30,15 5,13..10,15", "10,10..30,13 5,13..30,15");

        // L/T shape left overlapping
        test_add("10,10..30,15 5,10..15,12", "5,10..30,12 10,12..30,15");
        test_add("10,10..30,15 5,11..15,14", "10,10..30,11 5,11..30,14 10,14..30,15");
        test_add("10,10..30,15 5,13..15,15", "10,10..30,13 5,13..30,15");

        // L/T shape right abutting
        test_add("10,10..30,15 30,10..35,12", "10,10..35,12 10,12..30,15");
        test_add("10,10..30,15 30,11..35,14", "10,10..30,11 10,11..35,14 10,14..30,15");
        test_add("10,10..30,15 30,13..35,15", "10,10..30,13 10,13..35,15");

        // L/T shape right overlapping
        test_add("10,10..30,15 20,10..35,12", "10,10..35,12 10,12..30,15");
        test_add("10,10..30,15 20,11..35,14", "10,10..30,11 10,11..35,14 10,14..30,15");
        test_add("10,10..30,15 20,13..35,15", "10,10..30,13 10,13..35,15");

        // Cross shape
        test_add("10,10..30,15 15,5..25,20", "15,5..25,10 10,10..30,15 15,15..25,20");

        // Diagonal overlap
        test_add("10,10..30,15 20,12..40,20", "10,10..30,12 10,12..40,15 20,15..40,20");
        test_add("10,10..30,15  0,12..20,20", "10,10..30,12  0,12..30,15  0,15..20,20");
    }


    {
        let test_subtract = |input: &str, output: &str|
        {
            let mut trs = tickit::TickitRectSet::new();

            let inputs = rects_init_strp(input);

            trs.add(&inputs[0]);

            for i in inputs.iter().skip(1)
            {
                trs.subtract(i);
            }

            let exp_outputs = rects_init_strp(output);

            let got_outputs = trs.get_rects();

            let c1 = got_outputs.iter().zip(exp_outputs.iter()).all(|(got, exp)| diag_eq(got, exp));

            if exp_outputs.len() > got_outputs.len()
            {
                diag!("Received too many rects");
            }
            if exp_outputs.len() < got_outputs.len()
            {
                diag!("Received insufficient rects");
            }
            let c2 = exp_outputs.len() == got_outputs.len();

            tap.ok(c1 && c2, "tickit_rectset_subtract");
        };

        // Distinct regions
        test_subtract("10,10..30,15 10,20..30,22", "10,10..30,15");

        // Overlapping truncate left
        test_subtract("10,10..30,15 5,10..20,15", "20,10..30,15");

        // Overlapping truncate right
        test_subtract("10,10..30,15 20,10..35,15", "10,10..20,15");

        // Overlapping truncate top
        test_subtract("10,10..30,15 10,8..30,12", "10,12..30,15");

        // Overlapping truncate bottom
        test_subtract("10,10..30,15 10,13..30,18", "10,10..30,13");

        // Overlapping U left
        test_subtract("10,10..30,15 5,11..20,14", "10,10..30,11 20,11..30,14 10,14..30,15");

        // Overlapping U right
        test_subtract("10,10..30,15 20,11..35,14", "10,10..30,11 10,11..20,14 10,14..30,15");

        // Overlapping U top
        test_subtract("10,10..30,15 15,8..25,12", "10,10..15,12 25,10..30,12 10,12..30,15");

        // Overlapping U bottom
        test_subtract("10,10..30,15 15,13..25,18", "10,10..30,13 10,13..15,15 25,13..30,15");

        // Remove entirely
        test_subtract("10,10..30,15 8,8..32,17", "");
    }
}

fn fd_readb(fd: libc::c_int) -> Vec<u8>
{
    let mut buf: [u8, ..1024] = unsafe { std::mem::uninitialized() };

    let rv = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len() as libc::size_t) };
    if rv == -1
    {
        fail!("read() failed with errno #{}", std::os::errno())
    }

    buf.slice_to(rv as uint).to_vec()
}

fn fd_reads(fd: libc::c_int) -> String
{
    String::from_utf8(fd_readb(fd)).unwrap()
}

#[test]
fn test_10term_write()
{
    let mut tap = taplib::Tap::new();

    {
        /* We'll need a real filehandle we can write/read.
        * pipe() can make us one */
        let fd = unsafe { std::os::pipe().unwrap() };

        let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();

        tap.pass("tickit::TickitTerm::new_for_termtype");

        tt.set_output_fd(fd.writer);

        tap.is_int(tt.get_output_fd(), fd.writer, "tickit_term_get_output_fd");

        /* Already it should have written its DECSLRM probe string */
        let buffer = fd_reads(fd.reader);

        tap.is_str_escape(buffer, "\x1b[?69h\x1b[?69$p\x1b[?25$p\x1b[?12$p\x1bP$q q\x1b\\\x1b[G\x1b[K",
            "buffer after initialisation contains DECSLRM and cursor status probes");

        tt.print("Hello world!");

        let buffer = fd_reads(fd.reader);

        tap.is_int(buffer.len(), 12, "read() length after tickit_term_print");
        tap.is_str_escape(buffer, "Hello world!", "buffer after tickit_term_print");

        tt.print("another string here".slice_to(7));

        let buffer = fd_reads(fd.reader);

        tap.is_int(buffer.len(), 7, "read() length after tickit_term_printn");
        tap.is_str_escape(buffer, "another", "buffer after tickit_term_printn");

        tt.print(format!("{} {}!", "More", "messages").as_slice());

        let buffer = fd_reads(fd.reader);

        tap.is_int(buffer.len(), 14, "read() length after tickit_term_printf");
        tap.is_str_escape(buffer, "More messages!", "buffer after tickit_term_printf");

        tt.goto(2, 5);

        let buffer = fd_reads(fd.reader);

        tap.is_str_escape(buffer, "\x1b[3;6H", "buffer after tickit_term_goto line+col");

        tt.goto(4, -1);

        let buffer = fd_reads(fd.reader);

        tap.is_str_escape(buffer, "\x1b[5d", "buffer after tickit_term_goto line");

        tt.goto(-1, 10);

        let buffer = fd_reads(fd.reader);

        tap.is_str_escape(buffer, "\x1b[11G", "buffer after tickit_term_goto col");
    }

    tap.pass("tickit_term_destroy");
}

fn uslice<'a>(v: &'a Vec<u8>) -> &'a str
{
    std::str::from_utf8(v.as_slice()).unwrap()
}

#[test]
fn test_11term_output_screen()
{
    let mut tap = taplib::Tap::new();

    let mut buffer = std::sync::Mutex::new(Vec::<u8>::new());

    let output = |tt: &mut tickit::TickitTerm, bytes: &[u8]|
    {
        (*buffer.lock()).push_all(bytes);
    };

    {
        let mut tt = tickit::TickitTerm::new_for_termtype("screen").unwrap();
        tap.pass("tickit::TickitTerm::new_for_termtype");

        tap.is_str(tt.get_termtype(), "screen", "tickit_term_get_termtype");

        let _alive = tt.set_output_lively(output);

        tap.is_int(tt.get_output_fd(), -1, "tickit_term_get_output_fd");

        tt.set_size(24, 80);

        let (lines, cols) = tt.get_size();
        tap.is_int(lines, 24, "get_size lines");
        tap.is_int(cols,  80, "get_size cols");

        (*buffer.lock()).clear();
        tt.print("Hello world!");
        tap.is_str_escape(uslice(&(*buffer.lock())), "Hello world!", "buffer after tickit_term_print");

        /* These tests rely on whatever is in the terminfo database, so we should
         * try not do anything too out of the ordinary
         */

        (*buffer.lock()).clear();
        tt.goto(2, 5);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[3;6H", "buffer after tickit_term_goto line+col");

        (*buffer.lock()).clear();
        tt.goto(-1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\r", "buffer after tickit_term_goto col=0");

        (*buffer.lock()).clear();
        tt.move(2, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2B", "buffer after tickit_term_move down 2");

        (*buffer.lock()).clear();
        tt.move(-2, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2A", "buffer after tickit_term_move down 2");

        (*buffer.lock()).clear();
        tt.move(0, 2);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2C", "buffer after tickit_term_move right 2");

        (*buffer.lock()).clear();
        tt.move(0, -2);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2D", "buffer after tickit_term_move left 2");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 7, 80), 1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;10r\x1b[4;1H\x1b[M\x1b[1;24r", "buffer after tickit_term_scroll lines 3-9 1 down");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 15, 80), 8, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;18r\x1b[4;1H\x1b[8M\x1b[1;24r", "buffer after tickit_term_scroll lines 3-17 8 down");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 7, 80), -1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;10r\x1b[4;1H\x1b[L\x1b[1;24r", "buffer after tickit_term_scroll lines 3-9 1 up");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 15, 80), -8, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;18r\x1b[4;1H\x1b[8L\x1b[1;24r", "buffer after tickit_term_scroll lines 3-17 8 up");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(5, 0, 1, 80), 0, 3);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[6;1H\x1b[3P", "buffer after tickit_term_scrollrect line 5 3 right");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(6, 10, 2, 70), 0, 5);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[7;11H\x1b[5P\x1b[8;11H\x1b[5P", "buffer after tickit_term_scrollrect lines 6-7 cols 10-80 5 right");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(5, 0, 1, 80), 0, -3);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[6;1H\x1b[3@", "buffer after tickit_term_scrollrect line 5 3 left");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(6, 10, 2, 70), 0, -5);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[7;11H\x1b[5@\x1b[8;11H\x1b[5@", "buffer after tickit_term_scrollrect lines 6-7 cols 10-80 5 left");

        tap.is_int(tt.scrollrect(TickitRect::init_sized(3, 10, 5, 60), 1, 0), false, "tickit_term cannot scroll partial lines vertically");

        tap.is_int(tt.scrollrect(TickitRect::init_sized(3, 10, 5, 60), 0, 1), false, "tickit_term cannot scroll partial lines horizontally");

        (*buffer.lock()).clear();
        tt.clear();
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[H\x1b[J", "buffer after tickit_term_clear");

        (*buffer.lock()).clear();
        tt.erasech(1, Some(false));
        tap.is_str_escape(uslice(&(*buffer.lock())), " \x08", "buffer after tickit_term_erasech 1 nomove");

        (*buffer.lock()).clear();
        tt.erasech(3, Some(false));
        tap.is_str_escape(uslice(&(*buffer.lock())), "   \x1b[3D", "buffer after tickit_term_erasech 3 nomove");

        (*buffer.lock()).clear();
        tt.erasech(1, Some(true));
        tap.is_str_escape(uslice(&(*buffer.lock())), " ", "buffer after tickit_term_erasech 1 move");

        (*buffer.lock()).clear();
        tt.erasech(3, Some(true));
        tap.is_str_escape(uslice(&(*buffer.lock())), "   ", "buffer after tickit_term_erasech 3 move");

        (*buffer.lock()).clear();
        tt.erasech(10, Some(true));

        tap.is_str_escape(uslice(&(*buffer.lock())), "          ", "buffer after tickit_term_erasech 10 move");

    }

    tap.pass("tickit_term_destroy");
}

#[test]
fn test_11term_output_xterm()
{
    let mut tap = taplib::Tap::new();

    let mut buffer = std::sync::Mutex::new(Vec::<u8>::new());

    let output = |tt: &mut tickit::TickitTerm, bytes: &[u8]|
    {
        (*buffer.lock()).push_all(bytes);
    };

    {
        let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();
        tap.pass("tickit::TickitTerm::new_for_termtype");

        tap.is_str(tt.get_termtype(), "xterm", "tickit_term_get_termtype");

        let _alive = tt.set_output_lively(output);

        tap.is_int(tt.get_output_fd(), -1, "tickit_term_get_output_fd");

        tt.set_size(24, 80);

        let (lines, cols) = tt.get_size();
        tap.is_int(lines, 24, "get_size lines");
        tap.is_int(cols,  80, "get_size cols");

        (*buffer.lock()).clear();
        tt.print("Hello world!");
        tap.is_str_escape(uslice(&(*buffer.lock())), "Hello world!", "buffer after tickit_term_print");

        (*buffer.lock()).clear();
        tt.goto(2, 5);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[3;6H", "buffer after tickit_term_goto line+col");

        (*buffer.lock()).clear();
        tt.goto(3, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4H", "buffer after tickit_term_goto line+col0");

        (*buffer.lock()).clear();
        tt.goto(4, -1);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[5d", "buffer after tickit_term_goto line");

        (*buffer.lock()).clear();
        tt.goto(-1, 10);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[11G", "buffer after tickit_term_goto col");

        (*buffer.lock()).clear();
        tt.goto(-1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[G", "buffer after tickit_term_goto col0");

        (*buffer.lock()).clear();
        tt.move(1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[B", "buffer after tickit_term_move down 1");

        (*buffer.lock()).clear();
        tt.move(2, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2B", "buffer after tickit_term_move down 2");

        (*buffer.lock()).clear();
        tt.move(-1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[A", "buffer after tickit_term_move down 1");

        (*buffer.lock()).clear();
        tt.move(-2, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2A", "buffer after tickit_term_move down 2");

        (*buffer.lock()).clear();
        tt.move(0, 1);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[C", "buffer after tickit_term_move right 1");

        (*buffer.lock()).clear();
        tt.move(0, 2);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2C", "buffer after tickit_term_move right 2");

        (*buffer.lock()).clear();
        tt.move(0, -1);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[D", "buffer after tickit_term_move left 1");

        (*buffer.lock()).clear();
        tt.move(0, -2);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2D", "buffer after tickit_term_move left 2");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 7, 80), 1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;10r\x1b[4H\x1b[M\x1b[r", "buffer after tickit_term_scroll lines 3-9 1 down");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 15, 80), 8, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;18r\x1b[4H\x1b[8M\x1b[r", "buffer after tickit_term_scroll lines 3-17 8 down");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 7, 80), -1, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;10r\x1b[4H\x1b[L\x1b[r", "buffer after tickit_term_scroll lines 3-9 1 up");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(3, 0, 15, 80), -8, 0);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;18r\x1b[4H\x1b[8L\x1b[r", "buffer after tickit_term_scroll lines 3-17 8 up");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(5, 0, 1, 80), 0, 3);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[6H\x1b[3P", "buffer after tickit_term_scrollrect line 5 3 right");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(6, 10, 2, 70), 0, 5);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[7;11H\x1b[5P\x1b[8;11H\x1b[5P", "buffer after tickit_term_scrollrect lines 6-7 cols 10-80 5 right");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(5, 0, 1, 80), 0, -3);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[6H\x1b[3@", "buffer after tickit_term_scrollrect line 5 3 left");

        (*buffer.lock()).clear();
        tt.scrollrect(TickitRect::init_sized(6, 10, 2, 70), 0, -5);
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[7;11H\x1b[5@\x1b[8;11H\x1b[5@", "buffer after tickit_term_scrollrect lines 6-7 cols 10-80 5 left");

        tap.is_int(tt.scrollrect(TickitRect::init_sized(3, 10, 5, 60), 1, 0), false, "tickit_term cannot scroll partial lines vertically");

        tap.is_int(tt.scrollrect(TickitRect::init_sized(3, 10, 5, 60), 0, 1), false, "tickit_term cannot scroll partial lines horizontally");

        /* Now (belatedly) respond to the DECSLRM probe to enable more scrollrect options */
        tt.input_push_bytes("\x1b[?69;1$y".as_bytes());

        (*buffer.lock()).clear();
        tap.is_int(tt.scrollrect(TickitRect::init_sized(3, 10, 5, 60), 1, 0), true, "tickit_term can scroll partial lines vertically with DECSLRM enabled");
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;8r\x1b[11;70s\x1b[4;11H\x1b[M\x1b[r\x1b[s", "buffer after tickit_term_scroll lines 3-7 cols 10-69 down");

        (*buffer.lock()).clear();
        tap.is_int(tt.scrollrect(TickitRect::init_sized(3, 10, 5, 60), 0, 1), true, "tickit_term can scroll partial lines horizontally with DECSLRM enabled");
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[4;8r\x1b[11;70s\x1b[4;11H\x1b['~\x1b[r\x1b[s", "buffer after tickit_term_scroll lines 3-7 cols 10-69 right");

        (*buffer.lock()).clear();
        tap.is_int(tt.scrollrect(TickitRect::init_sized(3, 10, 1, 60), 0, 1), true, "tickit_term can scroll partial lines horizontally with DECSLRM enabled");
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[;70s\x1b[4;11H\x1b[P\x1b[s", "buffer after tickit_term_scroll line 3 cols 10-69 right");

        (*buffer.lock()).clear();
        tt.clear();
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[2J", "buffer after tickit_term_clear");

        (*buffer.lock()).clear();
        tt.erasech(1, Some(false));
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[X", "buffer after tickit_term_erasech 1 nomove");

        (*buffer.lock()).clear();
        tt.erasech(3, Some(false));
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[3X", "buffer after tickit_term_erasech 3 nomove");

        (*buffer.lock()).clear();
        tt.erasech(1, Some(true));
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[X\x1b[C", "buffer after tickit_term_erasech 1 move");

        (*buffer.lock()).clear();
        tt.erasech(3, Some(true));
        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[3X\x1b[3C", "buffer after tickit_term_erasech 3 move");
    }

    tap.pass("tickit_term_destroy");
}

#[test]
fn test_12term_modes()
{
    let mut tap = taplib::Tap::new();

    let mut buffer = std::sync::Mutex::new(Vec::<u8>::new());

    let output = |tt: &mut tickit::TickitTerm, bytes: &[u8]|
    {
        (*buffer.lock()).push_all(bytes);
    };

    {
        let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();

        tap.pass("tickit::TickitTerm::new_for_termtype");

        let _alive = tt.set_output_lively(output);

        (*buffer.lock()).clear();
        tt.setctl_int(tickit::c::TICKIT_TERMCTL_ALTSCREEN, 1);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[?1049h", "buffer after set_mode_altscreen on");

        let value = tt.getctl_int(tickit::c::TICKIT_TERMCTL_ALTSCREEN).unwrap();
        tap.is_int(value, 1, "get_mode_altscreen returns value");

        (*buffer.lock()).clear();
        tt.setctl_int(tickit::c::TICKIT_TERMCTL_ALTSCREEN, 1);

        tap.is_str_escape(uslice(&(*buffer.lock())), "", "set_mode_altscreen a second time is idempotent");

        (*buffer.lock()).clear();
        tt.setctl_int(tickit::c::TICKIT_TERMCTL_CURSORVIS, 0);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[?25l", "buffer after set_mode_cursorvis off");

        (*buffer.lock()).clear();
        tt.setctl_int(tickit::c::TICKIT_TERMCTL_MOUSE, tickit::c::TICKIT_TERM_MOUSEMODE_CLICK as int);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[?1000h\x1b[?1006h", "buffer after set_mode_mouse to click");

        (*buffer.lock()).clear();
        tt.setctl_int(tickit::c::TICKIT_TERMCTL_MOUSE, tickit::c::TICKIT_TERM_MOUSEMODE_DRAG as int);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[?1002h\x1b[?1006h", "buffer after set_mode_mouse to drag");

        (*buffer.lock()).clear();
        tt.setctl_str(tickit::c::TICKIT_TERMCTL_TITLE_TEXT, "title here");

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b]2;title here\x1b\\", "buffer after set title");

        (*buffer.lock()).clear();

        drop(tt); // TODO remove this hack (because of _alive / lively)
    }

    tap.pass("tickit_term_destroy");

    tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[?1002l\x1b[?1006l\x1b[?25h\x1b[?1049l", "buffer after termkey_term_destroy resets modes");
}

#[test]
fn test_13term_pen()
{
    let mut tap = taplib::Tap::new();

    let mut buffer = std::sync::Mutex::new(Vec::<u8>::new());

    let output = |tt: &mut tickit::TickitTerm, bytes: &[u8]|
    {
        (*buffer.lock()).push_all(bytes);
    };

    {
        let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();
        let _alive = tt.set_output_lively(output);

        let mut pen = tickit::TickitPen::new();

        (*buffer.lock()).clear();
        tt.setpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[m", "buffer after chpen empty");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, true);

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[1m", "buffer contains SGR 1 for chpen bold");

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "", "chpen again is a no-op");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, false);

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[m", "chpen disables bold");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, true);
        pen.set_bool_attr(tickit::c::TICKIT_PEN_UNDER, true);

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[1;4m", "chpen enables bold and under");

        pen.set_bool_attr(tickit::c::TICKIT_PEN_BOLD, false);
        pen.clear_attr(tickit::c::TICKIT_PEN_UNDER);

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[22m", "chpen disables bold");

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "", "chpen disable bold again is no-op");

        pen.clear_attr(tickit::c::TICKIT_PEN_BOLD);
        pen.set_bool_attr(tickit::c::TICKIT_PEN_UNDER, false);

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[m", "chpen disable under is reset");

        pen.clear_attr(tickit::c::TICKIT_PEN_UNDER);

        pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 1);
        pen.set_colour_attr(tickit::c::TICKIT_PEN_BG, 5);

        (*buffer.lock()).clear();
        tt.setpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[31;45m", "chpen foreground+background");

        pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 9);
        pen.clear_attr(tickit::c::TICKIT_PEN_BG);

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[91m", "chpen foreground high");

        pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 123);

        (*buffer.lock()).clear();
        tt.chpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[38;5;123m", "chpen foreground xterm256");

        pen.clear_attr(tickit::c::TICKIT_PEN_FG);
        pen.set_bool_attr(tickit::c::TICKIT_PEN_UNDER, true);

        (*buffer.lock()).clear();
        tt.setpen(&pen);

        tap.is_str_escape(uslice(&(*buffer.lock())), "\x1b[39;49;4m", "setpen resets colours, enables under");
    }
}

#[test]
fn test_14term_resize()
{
    use std::sync::Mutex;

    let tap = Mutex::new(taplib::Tap::new());

    struct LinesCols
    {
        lines: int,
        cols: int,
    }

    struct OnResize1<'a>
    {
        tap: &'a Mutex<taplib::Tap>,
        new1: &'a Mutex<LinesCols>,
        unbound: &'a Mutex<bool>,
    }

    struct OnResize2<'a>
    {
        tap: &'a Mutex<taplib::Tap>,
        new2: &'a Mutex<LinesCols>,
    }

    #[unsafe_destructor]
    impl<'a> Drop for OnResize1<'a>
    {
        fn drop(&mut self)
        {
            *self.unbound.lock() = true;
        }
    }

    let new1 = Mutex::new(LinesCols{lines: 0, cols: 0});
    let new2 = Mutex::new(LinesCols{lines: 0, cols: 0});
    let unbound = Mutex::new(false);

    let onr1 = OnResize1{tap: &tap, new1: &new1, unbound: &unbound};
    let onr2 = OnResize2{tap: &tap, new2: &new2};

    fn on_resize<'a>(tt: &mut tickit::TickitTerm, ev: &tickit::TickitEvent, data: &mut OnResize1<'a>)
    {
        match *ev
        {
            tickit::ResizeEvent{lines, cols} =>
            {
                data.tap.lock().pass("ev type to on_resize()");

                *data.new1.lock() = LinesCols{lines: lines, cols: cols};
            }
            _ =>
            {
                data.tap.lock().fail("ev type to on_resize()");
            }
        }
    }

    fn on_resize2<'a>(tt: &mut tickit::TickitTerm, ev: &tickit::TickitEvent, data: &mut OnResize2<'a>)
    {
        match *ev
        {
            tickit::ResizeEvent{lines, cols} =>
            {
                data.tap.lock().pass("ev type to on_resize2()");

                *data.new2.lock() = LinesCols{lines: lines, cols: cols};
            }
            _ =>
            {
                data.tap.lock().fail("ev type to on_resize2()");
            }
        }
    }

    {
        let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();
        tt.set_size(25, 80);

        let hookid = tt.bind_event(tickit::c::TICKIT_EV_RESIZE, on_resize, onr1);

        tt.set_size(30, 100);

        let LinesCols{lines: new_lines, cols: new_cols} = *new1.lock();
        tap.lock().is_int(new_lines,  30, "new_lines from event handler after set_size");
        tap.lock().is_int(new_cols,  100, "new_cols from event handler after set_size");

        tt.bind_event(tickit::c::TICKIT_EV_RESIZE, on_resize2, onr2);

        tt.set_size(35, 110);

        let LinesCols{lines: new_lines, cols: new_cols} = *new1.lock();
        let LinesCols{lines: new_lines2, cols: new_cols2} = *new2.lock();
        tap.lock().is_int(new_lines,   35, "new_lines from event handler after set_size");
        tap.lock().is_int(new_cols,   110, "new_cols from event handler after set_size");
        tap.lock().is_int(new_lines2,  35, "new_lines from event handler 2 after set_size");
        tap.lock().is_int(new_cols2,  110, "new_cols from event handler 2 after set_size");

        tt.unbind_event_id(hookid);

        tt.set_size(40, 120);

        let LinesCols{lines: new_lines, cols: new_cols} = *new1.lock();
        tap.lock().is_int(new_lines, 35, "new_lines still 35 after unbind event");
    }

    tap.lock().is_int(*unbound.lock(), true, "on_resize unbound after tickit_term_destroy");
}

#[test]
fn test_15term_input()
{
    use std::sync::Mutex;
    use std::time::duration::Duration;

    let mut tap = taplib::Tap::new();

    enum KeyEventData
    {
        KeyKeyEvent{key: String, mod_: tickit::c::X_Tickit_Mod},
        KeyTextEvent{text: String, mod_: tickit::c::X_Tickit_Mod},
    }
    type MouseEventData = tickit::TickitMouseEvent;

    let key_event: Mutex<Option<KeyEventData>> = Mutex::new(None);
    let mouse_event: Mutex<Option<MouseEventData>> = Mutex::new(None);

    fn on_key(tt: &mut tickit::TickitTerm, ev: &tickit::TickitEvent, data: &mut &Mutex<Option<KeyEventData>>)
    {
        let key = match *ev { tickit::KeyEvent(k) => { k } _ => { fail!(); } };
        *data.lock() = Some(
            match key
            {
                tickit::KeyKeyEvent{key, mod_} =>
                {
                    KeyKeyEvent{key: key.to_string(), mod_: mod_}
                }
                tickit::KeyTextEvent{text, mod_} =>
                {
                    KeyTextEvent{text: text.to_string(), mod_: mod_}
                }
            }
        )
    }

    fn on_mouse(tt: &mut tickit::TickitTerm, ev: &tickit::TickitEvent, data: &mut &Mutex<Option<MouseEventData>>)
    {
        let mouse = match *ev { tickit::MouseEvent(m) => { m } _ => { fail!(); } };
        *data.lock() = Some(mouse);
    }

    let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();
    tt.set_utf8(true);

    tap.ok(tt.get_utf8(), "get_utf8 true");

    tt.bind_event(tickit::c::TICKIT_EV_KEY,   on_key,   &key_event);
    tt.bind_event(tickit::c::TICKIT_EV_MOUSE, on_mouse, &mouse_event);

    tt.input_push_bytes("A".as_bytes());

    match (*key_event.lock()).take_unwrap()
    {
        KeyTextEvent{text, mod_} =>
        {
            tap.pass("keytype after push_bytes A");
            tap.is_str(text,  "A",               "keystr after push_bytes A");
            tap.ok(mod_.is_empty(),                 "keymod after push_bytes A");
        }
        _ =>
        {
            tap.bypass(3, "keytype after push_bytes A");
        }
    }

    tap.is_int(tt.input_check_timeout(), None, "term has no timeout after A");

    /* U+0109 - LATIN SMALL LETTER C WITH CIRCUMFLEX
    * UTF-8: 0xc4 0x89
    */
    tt.input_push_bytes("\u0109".as_bytes());

    match (*key_event.lock()).take_unwrap()
    {
        KeyTextEvent{text, mod_} =>
        {
            tap.pass("keytype after push_bytes U+0109");
            tap.is_str(text,  "\u0109",        "keystr after push_bytes U+0109");
            tap.ok(mod_.is_empty(),                 "keymod after push_bytes U+0109");
        }
        _ =>
        {
            tap.bypass(3, "keytype after push_bytes U+0109");
        }
    }

    tt.input_push_bytes("\x1b[A".as_bytes());

    match (*key_event.lock()).take_unwrap()
    {
        KeyKeyEvent{key, mod_} =>
        {
            tap.pass("keytype after push_bytes Up");
            tap.is_str(key,  "Up",             "keystr after push_bytes Up");
            tap.ok(mod_.is_empty(),                 "keymod after push_bytes Up");
        }
        _ =>
        {
            tap.bypass(3, "keytype after push_bytes Up");
        }
    }

    tt.input_push_bytes("\x01".as_bytes());

    match (*key_event.lock()).take_unwrap()
    {
        KeyKeyEvent{key, mod_} =>
        {
            tap.pass("keytype after push_bytes C-a");
            tap.is_str(key,  "C-a",            "keystr after push_bytes C-a");
            tap.ok(mod_ == tickit::c::TICKIT_MOD_CTRL,  "keymod after push_bytes C-a");
        }
        _ =>
        {
            tap.bypass(3, "keytype after push_bytes C-a");
        }
    }

    tap.is_int(tt.input_check_timeout(), None, "term has no timeout after Up");

    tt.input_push_bytes("\x1b[M !!".as_bytes());

    match (*mouse_event.lock()).unwrap()
    {
        tickit::MousePressEvent{button, line, col, mod_} =>
        {
            tap.pass("mousetype after mouse button press");
            tap.is_int(button, 1,                    "mousebutton after mouse button press");
            tap.is_int(line,   0,                    "mouseline after mouse button press");
            tap.is_int(col,    0,                    "mousecol after mouse button press");
            tap.ok(mod_.is_empty(),                    "mousemod after mouse button press");
        }
        _ =>
        {
            tap.bypass(5, "mousetype after mouse button press");
        }
    }

    tt.input_push_bytes("\x1b[M`!!".as_bytes());

    match (*mouse_event.lock()).unwrap()
    {
        tickit::MouseWheelEvent{dir, line, col, mod_} =>
        {
            tap.pass("mousetype after mouse wheel up");
            tap.is_int(dir, tickit::c::TICKIT_MOUSEWHEEL_UP, "mousebutton after mouse wheel up");
            tap.is_int(line,   0,                    "mouseline after mouse wheel up");
            tap.is_int(col,    0,                    "mousecol after mouse wheel up");
            tap.ok(mod_.is_empty(),                    "mousemod after mouse wheel up");
        }
        _ =>
        {
            tap.bypass(5, "mousetype after mouse wheel up");
        }
    }

    *key_event.lock() = None;
    tt.input_push_bytes("\x1b[".as_bytes());

    tap.ok((*key_event.lock()).is_none(), "keytype not set after push_bytes partial Down");

    tap.ok(tt.input_check_timeout().unwrap() > 0, "term has timeout after partial Down");

    tt.input_push_bytes("B".as_bytes());

    match (*key_event.lock()).take_unwrap()
    {
        KeyKeyEvent{key, mod_} =>
        {
            tap.pass("keytype after push_bytes completed Down");
            tap.is_str(key,  "Down",           "keystr after push_bytes completed Down");
        }
        _ =>
        {
            tap.bypass(2, "keytype after push_bytes completed Down");
        }
    }

    tap.is_int(tt.input_check_timeout(), None, "term has no timeout after completed Down");

    *key_event.lock() = None;
    tt.input_push_bytes("\x1b".as_bytes());

    tap.ok((*key_event.lock()).is_none(), "keytype not set after push_bytes Escape");

    let timeout_msec: Option<uint> = tt.input_check_timeout();
    tap.ok(timeout_msec.is_some() && timeout_msec.unwrap() > 0, "term has timeout after Escape");

    /* Add an extra milisecond timing grace */
    std::io::timer::sleep(Duration::milliseconds(timeout_msec.unwrap() as i32 + 1));

    tt.input_check_timeout();

    match (*key_event.lock()).take_unwrap()
    {
        KeyKeyEvent{key, mod_} =>
        {
            tap.pass("keytype after push_bytes completed Escape");
            tap.is_str(key,  "Escape",         "keystr after push_bytes completed Escape");
        }
        _ =>
        {
            tap.bypass(2, "keytype after push_bytes completed Escape");
        }
    }

    tap.is_int(tt.input_check_timeout(), None, "term has no timeout after completed Escape");
}

fn fd_write(fd: libc::c_int, buf: &[u8])
{
    let rv = unsafe { libc::write(fd, buf.as_ptr() as *const libc::c_void, buf.len() as libc::size_t) };
    assert!(rv == buf.len() as libc::ssize_t);
}

#[test]
fn test_16term_read()
{
    use std::sync::Mutex;
    use std::time::duration::Duration;

    let mut tap = taplib::Tap::new();

    enum KeyEventData
    {
        KeyKeyEvent{key: String},
        KeyTextEvent{text: String},
    }

    let key_event: Mutex<Option<KeyEventData>> = Mutex::new(None);

    fn on_key(tt: &mut tickit::TickitTerm, ev: &tickit::TickitEvent, data: &mut &Mutex<Option<KeyEventData>>)
    {
        let key = match *ev { tickit::KeyEvent(k) => { k } _ => { fail!(); } };
        *data.lock() = Some(
            match key
            {
                tickit::KeyKeyEvent{key, mod_: _} =>
                {
                    KeyKeyEvent{key: key.to_string()}
                }
                tickit::KeyTextEvent{text, mod_: _} =>
                {
                    KeyTextEvent{text: text.to_string()}
                }
            }
        )
    }

    /* We'll need a real filehandle we can write/read.
    * pipe() can make us one */
    let fd = unsafe { std::os::pipe().unwrap() };

    let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();
    tt.set_input_fd(fd.reader);

    tap.is_int(tt.get_input_fd(), fd.reader, "tickit_term_get_input_fd");

    tt.bind_event(tickit::c::TICKIT_EV_KEY, on_key, &key_event);

    fd_write(fd.writer, "A".as_bytes());
    tt.input_readable();

    match (*key_event.lock()).take_unwrap()
    {
        KeyTextEvent{text} =>
        {
            tap.pass("keytype after write A");
            tap.is_str(text,  "A",               "keystr after write A");
        }
        _ =>
        {
            tap.bypass(2, "keytype after write A");
        }
    }

    tap.is_int(tt.input_check_timeout(), None, "term has no timeout after A");

    (*key_event.lock()) = None;
    fd_write(fd.writer, "\x1b".as_bytes());
    tt.input_readable();

    tap.ok((*key_event.lock()).is_none(), "keytype not set after write Escape");

    let timeout_msec: Option<uint> = tt.input_check_timeout();
    tap.ok(timeout_msec.is_some() && timeout_msec.unwrap() > 0, "term has timeout after Escape");

    /* Add an extra milisecond timing grace */
    std::io::timer::sleep(Duration::milliseconds(timeout_msec.unwrap() as i32 + 1));

    tt.input_check_timeout();

    match (*key_event.lock()).take_unwrap()
    {
        KeyKeyEvent{key} =>
        {
            tap.pass("keytype after write completed Escape");
            tap.is_str(key,  "Escape",         "keystr after write completed Escape");
        }
        _ =>
        {
            tap.bypass(2, "keytype after write completed Escape");
        }
    }

    tap.is_int(tt.input_check_timeout(), None, "term has no timeout after completed Escape");
}

#[test]
fn test_17term_buffer()
{
    use std::sync::Mutex;

    let mut tap = taplib::Tap::new();

    let buffer = Mutex::new(Vec::<u8>::new());

    fn output(tt: &mut tickit::TickitTerm, bytes: &[u8], buffer: &mut &Mutex<Vec<u8>>)
    {
        (*buffer.lock()).push_all(bytes);
    }

    let mut tt = tickit::TickitTerm::new_for_termtype("xterm").unwrap();

    tap.pass("tickit::TickitTerm::new_for_termtype");

    tt.set_output_func(output, &buffer);
    tt.set_output_buffer(4096);

    (*buffer.lock()).clear();

    tt.print("Hello ");
    tap.is_str_escape(uslice(&(*buffer.lock())), "", "buffer empty after print");

    tt.print("world!");
    tap.is_str_escape(uslice(&(*buffer.lock())), "", "buffer still empty after second print");

    tt.flush();
    tap.is_str_escape(uslice(&(*buffer.lock())), "Hello world!", "buffer contains output after flush");
}

#[test]
fn test_19term_driver()
{
    use std::sync::Mutex;

    use tickit::{TickitPen,TickitRect,TickitTerm};
    use tickit::c::{TickitTermCtl};
    use tickit::drv::{CDriverRef,TickitTermDriverImpl};

    let mut tap = taplib::Tap::new();

    struct TestVtable;

    impl TickitTermDriverImpl for TestVtable
    {
        // fn attach(&mut self, cdr: CDriverRef, tt: &mut ::TickitTerm) {}
        // fn start(&mut self, cdr: CDriverRef) {}
        // fn started(&mut self, cdr: CDriverRef) -> bool { true }
        // fn stop(&mut self, cdr: CDriverRef) {}
        fn print(&mut self, cdr: CDriverRef, str_: &str)
        {
            cdr.write_str(format!("PRINT({})", str_).as_slice());
        }
        fn goto_abs(&mut self, cdr: CDriverRef, line: int, col: int) -> bool { false }
        fn move_rel(&mut self, cdr: CDriverRef, downward: int, rightward: int) {}
        fn scrollrect(&mut self, cdr: CDriverRef, rect: &TickitRect, downward: int, rightward: int) -> bool { false }
        fn erasech(&mut self, cdr: CDriverRef, count: int, moveend: Option<bool>) {}
        fn clear(&mut self, cdr: CDriverRef) {}
        fn chpen(&mut self, cdr: CDriverRef, delta: &TickitPen, final: &TickitPen) {}
        fn getctl_int(&mut self, cdr: CDriverRef, ctl: TickitTermCtl) -> Option<int>
        {
            match ctl
            {
                tickit::c::TICKIT_TERMCTL_COLORS => Some(8),
                _ => None,
            }
        }
        fn setctl_int(&mut self, cdr: CDriverRef, ctl: TickitTermCtl, value: int) -> bool
        {
            false
        }
        fn setctl_str(&mut self, cdr: CDriverRef, ctl: TickitTermCtl, value: &str) -> bool
        {
            false
        }
        // fn gotkey(&mut self, cdr: CDriverRef, tk: &mut TermKey, key: &TermKeyEvent) -> bool { false }
    }

    let buffer = Mutex::new(Vec::<u8>::new());

    fn output(tt: &mut tickit::TickitTerm, bytes: &[u8], buffer: &mut &Mutex<Vec<u8>>)
    {
        (*buffer.lock()).push_all(bytes);
    }

    let ttd = TestVtable;

    let mut tt = TickitTerm::new_for_driver(ttd);

    tap.pass("tickit_term_new_for_driver");

    tt.set_output_func(output, &buffer);
    tt.set_output_buffer(4096);

    (*buffer.lock()).clear();

    tt.print("Hello");
    tt.flush();

    tap.is_str(uslice(&(*buffer.lock())), "PRINT(Hello)", "buffer after print");
}

#[allow(dead_code)]
struct PenLog
{
    fg: Option<int>,
    bg: Option<int>,
    b: Option<bool>,
    u: Option<bool>,
    i: Option<bool>,
    rv: Option<bool>,
    strike: Option<bool>,
    af: Option<int>,
}

#[allow(dead_code)]
impl PenLog
{
    fn fg(self, v: int) -> PenLog { PenLog{fg: Some(v), ..self} }
    fn bg(self, v: int) -> PenLog { PenLog{bg: Some(v), ..self} }
    fn b(self, v: bool) -> PenLog { PenLog{b: Some(v), ..self} }
    fn u(self, v: bool) -> PenLog { PenLog{u: Some(v), ..self} }
    fn i(self, v: bool) -> PenLog { PenLog{i: Some(v), ..self} }
    fn rv(self, v: bool) -> PenLog { PenLog{rv: Some(v), ..self} }
    fn strike(self, v: bool) -> PenLog { PenLog{strike: Some(v), ..self} }
    fn af(self, v: int) -> PenLog { PenLog{af: Some(v), ..self} }
}

static pen_log: PenLog = PenLog{fg: None, bg: None, b: None, u: None, i: None, rv: None, strike: None, af: None};

enum LogExpectation<'a>
{
    GOTO(int, int),
    PRINT(&'a str),
    ERASECH(int, Option<bool>),
    CLEAR,
    SCROLLRECT(tickit::TickitRect, int, int),
    SETPEN(PenLog),
}

impl<'a> LogExpectation<'a>
{
    fn matches<'b>(&self, x: tickit::mock::LogEntry<'b>) -> bool
    {
        use tickit::mock::{Goto,Print,EraseCh,Clear,ScrollRect,SetPen};

        match *self
        {
            GOTO(l, c) =>
            {
                match x
                {
                    Goto{line, col} => { l == line && c == col }
                    _ => { false }
                }
            }
            ERASECH(c, me) =>
            {
                match x
                {
                    EraseCh{count, moveend} => { c == count && me == moveend }
                    _ => { false }
                }
            }
            PRINT(s) =>
            {
                match x
                {
                    Print{str_} => { s == str_ }
                    _ => { false }
                }
            }
            CLEAR =>
            {
                match x
                {
                    Clear => { true }
                    _ => { false }
                }
            }
            SCROLLRECT(re, d, ri) =>
            {
                match x
                {
                    ScrollRect{rect, downward, rightward} => { re == rect && d == downward && ri == rightward }
                    _ => { false }
                }
            }
            SETPEN(pl) =>
            {
                match x
                {
                    SetPen{pen} =>
                    {
                        // 'pen' is always a fully-specified thingy
                        pl.fg.unwrap_or(-1) == pen.get_colour_attr(tickit::c::TICKIT_PEN_FG)
                        && pl.bg.unwrap_or(-1) == pen.get_colour_attr(tickit::c::TICKIT_PEN_BG)
                        && pl.b.unwrap_or(false) == pen.get_bool_attr(tickit::c::TICKIT_PEN_BOLD)
                        && pl.u.unwrap_or(false) == pen.get_bool_attr(tickit::c::TICKIT_PEN_UNDER)
                        && pl.i.unwrap_or(false) == pen.get_bool_attr(tickit::c::TICKIT_PEN_ITALIC)
                        && pl.rv.unwrap_or(false) == pen.get_bool_attr(tickit::c::TICKIT_PEN_REVERSE)
                        && pl.strike.unwrap_or(false) == pen.get_bool_attr(tickit::c::TICKIT_PEN_STRIKE)
                        && pl.af.unwrap_or(-1) == pen.get_int_attr(tickit::c::TICKIT_PEN_ALTFONT)
                    }
                    _ => { false }
                }
            }
        }
    }
}

impl taplib::Tap
{
    fn is_display_text(&mut self, mt: &mut tickit::mock::MockTerm, name: &str, expects: &[&str])
    {
        let (lines, cols) = mt.tt.get_size();

        assert!(lines == expects.len())

        for line in range(0, lines)
        {
            let expect = expects[line];
            let got = mt.get_display_text(line, 0, cols);

            if expect == got.as_slice()
            {
                continue;
            }

            self.fail(name);
            diag!("Got line {:2} |{}|", line, got);
            diag!("Expected    |{}|", expect);

            return;
        }

        self.pass(name);
    }

    fn is_termlog<'a>(&mut self, mt: &mut tickit::mock::MockTerm, name: &str, expects: &[LogExpectation<'a>])
    {
        let loglen = mt.loglen();
        assert!(loglen == expects.len());

        for (i, exp) in expects.iter().enumerate()
        {
            {
                let got = mt.peeklog(i);
                if exp.matches(got)
                {
                    continue;
                }
            }

            self.fail(name);
            mt.clearlog();
            return;
        }
        mt.clearlog();
        self.pass(name);
    }
}

fn fillterm(tt: &mut tickit::TickitTerm)
{
    tt.goto(0, 0);
    tt.print("0000000000");
    tt.goto(1, 0);
    tt.print("1111111111");
    tt.goto(2, 0);
    tt.print("2222222222");
}

fn make_term(lines: int, cols: int) -> tickit::mock::MockTerm
{
    tickit::mock::MockTerm::new(lines, cols)
}


#[test]
fn test_20mockterm()
{
    let mut tap = taplib::Tap::new();

    let mut mt = make_term(3, 10);

    tap.is_termlog(&mut mt, "Termlog initially",
        []);
    tap.is_display_text(&mut mt, "Display initially",
        [
            "          ",
            "          ",
            "          ",
        ]);

    mt.tt.goto(1, 5);
    tap.is_termlog(&mut mt, "Termlog after goto",
        [
            GOTO(1,5),
        ]);

    mt.tt.print("foo");

    tap.is_termlog(&mut mt, "Termlog after print",
        [
            PRINT("foo"),
        ]);
    tap.is_display_text(&mut mt, "Display after print",
        [
            "          ",
            "     foo  ",
            "          ",
        ]);

    mt.tt.print("l");

    tap.is_termlog(&mut mt, "Termlog after printn 1",
        [
            PRINT("l"),
        ]);
    tap.is_display_text(&mut mt, "Display after printn 1",
        [
            "          ",
            "     fool ",
            "          ",
        ]);

    mt.tt.goto(2, 0);
    mt.tt.print("Ĉu vi?");

    tap.is_termlog(&mut mt, "Termlog after print UTF-8",
        [
            GOTO(2,0),
            PRINT("Ĉu vi?"),
        ]);
    tap.is_display_text(&mut mt, "Display after print UTF-8",
        [
            "          ",
            "     fool ",
            "Ĉu vi?    ",
        ]);

    // U+FF10 = Fullwidth digit zero = EF BC 90
    mt.tt.print("\uff10");

    tap.is_termlog(&mut mt, "Termlog after print UTF-8 fullwidth",
        [
            PRINT("０"),
        ]);
    tap.is_display_text(&mut mt, "Display after print UTF-8 fullwidth",
        [
            "          ",
            "     fool ",
            "Ĉu vi?０  ",
        ]);

    mt.tt.clear();

    tap.is_termlog(&mut mt, "Termlog after clear",
        [
            CLEAR,
        ]);
    tap.is_display_text(&mut mt, "Display after clear",
        [
            "          ",
            "          ",
            "          ",
        ]);

    let fg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 3);

    mt.tt.setpen(&fg_pen);

    tap.is_termlog(&mut mt, "Termlog after setpen",
        [
            SETPEN(pen_log.fg(3)),
        ]);

    let bg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 6);

    mt.tt.chpen(&bg_pen);

    tap.is_termlog(&mut mt, "Termlog after chpen",
        [
            SETPEN(pen_log.fg(3).bg(6)),
        ]);

    // Now some test content for scrolling
    fillterm(&mut mt.tt);
    mt.clearlog();

    tap.is_display_text(&mut mt, "Display after scroll fill",
        [
            "0000000000",
            "1111111111",
            "2222222222",
        ]);

    tap.ok(mt.tt.scrollrect(TickitRect::init_sized(0,0,3,10),  1,0), "Scrollrect down OK");
    tap.is_termlog(&mut mt, "Termlog after scroll 1 down",
        [
            SCROLLRECT(TickitRect::init_sized(0,0,3,10),  1,0),
        ]);
    tap.is_display_text(&mut mt, "Display after scroll 1 down",
        [
            "1111111111",
            "2222222222",
            "          ",
        ]);

    tap.ok(mt.tt.scrollrect(TickitRect::init_sized(0,0,3,10), -1,0), "Scrollrect up OK");
    tap.is_termlog(&mut mt, "Termlog after scroll 1 up",
        [
            SCROLLRECT(TickitRect::init_sized(0,0,3,10), -1,0),
        ]);
    tap.is_display_text(&mut mt, "Display after scroll 1 up",
        [
            "          ",
            "1111111111",
            "2222222222",
        ]);

    fillterm(&mut mt.tt);
    mt.clearlog();

    mt.tt.scrollrect(TickitRect::init_sized(0,0,2,10),  1,0);
    tap.is_termlog(&mut mt, "Termlog after scroll partial 1 down",
        [
            SCROLLRECT(TickitRect::init_sized(0,0,2,10),  1,0),
        ]);
    tap.is_display_text(&mut mt, "Display after scroll partial 1 down",
        [
            "1111111111",
            "          ",
            "2222222222",
        ]);

    mt.tt.scrollrect(TickitRect::init_sized(0,0,2,10), -1,0);
    tap.is_termlog(&mut mt, "Termlog after scroll partial 1 up",
        [
            SCROLLRECT(TickitRect::init_sized(0,0,2,10), -1,0),
        ]);
    tap.is_display_text(&mut mt, "Display after scroll partial 1 up",
        [
            "          ",
            "1111111111",
            "2222222222",
        ]);

    for line in range(0, 3)
    {
        mt.tt.goto(line, 0);
        mt.tt.print("ABCDEFGHIJ");
    }
    mt.clearlog();

    mt.tt.scrollrect(TickitRect::init_sized(0,5,1,5), 0,2);
    tap.is_termlog(&mut mt, "Termlog after scroll right",
        [
            SCROLLRECT(TickitRect::init_sized(0,5,1,5), 0, 2),
        ]);
    tap.is_display_text(&mut mt, "Display after scroll right",
        [
            "ABCDEHIJ  ",
            "ABCDEFGHIJ",
            "ABCDEFGHIJ",
        ]);

    mt.tt.scrollrect(TickitRect::init_sized(1,5,1,5), 0,-3);
    tap.is_termlog(&mut mt, "Termlog after scroll left",
        [
            SCROLLRECT(TickitRect::init_sized(1,5,1,5), 0,-3),
        ]);
    tap.is_display_text(&mut mt, "Display after scroll left",
        [
            "ABCDEHIJ  ",
            "ABCDE   FG",
            "ABCDEFGHIJ",
        ]);

    mt.tt.goto(2, 3);
    mt.tt.erasech(5, None);

    tap.is_termlog(&mut mt, "Termlog after erasech",
        [
            GOTO(2,3),
            ERASECH(5, None),
        ]);
    tap.is_display_text(&mut mt, "Display after erasech",
        [
            "ABCDEHIJ  ",
            "ABCDE   FG",
            "ABC     IJ",
        ]);
}

#[test]
fn test_30renderbuffer_span()
{
    let mut tap = taplib::Tap::new();

    let mut mt = make_term(25, 80);

    let mut rb = tickit::TickitRenderBuffer::new(10, 20);

    tap.pass("tickit_renderbuffer_new");

    let (lines, cols) = rb.get_size();
    tap.is_int(lines, 10, "get_size lines");
    tap.is_int(cols,  20, "get_size cols");

    rb.flush_to_term(&mut mt.tt);
    tap.is_termlog(&mut mt, "Empty RenderBuffer renders nothing to term",
        []);

    tap.ok(!rb.get_cell_active(0, 0), "get_cell_active SKIP");

    // Absolute spans
    {
        // Direct pen
        let fg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 1);
        let len = rb.text_at(0, 1, "text span", Some(&fg_pen));
        tap.is_int(len, 9, "len from text_at");
        rb.erase_at(1, 1, 5, Some(&fg_pen));

        // Stored pen
        let bg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 2);
        rb.setpen(&bg_pen);
        rb.text_at(2, 1, "another span", None);
        rb.erase_at(3, 1, 10, None);

        // Combined pen
        rb.text_at(4, 1, "third span", Some(&fg_pen));
        rb.erase_at(5, 1, 7, Some(&fg_pen));

        let buffer = rb.get_cell_text(0, 1);
        tap.is_int(buffer.len(), 1, "get_cell_text TEXT at 0,1");
        tap.is_str(buffer, "t", "buffer text at TEXT 0,1");
        tap.is_int(rb.get_cell_pen(0, 1).get_colour_attr(tickit::c::TICKIT_PEN_FG), 1,
            "get_cell_pen FG at 0,1");

        let buffer = rb.get_cell_text(0, 2);
        tap.is_int(buffer.len(), 1, "get_cell_text TEXT at 0,2");
        tap.is_str(buffer, "e", "buffer text at TEXT 0,2");

        let buffer = rb.get_cell_text(1, 1);
        tap.is_int(buffer.len(), 0, "get_cell_text ERASE at 1,1");

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders text to terminal",
            [
                GOTO(0,1), SETPEN(pen_log.fg(1)), PRINT("text span"),
                GOTO(1,1), SETPEN(pen_log.fg(1)), ERASECH(5,None),
                GOTO(2,1), SETPEN(pen_log.bg(2)), PRINT("another span"),
                GOTO(3,1), SETPEN(pen_log.bg(2)), ERASECH(10,None),
                GOTO(4,1), SETPEN(pen_log.fg(1).bg(2)), PRINT("third span"),
                GOTO(5,1), SETPEN(pen_log.fg(1).bg(2)), ERASECH(7,None),
            ]);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer now empty after render to terminal",
            []);
    }

    // UTF-8 handling
    {
        let len = rb.text_at(6, 0, "somé text ĉi tie", None);
        tap.is_int(len, 16, "len from text_at UTF-8");

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders UTF-8 text",
            [
                GOTO(6,0), SETPEN(pen_log), PRINT("somé text ĉi tie"),
            ]);
    }

    // Span splitting
    {
        let b_pen = tickit::TickitPen::new().with_bool_attr(tickit::c::TICKIT_PEN_BOLD, true);

        // aaaAAaaa
        rb.text_at(0, 0, "aaaaaaaa", None);
        rb.text_at(0, 3, "AA", Some(&b_pen));

        // BBBBBBBB
        rb.text_at(1, 2, "bbbb", None);
        rb.text_at(1, 0, "BBBBBBBB", Some(&b_pen));

        // cccCCCCC
        rb.text_at(2, 0, "cccccc", None);
        rb.text_at(2, 3, "CCCCC", Some(&b_pen));

        // DDDDDddd
        rb.text_at(3, 2, "dddddd", None);
        rb.text_at(3, 0, "DDDDD", Some(&b_pen));

        // empty text should do nothing
        rb.text_at(4, 4, "", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer spans can be split",
            [
                GOTO(0,0), SETPEN(pen_log), PRINT("aaa"), SETPEN(pen_log.b(true)), PRINT("AA"), SETPEN(pen_log), PRINT("aaa"),
                GOTO(1,0), SETPEN(pen_log.b(true)), PRINT("BBBBBBBB"),
                GOTO(2,0), SETPEN(pen_log), PRINT("ccc"), SETPEN(pen_log.b(true)), PRINT("CCCCC"),
                GOTO(3,0), SETPEN(pen_log.b(true)), PRINT("DDDDD"), SETPEN(pen_log), PRINT("ddd"),
            ]);
    }

    {
        rb.text_at(0, 0, "abcdefghijkl", None);
        rb.text_at(0, 2, "-", None);
        rb.text_at(0, 4, "-", None);
        rb.text_at(0, 6, "-", None);
        rb.text_at(0, 8, "-", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders overwritten text split chunks",
            [
                GOTO(0,0),
                SETPEN(pen_log), PRINT("ab"),
                SETPEN(pen_log), PRINT("-"), // c
                SETPEN(pen_log), PRINT("d"),
                SETPEN(pen_log), PRINT("-"), // e
                SETPEN(pen_log), PRINT("f"),
                SETPEN(pen_log), PRINT("-"), // g
                SETPEN(pen_log), PRINT("h"),
                SETPEN(pen_log), PRINT("-"), // i
                SETPEN(pen_log), PRINT("jkl"),
            ]);
    }

    // VC spans
    {
        // Direct pen
        let fg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 3);
        rb.goto(0, 2);
        let len = rb.text("text span", Some(&fg_pen));
        tap.is_int(len, 9, "len from text");

        rb.goto(1, 2);
        rb.erase(5, Some(&fg_pen));

        // Stored pen
        let bg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 4);
        rb.setpen(&bg_pen);
        rb.goto(2, 2); rb.text("another span", None);
        rb.goto(3, 2); rb.erase(10, None);

        // Combined pens
        rb.goto(4, 2); rb.text("third span", Some(&fg_pen));
        rb.goto(5, 2); rb.erase(7, Some(&fg_pen));

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders text at VC",
            [
                GOTO(0,2), SETPEN(pen_log.fg(3)), PRINT("text span"),
                GOTO(1,2), SETPEN(pen_log.fg(3)), ERASECH(5,None),
                GOTO(2,2), SETPEN(pen_log.bg(4)), PRINT("another span"),
                GOTO(3,2), SETPEN(pen_log.bg(4)), ERASECH(10,None),
                GOTO(4,2), SETPEN(pen_log.fg(3).bg(4)), PRINT("third span"),
                GOTO(5,2), SETPEN(pen_log.fg(3).bg(4)), ERASECH(7,None),
            ]);
    }

    // Translation
    {
        rb.translate(3, 5);

        let len = rb.text_at(0, 0, "at 0,0", None);
        tap.is_int(len, 6, "len from text_at translated");

        rb.goto(1, 0);

        let (line, col) = rb.get_cursorpos().unwrap();
        tap.is_int(line, 1, "RenderBuffer line position after translate");
        tap.is_int(col,  0, "RenderBuffer column position after translate");

        rb.text("at 1,0", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders text with translation",
            [
                GOTO(3,5), SETPEN(pen_log), PRINT("at 0,0"),
                GOTO(4,5), SETPEN(pen_log), PRINT("at 1,0"),
            ]);
    }

    // Eraserect
    {
        rb.eraserect(&TickitRect{top: 2, left: 3, lines: 5, cols: 8}, None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders eraserect",
            [
                GOTO(2,3), SETPEN(pen_log), ERASECH(8,None),
                GOTO(3,3), SETPEN(pen_log), ERASECH(8,None),
                GOTO(4,3), SETPEN(pen_log), ERASECH(8,None),
                GOTO(5,3), SETPEN(pen_log), ERASECH(8,None),
                GOTO(6,3), SETPEN(pen_log), ERASECH(8,None),
            ]);
    }

    // Clear
    {
        let bg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 3);

        rb.clear(Some(&bg_pen));

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders clear",
            [
                GOTO(0,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(1,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(2,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(3,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(4,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(5,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(6,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(7,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(8,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
                GOTO(9,0), SETPEN(pen_log.bg(3)), ERASECH(20,None),
            ]);
    }
}

#[test]
fn test_31renderbuffer_line()
{
    let mut tap = taplib::Tap::new();

    let mut mt = make_term(25, 80);

    let mut rb = tickit::TickitRenderBuffer::new(30, 30);

    // Simple lines, explicit pen
    {
        let fg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 1);

        rb.hline_at(10, 10, 14, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.hline_at(11, 10, 14, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::TICKIT_LINECAP_START);
        rb.hline_at(12, 10, 14, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::TICKIT_LINECAP_END);
        rb.hline_at(13, 10, 14, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::TICKIT_LINECAP_BOTH);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders hline_at to terminal",
            [
                GOTO(10,10), SETPEN(pen_log.fg(1)), PRINT("╶───╴"),
                GOTO(11,10), SETPEN(pen_log.fg(1)), PRINT("────╴"),
                GOTO(12,10), SETPEN(pen_log.fg(1)), PRINT("╶────"),
                GOTO(13,10), SETPEN(pen_log.fg(1)), PRINT("─────"),
            ]);

        rb.vline_at(10, 13, 10, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.vline_at(10, 13, 11, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::TICKIT_LINECAP_START);
        rb.vline_at(10, 13, 12, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::TICKIT_LINECAP_END);
        rb.vline_at(10, 13, 13, tickit::c::TICKIT_LINE_SINGLE, Some(&fg_pen), tickit::c::TICKIT_LINECAP_BOTH);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders vline_at to terminal",
            [
                GOTO(10,10), SETPEN(pen_log.fg(1)), PRINT("╷│╷│"),
                GOTO(11,10), SETPEN(pen_log.fg(1)), PRINT("││││"),
                GOTO(12,10), SETPEN(pen_log.fg(1)), PRINT("││││"),
                GOTO(13,10), SETPEN(pen_log.fg(1)), PRINT("╵╵││"),
            ]);
    }

    // Lines setpen
    {
        let bg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 3);

        rb.setpen(&bg_pen);

        rb.hline_at(10, 8, 12, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.vline_at(8, 12, 10, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);

        let mask = rb.get_cell_linemask(9, 10);
        tap.is_int(mask.north, tickit::c::TICKIT_LINE_SINGLE, "get_cell_linemask north");
        tap.is_int(mask.south, tickit::c::TICKIT_LINE_SINGLE, "get_cell_linemask south");
        tap.is_int(mask.east,  tickit::c::X_TICKIT_LINE_NONE, "get_cell_linemask east");
        tap.is_int(mask.west,  tickit::c::X_TICKIT_LINE_NONE, "get_cell_linemask west");

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders lines with stored pen",
            [
                GOTO( 8,10), SETPEN(pen_log.bg(3)),   PRINT("╷"),
                GOTO( 9,10), SETPEN(pen_log.bg(3)),   PRINT("│"),
                GOTO(10, 8), SETPEN(pen_log.bg(3)), PRINT("╶─┼─╴"),
                GOTO(11,10), SETPEN(pen_log.bg(3)),   PRINT("│"),
                GOTO(12,10), SETPEN(pen_log.bg(3)),   PRINT("╵"),
            ]);
    }

    // Line merging
    {
        rb.hline_at(10, 10, 14, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.hline_at(11, 10, 14, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.hline_at(12, 10, 14, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.vline_at(10, 12, 10, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.vline_at(10, 12, 12, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.vline_at(10, 12, 14, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders line merging",
            [
                GOTO(10,10), SETPEN(pen_log), PRINT("┌─┬─┐"),
                GOTO(11,10), SETPEN(pen_log), PRINT("├─┼─┤"),
                GOTO(12,10), SETPEN(pen_log), PRINT("└─┴─┘"),
            ]);
    }
}

#[test]
fn test_32renderbuffer_char()
{
    let mut tap = taplib::Tap::new();

    let mut mt = make_term(25, 80);

    let mut rb = tickit::TickitRenderBuffer::new(10, 20);

    // Absolute characters
    {
        let fg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 4);

        rb.char_at(5, 5, 0x41 as char, Some(&fg_pen));
        rb.char_at(5, 6, 0x42 as char, Some(&fg_pen));
        rb.char_at(5, 7, 0x43 as char, Some(&fg_pen));

        let buffer = rb.get_cell_text(5, 5);
        tap.is_int(buffer.len(), 1, "get_cell_text CHAR at 5,5");
        tap.is_str(buffer, "A", "buffer text at 5,5");

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders char_at to terminal",
            [
                GOTO(5,5), SETPEN(pen_log.fg(4)), PRINT("A"),
                           SETPEN(pen_log.fg(4)), PRINT("B"),
                           SETPEN(pen_log.fg(4)), PRINT("C"),
            ]);
    }

    // Characters setpen
    {
        let bg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 5);
        rb.setpen(&bg_pen);

        rb.char_at(5, 5, 0x44 as char, None);
        rb.char_at(5, 6, 0x45 as char, None);
        rb.char_at(5, 7, 0x46 as char, None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders char_at with stored pen",
            [
                GOTO(5,5), SETPEN(pen_log.bg(5)), PRINT("D"),
                           SETPEN(pen_log.bg(5)), PRINT("E"),
                           SETPEN(pen_log.bg(5)), PRINT("F"),
            ]);
    }

    // VC characters
    {
        rb.goto(0, 4);

        // Direct pen
        let fg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 5);
        rb.char(0x47 as char, Some(&fg_pen));

        // Stored pen
        let bg_pen = tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 6);
        rb.setpen(&bg_pen);
        rb.char(0x48 as char, None);

        // Combined pens
        rb.char(0x49 as char, Some(&fg_pen));

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders chars at VC",
            [
                GOTO(0,4), SETPEN(pen_log.fg(5)), PRINT("G"),
                           SETPEN(pen_log.bg(6)), PRINT("H"),
                           SETPEN(pen_log.fg(5).bg(6)), PRINT("I"),
            ]);
    }

    // Characters with translation
    {
        rb.translate(3, 5);

        rb.char_at(1, 1, 0x31 as char, None);
        rb.char_at(1, 2, 0x32 as char, None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders char_at with translation",
            [
                GOTO(4,6), SETPEN(pen_log), PRINT("1"),
                           SETPEN(pen_log), PRINT("2"),
            ]);
    }
}

#[test]
fn test_33renderbuffer_clip()
{
    let mut tap = taplib::Tap::new();

    let mut mt = make_term(25, 80);

    let mut rb = tickit::TickitRenderBuffer::new(10, 20);

    // Clipping to edge
    {
        let len = rb.text_at(-1, 5, "TTTTTTTTTT", None);
        tap.is_int(len, 10, "len from text_at clipped off top");
        let len = rb.text_at(11, 5, "BBBBBBBBBB", None);
        tap.is_int(len, 10, "len from text_at clipped off bottom");
        let len = rb.text_at(4, -3, "[LLLLLLLL]", None);
        tap.is_int(len, 10, "len from text_at clipped off left");
        let len = rb.text_at(5, 15, "[RRRRRRRR]", None);
        tap.is_int(len, 10, "len from text_at clipped off right");

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer text rendering with clipping",
            [
                GOTO(4,0), SETPEN(pen_log), PRINT("LLLLLL]"),
                GOTO(5,15), SETPEN(pen_log), PRINT("[RRRR"),
            ]);

        rb.erase_at(-1, 5, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 1)));
        rb.erase_at(11, 5, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 2)));
        rb.erase_at(4, -3, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 3)));
        rb.erase_at(5, 15, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 4)));

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer erasech rendering with clipping",
            [
                GOTO(4,0), SETPEN(pen_log.fg(3)), ERASECH(7,None),
                GOTO(5,15), SETPEN(pen_log.fg(4)), ERASECH(5,None),
            ]);

        rb.goto(2, 18);
        rb.text("A", None);
        rb.text("B", None);
        rb.text("C", None);
        rb.text("D", None);
        rb.text("E", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer text at VC with clipping",
            [
                GOTO(2,18), SETPEN(pen_log), PRINT("A"),
                            SETPEN(pen_log), PRINT("B"),
            ]);
    }

    // Clipping to rect
    {
        rb.clip(&TickitRect{top: 2, left: 2, lines: 6, cols: 16});

        rb.text_at(1, 5, "TTTTTTTTTT", None);
        rb.text_at(9, 5, "BBBBBBBBBB", None);
        rb.text_at(4, -3, "[LLLLLLLL]", None);
        rb.text_at(5, 15, "[RRRRRRRR]", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders text rendering with rect clipping",
            [
                GOTO(4,2), SETPEN(pen_log), PRINT("LLLL]"),
                GOTO(5,15), SETPEN(pen_log), PRINT("[RR"),
            ]);

        rb.clip(&TickitRect{top: 2, left: 2, lines: 6, cols: 16});

        rb.erase_at(1, 5, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 1)));
        rb.erase_at(9, 5, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 2)));
        rb.erase_at(4, -3, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 3)));
        rb.erase_at(5, 15, 10, Some(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 4)));

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer renders erasech with rect clipping",
            [
                GOTO(4,2), SETPEN(pen_log.fg(3)), ERASECH(5,None),
                GOTO(5,15), SETPEN(pen_log.fg(4)), ERASECH(3,None),
            ]);
    }

    // Clipping with translation
    {
        rb.translate(3, 5);

        rb.clip(&TickitRect{top: 2, left: 2, lines: 3, cols: 5});

        rb.text_at(1, 0, "1111111111", None);
        rb.text_at(2, 0, "2222222222", None);
        rb.text_at(3, 0, "3333333333", None);
        rb.text_at(4, 0, "4444444444", None);
        rb.text_at(5, 0, "5555555555", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer clipping rectangle translated",
            [
                GOTO(5,7), SETPEN(pen_log), PRINT("22222"),
                GOTO(6,7), SETPEN(pen_log), PRINT("33333"),
                GOTO(7,7), SETPEN(pen_log), PRINT("44444"),
            ]);
    }
}

#[test]
fn test_34renderbuffer_save()
{
    let mut tap = taplib::Tap::new();

    let mut mt = make_term(25, 80);

    let mut rb = tickit::TickitRenderBuffer::new(10, 20);

    // Position
    {
        rb.goto(2, 2);

        {
            rb.save();

            rb.goto(4, 4);
            let (line, col) = rb.get_cursorpos().unwrap();
            tap.is_int(line, 4, "line before restore");
            tap.is_int(col,  4, "col before restore");

            rb.restore();
        }

        let (line, col) = rb.get_cursorpos().unwrap();
        tap.is_int(line, 2, "line after restore");
        tap.is_int(col,  2, "col after restore");

        rb.text("some text", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "Stack saves/restores virtual cursor position",
            [
                GOTO(2,2), SETPEN(pen_log), PRINT("some text"),
            ]);
    }

    // Clipping
    {
        rb.text_at(0, 0, "0000000000", None);

        {
            rb.save();
            rb.clip(&TickitRect{top: 0, left: 2, lines: 10, cols: 16});

            rb.text_at(1, 0, "1111111111", None);

            rb.restore();
        }

        rb.text_at(2, 0, "2222222222", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "Stack saves/restores clipping region",
            [
                GOTO(0,0), SETPEN(pen_log), PRINT("0000000000"),
                GOTO(1,2), SETPEN(pen_log), PRINT("11111111"),
                GOTO(2,0), SETPEN(pen_log), PRINT("2222222222"),
            ]);
    }

    // Pen
    {
        rb.goto(3, 0);

        rb.setpen(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, 1));

        rb.text("123", None);

        {
            rb.savepen();

            rb.setpen(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_FG, 4));

            rb.text("456", None);

            rb.setpen(&tickit::TickitPen::new().with_colour_attr(tickit::c::TICKIT_PEN_BG, -1));

            rb.text("789", None);

            rb.restore();
        }

        rb.text("ABC", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "Stack saves/restores render pen",
            [
                GOTO(3,0), SETPEN(pen_log.bg(1)), PRINT("123"),
                           SETPEN(pen_log.bg(1).fg(4)), PRINT("456"),
                           SETPEN(pen_log), PRINT("789"),
                           SETPEN(pen_log.bg(1)), PRINT("ABC"),
            ]);

        rb.goto(4, 0);

        rb.setpen(&tickit::TickitPen::new().with_bool_attr(tickit::c::TICKIT_PEN_REVERSE, true));

        rb.text("123", None);

        {
            rb.savepen();

            rb.setpen(&tickit::TickitPen::new().with_bool_attr(tickit::c::TICKIT_PEN_REVERSE, false));

            rb.text("456", None);

            rb.restore();
        }

        rb.text("789", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "Stack saves/restores allow zeroing pen attributes",
            [
                GOTO(4,0), SETPEN(pen_log.rv(true)), PRINT("123"),
                           SETPEN(pen_log), PRINT("456"),
                           SETPEN(pen_log.rv(true)), PRINT("789"),
            ]);
    }

    // Translation
    {
        rb.text_at(0, 0, "A", None);

        rb.save();
        {
            rb.translate(2, 2);

            rb.text_at(1, 1, "B", None);
        }
        rb.restore();

        rb.text_at(2, 2, "C", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "Stack saves/restores translation offset",
            [
                GOTO(0,0), SETPEN(pen_log), PRINT("A"),
                GOTO(2,2), SETPEN(pen_log), PRINT("C"),
                GOTO(3,3), SETPEN(pen_log), PRINT("B"),
            ]);
    }
}

#[test]
fn test_35renderbuffer_mask()
{
    let mut tap = taplib::Tap::new();

    let mut mt = make_term(25, 80);

    let mut rb = tickit::TickitRenderBuffer::new(10, 20);

    let mask = TickitRect{top: 3, left: 5, lines: 4, cols: 6};

    // Mask over text
    {
        rb.mask(&mask);

        rb.text_at(3, 2, "ABCDEFG", None);      // before
        rb.text_at(4, 6, "HI", None);           // inside
        rb.text_at(5, 8, "JKLMN", None);        // after
        rb.text_at(6, 2, "OPQRSTUVWXYZ", None); // spanning

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer masking around text",
            [
                GOTO(3, 2), SETPEN(pen_log), PRINT("ABC"),
                GOTO(5,11), SETPEN(pen_log), PRINT("MN"),
                GOTO(6, 2), SETPEN(pen_log), PRINT("OPQ"),
                GOTO(6,11), SETPEN(pen_log), PRINT("XYZ"),
            ]);
    }

    // Mask over erase
    {
        rb.mask(&mask);

        rb.erase_at(3, 2,  6, None); // before
        rb.erase_at(4, 6,  2, None); // inside
        rb.erase_at(5, 8,  5, None); // after
        rb.erase_at(6, 2, 12, None); // spanning

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer masking around erasech",
            [
                GOTO(3, 2), SETPEN(pen_log), ERASECH(3,None),
                GOTO(5,11), SETPEN(pen_log), ERASECH(2,None),
                GOTO(6, 2), SETPEN(pen_log), ERASECH(3,None),
                GOTO(6,11), SETPEN(pen_log), ERASECH(3,None),
            ]);
    }

    // Mask over lines
    {
        rb.mask(&mask);

        rb.hline_at(3, 2,  8, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.hline_at(4, 6,  8, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.hline_at(5, 8, 13, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);
        rb.hline_at(6, 2, 14, tickit::c::TICKIT_LINE_SINGLE, None, tickit::c::X_TICKIT_LINECAP_NEITHER);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer masking around lines",
            [
                GOTO(3, 2), SETPEN(pen_log), PRINT("╶──"),
                GOTO(5,11), SETPEN(pen_log), PRINT("──╴"),
                GOTO(6, 2), SETPEN(pen_log), PRINT("╶──"),
                GOTO(6,11), SETPEN(pen_log), PRINT("───╴"),
            ]);
    }

    // Restore removes masks
    {
        rb.save();
        {
            rb.mask(&mask);

            rb.text_at(3, 0, "AAAAAAAAAAAAAAAAAAAA", None);
        }
        rb.restore();

        rb.text_at(4, 0, "BBBBBBBBBBBBBBBBBBBB", None);

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer save/restore removes mask",
            [
                GOTO(3, 0), SETPEN(pen_log), PRINT("AAAAA"),
                GOTO(3,11), SETPEN(pen_log), PRINT("AAAAAAAAA"),
                GOTO(4, 0), SETPEN(pen_log), PRINT("BBBBBBBBBBBBBBBBBBBB"),
            ]);
    }

    // translate over mask
    {
        rb.mask(&TickitRect{top: 2, left: 2, lines: 1, cols: 1});

        {
            rb.save();
            rb.translate(0, 0);
            rb.text_at(0, 0, "A", None);
            rb.restore();
        }
        {
            rb.save();
            rb.translate(0, 2);
            rb.text_at(0, 0, "B", None);
            rb.restore();
        }
        {
            rb.save();
            rb.translate(2, 0);
            rb.text_at(0, 0, "C", None);
            rb.restore();
        }
        {
            rb.save();
            rb.translate(2, 2);
            rb.text_at(0, 0, "D", None);
            rb.restore();
        }

        rb.flush_to_term(&mut mt.tt);
        tap.is_termlog(&mut mt, "RenderBuffer translate over mask",
            [
                GOTO(0,0), SETPEN(pen_log), PRINT("A"),
                GOTO(0,2), SETPEN(pen_log), PRINT("B"),
                GOTO(2,0), SETPEN(pen_log), PRINT("C"),
                // D was masked
            ]);
    }

    // Mask out of limits doesn't SEGV
    {
        rb.save();

        rb.mask(&TickitRect{top: 0, left: 0, lines: 50, cols: 200});

        rb.mask(&TickitRect{top: -10, left: -20, lines: 5, cols: 20});

        rb.restore();
    }
}
