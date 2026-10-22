use std::hash::{DefaultHasher, Hash, Hasher};

use crate::utils::Value;

#[test]
pub fn test_eq_value(){
    /*
        * Esto no funcionara si el tipo de los numeros es diferente
        * Es decir, si tratas de comparar un 1u8 y un 1u16 fallara 
        */
    let a:Box<dyn Value> = Box::new(1u8);
    let b:Box<dyn Value> = Box::new(1u8);

    assert!(a.eq_dyn(&b));
    assert!(b.eq_dyn(&a));
}

#[test]
pub fn test_hash_value(){
    let v = 300u16;
    let mut hasher_1 = DefaultHasher::new();
    v.hash(&mut hasher_1);
    let hash_1 = hasher_1.finish();

    let v_box:Box<dyn Value> = Box::new(v);
    let mut hasher_2 = DefaultHasher::new();
    v_box.hash_dyn(&mut hasher_2);
    let hash_2 = hasher_2.finish();
    
    assert_eq!(hash_1, hash_2);
}
