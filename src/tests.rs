use super::LinkedList;

fn create_list<T: Default>() -> LinkedList<T> {
    LinkedList::new(T::default())
}

#[test]
fn get_item() {
    let mut list = create_list();

    list.add(45);

    assert_eq!(i32::default(), *list.get(0).unwrap());
    assert_eq!(45, *list.get(1).unwrap());
}

#[test]
#[should_panic]
fn get_non_existing() {
    let mut list = create_list::<i32>();

    let data = *list.get(1).unwrap();
}

#[test]
#[should_panic]
fn negative_index() {
    let mut list = create_list::<i32>();

    let data = *list.get(-1).unwrap();
}

#[test]
fn add_element() {
    let mut list = create_list();

    assert_eq!(
        i32::default(),
        *list.get(0).unwrap(),
        "First element is not default value"
    );

    list.add(55);

    assert_eq!(55, *list.get(1).unwrap());
}

#[test]
fn remove_element() {
    let mut list = create_list();

    list.add(100);

    assert_eq!(100, *list.get(1).unwrap());

    list.remove(1);

    assert!(list.get(1).is_err());
}
