extern crate windows_win;
extern crate winapi;

use self::windows_win::TimerBuilder;
use self::windows_win::raw::timer;
use self::winapi::ctypes::{
    c_void,
    c_uchar,
};

use std::ptr;

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

    let timer = timer::DEFAULT_TIMER_QUEUE.timer(Some(callback), ptr::null_mut(), 1, 0, timer::DEFAULT_TIMER_FLAGS).expect("To crate timer");
    sleep(10);
    timer.delete(timer::Wait).expect("To delete timer");
}

fn rust_callback() {
    println!("rust_callback");
}

#[test]
fn test_timer_builder() {
    let timer = TimerBuilder::new().rust_callback(rust_callback).single(1).build().expect("To build timer");
    sleep(10);
    timer.delete(timer::Wait).expect("To delete timer");

    let timer = TimerBuilder::new().single(1).build().expect("To build timer");
    timer.delete(timer::Wait).expect("To delete timer");
}
