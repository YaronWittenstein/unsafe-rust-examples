#[test]
fn test_vec_as_ptr() {
    let vec: Vec<u8> = vec![10, 20, 30];

    let ptr = vec.as_ptr();

    let first_ptr: *const u8 = ptr;
    let second_ptr: *const u8 = unsafe { ptr.offset(1) };
    let third_ptr: *const u8 = unsafe { ptr.offset(2) };

    assert_eq!(10, unsafe { *first_ptr });
    assert_eq!(20, unsafe { *second_ptr });
    assert_eq!(30, unsafe { *third_ptr });
}

#[test]
fn test_vec_as_mut_ptr() {
    let mut vec: Vec<u8> = vec![10, 20];

    let first_ptr: *mut u8 = vec.as_mut_ptr();
    let second_ptr: *mut u8;

    unsafe {
        *first_ptr = 30;

        second_ptr = first_ptr.offset(1);
        *second_ptr = 40;
    }

    assert_eq!(30, unsafe { *first_ptr });
    assert_eq!(40, unsafe { *second_ptr });
}
