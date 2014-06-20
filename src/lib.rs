#![crate_id = "tickit#0.0.2014.5.2.0"]
#![crate_type = "dylib"]

pub fn get_hello() -> String
{
    return "Hello, World!".to_string()
}

pub fn hello()
{
    println!("{}", get_hello());
}
