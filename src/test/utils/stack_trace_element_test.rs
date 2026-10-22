use crate::utils::StackTraceElement;

#[test]
pub fn ste_display_known_source() {
    let e = StackTraceElement::new("com.example.Foo", "bar", Some("Foo.rs"), Some(42u32));
    assert_eq!(e.to_string(), "com.example.Foo.bar(Foo.rs:42)");
}

#[test]
pub fn ste_display_native() {
    let e = StackTraceElement::new("Foo", "bar", None::<&str>, None)
        .with_native_method(true);
    assert_eq!(e.to_string(), "Foo.bar(Native Method)");
}

#[test]
pub fn ste_display_unknown() {
    let e = StackTraceElement::new("Foo", "bar", None::<&str>, None);
    assert_eq!(e.to_string(), "Foo.bar(Unknown Source)");
}