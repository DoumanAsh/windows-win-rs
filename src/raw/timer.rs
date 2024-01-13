//! Windows timers API
use crate::sys::{
    QueryPerformanceFrequency,
    QueryPerformanceCounter,
    LARGE_INTEGER,
    CreateTimerQueue,
    CreateTimerQueueTimer,
    DeleteTimerQueueEx,
    DeleteTimerQueueTimer,
    ChangeTimerQueueTimer,
    HANDLE,
    INVALID_HANDLE_VALUE,
    WT_EXECUTEINTIMERTHREAD,
    WT_EXECUTEINPERSISTENTTHREAD,
    WT_EXECUTELONGFUNCTION,
    WT_EXECUTEONLYONCE,
    WT_TRANSFER_IMPERSONATION,
    WAITORTIMERCALLBACK,
    c_int,
    c_ulong,
    c_void,
};
use crate::utils::{self, Result};

use core::{ptr, mem};

///Retrieves the frequency of the performance counter.
///
///The frequency of the performance counter is fixed at system boot and is consistent across all processors.
///Therefore, the frequency need only be queried upon application initialization, and the result can be cached.
pub fn query_performance_frequency() -> Result<i64> {
    let mut counter: LARGE_INTEGER = unsafe { mem::zeroed() };

    unsafe {
        match QueryPerformanceFrequency(&mut counter as *mut _) {
            0 => Err(utils::get_last_error()),
            _ => Ok(counter.QuadPart)
        }
    }
}

///Retrieves the current value of the performance counter, which is a high resolution (<1us) time
///stamp that can be used for time-interval measurements.
pub fn query_performance_counter() -> Result<i64> {
    let mut counter: LARGE_INTEGER = unsafe { mem::zeroed() };

    unsafe {
        match QueryPerformanceCounter(&mut counter as *mut _) {
            0 => Err(utils::get_last_error()),
            _ => Ok(counter.QuadPart)
        }
    }
}

///Describes how to delete timer/queue
pub trait CompleteEvent {
    #[doc(hidden)]
    fn handle() -> HANDLE;
}

///Schedules delete and exit immediately
pub struct NoWait;
impl CompleteEvent for NoWait {
    fn handle() -> HANDLE {
        ptr::null_mut()
    }
}

///Waits for all callback functions to finish
pub struct Wait;
impl CompleteEvent for Wait {
    fn handle() -> HANDLE {
        INVALID_HANDLE_VALUE
    }
}

#[derive(Copy, Clone)]
///Describes timer flags
pub struct TimerFlags {
    inner: c_ulong
}

///Default timer flags to only execute callback on non-IO thread
pub const DEFAULT_TIMER_FLAGS: TimerFlags = TimerFlags {
    inner: 0
};

impl TimerFlags {
    ///Creates new instance of default flags
    pub fn new() -> Self {
        DEFAULT_TIMER_FLAGS
    }

    ///The callback function is invoked by the timer thread itself.
    ///
    ///This flag should be used only for short tasks or it could affect other timer operations.
    pub fn on_timer_thread(mut self) -> Self {
        self.inner |= WT_EXECUTEINTIMERTHREAD;
        self
    }

    ///The callback function is queued to a thread that never terminates.
    ///
    ///It does not guarantee that the same thread is used each time.
    ///This flag should be used only for short tasks or it could affect other timer operations.
    pub fn on_persist(mut self) -> Self {
        self.inner |= WT_EXECUTEINPERSISTENTTHREAD;
        self
    }

    ///The callback function can perform a long wait.
    ///
    ///This flag helps the system to decide if it should create a new thread.
    pub fn long_fn(mut self) -> Self {
        self.inner |= WT_EXECUTELONGFUNCTION;
        self
    }

    ///The timer will be set to the signaled state only once.
    ///
    ///If this flag is set, the Period parameter must not be set.
    pub fn only_once(mut self) -> Self {
        self.inner |= WT_EXECUTEONLYONCE;
        self
    }

    ///Callback functions will use the current access token, whether it is a process or
    ///impersonation token.
    ///
    ///If this flag is not specified, callback functions execute only with the
    ///process token.
    pub fn transfer_impersonation(mut self) -> Self {
        self.inner |= WT_TRANSFER_IMPERSONATION;
        self
    }
}

///Queue for timer
///
///By default `Drop` implementation deletes timer without waiting for
///queue to finish currently executing callbacks.
///If you want to wait then you can use `delete` method.
///
///Alternatively you can use default system queue by accessing `DEFAULT_TIMER_QUEUE`
///In this case it is impossible to delete queue and `delete` always returns `Ok`
pub struct TimerQueue {
    handle: HANDLE
}

impl TimerQueue {
    ///Creates new instance of queue
    pub fn new() -> Result<Self> {
        let handle = unsafe { CreateTimerQueue() };

        match handle.is_null() {
            true => Err(utils::get_last_error()),
            false => Ok(Self { handle })
        }
    }

    #[inline]
    fn inner_delete<T: CompleteEvent>(&self) -> c_int {
        match self.handle.is_null() {
            true => 1,
            false => unsafe { DeleteTimerQueueEx(self.handle, T::handle()) },
        }
    }

    ///Deletes queue and consumes it.
    ///
    ///Note that it invalidates all timers produced by it.
    pub fn delete<T: CompleteEvent>(self, _event: T) -> Result<()> {
        let result = match self.inner_delete::<T>() {
            0 => Err(utils::get_last_error()),
            _ => Ok(())
        };

        mem::forget(self);
        result
    }

    ///Creates new timer on queue.
    ///
    ///## Parameters
    ///
    ///- `cb` - C function to be executed.
    ///- `param` - Pointer to callback parameter.
    ///- `due_time` - The amount of time in milliseconds relative to the current time that must elapse before the timer is signaled for the first time.
    ///- `period` - The period of the timer, in milliseconds. If this parameter is zero, the timer is signaled once. If this parameter is greater than zero, the timer is periodic. A periodic timer automatically reactivates each time the period elapses, until the timer is canceled.
    ///- `flags` - Timer flags
    pub fn timer(&self, cb: WAITORTIMERCALLBACK, param: *mut c_void, due_time: c_ulong, period: c_ulong, flags: TimerFlags) -> Result<QueueTimer> {
        let mut timer: *mut c_void = ptr::null_mut();

        match unsafe { CreateTimerQueueTimer(&mut timer as *mut _, self.handle, cb, param, due_time, period, flags.inner) } {
            0 => Err(utils::get_last_error()),
            _ => Ok(QueueTimer { queue: self.handle, inner: timer })
        }
    }
}

///Raw type of callback function
pub type CallbackType = WAITORTIMERCALLBACK;

///Default Timer queue
pub const DEFAULT_TIMER_QUEUE: TimerQueue = TimerQueue {
    handle: ptr::null_mut()
};

impl Default for TimerQueue {
    fn default() -> Self {
        DEFAULT_TIMER_QUEUE
    }
}

impl Drop for TimerQueue {
    fn drop(&mut self) {
        let _ = self.inner_delete::<NoWait>();
    }
}

unsafe impl Send for TimerQueue {}
unsafe impl Sync for TimerQueue {}

///Timer that schedules callback on thread pool
///
///By default `Drop` implementation deletes queue without waiting for
///callback to be finished.
///If you want to wait then you can use `delete` method.
pub struct QueueTimer {
    queue: HANDLE,
    inner: HANDLE,
}

impl QueueTimer {
    #[inline]
    fn inner_delete<T: CompleteEvent>(&self) -> c_int {
        unsafe { DeleteTimerQueueTimer(self.queue, self.inner, T::handle()) }
    }

    ///Cancels timer without consuming it
    ///
    ///User must ensure that drop is not called by forgetting timer
    pub unsafe fn cancel<T: CompleteEvent>(&self, _event: T) -> Result<()> {
        match self.inner_delete::<T>() {
            0 => Err(utils::get_last_error()),
            _ => Ok(())
        }
    }

    ///Resets timer with new values of due_time and period
    ///
    ///Note: if you call it on a one-shot timer (its period is zero) that has already expired, the timer is not
    ///updated.
    pub fn reset(&self, due_time: c_ulong, period: c_ulong) -> Result<()> {
        match unsafe { ChangeTimerQueueTimer(self.queue, self.inner, due_time, period) } {
            0 => Err(utils::get_last_error()),
            _ => Ok(())
        }
    }

    ///Deletes timer and consumes it.
    pub fn delete<T: CompleteEvent>(self, event: T) -> Result<()> {
        let result = unsafe { self.cancel(event) };

        mem::forget(self);
        result
    }
}

impl Drop for QueueTimer {
    fn drop(&mut self) {
        let _ = self.inner_delete::<NoWait>();
    }
}

unsafe impl Send for QueueTimer {}
unsafe impl Sync for QueueTimer {}
