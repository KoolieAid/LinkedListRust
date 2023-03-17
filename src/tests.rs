use super::*;

#[test]
fn construct() {
    let list = LinkedList::new(3);

    dbg!(&list);
    assert!(true);
}

#[test]
fn get_first_element() {
    let list = LinkedList::new(42);
    assert_eq!(list[0], 42);
}