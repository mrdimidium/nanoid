use std::borrow::Cow;

static URL_SYMBOLS : &'static str =
    "_~0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub extern fn generate<'a>() -> Cow<'a, str> {
    let input : &'a str = URL_SYMBOLS;

    return input.into();
}

#[test]
fn it_works() {
    assert_eq!(generate(), URL_SYMBOLS);
}
