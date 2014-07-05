extern crate libc;
extern crate native;

extern crate tickit;

#[start]
fn start(argc: int, argv: *const *const u8) -> int
{
    native::start(argc, argv, main)
}

fn render_modifier(tt: &mut tickit::TickitTerm, mods: tickit::c::X_Tickit_Mod)
{
    if mods.is_empty()
    {
        return;
    }
    let mut pipe: int = 0;
    tt.erasech(3, 1);
    tt.print("<");

    if !(mods & tickit::c::TICKIT_MOD_SHIFT).is_empty()
    {
        tt.print(if pipe != 0 { "|SHIFT" } else { "SHIFT" });
        pipe += 1;
    }
    if !(mods & tickit::c::TICKIT_MOD_ALT).is_empty()
    {
        tt.print(if pipe != 0 { "|ALT" } else { "ALT" });
        pipe += 1;
    }
    if !(mods & tickit::c::TICKIT_MOD_CTRL).is_empty()
    {
        tt.print(if pipe != 0 { "|CTRL" } else { "CTRL" });
        pipe += 1;
    }
    let _ = pipe;

    tt.print(">");
}

fn render_key<'a>(tt: &mut tickit::TickitTerm, ev: Option<&tickit::TickitKeyEvent<'a>>)
{
    tt.goto(2, 2);
    tt.print("Key:");

    tt.goto(4, 4);
    let ev = match ev
    {
        Some(v) => { v }
        None => { return; }
    };
    let (str_, mods) = match *ev
    {
        tickit::KeyTextEvent{text, mod_} => { tt.print("text "); (text, mod_) }
        tickit::KeyKeyEvent{key, mod_} => { tt.print("key  "); (key, mod_) }
    };
    tt.print(str_);
    render_modifier(tt, mods);
    tt.erasech(30, -1);
}

fn render_mouse(tt: &mut tickit::TickitTerm, ev: Option<&tickit::TickitMouseEvent>)
{
    tt.goto(8, 2);
    tt.print("Mouse:");

    tt.goto(10, 4);
    let ev = match ev
    {
        Some(v) => { v }
        None => { return; }
    };
    let mods = match *ev
    {
        tickit::MousePressEvent{button, line, col, mod_} =>
        {
            tt.print(format!("press   button {} at ({},{})", button, line, col).as_slice());
            mod_
        }
        tickit::MouseDragEvent{button, line, col, mod_} =>
        {
            tt.print(format!("drag    button {} at ({},{})", button, line, col).as_slice());
            mod_
        }
        tickit::MouseReleaseEvent{button, line, col, mod_} =>
        {
            tt.print(format!("release button {} at ({},{})", button, line, col).as_slice());
            mod_
        }
        tickit::MouseWheelEvent{dir, line, col, mod_} =>
        {
            tt.print(format!("wheel {} at ({},{})", if dir == tickit::c::TICKIT_MOUSEWHEEL_DOWN { "down" } else { "up" }, line, col).as_slice());
            mod_
        }
    };

    render_modifier(tt, mods);
    tt.erasech(20, -1);
}

fn event(tt: &mut tickit::TickitTerm, ev: &tickit::TickitEvent)
{
    match *ev
    {
        tickit::ResizeEvent{..} => {}
        tickit::ChangeEvent => {}
        tickit::UnbindEvent => {}

        tickit::KeyEvent(key) =>
        {
            match key
            {
                tickit::KeyKeyEvent{key, ..} if key == "C-c" =>
                {
                    // should raise signal?
                    fail!("not really failing, just exiting");
                    //return;
                }
                _ => {}
            }

            render_key(tt, Some(&key));
        }

        tickit::MouseEvent(mouse) =>
        {
            render_mouse(tt, Some(&mouse));
        }
        _ => {}
    }
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
    tt.setctl_int(tickit::c::TICKIT_TERMCTL_MOUSE, tickit::c::TICKIT_TERM_MOUSEMODE_DRAG as int);
    tt.setctl_int(tickit::c::TICKIT_TERMCTL_KEYPAD_APP, 1);
    tt.clear();

    tt.x_bind_event_forever(tickit::c::TICKIT_EV_KEY|tickit::c::TICKIT_EV_MOUSE, event);

    render_key(&mut tt, None);
    render_mouse(&mut tt, None);

    // TODO figure out why this isn't working
    // My guess is that the other thread is keeping it alive.
    while hack.rx.try_recv().is_err()
    {
        tt.input_wait(None);
    }
}
