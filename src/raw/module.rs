//! Provides functions to interact with modules.

use core::ptr;

use crate::sys::*;
use crate::utils::{self, Result};

#[macro_export]
///Converts ident to module address.
///
///Mostly to be used with module's functions.
macro_rules! module_to_addr {
    ($mod:ident) => (&$mod as *const _ as *const u16)
}

///Retrieves module handle by using address inside.
///
///Underhood it uses flags `GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT`
///Due to that produced HMODULE must not be passed to `FreeLibrary`
///
///Use macro `module_to_addr!` to convert local function into module address.
pub fn get_module_handle_from_addr(module_addr: LPCWSTR) -> Result<HMODULE> {
    //GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT
    const FLAGS: DWORD = 0x00000004 | 0x00000002;
    let mut result: HMODULE = ptr::null_mut();

    unsafe {
        if GetModuleHandleExW(FLAGS, module_addr, &mut result as *mut HMODULE) == 0 {
            return Err(utils::get_last_error());
        }
    }

    Ok(result)
}

///Retrieves file name of module
pub fn get_module_name(module: HMODULE) -> Result<String> {
    let buf_len = MAX_PATH as u32;
    let mut result: Vec<u16> = vec![0; buf_len as usize];
    let text_ptr = result.as_mut_ptr() as LPWSTR;

    let buf_len = unsafe { GetModuleFileNameW(module, text_ptr, buf_len) };

    if buf_len == 0 {
        Err(utils::get_last_error())
    }
    else {
        Ok(String::from_utf16_lossy(&result[..buf_len as usize]))
    }
}
