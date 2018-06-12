use super::*;
use std::string::ToString;
#[test]
fn cst_to_string() {
    let v: Vec<C> = vec![C::C('g'), C::N(103)];
    let c: CSt = v.into();
    let s = c.to_string();
    assert_eq!(&s, "gg");
}
#[test]
#[should_panic(expected = "4000000 not valid unicode codepoint: CharTryFromError(())")]
fn cst_to_string_invalid() {
    let v: Vec<C> = vec![C::N(4000000)];
    let c: CSt = v.into();
    let _s = c.to_string();
}
