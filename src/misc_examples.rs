#[test]
fn test_unsafe_cell() {
    use std::cell::UnsafeCell;

    let cell = UnsafeCell::new(10);

    assert_eq!(10, cell.into_inner());
}
