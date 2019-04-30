#[test]
fn ptr_read() {
    let val: u32 = 10;
    let ptr = &val as *const u32;

    assert_eq!(10, unsafe { std::ptr::read(ptr) });
}

#[test]
fn ptr_write() {
    let mut val: u32 = 10;
    let ptr = &mut val as *mut u32;

    unsafe { std::ptr::write(ptr, 20) };

    assert_eq!(20, unsafe { std::ptr::read(ptr) });
}

#[test]
fn ptr_replace() {
    let mut val: u32 = 10;

    let ptr = &mut val as *mut u32;

    unsafe { std::ptr::replace(ptr, 20) };

    assert_eq!(20, unsafe { std::ptr::read(ptr) });
}

#[test]
fn ptr_swap() {
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

#[test]
fn ptr_to_mut_ptr() {
    let val: u32 = 10;

    let ptr = &val as *const u32;

    let mut_ptr = ptr as *mut u32;

    assert_eq!(10, unsafe { *mut_ptr });

    unsafe { std::ptr::write(mut_ptr, 20) };

    assert_eq!(20, unsafe { *mut_ptr });
}

#[test]
fn c_int_ptr() {
    use std::os::raw::c_int;

    let val: u32 = 10;

    let int_ptr: *const c_int = &val as *const u32 as *const c_int;

    assert_eq!(10, unsafe { *int_ptr });
}
