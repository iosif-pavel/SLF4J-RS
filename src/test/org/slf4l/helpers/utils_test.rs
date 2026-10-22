use crate::org::slf4j::helpers::{safe_get_boolean_system_property, safe_get_system_property, set_system_property};

#[test]
fn set_and_get_property(){
    set_system_property("test", "test_value");
    let tst = safe_get_system_property("test");
    assert!(tst.is_some(), "Value expected");
    assert_eq!("test_value", tst.unwrap())
}

#[test]
fn set_and_get_bool_property(){
    set_system_property("test", "true");
    let tst = safe_get_boolean_system_property("test");
    assert!(tst);
}