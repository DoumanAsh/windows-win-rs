use windows_win::TimerBuilder;
use windows_win::raw::timer;
use windows_win::sys::{c_void, c_uchar};

use std::ptr;
use std::sync::atomic;

fn sleep(ms: u64) {
    use std::time;
    use std::thread;

    thread::sleep(time::Duration::from_millis(ms));
}

#[test]
fn test_performance_counter() {
    let result = timer::query_performance_frequency().expect("To get counter");
    assert!(result > 0);

    let result = timer::query_performance_counter().expect("To get counter");
    assert!(result > 0);
}

unsafe extern "system" fn callback(_: *mut c_void, _: c_uchar) {
    println!("raw callback");
}

#[test]
fn test_timer_queue() {
    let queue = timer::TimerQueue::new().expect("To crate queue");
    queue.delete(timer::Wait).expect("To delete queue");

    let queue = timer::TimerQueue::default();
    queue.delete(timer::Wait).expect("To delete default queue");

    timer::DEFAULT_TIMER_QUEUE.delete(timer::Wait).expect("To delete default queue");

    let timer = timer::DEFAULT_TIMER_QUEUE.timer(Some(callback), ptr::null_mut(), 0, 900, timer::DEFAULT_TIMER_FLAGS).expect("To crate timer");
    sleep(10);
    timer.reset(1, 0).expect("To reset");
    sleep(10);
    timer.delete(timer::Wait).expect("To delete timer");
}

static RUST_CB_COUNT: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
fn rust_callback() {
    RUST_CB_COUNT.fetch_add(1, atomic::Ordering::Relaxed);
    println!("rust_callback");
}

#[test]
fn test_timer_builder() {
    let timer = TimerBuilder::new().rust_callback(rust_callback).single(1).interval(900).build().expect("To build timer");
    assert_eq!(RUST_CB_COUNT.load(atomic::Ordering::Relaxed), 0);
    sleep(100);
    assert_eq!(RUST_CB_COUNT.load(atomic::Ordering::Relaxed), 1);
    timer.delete(timer::Wait).expect("To delete timer");

    let timer = TimerBuilder::new().rust_callback(rust_callback).single(900).interval(900).build().expect("To build timer");
    sleep(100);
    assert_eq!(RUST_CB_COUNT.load(atomic::Ordering::Relaxed), 1);
    timer.reset(5, 0).expect("To reset");
    sleep(100);
    assert_eq!(RUST_CB_COUNT.load(atomic::Ordering::Relaxed), 2);
    timer.delete(timer::Wait).expect("To delete timer");

    let timer = TimerBuilder::new().rust_callback(rust_callback).single(0).interval(60).build().expect("To build timer");
    sleep(100);
    assert_eq!(RUST_CB_COUNT.load(atomic::Ordering::Relaxed), 4);
    timer.delete(timer::Wait).expect("To delete timer");

    let timer = TimerBuilder::new().single(1).build().expect("To build timer");
    timer.delete(timer::Wait).expect("To delete timer");
}
