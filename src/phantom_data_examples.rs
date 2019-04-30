#[test]
fn phantom_data() {
    use std::marker::PhantomData;

    struct MyStruct<'a, T: 'a> {
        items: *const T,
        phantom: PhantomData<&'a T>,
    }

    let vec = vec![10, 20, 30];
    let items_ptr: *const u8 = vec.as_ptr();

    let instance = MyStruct {
        items: items_ptr,
        phantom: PhantomData,
    };

    assert_eq!(10, unsafe { std::ptr::read(instance.items) });
    assert_eq!(20, unsafe { std::ptr::read(instance.items.add(1)) });
    assert_eq!(30, unsafe { std::ptr::read(instance.items.add(2)) });
}
