#![crate_id = "tickit#0.0.2014.5.2.0"]
#![crate_type = "dylib"]

mod c;

pub fn get_hello() -> String
{
    return "Hello, World!".to_string()
}

pub fn hello()
{
    println!("{}", get_hello());
}

pub struct TickitTerm
{
    tt: *mut c::TickitTerm,
}

impl TickitTerm
{
    pub fn new() -> TickitTerm
    {
        unsafe
        {
            TickitTerm{tt: c::tickit_term_new()}
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
        }
    }
}
