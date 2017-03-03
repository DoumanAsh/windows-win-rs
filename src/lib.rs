#![cfg(windows)]
//! Windows WinAPI
//!
//! Some windows hacking library with utilities to find windows and access them.
//!

use std::mem;
use std::os;
use std::io;
use std::ptr;
use std::ffi;

#[path="raw/mod.rs"]
mod inner_raw;
mod utils;

pub mod raw {
    //! Provides direct bindings to WinAPI functions of crate.
    pub use inner_raw::process;
    pub use inner_raw::window;
    pub use inner_raw::message;
}

use os::windows::raw::HANDLE;
use inner_raw::winapi::{
    HWND,
    UINT,
    MSG
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
    pub fn open(pid: u32, access_rights: u32) -> io::Result<Process> {
        match raw::process::open(pid, access_rights) {
            Ok(handle) => Ok(Process {
                pid: pid,
                inner: handle,
            }),
            Err(error) => Err(error),
        }
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
    pub fn exe_path(&self) -> io::Result<String> {
        raw::process::get_exe_path(self.inner)
    }

    #[inline]
    ///Retrieves handle to process's window
    ///
    ///# Note
    ///
    ///It can return ```None``` if process hasn't created window.
    pub fn window(&self) -> io::Result<Option<HWND>> {
        raw::window::get_by_pid(self.pid)
    }

    #[inline]
    ///Reads memory from process.
    ///
    ///# Parameters:
    ///
    ///* ```base_addr``` - Address from where to start reading.
    ///* ```storage``` - Storage to hold memory. Its `len` determines amount of bytes to read.
    pub fn read_memory(&self, base_addr: u32, storage: &mut [u8]) -> io::Result<()> {
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
    pub fn write_memory(&self, base_addr: u32, data: &[u8]) -> io::Result<()> {
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
    pub fn into_inner(&mut self) -> MSG {
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
    type Item = io::Result<Msg>;

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
