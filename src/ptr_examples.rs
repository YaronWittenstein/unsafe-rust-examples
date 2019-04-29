#[test]
fn test_ptr_read() {
    let val: u32 = 10;
    let ptr = &val as *const u32;

    assert_eq!(10, unsafe { std::ptr::read(ptr) });
}

#[test]
fn test_ptr_write() {
    let mut val: u32 = 10;
    let ptr = &mut val as *mut u32;

    unsafe { std::ptr::write(ptr, 20) };

    assert_eq!(20, unsafe { std::ptr::read(ptr) });
}

#[test]
fn test_ptr_replace() {
    let mut val: u32 = 10;

    let ptr = &mut val as *mut u32;

    unsafe { std::ptr::replace(ptr, 20) };

    assert_eq!(20, unsafe { std::ptr::read(ptr) });
}

#[test]
fn test_ptr_swap() {
    let mut val1: u32 = 10;
    let mut val2: u32 = 20;

    let ptr1 = &mut val1 as *mut u32;
    let ptr2 = &mut val2 as *mut u32;

    assert_eq!(10, unsafe { std::ptr::read(ptr1) });
    assert_eq!(20, unsafe { std::ptr::read(ptr2) });

    unsafe { std::ptr::swap(ptr1, ptr2) };

    assert_eq!(20, unsafe { std::ptr::read(ptr1) });
    assert_eq!(10, unsafe { std::ptr::read(ptr2) });
}
