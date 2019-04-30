#[test]
fn mem_transmute() {
    fn inc_fn(x: u32) -> u32 {
        x + 1
    };

    let fn_ptr = inc_fn as *const u32;

    let func: fn(i32) -> i32 = unsafe { std::mem::transmute(fn_ptr) };

    assert_eq!(11, func(10));
}

#[test]
fn mem_maybe_uninitialized() {
    let mut s: String = unsafe { std::mem::uninitialized() };

    unsafe {
        std::ptr::write(&mut s, "ABCD".to_string());
    }

    assert_eq!("ABCD", s.as_str());

    std::mem::forget(s);
}
