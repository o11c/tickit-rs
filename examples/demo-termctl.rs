extern crate libc;
extern crate native;

extern crate signals;

extern crate tickit;

#[start]
fn start(argc: int, argv: *const *const u8) -> int
{
    native::start(argc, argv, main)
}

// TODO - figure out how to write the API for event handlers with data,
// so I can avoid this unsafety.
static mut vis: bool = true;
static mut blink: bool = true;
static mut shape: tickit::c::TickitTermCursorShape = tickit::c::TICKIT_TERM_CURSORSHAPE_BLOCK;


fn render_modes(tt: &mut tickit::TickitTerm)
{
    tt.goto(5, 3);
    tt.print("Cursor visible:  ");
    tt.print(if unsafe { vis } { "| >On< |  Off  |" } else { "|  On  | >Off< |" });

    tt.goto(7, 3);
    tt.print("Cursor blink:    ");
    tt.print(if unsafe { blink } { "| >On< |  Off  |" } else { "|  On  | >Off< |" });

    tt.goto(9, 3);
    tt.print("Cursor shape:    ");
    tt.print(
        match unsafe { shape }
        {
            tickit::c::TICKIT_TERM_CURSORSHAPE_BLOCK => { "| >Block< |  Under  |  Bar  |" }
            tickit::c::TICKIT_TERM_CURSORSHAPE_UNDER => { "|  Block  | >Under< |  Bar  |" }
            tickit::c::TICKIT_TERM_CURSORSHAPE_LEFT_BAR => { "|  Block  |  Under  | >Bar< |" }
        }
    );

    tt.goto(20, 10);
    tt.print("Cursor  >   <");
    tt.goto(20, 20);
}

fn event(tt: &mut tickit::TickitTerm, ev: &tickit::TickitEvent)
{
    let (line, col) = match *ev
    {
        tickit::MouseEvent(tickit::MousePressEvent{button: 1, line, col, mod_: _}) =>
        {
            (line, col)
        }
        _ => { return; }
    };

    if line == 5
    {
        if col >= 21 && col <= 26
        {
            unsafe { vis = true; }
        }
        else if col >= 28 && col <= 34
        {
            unsafe { vis = false; }
        }
        else
        {
            return;
        }

        tt.setctl_int(tickit::c::TICKIT_TERMCTL_CURSORVIS, unsafe { vis } as int);
    }

    if line == 7
    {
        if col >= 21 && col <= 26
        {
            unsafe { blink = true; }
        }
        else if col >= 28 && col <= 34
        {
            unsafe { blink = false; }
        }
        else
        {
            return;
        }

        tt.setctl_int(tickit::c::TICKIT_TERMCTL_CURSORBLINK, unsafe { blink } as int);
    }

    if line == 9
    {
        if col >= 21 && col <= 29
        {
            unsafe { shape = tickit::c::TICKIT_TERM_CURSORSHAPE_BLOCK; }
        }
        else if col >= 31 && col <= 39
        {
            unsafe { shape = tickit::c::TICKIT_TERM_CURSORSHAPE_UNDER; }
        }
        else if col >= 40 && col <= 47
        {
            unsafe { shape = tickit::c::TICKIT_TERM_CURSORSHAPE_LEFT_BAR; }
        }
        else
        {
            return;
        }

        tt.setctl_int(tickit::c::TICKIT_TERMCTL_CURSORSHAPE, unsafe { shape } as int);
    }

    render_modes(tt);
}

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
    tt.setctl_int(tickit::c::TICKIT_TERMCTL_MOUSE, tickit::c::TICKIT_TERM_MOUSEMODE_CLICK as int);
    tt.clear();

    tt.x_bind_event_forever(tickit::c::TICKIT_EV_MOUSE, event);

    render_modes(&mut tt);

    let sig = signals::Signals::new().unwrap();
    sig.subscribe(signals::Interrupt);

    while sig.receiver().try_recv().is_err()
    {
        tt.input_wait(None);
    }
}
