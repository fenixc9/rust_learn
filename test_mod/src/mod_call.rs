mod A {
    struct A1;
    use super::B;
    fn fooA(){
        let b1 = B::B1 {};
    }
}

mod B{
    pub struct B1;
    fn fooB(){}
}

pub struct OutSide;