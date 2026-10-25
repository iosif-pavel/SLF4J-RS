use crate::org::slf4j::event::Level;

#[test]
pub fn levels_eq(){
    let a = Level::Debug;
    let b = Level::int_to_level(10);

    assert!(b.is_some(), "Level expected");
    assert_eq!(a, b.unwrap());
}
#[test]
pub fn levels_incorrect_number(){
    let a = Level::int_to_level(5);

    assert!(a.is_none(), "NO EXISTE NINGUN LEVEL CON EL IDENTIFICADOR 5")
}