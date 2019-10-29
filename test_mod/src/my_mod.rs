pub mod A {
    pub struct C;
    pub fn getC(){
    }
}

pub mod B {
    use self::super::A;
    pub struct D;
    pub fn getC(){
        let c = A::C {};
    }
}