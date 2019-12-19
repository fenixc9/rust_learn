#![feature(proc_macro_hygiene)]

use pm::count_tt1;
use pm::*;
use std::collections::HashMap;

fn main() {}

/// 声明宏
macro_rules! list {
     ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*                          // 多次匹配会多次运行这个代码块.
            temp_vec
        }
    }
}

macro_rules! say_hello {
    () => {
        {
           println!("this is print from macro");
        }
    }
}

macro_rules! count_tt {
    ()      => (0usize);
    ($e:tt) => (1usize);
    ($($s:tt $t:tt)*) => (
        count_tt!($($t)*) << 1 | 0
    );
    ($e:tt $($s:tt $t:tt)*) => (
        count_tt!($($t)*) << 1 | 1
    );
}

#[test]
fn test_macro_list() {
    let vec = list!(1,3,4,4);
    for x in vec {
        println!("{}", x);
    }
}

#[test]
fn test_say_hello() {
    say_hello!()
}

#[test]
fn test_count_tt() {
    println!("{}", count_tt!(1,2,3,4));
    println!("{}", count_tt!(1));
    println!("{}", count_tt!(1,2));
}

#[test]
fn test_hw() {}


#[test]
fn test_count_tt1() {
    println!("{}", count_tt1!(1,1,1,1));
}

trait HelloWorld {
    fn hw(&self) -> ();
}

#[derive(HelloWorld)]
struct Student {
    a: String
}

#[test]
fn test_HelloWorld() {
    let student = Student { a: "aaaaaaaaa".to_string() };
    student.hw();
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {$a * ($b + $c)};
}
#[test]
fn test_add() {
    let i = multiply_add!(4,2,3);
    println!("{}", i);
}

macro_rules! print3 {
    ($e:expr) => {
        println!("{}", $e);
        println!("{}", $e);
        println!("{}", $e);
    };
}

#[test]
fn test_print3() {
    print3!("123123123");
}

macro_rules! match_macro {
    ($a:tt + $b:tt) => {"got an addition"};
    (($i:ident)) => {"got an identifier"};
    ($($other:tt)*) => {"got something else"};
}

#[test]
fn test_match_macro() {
    println!("{}", match_macro!(caravan));
    println!("{}", match_macro!(a+b));
    println!("{}", match_macro!(caravan));
}