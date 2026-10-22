use crate::utils::Throwable;

#[test]
pub fn throwable_display() {
    assert_eq!(Throwable::with_message("boom").to_string(), "Throwable: boom");
}

#[test]
pub fn throwable_capture_has_frames() {
    let t = Throwable::capture("test");
    assert!(!t.stack_trace().is_empty(), "expected captured frames");
}

#[test]
pub fn cause_chain() {
    let root = Throwable::with_message("root");
    let mid = Throwable::capture_caused_by("mid", root);
    let top = Throwable::capture_caused_by("top", mid);
    assert_eq!(top.message(), Some("top"));
    assert_eq!(top.cause().unwrap().message(), Some("mid"));
    assert_eq!(top.cause().unwrap().cause().unwrap().message(), Some("root"));
}

#[test]
pub fn from_rust_error_walks_source() {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct Leaf;
    impl fmt::Display for Leaf {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("leaf") }
    }
    impl Error for Leaf {}

    #[derive(Debug)]
    struct Wrapper(Leaf);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("wrapper") }
    }
    impl Error for Wrapper {
        fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.0) }
    }

    let t = Throwable::from_error(&Wrapper(Leaf));
    assert_eq!(t.message(), Some("wrapper"));
    assert_eq!(t.cause().unwrap().message(), Some("leaf"));
}