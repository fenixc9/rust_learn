fn main() {}

#[test]
fn t1() {
    let x = "1";
    let y = "2";
    let longest1 = longest(x, y);
    println!("longest:{}", longest1);
    assert_eq!(longest1, "a")
}

/// 生命周期的检查是一个静态检测
/// 这样的代码编译器就会报错
///```
/// fn longest1<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
///    if x.len() > y.len() {
///        x
///    } else {
///        y
///    }
///}
///
///```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn t2() {
    let longest1 = longest1("1", "2");
    println!("longest:{}", longest1);
}


