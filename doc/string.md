# Rust的字符串
Rust的字符串分两种：str和String
* 所有代码里写的字符串都是str，这个是不可变的
* String是堆上分配UTF8，可变长的。str是String的切片

### String
String是preclude中的，所以不需要任何use导入。
```rust
#[test]
fn test1() {
    let s = String::from("abc");
    assert_eq!(s, "abc")
}
```
##### str转换为String
```rust
#[test]
fn t2(){
    let s = "abc".to_string();
    // 这里的s是一个String，而abc是一个&str
    assert_eq!(s,"abc")
}
```
##### String转换为str
```rust
fn showStr(s: &str) {
    assert_eq!(s,"abc")
}

#[test]
fn t3(){
    let s = "abc".to_string();
    showStr(&*s)
}
```
&*二连

##### String,char和byte
ascii字符串转char和byte
```rust
/// string和byte数组
#[test]
fn t4(){
    let s = "abc".to_string();

    // ascii 字符串转byte
    let x = s.as_bytes();
    for u in x {
        println!("{}",u);
    }
    //97
    //98
    //99

    // ascii 字符串转char
    let chars = s.chars();
    for c in chars {
        println!("{}",c);
    }
    //a
    //b
    //c

}
```
中文字符串转char和byte
```rust
/// 中文字符串
#[test]
fn t5(){
    let s = "中国上海魔都".to_string();

    // ascii 字符串转byte
    let x = s.as_bytes();
    for u in x {
        println!("{}",u);
    }
    //228
    //184
    //173
    //229
    //155
    //189
    //228
    //184
    //138
    //230
    //181
    //183
    //233
    //173
    //148
    //233
    //131
    //189

    // ascii 字符串转char
    let chars = s.chars();
    for c in chars {
        println!("{}",c);
    }
    //中
    //国
    //上
    //海
    //魔
    //都

}
```
byte数组转string
```rust
/// byte数组转string
#[test]
fn t6() {
    let v = vec![233, 131, 189];
    let s = String::from_utf8(v).unwrap();
    println!("{}", s);
    //都
}
```


##### string的split操作
```rust
#[test]
fn t8() {
    let s = String::from("hello world");
    let x = s.split_ascii_whitespace().next().unwrap();
    assert_eq!("hello", x)
}
```
