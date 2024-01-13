//! Provides functions to handle windows messages.

use core::{mem, ptr};

use crate::sys::*;
use crate::utils::{self, Result};

///Retrieves a message from the calling thread's message queue. A blocking call.
///
///# Parameters:
///
///* ```window``` - A handle to the window whose messages are to be retrieved. The window must belong to the current thread.
///* ```range_low``` - Integer value of the lowest message to retrieve.
///* ```range_high``` - Integer value of the highest message to retrieve.
///
///# Return
///
///* ```Ok``` - Successfully retrieved message..
///* ```Err``` - Impossible to retrieve message.
pub fn get(window: Option<HWND>, range_low: Option<UINT>, range_high: Option<UINT>) -> Result<MSG> {
    let mut msg: MSG = unsafe { mem::zeroed() };

    let result = unsafe { GetMessageW(&mut msg as LPMSG,
                                      window.unwrap_or(ptr::null_mut()),
                                      range_low.unwrap_or(0),
                                      range_high.unwrap_or(0)) };

    if result < 0 {
        Err(utils::get_last_error())
    }
    else {
        Ok(msg)
    }
}

///Retrieves a message from the calling thread's message queue.
///
///A non-blocking version of [get()](fn.get.html).
///
///If no message presents in queue then returns None.
///
///# Parameters:
///
///* ```window``` - A handle to the window whose messages are to be retrieved. The window must belong to the current thread.
///* ```range_low``` - Integer value of the lowest message to retrieve.
///* ```range_high``` - Integer value of the highest message to retrieve.
///* ```handle_type``` - Determines how retrieved message is handled. See [details](https://msdn.microsoft.com/en-us/library/windows/desktop/ms644943(v=vs.85).aspx)
///
///# Return
///
///* ```Ok``` - Successfully retrieved message..
///* ```Err``` - Impossible to retrieve message.
pub fn peek(window: Option<HWND>, range_low: Option<UINT>, range_high: Option<UINT>, handle_type: Option<UINT>) -> Result<Option<MSG>> {
    let mut msg: MSG = unsafe { mem::zeroed() };

    let result = unsafe { PeekMessageW(&mut msg as LPMSG,
                                       window.unwrap_or(ptr::null_mut()),
                                       range_low.unwrap_or(0),
                                       range_high.unwrap_or(0),
                                       handle_type.unwrap_or(0)) };

    if result < 0 {
        Err(utils::get_last_error())
    }
    else if result == 0 {
        Ok(None)
    }
    else {
        Ok(Some(msg))
    }
}

#[inline]
///Translates virtual-key messages into character messages.
///
///The character messages are posted to the calling thread's message queue.
///
///# Parameters:
///
///* ```msg``` - Pointer to message retrieved by [get()](fn.get.html) or [peek()](fn.peek.html).
///
///# Return
///
///* ```true``` - Translation happened.
///* ```false``` - Otherwise.
pub fn translate(msg: LPMSG) -> bool {
    unsafe {
        TranslateMessage(msg) != 0
    }
}

#[inline]
///Dispatches a message to a window procedure.
///
///# Parameters:
///
///* ```msg``` - Pointer to message retrieved by [get()](fn.get.html) or [peek](fn.peek.html).
///
///# Return:
///
///Integer value whose meaning depends on dispatched messaged. Can be ignored.
pub fn dispatch(msg: LPMSG) -> LRESULT {
    unsafe {
        DispatchMessageW(msg)
    }
}
