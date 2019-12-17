use std::collections::HashMap;

fn main() {}

/// 简单的场景，注意get返回的是一个引用
#[test]
fn t1() {
    let mut map = HashMap::new();

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    assert_eq!(*map.get("c").unwrap(), 3);
}

/// 简单的场景，注意get返回的是一个引用
#[test]
fn t2() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    let a = 3;
    assert_eq!(*map.get(&a).unwrap(), 3);
}

/// insert传入的是一个引用，生命周期结束后就回收
#[test]
fn t3() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    {
        let a = 1;
        map.insert(a, 1);
    }
    map.insert(2, 2);
    map.insert(3, 3);
    let a = 1;
    assert_eq!(*map.get(&a).unwrap(), 3);
}

/// insert传入的是一个引用，生命周期结束后就回收
#[test]
fn t4() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    assert_eq!(map.get(&1).unwrap(), &1)
}

/// size和cap
#[test]
fn t5() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    map.insert(4, 4);
    map.insert(5, 5);
    map.insert(6, 6);

    let cap = map.capacity();
    let len = map.len();
    assert_eq!(cap, 7);
    assert_eq!(len, 6);
}

/// size和cap
#[test]
fn t6() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    let cap = map.capacity();
    let len = map.len();
    assert_eq!(cap, 3);
    assert_eq!(cap, 3);
}

