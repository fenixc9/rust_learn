fn test_option1() {
    let mut node = Node { t: None };
    node.t = Some(1)

}

struct Node<T> {
    t: Option<T>
}