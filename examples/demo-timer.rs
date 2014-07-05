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
    tt.clear();

    let mut counter: int = 0;

    while hack.rx.try_recv().is_err()
    {
        let to = libc::timeval{ tv_sec: 1, tv_usec: 0 };
        tt.input_wait(Some(to));

        tt.goto(5, 5);
        tt.print(format!("Counter {}", counter).as_slice());
        counter += 1;
    }
}

