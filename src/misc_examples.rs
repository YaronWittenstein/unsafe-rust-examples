#[test]
fn unsafe_cell() {
    use std::cell::UnsafeCell;

    let cell = UnsafeCell::new(10);

    assert_eq!(10, cell.into_inner());
}

#[test]
fn unsafe_trait() {
    unsafe trait DoWork {
        fn do_safe_work(&mut self, x: i32) -> i32;
        unsafe fn do_unsafe_work(&mut self, x: i32) -> i32;
    }

    struct Worker {
        work_done: i32,
    }

    unsafe impl DoWork for Worker {
        fn do_safe_work(&mut self, x: i32) -> i32 {
            self.work_done += x;
            self.work_done
        }

        unsafe fn do_unsafe_work(&mut self, x: i32) -> i32 {
            self.work_done += x;
            self.work_done
        }
    }

    let mut w = Worker { work_done: 0 };

    w.do_safe_work(10);
    assert_eq!(10, w.work_done);

    unsafe {
        w.do_unsafe_work(20);
    }

    assert_eq!(30, w.work_done);
}
