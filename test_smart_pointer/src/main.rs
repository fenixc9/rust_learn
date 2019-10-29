use std::rc::Rc;

#[test]
fn t1() {
    let rc = Rc::new(32);
    assert_eq!(1, Rc::strong_count(&rc));
    let rc1 = rc.clone();
    assert_eq!(2, Rc::strong_count(&rc));
}

#[test]
fn t2() {
    let rc = Rc::new("this is a str");
    assert_eq!(rc.len(), 13);
}

#[test]
fn t3() {
    let rc = Rc::new("this is a str");

}