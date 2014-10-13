extern crate libc;
extern crate native;

extern crate signals;

extern crate tickit;

#[start]
fn start(argc: int, argv: *const *const u8) -> int
{
    native::start(argc, argv, main)
}

static COLOURS: &'static [(&'static str, int)] =
&[
    ("red   ", 1),
    ("blue  ", 4),
    ("green ", 2),
    ("yellow", 3),
];

static ATTRS: &'static [(&'static str, tickit::c::TickitPenAttr)] =
&[
    ("bold",          tickit::c::TICKIT_PEN_BOLD),
    ("underline",     tickit::c::TICKIT_PEN_UNDER),
    ("italic",        tickit::c::TICKIT_PEN_ITALIC),
    ("strikethrough", tickit::c::TICKIT_PEN_STRIKE),
    ("reverse video", tickit::c::TICKIT_PEN_REVERSE),
];

fn main()
{
    let mut tt = match tickit::TickitTerm::new()
    {
        Ok(o) => { o }
        Err(errno) => { fail!("Cannot create TickitTerm - errno #{}", errno); }
    };

    tt.set_input_fd(libc::STDIN_FILENO);
    tt.set_output_fd(libc::STDOUT_FILENO);
    let await = libc::timeval{ tv_sec: 0, tv_usec: 50000 };
    tt.await_started(Some(await));

    tt.setctl_int(tickit::c::TICKIT_TERMCTL_ALTSCREEN, 1);
    tt.setctl_int(tickit::c::TICKIT_TERMCTL_CURSORVIS, 0);
    tt.clear();

    let default_pen = tickit::TickitPen::new();

    let mut pen = tickit::TickitPen::new();

    /* ANSI colours foreground */
    tt.goto(0, 0);

    for &(ref name, ref val) in COLOURS.iter()
    {
        pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, *val);
        tt.setpen(&pen);
        tt.print(format!("fg {}", name).as_slice());

        tt.setpen(&default_pen);
        tt.print("     ");
    }

    tt.goto(2, 0);

    /* ANSI high-brightness colours foreground */
    for &(ref name, ref val) in COLOURS.iter()
    {
        pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, val+8);
        tt.setpen(&pen);
        tt.print(format!("fg hi-{}", name).as_slice());

        tt.setpen(&default_pen);
        tt.print("  ");
    }

    /* ANSI colours background */
    tt.goto(4, 0);

    pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 0);

    for &(ref name, ref val) in COLOURS.iter()
    {
        pen.set_colour_attr(tickit::c::TICKIT_PEN_BG, *val);
        tt.setpen(&pen);
        tt.print(format!("bg {}", name).as_slice());

        tt.setpen(&default_pen);
        tt.print("     ");
    }

    tt.goto(6, 0);

    /* ANSI high-brightness colours background */
    for &(ref name, ref val) in COLOURS.iter()
    {
        pen.set_colour_attr(tickit::c::TICKIT_PEN_BG, val+8);
        tt.setpen(&pen);
        tt.print(format!("bg hi-{}", name).as_slice());

        tt.setpen(&default_pen);
        tt.print("  ");
    }

    pen.clear_attr(tickit::c::TICKIT_PEN_FG);
    pen.clear_attr(tickit::c::TICKIT_PEN_BG);

    /* Some interesting rendering attributes */
    for (i, &(ref name, ref attr)) in ATTRS.iter().enumerate()
    {
        tt.goto((8 + 2*i) as int, 0);

        pen.set_bool_attr(*attr, true);
        tt.setpen(&pen);
        tt.print(*name);

        pen.clear_attr(*attr);
    }

    tt.goto(18, 0);

    pen.set_int_attr(tickit::c::TICKIT_PEN_ALTFONT, 1);
    tt.setpen(&pen);
    tt.print("alternate font");

    let sig = signals::Signals::new().unwrap();
    sig.subscribe(signals::Interrupt);

    while sig.receiver().try_recv().is_err()
    {
        tt.input_wait(None);
    }
}
