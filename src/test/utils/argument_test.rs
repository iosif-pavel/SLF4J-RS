use std::{sync::Arc, time::Instant};

use crate::utils::{Argument, CValue};

#[test]
pub fn test_argument_value() {
    let a = Argument::new_value(Arc::new(32u16));
    let a = format!("{:?}", a.get());
    let b = Argument::new_value(Arc::new(32u32));
    let b = format!("{:?}", b.get());
    println!("{:?} == {:?}", a, b);
    assert_eq!(a, b)
}

#[test]
pub fn test_argument_lazy(){
    let a = Argument::new_lazy(
        Arc::new(get_instant)
    );
    
    let b = Argument::new_lazy(
        Arc::new(get_instant)
    );
    let a = format!("{}", a.get());
    let b = format!("{}", b.get());
    println!("{:?} == {:?}", a, b);
    assert_eq!(a, b)
}

fn get_instant()->Arc<dyn CValue>{
    let inst = Instant::now();
    let dur = inst.elapsed().as_secs();
    let boxx: Arc<dyn CValue> = Arc::new(dur);
    boxx
}

