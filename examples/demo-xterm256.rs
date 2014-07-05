extern crate libc;
extern crate native;

extern crate tickit;

#[start]
fn start(argc: int, argv: *const *const u8) -> int
{
    native::start(argc, argv, main)
}

fn main()
{
    let hack = tickit::signal_hacks::RemoteGreenSignalListener::new();

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
    tt.setctl_str(tickit::c::TICKIT_TERMCTL_TITLE_TEXT, "XTerm256 colour demo");
    tt.clear();

    let default_pen = tickit::TickitPen::new();

    let mut pen = tickit::TickitPen::new();

    tt.goto(0, 0);
    tt.setpen(&default_pen);
    tt.print("ANSI");

    tt.goto(2, 0);
    for i in range(0, 16)
    {
        pen.set_colour_attr(tickit::c::TICKIT_PEN_BG, i);
        tt.setpen(&pen);
        tt.print(format!("[{:02d}]", i).as_slice());
    }

    tt.goto(4, 0);
    tt.setpen(&default_pen);
    tt.print("216 RGB cube");

    for y in range(0, 6)
    {
        tt.goto(6+y, 0);
        for x in range(0, 36)
        {
            pen.set_colour_attr(tickit::c::TICKIT_PEN_BG, y*36 + x + 16);
            tt.setpen(&pen);
            tt.print("  ");
        }
    }

    tt.goto(13, 0);
    tt.setpen(&default_pen);
    tt.print("24 Greyscale ramp");

    tt.goto(15, 0);
    for i in range(0, 24)
    {
        pen.set_colour_attr(tickit::c::TICKIT_PEN_BG, 232 + i);
        if i > 12
        {
            pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 0);
        }
        tt.setpen(&pen);
        tt.print(format!("g{:02d}", i).as_slice());
    }

    while hack.rx.try_recv().is_err()
    {
        tt.input_wait(None);
    }
}
