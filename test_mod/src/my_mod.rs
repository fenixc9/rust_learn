pub mod A {
    pub struct C;
    pub fn getC(){
    }
}


pub mod B {
    use self::super::A;
    pub struct D;
    use crate::mod_call;
    pub fn getC(){
        let c = A::C {};
        let side = mod_call::OutSide {};
    }
}