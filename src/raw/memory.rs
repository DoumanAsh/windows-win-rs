//! Provides functions to interact with memory.

use ::io;
use ::mem;
use ::inner_raw as raw;
use self::raw::winapi::*;

use ::utils;

///Retrieves information about virtual memory of specified process.
///
///Wrapper over `VirtualQueryEx`
///
///In case no information is available i.e. `VirtualQueryEx` returns 0
///function returns None.
pub fn virtual_query_ex(handle: HANDLE, base: *const c_void) -> io::Result<MEMORY_BASIC_INFORMATION> {
    let mut info: MEMORY_BASIC_INFORMATION = unsafe { mem::zeroed() };

    if unsafe { VirtualQueryEx(handle, base, &mut info as *mut _, mem::size_of_val(&info) as SIZE_T) } != 0 {
        Ok(info)
    }
    else {
        Err(utils::get_last_error())
    }
}
