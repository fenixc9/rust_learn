## mod
* Rust语言中，一个源码文件（rs）文件就是一个模块（mod）。
* 一个源码文件的模块还可以包含子模块。
#### 基本调用 
```rust
//my_mod.rs
pub mod A {
    pub struct C;
    pub fn getC(){

    }
}

pub mod B {
    pub struct D;
    pub fn getD(){

    }
}
```
可以看到一个名为test_mod.rs的文件包含了两个模块A和B
这时候在main.rs文件中只要这样就可以调用模块A和模块B了，
```rust
mod my_mod;
fn main() {
    let a = test_mod::A::C {};
    let d = test_mod::B::D {};
}
```
可以看到，必须要声明`mod my_mod;`表明调用了mod文件，同时被调用的module也必须是pub。
#### mod子模块互相调用
比如my_mod.rs下AB两个模块互相调用
```rust
// mod_call.rs
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
```
可以看到use super关键字可以访问到另一个子模块。

#### 模块文件之间调用
```rust
// my_mod.rs
pub mod B {
    use self::super::A;
    pub struct D;
    use crate::mod_call;
    pub fn getC(){
        let c = A::C {};
        let side = mod_call::OutSide {};
    }
}
```
可以看到use crate可以访问到当前crate下的任意mod

