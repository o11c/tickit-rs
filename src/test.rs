extern crate tickit;

#[test]
fn test_hello()
{
    assert!(tickit::get_hello().as_slice() == "Hello, World!");
}
