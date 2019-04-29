#[test]
fn test_repr_rust_struct() {
    use std::mem::size_of;

    struct MyStructRust {
        _a: usize,
        _b: String,
    }

    assert_eq!(
        size_of::<usize>() + size_of::<String>(),
        size_of::<MyStructRust>()
    );
}
#[test]
fn test_repr_c_struct() {
    use std::mem::size_of;

    #[repr(C)]
    struct MyStructC {
        _a: usize,
        _b: *const u8,
    }

    assert_eq!(
        size_of::<usize>() + size_of::<*const u8>(),
        std::mem::size_of::<MyStructC>()
    );
}
