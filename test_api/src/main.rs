use std::fs::{File, Metadata, read_to_string};
use std::io::{Error, Read};
use std::io;

fn main() {
    println!("Hello, world!");
}

#[test]
fn f1() {
    let mut result = File::open("Cargo.toml").unwrap();
    let mut a = String::new();
    let r = result.read_to_string(&mut a).unwrap();
    println!("size {} \n{}", r, a);
}

#[test]
fn f2() {
    let mut result =
        File::open("Cargo.toml")
            .map(|mut f| {
                let mut s = String::new();
                f.read_to_string(&mut s);
                return s;
            })
            .map(|s| {
                println!("{}", s);
            });
}

#[test]
fn f3() {
    let file = File::open("Cargo.toml1").expect("some thing wrong");
}

#[test]
fn f4() {
    let file1 = File::open("Cargo.toml").unwrap();
    let file2 = File::open("src\\main.rs").unwrap();
    let mut chain = file1.chain(file2);
    let mut buffer = String::new();

    let result = chain.read_to_string(&mut buffer);
    println!("{}:", buffer);
}

#[test]
fn f5() {
    let result = process_err("Cargo.toml".to_string());
    println!("{}", result.unwrap());
}

// 对于返回值是Result的函数，可以用 ? 代替
fn process_err(s: String) -> io::Result<String> {
    let mut f = File::open(s)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[test]
fn f6() {
    let result = read_to_string("Cargo.toml");
}