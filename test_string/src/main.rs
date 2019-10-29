use std::str::Chars;

fn main() {}

#[test]
fn test1() {
    let s = String::from("abc");
    assert_eq!(s, "abc")
}

#[test]
fn t2() {
    let s = "abc".to_string();
    assert_eq!(s, "abc")
}

fn showStr(s: &str) {
    assert_eq!(s, "abc")
}

#[test]
fn t3() {
    let s = "abc".to_string();
    showStr(&*s)
}

/// string和byte数组
#[test]
fn t4() {
    let s = "abc".to_string();

    // ascii 字符串转byte
    let x = s.as_bytes();
    for u in x {
        println!("{}", u);
    }
    //97
    //98
    //99

    // ascii 字符串转char
    let chars = s.chars();
    for c in chars {
        println!("{}", c);
    }
    //a
    //b
    //c
}

/// 中文字符串
#[test]
fn t5() {
    let s = "中国上海魔都".to_string();
    // ascii 字符串转byte
    let x = s.as_bytes();
    for u in x {
        println!("{}", u);
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
        println!("{}", c);
    }
    //中
    //国
    //上
    //海
    //魔
    //都
}

/// byte数组转string
#[test]
fn t6() {
    let v = vec![233, 131, 189];
    let s = String::from_utf8(v).unwrap();
    println!("{}", s);
    //都
}

#[test]
fn t7() {
    let s = String::from_utf8(vec![b'h', b'e', b'l', b'l', b'o']).unwrap();
    assert_eq!(s, "hello")
}


// String的split操作
#[test]
fn t8() {
    let s = String::from("hello world");
    let x = s.split_ascii_whitespace().next().unwrap();
    assert_eq!("hello", x);

    let x1: Vec<&str> = "Hello world".split(" ").collect();
    assert_eq!(x1, ["Hello", "world"]);

    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);
}
