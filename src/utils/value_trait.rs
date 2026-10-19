use std::{
    any::Any, 
    fmt::{
        Debug,
        Display
    }, 
    hash::{
        Hash,
        Hasher
    }
};

pub trait Value: Debug + Display + Send + Sync + 'static{
    fn as_any(&self)->&dyn Any;
    fn eq_dyn(&self, other:&dyn Value)->bool;
    fn hash_dyn(&self, state:&mut dyn Hasher);

    fn ne_dyn(&self, other:&dyn Value)->bool{
        !self.eq_dyn(other)
    }
}

impl Value for Box<dyn Value> {
    fn eq_dyn(&self, other:&dyn Value)->bool {
        (**self).eq_dyn(other)
    }
    fn hash_dyn(&self, state:&mut dyn Hasher) {
        (**self).hash_dyn(state);
    }
    fn as_any(&self)->&dyn Any {
        (**self).as_any()
    }
}

macro_rules! impl_primitivos {
    ($($tipo:ty),* $(,)?) => {
        $(
            impl Value for $tipo {
                fn as_any(&self)-> &dyn Any{
                    self
                }
                fn eq_dyn(&self, other:&dyn Value)->bool{
                    other
                        .as_any()
                        .downcast_ref::<Self>()
                        .is_some_and(|value| self == value)
                }
                fn hash_dyn(&self, state:&mut dyn Hasher){
                    let mut state = state;
                    self.hash(&mut state)
                }
            }
        )*
    };
}

impl_primitivos!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    bool, char
);

impl Value for f32 {
    fn as_any(&self)->&dyn Any {
        self
    }

    fn eq_dyn(&self, other:&dyn Value)->bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(
                |value| 
                    float32_key(*self) == float32_key(*value)
            )
    }

    fn hash_dyn(&self, state:&mut dyn Hasher) {
        state.write_u32(float32_key(*self));
    }
}

impl Value for f64 {
    fn as_any(&self)->&dyn Any {
        self
    }

    fn eq_dyn(&self, other:&dyn Value)->bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(
                |value| 
                    float64_key(*self) == float64_key(*value))
    }

    fn hash_dyn(&self, state:&mut dyn Hasher) {
        state.write_u64(float64_key(*self));
    }
}

fn float32_key(value:f32)->u32{
    if value.is_nan(){
        f32::NAN.to_bits()
    }else if value.is_infinite() {
        f32::INFINITY.to_bits()
    }else {
        value.to_bits()
    }
}

fn float64_key(value:f64)->u64{
    if value.is_nan(){
        f64::NAN.to_bits()
    }else if value.is_infinite(){
        f64::INFINITY.to_bits()
    }else {
        value.to_bits()
    }
}