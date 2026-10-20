#[cfg(test)]
pub mod tests {
    use std::{any::Any, time::Instant};

    use crate::utils::{
        Argument, CLazy, CValue, StackTraceElement, Throwable,
        
    };

    // StackTrace and Throwable test
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

    #[test]
    pub fn test_argument_value() {
        let a = Argument::new_value(Box::new(32u16));
        let a = format!("{:?}", a.get());
        let b = Argument::new_value(Box::new(32u32));
        let b = format!("{:?}", b.get());
        println!("{:?} == {:?}", a, b);
        assert_eq!(a, b)
    }

    #[test]
    pub fn test_argument_lazy(){
        let a = Argument::new_lazy(
            Box::new(get_instant)
        );
        
        let b = Argument::new_lazy(
            Box::new(get_instant)
        );
        let a = format!("{}", a.get());
        let b = format!("{}", b.get());
        println!("{:?} == {:?}", a, b);
        assert_eq!(a, b)
    }

    fn get_instant()->Box<dyn CValue>{
        let inst = Instant::now();
        let dur = inst.elapsed().as_secs();
        let boxx: Box<dyn CValue> = Box::new(dur);
        boxx
    }
}