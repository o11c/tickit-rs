#![feature(macro_rules)]

extern crate libc;

extern crate tickit;

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

    impl Tap
    {
        pub fn plan_tests(&mut self, n: uint)
        {
            self.total = n;
            println!("1..{}", n);
        }
        #[allow(dead_code)]
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
fn test_00nothing()
{
    let mut tap = taplib::Tap::new();
    tap.is_str_escape("'\n\"", "\"\\n'", "nothing");
}
