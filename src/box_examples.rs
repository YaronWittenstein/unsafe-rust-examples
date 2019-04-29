#[test]
fn test_box_into_raw() {
    let boxed_val = Box::new(10u32);

    let ptr: *const u32 = Box::into_raw(boxed_val);
    assert_eq!(10u32, unsafe { *ptr });
}

#[test]
fn test_box_from_raw() {
    let boxed = Box::new(10u32);
    let ptr: *mut u32 = Box::into_raw(boxed);

    assert_eq!(Box::new(10), unsafe { Box::from_raw(ptr) });
}

#[test]
fn test_box_leak() {
    let boxed_val = Box::new(10);

    let val_ref: &mut u8 = Box::leak(boxed_val);
    assert_eq!(10, *val_ref);
}
