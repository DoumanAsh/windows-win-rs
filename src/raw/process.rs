//! Provides functions to interact with processes.

use core::{ptr, mem};
use core::ffi::c_void;

use crate::sys::*;
use crate::utils::{self, Result};

///Opens process by pid.
///
///# Note:
///See information about access rights:
///https://msdn.microsoft.com/en-us/library/windows/desktop/ms684880%28v=vs.85%29.aspx
///
///# Parameters
///
///* ```pid``` - Pid of the process.
///* ```access_rights``` - Bit mask that specifies desired access rights.
///
///# Return
///
///* ```Ok``` - Handle to opened process.
///* ```Err``` - Error reason.
pub fn open(pid: u32, access_rights: u32) -> Result<HANDLE> {
    let result = unsafe {OpenProcess(access_rights, 0, pid) };

    if result.is_null() {
        return Err(utils::get_last_error());
    }

    Ok(result)
}

///Closes opened process.
///
///# Parameters
///
///* ```process``` - pointer to a opened process.
///
///# Return
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn close(process: HANDLE) -> Result<()> {
    let result = unsafe {CloseHandle(process) };

    if result == 0 {
        return Err(utils::get_last_error());
    }

    Ok(())
}

///Reads process memory.
///
///# Parameters
///
///* ```process``` - Pointer to a opened process.
///* ```base_addr``` - Address from where to start reading.
///* ```storage``` - Storage to hold memory. Its `len` determines amount of bytes to read.
///
///# Return
///
///* ```Ok``` - Vector with data.
///* ```Err``` - Error reason.
pub fn read_memory(process: HANDLE, base_addr: usize, storage: &mut [u8]) -> Result<()> {
    let read_size = storage.len();
    let ret_val = unsafe {ReadProcessMemory(process,
                                            base_addr as *const c_void,
                                            storage.as_mut_ptr() as *mut c_void,
                                            read_size as SIZE_T,
                                            ptr::null_mut())};

    if ret_val == 0 {
        Err(utils::get_last_error())
    }
    else {
        Ok(())
    }
}

///Writes into process memory.
///
///# Parameters
///
///* ```process``` - Pointer to a opened process.
///* ```base_addr``` - Address from where to start writing.
///* ```data``` - Slice with write data.
///
///# Return
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn write_memory(process: HANDLE, base_addr: usize, data: &[u8]) -> Result<()> {
    let ret_val = unsafe {WriteProcessMemory(process,
                                             base_addr as *mut c_void,
                                             data.as_ptr() as *const c_void,
                                             data.len() as SIZE_T,
                                             ptr::null_mut())};
    if ret_val == 0 {
        return Err(utils::get_last_error());
    }

    Ok(())
}

///Gets full path to process's exectuable.
///
///# Note
///
/// The process MUST be opened with either PROCESS_QUERY_INFORMATION or PROCESS_QUERY_LIMITED_INFORMATION flag.
///
///# Parameters
///
///* ```process``` - Pointer to a opened process.
///
///# Return
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn get_exe_path(process: HANDLE) -> Result<String> {
    let mut buf_len = MAX_PATH as u32;
    let mut result: Vec<u16> = vec![0; buf_len as usize];
    let text_ptr = result.as_mut_ptr() as LPWSTR;

    unsafe {
        if QueryFullProcessImageNameW(process, 0, text_ptr, &mut buf_len as *mut u32) == 0 {
            return Err(utils::get_last_error());
        }
    }

    Ok(String::from_utf16_lossy(&result[..buf_len as usize]))
}

#[inline]
///Retrieves process ID of Handle.
pub fn get_id(process: HANDLE) -> c_ulong {
    unsafe { GetProcessId(process) }
}

#[inline]
///Retrieves pseudo-handler of the calling process.
pub fn get_current_handle() -> HANDLE {
    unsafe { GetCurrentProcess() }
}

///Terminates process.
///
///# Parameters
///
///* ```process``` - Pointer to a opened process.
///* ```code``` - Exit code that shall be used by affected process.
///
///# Note:
///
///It prevents process from running any clean-up.
pub fn terminate(process: HANDLE, code: c_uint) -> Result<()> {
    if unsafe { TerminateProcess(process, code) } != 0 {
        Ok(())
    }
    else {
        Err(utils::get_last_error())
    }
}

///Checks whether current process is elevated.
pub fn is_self_elevated() -> bool {
    is_elevated(unsafe { GetCurrentProcess() })
}

///Determines whether the current process is elevated.
///
///# Parameters
///
///* ```process``` - Pointer to a opened process.
pub fn is_elevated(process: HANDLE) -> bool {
    let mut token: HANDLE = ptr::null_mut();

    unsafe {
        if OpenProcessToken(process, TOKEN_QUERY, &mut token as *mut HANDLE) == 0 {
            //Most likely no access rights
            return false;
        }
    }

    let mut evalutation: TOKEN_ELEVATION = unsafe { mem::zeroed() };
    let eval_ptr = &mut evalutation as *mut TOKEN_ELEVATION as *mut c_void;
    let mut len = mem::size_of::<TOKEN_ELEVATION>() as DWORD;

    let result = match unsafe { GetTokenInformation(token, TOKEN_INFORMATION_CLASS::TokenElevation, eval_ptr, len, &mut len as *mut _) } {
        0 => false,
        _ => evalutation.TokenIsElevated != 0,
    };

    unsafe {
        CloseHandle(token);
    }

    result
}
