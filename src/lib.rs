#![cfg(windows)]
//! Windows WinAPI
//!
//! Some windows hacking library with utilities to find windows and access them.
//!

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::not_unsafe_ptr_arg_deref))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::derivable_impls))]

pub mod sys;

use std::ffi;
use core::{ptr, mem, convert};

#[path="raw/mod.rs"]
mod inner_raw;
pub mod utils;
pub mod ui;

pub use utils::{ErrorCode, Result};

pub mod raw {
    //! Provides direct bindings to WinAPI functions of crate.
    pub use super::inner_raw::process;
    pub use super::inner_raw::window;
    pub use super::inner_raw::message;
    pub use super::inner_raw::file;
    pub use super::inner_raw::memory;
    pub use super::inner_raw::module;
    pub use super::inner_raw::timer;
}

use sys::{
    HANDLE,
    HWND,
    UINT,
    WPARAM,
    LPARAM,
    LRESULT,
    MSG,
    c_uint,
    c_ulong,
    c_void,
    c_uchar,
    SW_SHOW,
    SW_HIDE
};

///Windows process representation
pub struct Process {
    pid: u32,
    inner: HANDLE,
}

impl Process {
    ///Creates handle to a new process by opening it through pid.
    ///
    ///# Note:
    ///See information about access rights:
    ///https://msdn.microsoft.com/en-us/library/windows/desktop/ms684880%28v=vs.85%29.aspx
    ///
    ///# Parameters:
    ///
    ///* ```pid``` - Pid of the process.
    ///* ```access_rights``` - Bit mask that specifies desired access rights.
    ///
    ///# Return:
    ///
    ///* ```Ok``` - Process struct.
    ///* ```Err``` - Error reason.
    pub fn open(pid: u32, access_rights: u32) -> utils::Result<Process> {
        match raw::process::open(pid, access_rights) {
            Ok(handle) => Ok(Process {
                pid: pid,
                inner: handle,
            }),
            Err(error) => Err(error),
        }
    }

    ///Creates instance from existing handle
    pub fn from_raw(handle: HANDLE) -> Self {
        Process {
            pid: raw::process::get_id(handle),
            inner: handle
        }
    }

    #[inline]
    ///Retrieves underlying handle.
    pub fn inner(&self) -> HANDLE {
        self.inner
    }

    #[inline]
    ///Retrieves underlying handle and consumes self.
    ///
    ///Basically you're responsible to close handle now.
    pub fn into_inner(self) -> HANDLE {
        let result = self.inner;
        mem::forget(self);
        result
    }

    #[inline]
    ///Gets full path to process's exectuable.
    ///
    ///# Note
    ///
    /// The process MUST be opened with either PROCESS_QUERY_INFORMATION or PROCESS_QUERY_LIMITED_INFORMATION flag.
    ///
    ///# Return
    ///
    ///* ```Ok``` - Success.
    ///* ```Err``` - Error reason.
    pub fn exe_path(&self) -> Result<String> {
        raw::process::get_exe_path(self.inner)
    }

    #[inline]
    ///Retrieves handle to process's window
    ///
    ///# Note
    ///
    ///It can return ```None``` if process hasn't created window.
    pub fn window(&self) -> Result<Option<HWND>> {
        raw::window::get_by_pid(self.pid)
    }

    #[inline]
    ///Reads memory from process.
    ///
    ///# Parameters:
    ///
    ///* ```base_addr``` - Address from where to start reading.
    ///* ```storage``` - Storage to hold memory. Its `len` determines amount of bytes to read.
    pub fn read_memory(&self, base_addr: usize, storage: &mut [u8]) -> Result<()> {
        raw::process::read_memory(self.inner, base_addr, storage)
    }

    #[inline]
    ///Writes into process memory.
    ///
    ///# Parameters:
    ///
    ///* ```base_addr``` - Address from where to start writing.
    ///* ```data``` - Slice with write data.
    ///
    ///# Return:
    ///
    ///* ```Ok``` - Success.
    ///* ```Err``` - Error reason.
    pub fn write_memory(&self, base_addr: usize, data: &[u8]) -> Result<()> {
        raw::process::write_memory(self.inner, base_addr, data)
    }

    ///Closes process
    ///
    ///# Note:
    ///
    ///There is no need to explicitly close the process.
    ///
    ///It shall be closed automatically when being dropped.
    pub fn close(&mut self) {
        if !self.inner.is_null() {
            raw::process::close(self.inner).expect("Unable to close process");
            self.inner = ptr::null_mut();
        }
    }

    ///Forces termination of process and consumes itself.
    ///
    ///For details see [raw::process::terminate()](raw/process/fn.terminate.html).
    pub fn terminate(self, exit_code: c_uint) -> Result<()> {
        raw::process::terminate(self.inner, exit_code).map(|_| {
            let _ = self.into_inner();
        })
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.close()
    }
}

///Wrapper over Windows messages.
///
///On drop it translates and dispatches message.
///You can do it yourself though.
pub struct Msg {
    inner: MSG
}

impl Msg {
    ///Creates new instance by taking raw `MSG`
    pub fn new(message: MSG) -> Msg {
        Msg {
            inner: message
        }
    }

    #[inline]
    ///Message identifier.
    pub fn id(&self) -> UINT {
        self.inner.message
    }

    #[inline]
    ///Pointer to inner message.
    pub fn as_ptr(&self) -> *const MSG {
        &self.inner as *const MSG
    }

    #[inline]
    ///Mutable pointer to inner message.
    pub fn as_mut_ptr(&mut self) -> *mut MSG {
        &mut self.inner as *mut MSG
    }

    #[inline]
    ///Retrieves raw Windows Message.
    ///
    ///Ownership is not passed so do not manually dispatch it.
    pub fn inner(&self) -> MSG {
        self.inner
    }

    #[inline]
    ///Retrieves raw Windows Message and transfers ownership.
    ///
    ///After that user is responsible to dispatch message.
    pub fn into_inner(self) -> MSG {
        let result = self.inner;
        mem::forget(self);
        result
    }

    #[inline]
    ///Drops and Dispatches underlying Windows Message.
    ///You cannot use it after that.
    pub fn dispatch(self) {
        drop(self);
    }
}

impl Drop for Msg {
    fn drop(&mut self) {
        raw::message::translate(self.as_mut_ptr());
        raw::message::dispatch(self.as_mut_ptr());
    }
}

///Iterator over Windows messages
///
///Under hood it uses [get()](raw/message/fn.get.html).
///
///Similarly to this function you can configure:
///
///* window - For which window to received messages.
///* range - Range of message identifiers to receive.
pub struct Messages {
    window: Option<HWND>,
    range: (Option<UINT>, Option<UINT>),
    is_block: bool
}

impl Messages {
    ///Initializes new iterator with default no filtering.
    pub fn new() -> Messages {
        Messages {
            window: None,
            range: (None, None),
            is_block: true
        }
    }

    ///Sets window for which to receive messages.
    pub fn window(&mut self, window: Option<HWND>) -> &mut Messages {
        self.window = window;
        self
    }

    ///Sets low range of message identifiers.
    pub fn low(&mut self, low: Option<UINT>) -> &mut Messages {
        self.range.0 = low;
        self
    }

    ///Sets high range of message identifiers.
    pub fn high(&mut self, high: Option<UINT>) -> &mut Messages {
        self.range.1 = high;
        self
    }

    ///Sets blocking mode.
    pub fn blocking(&mut self) -> &mut Messages {
        self.is_block = true;
        self
    }

    ///Sets non blocking mode.
    ///
    ///You can provide how to handle retrieved messages as in [peek()](raw/message/fn.peek.html).
    ///It sets `PM_REMOVE` to remove message, but not that it is not always guaranteed.
    ///See docs on `PeekMessage`
    pub fn non_blocking(&mut self) -> &mut Messages {
        self.is_block = false;
        self
    }
}

impl Iterator for Messages {
    type Item = Result<Msg>;

    ///Retrieves next message in queue.
    ///
    ///Blocking call.
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_block {
            Some(raw::message::get(self.window, self.range.0, self.range.1).map(|msg| Msg::new(msg)))
        }
        else {
            match raw::message::peek(self.window, self.range.0, self.range.1, Some(0x0001)) {
                Ok(Some(msg)) => Some(Ok(Msg::new(msg))),
                Ok(None) => None,
                Err(error) => Some(Err(error))
            }
        }
    }
}

///Convenient wrapper over Window.
///
///Note that while you can use it with any window.
///It makes no sense in taking ownership of not created by you windows.
///
///This struct destroys window on drop and it is bad idea to do it for not your own window.
///If lucky, it fails but still not great idea.
pub struct Window {
    inner: HWND
}

impl Window {
    #[inline]
    ///Creates new instance by taking ownership over provided window.
    pub fn from_hwnd(window: HWND) -> Self {
        Window { inner: window }
    }

    #[inline]
    ///Creates window from instance of window builder.
    pub fn from_builder(builder: &mut raw::window::Builder) -> Result<Self> {
        builder.create().map(|win| Window::from_hwnd(win))
    }

    #[inline]
    ///Returns underlying window.
    ///
    ///Ownership is not passed.
    pub fn inner(&self) -> HWND {
        self.inner
    }

    #[inline]
    ///Transfers ownership of underlying window.
    pub fn into_inner(self) -> HWND {
        let result = self.inner;
        mem::forget(self);
        result
    }

    #[inline]
    ///Shows window.
    ///
    ///Returns true if previously it wasn't visible
    pub fn show(&self) -> bool {
        !raw::window::show(self.inner, SW_SHOW)
    }

    #[inline]
    ///Hide window.
    ///
    ///Returns true if previously it was visible
    pub fn hide(&self) -> bool {
        raw::window::show(self.inner, SW_HIDE)
    }

    #[inline]
    ///Returns whether window is visible.
    pub fn is_visible(&self) -> bool {
        raw::window::is_visible(self.inner)
    }

    #[inline]
    ///Retrieves window's class.
    pub fn class(&self) -> Result<String> {
        raw::window::get_class(self.inner)
    }

    #[inline]
    ///Retrieves window's title.
    pub fn title(&self) -> Result<String> {
        raw::window::get_text(self.inner)
    }

    #[inline]
    ///Retrieves tuple of thread and process ids.
    pub fn thread_pid(&self) -> (u32, u32) {
        raw::window::get_thread_process_id(self.inner)
    }

    #[inline]
    ///Sends message to underlying window.
    ///
    ///For more information refer to [send_message()](raw/window/fn.send_message.html)
    pub fn send_message(&self, msg_type: UINT, w_param: WPARAM, l_param: LPARAM, timeout: Option<UINT>) -> Result<LRESULT> {
        raw::window::send_message(self.inner, msg_type, w_param, l_param, timeout)
    }

    #[inline]
    ///Sends `BM_CLICK` message to underlying window.
    ///
    ///For mores information refer to [send_push_button()](raw/window/fn.send_push_button.html)
    pub fn send_push_button(&self, timeout: Option<UINT>) -> Result<LRESULT> {
        raw::window::send_push_button(self.inner, timeout)
    }

    #[inline]
    ///Sends `WM_SETTEXT` message to underlying window with new text.
    ///
    ///For more information refer to [send_set_text()](raw/window/fn.send_set_text.html)
    pub fn send_set_text<T: AsRef<ffi::OsStr>>(&self, text: T) -> bool {
        raw::window::send_set_text(self.inner, text)
    }

    #[inline]
    ///Sends `WM_GETTEXT` message to underlying window and returns, if possible, corresponding text.
    ///
    ///For more information refer to [send_get_text()](raw/window/fn.send_get_text.html)
    pub fn send_get_text(&self) -> Option<String> {
        raw::window::send_get_text(self.inner)
    }

    #[inline]
    ///Sends `WM_SYSCOMMAND` message to underlying window and returns, if possible, corresponding text.
    ///
    ///For more information refer to [send_sys_command()](raw/window/fn.send_sys_command.html)
    pub fn send_sys_command(&self, cmd_type: WPARAM, l_param: LPARAM) -> bool {
        raw::window::send_sys_command(self.inner, cmd_type, l_param)
    }

    #[inline]
    ///Destroys underlying window and drops self.
    pub fn destroy(self) {
        drop(self);
    }
}

impl convert::From<HWND> for Window {
    fn from(window: HWND) -> Window {
        Window { inner: window }
    }
}

impl convert::Into<HWND> for Window {
    fn into(self) -> HWND {
        self.into_inner()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        raw::window::destroy(self.inner);
    }
}

enum TimerCallbackType {
    None,
    Raw(raw::timer::CallbackType, *mut c_void),
}

enum TimeoutType {
    None,
    Single(c_ulong),
    Interval(c_ulong),
    Both(c_ulong, c_ulong)
}

impl TimeoutType {
    fn into_raw(self) -> (c_ulong, c_ulong) {
        match self {
            TimeoutType::None => (0, 0),
            TimeoutType::Single(delay) => (delay, 0),
            TimeoutType::Interval(interval) => (0, interval),
            TimeoutType::Both(delay, interval) => (delay, interval),
        }
    }
}

unsafe extern "system" fn timer_rust_callback(param: *mut c_void, _: c_uchar) {
    if !param.is_null() {
        let cb: fn() -> () = mem::transmute(param);
        cb();
    }
}

///WinAPI timer builder
///
///The same timer can act as one-shot timer and/or interval timer.
///
///## Configuration
///
///When `single` method is called timer is configured as one-shot.
///
///When `interval` method is called timer is configured as interval.
///
///When both of the above  are called timer is configured as one-shot, after which it starts
///to run in interval.
///
///By default timer starts as one-shot with timeout 0.
pub struct TimerBuilder<'a> {
    queue: Option<&'a raw::timer::TimerQueue>,
    callback: TimerCallbackType,
    timeout: TimeoutType,
    flags: raw::timer::TimerFlags
}

impl<'a> TimerBuilder<'a> {
    ///Creates new instance
    pub fn new() -> Self {
        Self {
            queue: None,
            callback: TimerCallbackType::None,
            timeout: TimeoutType::None,
            flags: raw::timer::DEFAULT_TIMER_FLAGS
        }
    }

    ///Sets raw C function as callback
    pub fn raw_callback(mut self, cb: raw::timer::CallbackType, param: Option<*mut c_void>) -> Self {
        self.callback = TimerCallbackType::Raw(cb, param.unwrap_or(ptr::null_mut()));
        self
    }

    ///Sets Rust function pointer as callback
    pub fn rust_callback(mut self, cb: fn() -> ()) -> Self {
        self.callback = TimerCallbackType::Raw(Some(timer_rust_callback), cb as _ );
        self
    }

    ///Sets timer queue.
    ///
    ///If not set, default shall be used.
    pub fn queue(mut self, queue: &'a raw::timer::TimerQueue) -> Self {
        self.queue = Some(queue);
        self
    }

    ///Makes timer to fire single time after delay in milliseconds.
    pub fn single(mut self, delay: c_ulong) -> Self {
        self.timeout = match self.timeout {
            TimeoutType::Interval(interval) => TimeoutType::Both(delay, interval),
            _ => TimeoutType::Single(delay)
        };
        self
    }

    ///Makes timer to fire with interval in milliseconds.
    pub fn interval(mut self, interval: c_ulong) -> Self {
        self.timeout = match self.timeout {
            TimeoutType::Single(delay) => TimeoutType::Both(delay, interval),
            _ => TimeoutType::Interval(interval)
        };
        self
    }

    ///Specifies timer flags.
    ///
    ///Default is `raw::timer::DEFAULT_TIMER_FLAGS`.
    pub fn flags(mut self, flags: raw::timer::TimerFlags) -> Self {
        self.flags = flags;
        self
    }

    ///Creates timer.
    pub fn build(self) -> Result<raw::timer::QueueTimer> {
        static DEFAULT: raw::timer::TimerQueue = raw::timer::DEFAULT_TIMER_QUEUE;

        let queue = self.queue.unwrap_or(&DEFAULT);
        let (delay, period) = self.timeout.into_raw();
        let (cb, param) = match self.callback {
            TimerCallbackType::None => (None, ptr::null_mut()),
            TimerCallbackType::Raw(cb, param) => (cb, param),
        };

        queue.timer(cb, param, delay, period, self.flags)
    }
}
