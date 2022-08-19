#[test]
fn compare_and_swap() {
    use std::sync::atomic::{AtomicI16, Ordering};

    let some_var = AtomicI16::new(5);

    assert_eq!(some_var.compare_and_swap(5, 10, Ordering::Relaxed), 5);
    assert_eq!(some_var.load(Ordering::Relaxed), 10);

    assert_eq!(some_var.compare_and_swap(6, 12, Ordering::Relaxed), 10);
    assert_eq!(some_var.load(Ordering::Relaxed), 10);
}


#[test]
fn compare_exchange() {
    use std::sync::atomic::{AtomicI16, Ordering};

    let some_var = AtomicI16::new(5);

    assert_eq!(some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed), Ok(5));
    assert_eq!(some_var.load(Ordering::Relaxed), 10);

    assert_eq!(some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire), Err(10));
    assert_eq!(some_var.load(Ordering::Relaxed), 10);
}