#[no_mangle]
pub extern fn nanoid() -> i32 {
    5
}

#[test]
fn it_works() {
    assert_eq!(nanoid(), 5);
}
