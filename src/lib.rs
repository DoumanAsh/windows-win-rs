//! Windows WinAPI
//!
//! Some windows hacking library with utilities to find windows and access them.
//!

extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate windows_error;

use windows_error::WindowsError;

//WinAPI types
use winapi::windef::{
    HWND
};
use winapi::minwindef::LPARAM;
use winapi::winnt::HANDLE;

//WinAPI constants
use winapi::winuser::{
    SMTO_BLOCK,
    WM_SYSCOMMAND,
    WM_GETTEXT,
    WM_GETTEXTLENGTH,
    WM_SETTEXT
};

//WinAPI functions
use user32::{
    FindWindowW,
    FindWindowExW,
    IsWindowVisible,
    GetWindowTextW,
    SendMessageW,
    SendMessageTimeoutW,
    RealGetWindowClassW,
    EnumChildWindows,
    EnumWindows,
    GetWindowThreadProcessId
};

use kernel32::{
    OpenProcess,
    CloseHandle,
    ReadProcessMemory,
    WriteProcessMemory
};

///Determines if window is visible.
///
///#Note:
/// The visibility state of a window is indicated by the **WS_VISIBLE** style bit.
///
/// When **WS_VISIBLE** is set, the window is displayed and subsequent drawing into it is displayed as long as the window has the **WS_VISIBLE** style.
///
///# Parameters:
///
///* ```window``` - A handle to the window to be tested.
///
///# Return:
///
///* ```true``` - If window is visible.
///* ```false``` - Otherwise.
pub fn is_window_visible(window: HWND) -> bool {
    return unsafe {IsWindowVisible(window) != 0};
}

///Retrieves window's class name.
///
///# Parameters:
///
///* ```window``` - A handle to the window to be tested.
///
///# Return:
///
///* ```Ok``` - Contains name of class.
///* ```Err``` - Error reason.
pub fn get_window_class(window: HWND) -> Result<String, WindowsError> {
    const BUF_SIZE: usize = 512;
    let mut buff: [u16; BUF_SIZE] = [0; BUF_SIZE];

    let writ_chars = unsafe { RealGetWindowClassW(window,
                                                  buff.as_mut_ptr(),
                                                  BUF_SIZE as u32) };

    if writ_chars == 0 {
        return Err(WindowsError::from_last_err());
    }

    Ok(String::from_utf16_lossy(&buff[0..writ_chars as usize]))
}

///Retrieves window's title.
///
///# Parameters:
///
///* ```window``` - A handle to the window to be tested.
///
///# Return:
///
///* ```Ok``` - Contains name of class.
///* ```Err``` - Error reason.
pub fn get_window_text(window: HWND) -> Result<String, WindowsError> {
    const BUF_SIZE: usize = 512;
    let mut buff: [u16; BUF_SIZE] = [0; BUF_SIZE];

    let writ_chars = unsafe { GetWindowTextW(window,
                                             buff.as_mut_ptr(),
                                             BUF_SIZE as i32) };

    if writ_chars == 0 {
        return Err(WindowsError::from_last_err());
    }

    Ok(String::from_utf16_lossy(&buff[0..writ_chars as usize]))
}

unsafe extern "system" fn window_filter_by_class(window: HWND, param: LPARAM) -> i32 {
    let param = &mut *(param as *mut (&str, Vec<HWND>));

    let class_name = param.0;
    let mut windows_vec = &mut param.1;

    if let Ok(window_class) = get_window_class(window) {
        if window_class == class_name {
            windows_vec.push(window);
        }
    }

    1
}

///Retrieves list of handles to specific window class
///
///# Parameters:
///
///* ```class_name``` - A name of class for which handle to be looked up.
///* ```parent``` - Handle of parent window to look up through its children only. Optional.
///
///# Return:
///
///* ```Ok``` - Vector of handles.
///* ```Err``` - Error reason.
pub fn get_windows_by_class(class_name: &str, parent: Option<HWND>) -> Result<Vec<HWND>, WindowsError> {
    let found_windows: Vec<HWND> = vec![];
    let mut param = (class_name, found_windows);
    let lparam = &mut param as *mut _ as LPARAM;

    let result: i32;

    if let Some(parent_window) = parent {
        result = unsafe { EnumChildWindows(parent_window, Some(window_filter_by_class), lparam) };
    }
    else {
        result = unsafe { EnumWindows(Some(window_filter_by_class), lparam) };
    }

    if result == 0 {
        return Err(WindowsError::from_last_err());
    }

    Ok(param.1)
}

unsafe extern "system" fn window_filter_by_name(window: HWND, param: LPARAM) -> i32 {
    let param = &mut *(param as *mut (&str, Vec<HWND>));

    let title_name = param.0;
    let mut windows_vec = &mut param.1;

    if let Ok(window_class) = get_window_text(window) {
        if window_class == title_name {
            windows_vec.push(window);
        }
    }

    1
}

///Retrieves list of handles to windows with specific title's text.
///
///# Parameters:
///
///* ```name``` - Window's title text.
///* ```parent``` - Handle of parent window to look up through its children only. Optional.
///
///# Return:
///
///* ```Ok``` - Vector of handles.
///* ```Err``` - Error reason.
pub fn get_windows_by_title(name: &str, parent: Option<HWND>) -> Result<Vec<HWND>, WindowsError> {
    let found_windows: Vec<HWND> = vec![];
    let mut param = (name, found_windows);
    let lparam = &mut param as *mut _ as LPARAM;

    let result: i32;

    if let Some(parent_window) = parent {
        result = unsafe { EnumChildWindows(parent_window, Some(window_filter_by_name), lparam) };
    }
    else {
        result = unsafe { EnumWindows(Some(window_filter_by_name), lparam) };
    }

    if result == 0 {
        return Err(WindowsError::from_last_err());
    }

    Ok(param.1)
}

///Retrieves the identifier of the thread and process that created the specified window.
///
///# Parameters:
///
///* ```window``` - Handle to a window.
///
///# Return(tuple):
///
///1. Process pid
///2. Thread id.
pub fn get_windows_thread_process_id(window: HWND) -> (u32, u32) {
    let mut process_pid: u32 = 0;
    let thread_pid = unsafe {GetWindowThreadProcessId(window, &mut process_pid)};

    (process_pid, thread_pid)
}

///Opens process by pid.
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
///* ```Ok``` - Handle to opened process.
///* ```Err``` - Error reason.
pub fn open_process(pid: u32, access_rights: u32) -> Result<HANDLE, WindowsError> {
    let result = unsafe {OpenProcess(access_rights, 0, pid) };

    if result.is_null() {
        return Err(WindowsError::from_last_err());
    }

    Ok(result)
}

///Closes opened process.
///
///# Parameters:
///
///* ```process``` - pointer to a opened process.
///
///# Return:
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn close_process(process: HANDLE) -> Result<(), WindowsError> {
    let result = unsafe {CloseHandle(process) };

    if result == 0 {
        return Err(WindowsError::from_last_err());
    }

    Ok(())
}

///Reads process memory.
///
///# Parameters:
///
///* ```process``` - Pointer to a opened process.
///* ```base_addr``` - Address from where to start reading.
///* ```read_size``` - Length of data to read.
///
///# Return:
///
///* ```Ok``` - Vector with data.
///* ```Err``` - Error reason.
pub fn read_process_memory(process: HANDLE, base_addr: u32, read_size: usize) -> Result<Vec<u8>, WindowsError> {
    let mut result = vec![0 as u8; read_size];
    let ret_val = unsafe {ReadProcessMemory(process,
                                            base_addr as *const winapi::c_void,
                                            result.as_mut_ptr() as *mut winapi::c_void,
                                            read_size as u64,
                                            std::ptr::null_mut())};

    if ret_val == 0 {
        return Err(WindowsError::from_last_err());
    }

    Ok(result)
}

///Writes into process memory.
///
///# Parameters:
///
///* ```process``` - Pointer to a opened process.
///* ```base_addr``` - Address from where to start writing.
///* ```data``` - Slice with write data.
///
///# Return:
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn write_process_memory(process: HANDLE, base_addr: u32, data: &[u8]) -> Result<(), WindowsError> {
    let ret_val = unsafe {WriteProcessMemory(process,
                                             base_addr as *mut winapi::c_void,
                                             data.as_ptr() as *const winapi::c_void,
                                             data.len() as u64,
                                             std::ptr::null_mut())};
    if ret_val == 0 {
        return Err(WindowsError::from_last_err());
    }

    Ok(())
}
