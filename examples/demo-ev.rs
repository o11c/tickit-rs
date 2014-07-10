// this exists just to prove that events can happen without staleness
extern crate tickit;
extern crate libc;

fn term_out()
{
    println!("o0");
    {
        let term = tickit::TickitTerm::new();
        println!("o1");
        drop(term);
    }
    println!("o2");
    let mut o: int = 0;
    {
        {
            println!("o3");
            let mut term = tickit::TickitTerm::new_for_termtype("xterm").unwrap();
            term.set_input_fd(0);
            println!("o4");
            let life = term.set_output_lively(|tt: &mut tickit::TickitTerm, arr: &[u8]| { o += 1; println!("output {} for {}", arr.len(), tt.get_input_fd()); });
            println!("o5");
            term.await_started(Some(libc::timeval{tv_sec: 1, tv_usec: 0}));
            println!("o6");
            drop(life);
            println!("o7");
        }
        println!("o8");

        let escaped_life =
        {
            println!("o9");
            let mut term = tickit::TickitTerm::new_for_termtype("xterm").unwrap();
            term.set_input_fd(0);
            println!("o10");
            let life = term.set_output_lively(|tt: &mut tickit::TickitTerm, arr: &[u8]| { o += 1; println!("output {} for {}", arr.len(), tt.get_input_fd()); });
            println!("o11");
            term.await_started(Some(libc::timeval{tv_sec: 1, tv_usec: 0}));
            println!("o12");
            life
        };
        println!("o13");
        drop(escaped_life);
    }
    println!("o14");
    println!("value: {}", o);
}

fn term()
{
    println!("t0");
    {
        let term = tickit::TickitTerm::new();
        println!("t1");
        drop(term);
    }
    println!("t2");
    let mut o: int = 0;
    {
        {
            println!("t3");
            let mut term = tickit::TickitTerm::new().unwrap();
            term.set_input_fd(0);
            term.set_output_fd(1);
            println!("t4");
            let life = term.bind_event_lively(tickit::c::TICKIT_EV_KEY, |term: &mut tickit::TickitTerm, ev: &tickit::TickitEvent| { o += 1; match *ev { tickit::KeyEvent(tickit::KeyKeyEvent{key: x, mod_: _}) => { println!("term input {} = '{}'", term.get_input_fd(), x); } _ => { println!("huh?") } } });
            println!("t5");
            println!("type something");
            term.input_wait(None);
            println!("t6");
            drop(life);
            println!("t7");
        }
        println!("t8");

        let escaped_life =
        {
            println!("t9");
            let mut term = tickit::TickitTerm::new().unwrap();
            term.set_input_fd(0);
            term.set_output_fd(1);
            println!("t10");
            let life = term.bind_event_lively(tickit::c::TICKIT_EV_KEY, |term: &mut tickit::TickitTerm, ev: &tickit::TickitEvent| { o += 1; match *ev { tickit::KeyEvent(tickit::KeyKeyEvent{key: x, mod_: _}) => { println!("term input {} = '{}'", term.get_input_fd(), x); } _ => { println!("huh?") } } });
            println!("t11");
            println!("type something");
            term.input_wait(None);
            println!("t12");
            life
        };
        println!("t13");
        drop(escaped_life);
    }
    println!("t14");
    println!("value: {}", o);
}

fn pen()
{
    println!("p0");
    {
        let pen = tickit::TickitPen::new();
        println!("p1");
        drop(pen);
    }
    println!("p2");
    let mut o: int = 0;
    {
        {
            println!("p3");
            let mut pen = tickit::TickitPen::new();
            println!("p4");
            let life = pen.bind_event_lively(tickit::c::TICKIT_EV_CHANGE, |pen: &mut tickit::TickitPen, ev: &tickit::TickitEvent| { o += 1; match *ev { tickit::ChangeEvent => { println!("pen changed, emp = {}, def = {}", pen.is_nonempty(), pen.is_nondefault()); } _ => { println!("huh?") } } });
            println!("p5");
            pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 3);
            println!("p6");
            drop(life);
            println!("p7");
        }
        println!("p8");

        let escaped_life =
        {
            println!("p9");
            let mut pen = tickit::TickitPen::new();
            println!("p10");
            let life = pen.bind_event_lively(tickit::c::TICKIT_EV_CHANGE, |pen: &mut tickit::TickitPen, ev: &tickit::TickitEvent| { o += 1; match *ev { tickit::ChangeEvent => { println!("pen changed, emp = {}, def = {}", pen.is_nonempty(), pen.is_nondefault()); } _ => { println!("huh?") } } });
            println!("p11");
            pen.set_colour_attr(tickit::c::TICKIT_PEN_FG, 3);
            println!("p12");
            life
        };
        println!("p13");
        drop(escaped_life);
    }
    println!("p14");
    println!("value: {}", o);
}

fn main()
{
    term_out();
    term();
    pen();
}
