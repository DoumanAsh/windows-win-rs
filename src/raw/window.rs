//! Provides functions to interact with windows.

use std::ffi;
use std::os::windows::ffi::OsStrExt;
use core::ptr;

use crate::sys::SetLastErrorEx;

use crate::sys::*;
use crate::utils::{self, Result};

///Determines if window is visible.
///
///#Note:
/// The visibility state of a window is indicated by the **WS_VISIBLE** style bit.
///
/// When **WS_VISIBLE** is set, the window is displayed and subsequent drawing into it is displayed as long as the window has the **WS_VISIBLE** style.
///
///# Parameters
///
///* ```window``` - A handle to the window to be tested.
///
///# Returns
///
///* ```true``` - If window is visible.
///* ```false``` - Otherwise.
pub fn is_visible(window: HWND) -> bool {
    return unsafe {IsWindowVisible(window) != 0};
}

///Retrieves window's class name.
///
///# Parameters
///
///* ```window``` - A handle to the window.
///
///# Returns
///
///* ```Ok``` - Contains name of class.
///* ```Err``` - Error reason.
pub fn get_class(window: HWND) -> Result<String> {
    const BUF_SIZE: usize = 512;
    let mut buff: [u16; BUF_SIZE] = [0; BUF_SIZE];

    let writ_chars = unsafe { RealGetWindowClassW(window,
                                                  buff.as_mut_ptr(),
                                                  BUF_SIZE as u32) };

    if writ_chars == 0 {
        return Err(utils::get_last_error());
    }

    Ok(String::from_utf16_lossy(&buff[0..writ_chars as usize]))
}

///Retrieves window's title.
///
///# Parameters
///
///* ```window``` - A handle to the window to be tested.
///
///# Return
///
///* ```Ok``` - Contains name of class.
///* ```Err``` - Error reason.
pub fn get_text(window: HWND) -> Result<String> {
    const BUF_SIZE: usize = 512;
    let mut buff: [u16; BUF_SIZE] = [0; BUF_SIZE];

    let writ_chars = unsafe { GetWindowTextW(window,
                                             buff.as_mut_ptr(),
                                             BUF_SIZE as i32) };

    if writ_chars == 0 {
        return Err(utils::get_last_error());
    }

    Ok(String::from_utf16_lossy(&buff[0..writ_chars as usize]))
}

unsafe extern "system" fn callback_enum_windows<T: FnMut(HWND)>(window: HWND, param: LPARAM) -> i32 {
    let func = &mut *(param as *mut T);

    func(window);

    1
}

unsafe extern "system" fn callback_enum_windows_until<T: FnMut(HWND) -> i32>(window: HWND, param: LPARAM) -> i32 {
    let func = &mut *(param as *mut T);

    func(window)
}

///Enumerates over windows handles and calls callback on each
///
///# Parameters
///
///* ```parent``` - Handle of parent window to look up through its children only. Optional.
///* ```cmp_func``` - Callback that will be called on each window.
///
///# Return
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn enum_by<T: FnMut(HWND)>(parent: Option<HWND>, mut cmp_func: T) -> Result<()> {
    let lparam = &mut cmp_func as *mut _ as LPARAM;

    let result: i32;

    if let Some(parent_window) = parent {
        result = unsafe { EnumChildWindows(parent_window, Some(callback_enum_windows::<T>), lparam) };
    }
    else {
        result = unsafe { EnumWindows(Some(callback_enum_windows::<T>), lparam) };
    }

    if result == 0 {
        return Err(utils::get_last_error());
    }

    Ok(())
}

///Enumerates over windows handles and calls callback on each
///
///# Note
/// Enumeration continues until callback return non-zero value.
///
///# WinAPI error
///
///Due to `enum_by_until` allowing to interrupt enumeration, having error set
///cause inability detect accurately whether enumeration failed or has been interrupted.
///Hence this function always set last error to `0`.
///
///# Parameters
///
///* ```parent``` - Handle of parent window to look up through its children only. Optional.
///* ```cmp_func``` - Callback that will be called on each window.
///
///# Return
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn enum_by_until<T: FnMut(HWND) -> i32>(parent: Option<HWND>, mut cmp_func: T) -> Result<()> {
    let lparam = &mut cmp_func as *mut _ as LPARAM;

    let result: i32;

    //Necessary if we want to guarantee that we can correctly detect interrupt of enumeration.
    unsafe { SetLastErrorEx(0, 0) };
    if let Some(parent_window) = parent {
        result = unsafe { EnumChildWindows(parent_window, Some(callback_enum_windows_until::<T>), lparam) };
    }
    else {
        result = unsafe { EnumWindows(Some(callback_enum_windows_until::<T>), lparam) };
    }

    //If cmp_func returns 0 then EnumWindows too.
    //But it is not an error case.
    if result == 0 {
        let error = utils::get_last_error();

        if error.raw_code() != 0 {
            return Err(utils::get_last_error());
        }
    }

    Ok(())
}

///Retrieves handle to a window by pid using `enum_by_until`.
///
///# Parameters
///
///* ```pid``` - Pid of the process
///
///# Return
///
///* ```Ok``` - Success.
///* ```Err``` - Error reason.
pub fn get_by_pid(pid: u32) -> Result<Option<HWND>> {
    let mut found_window: Option<HWND> = None;

    let res = enum_by_until(None,
                            |handle: HWND| {
                                let (process_pid, _) = get_thread_process_id(handle);
                                if process_pid == pid {
                                    found_window = Some(handle);
                                    return 0;
                                }
                                1
                            });

    if let Err(error) = res {
        Err(error)
    }
    else {
        Ok(found_window)
    }

}

///Retrieves list of handles to specific window class
///
///# Parameters
///
///* ```class_name``` - a name of class for which handle to be looked up.
///* ```parent``` - handle of parent window to look up through its children only. optional.
///
///# Return
///
///* ```ok``` - vector of handles.
///* ```err``` - error reason.
pub fn get_by_class(class_name: &str, parent: Option<HWND>) -> Result<Vec<HWND>> {
    let mut found_windows: Vec<HWND> = vec![];

    let res = enum_by(parent,
                      |handle: HWND| {
                          if let Ok(window_class) = get_class(handle) {
                              if window_class == class_name {
                                  found_windows.push(handle);
                              }
                          }
                      });

    if let Err(error) = res {
        Err(error)
    }
    else {
        Ok(found_windows)
    }
}

///Retrieves list of handles to windows with specific title's text.
///
///# Parameters
///
///* ```name``` - Window's title text.
///* ```parent``` - Handle of parent window to look up through its children only. Optional.
///
///# Return
///
///* ```Ok``` - Vector of handles.
///* ```Err``` - Error reason.
pub fn get_by_title(name: &str, parent: Option<HWND>) -> Result<Vec<HWND>> {
    let mut found_windows: Vec<HWND> = vec![];

    let res = enum_by(parent,
                      |handle: HWND| {
                          if let Ok(window_title) = get_text(handle) {
                              if window_title == name {
                                  found_windows.push(handle);
                              }
                          }
                      });

    if let Err(error) = res {
        Err(error)
    }
    else {
        Ok(found_windows)
    }
}

///Retrieves the identifier of the thread and process that created the specified window.
///
///# Parameters
///
///* ```window``` - Handle to a window.
///
///# Return(tuple)
///
///1. Process pid
///2. Thread id.
pub fn get_thread_process_id(window: HWND) -> (u32, u32) {
    let mut process_pid: u32 = 0;
    let thread_pid = unsafe {GetWindowThreadProcessId(window, &mut process_pid)};

    (process_pid, thread_pid)
}

///Search for a window's handle.
///
///# Parameters
///
///* ```class_name``` - Name of window's class.
///* ```window_name``` - Window's title.
///
///# Return
///
///* ```Ok``` - Handle to window.
///* ```Err``` - Error reason.
pub fn find<T: AsRef<ffi::OsStr>>(class_name: T, window_name: Option<T>) -> Result<HWND> {
    let result: HWND;
    let mut class_name: Vec<u16> = class_name.as_ref().encode_wide().collect();
    class_name.push(0);
    let class_name = class_name.as_ptr();

    if let Some(window_name) = window_name {
        let mut window_name: Vec<u16> = window_name.as_ref().encode_wide().collect();
        window_name.push(0);
        let window_name = window_name.as_ptr();

        result = unsafe {FindWindowW(class_name, window_name)};
    }
    else {
        result = unsafe {FindWindowW(class_name, ptr::null())};
    }

    if result.is_null() {
        return Err(utils::get_last_error());
    }

    Ok(result)
}

///Search for a window's child.
///
///# Parameters
///
///* ```class_name``` - Name of window's class.
///* ```window_name``` - Window's title.
///* ```parent``` - Handle to a parent window. Default is desktop.
///* ```child_after``` - Handle to a child window after which to start search.
///
///# Return
///
///* ```Ok``` - Handle to window.
///* ```Err``` - Error reason.
pub fn find_child<T: AsRef<ffi::OsStr>>(class_name: T,
                                        window_name: Option<T>,
                                        parent: Option<HWND>,
                                        child_after: Option<HWND>) -> Result<HWND> {
    let result: HWND;
    let mut class_name: Vec<u16> = class_name.as_ref().encode_wide().collect();
    class_name.push(0);
    let class_name = class_name.as_ptr();

    let parent = parent.unwrap_or(0x0 as HWND);
    let child_after = child_after.unwrap_or(0x0 as HWND);

    if let Some(window_name) = window_name {
        let mut window_name: Vec<u16> = window_name.as_ref().encode_wide().collect();
        window_name.push(0);
        let window_name = window_name.as_ptr();

        result = unsafe {FindWindowExW(parent, child_after, class_name, window_name)};
    }
    else {
        result = unsafe {FindWindowExW(parent, child_after, class_name, ptr::null())};
    }

    if result.is_null() {
        return Err(utils::get_last_error());
    }

    Ok(result)
}

///Sends message to a window.
///
///# Note
///All messages that this function sends are blocking.
///
///You can specify timeout for how long to block.
///
///# Parameters
///
///* ```window``` - Handle to the window for which to send.
///* ```msg_type``` - Type of message. See WinAPI docs.
///* ```wParam``` - Additional message specific parameter.
///* ```lParam``` - Additional message specific parameter.
///* ```timeout``` - Optional timeout in milliseconds.
///
///# Return
///
///* ```Ok``` - Message has been sent  successfully.
///* ```Err``` - Error reason. Relevant only to message with timeout.
pub fn send_message(window: HWND,
                    msg_type: UINT,
                    w_param: WPARAM,
                    l_param: LPARAM,
                    timeout: Option<UINT>) -> Result<LRESULT> {
    if let Some(timeout) = timeout {
        unsafe {
            let mut result: ULONG_PTR = 0;
            let result_ptr = &mut result as PDWORD_PTR;
            if SendMessageTimeoutW(window, msg_type, w_param, l_param, SMTO_BLOCK, timeout, result_ptr) == 0 {
                return Err(utils::get_last_error());
            }
            Ok(result as LRESULT)
        }
    }
    else {
        unsafe {
            Ok(SendMessageW(window, msg_type, w_param, l_param))
        }
    }
}

///Button click message type
const BM_CLICK: c_uint = 0x00F5;

///Sends push button message to a window.
///
///# Parameters
///
///* ```window``` - Handle to the window for which to send.
///* ```timeout``` - Optional timeout in milliseconds.
pub fn send_push_button(window: HWND, timeout: Option<UINT>) -> Result<LRESULT> {
    send_message(window, BM_CLICK, 0, 0, timeout)
}

///Sends set text message to a window.
///
///# Parameters
///
///* ```window``` - Handle to the window for which to send.
///* ```text``` - Text with which to update window.
///
///# Return
///
///* ```true``` - On success.
///* ```false``` - Otherwise.
pub fn send_set_text<T: AsRef<ffi::OsStr>>(window: HWND, text: T) -> bool {
    let mut text: Vec<u16> = text.as_ref().encode_wide().collect();
    text.push(0);
    let text = text.as_ptr() as LPARAM;

    let result = send_message(window, WM_SETTEXT, 0, text, None);
    result.is_ok() && result.unwrap() != 0
}

///Sends get text message to a window
///
///# Parameters
///
///* ```window``` - Handle to the window for which to send.
///
///# Return
///
///* ```String``` - Window's text.
///* ```None``` - If there is no text.
pub fn send_get_text(window: HWND) -> Option<String> {
    //Does not include null char
    let buf_len = send_message(window, WM_GETTEXTLENGTH, 0, 0, None).unwrap();

    if buf_len == 0 {
        return None
    }

    let buf_len = buf_len + 1;

    let text: Vec<u16> = vec![0; buf_len as usize];
    let text_ptr = text.as_ptr() as LPARAM;
    //Does not include null char
    let buf_len = send_message(window, WM_GETTEXT, buf_len as WPARAM, text_ptr, None).unwrap() as usize;

    Some(String::from_utf16_lossy(&text[..buf_len]))
}

///Sends sys command to a window.
///
///Refer to https://msdn.microsoft.com/en-us/library/windows/desktop/ms646360%28v=vs.85%29.aspx
///
///# Parameters
///
///* ```window``` - Handle to the window for which to send.
///* ```cmd_type``` - Type of sys command.
///* ```l_param``` - Mouse & screen coordinates.
///
///# Return
///
///* ```true``` - On success.
///* ```false``` - Otherwise.
pub fn send_sys_command(window: HWND, cmd_type: WPARAM, l_param: LPARAM) -> bool {
    let result = send_message(window, WM_SYSCOMMAND, cmd_type, l_param, None);
    //Return is zero if Application proceed message.
    result.is_ok() && result.unwrap() == 0
}

#[inline]
///Retrieves the window handle to the active window attached to the calling thread's message queue.
pub fn get_active() -> HWND {
    unsafe {
        GetActiveWindow()
    }
}

#[inline]
///Retrieves the window handle used by the console associated with the calling process.
pub fn get_console() -> HWND {
    unsafe {
        GetConsoleWindow()
    }
}

///A window builder.
///
///To successfully create window at least class name should be specified.
///You can use pre-defined [classes](https://msdn.microsoft.com/en-us/library/windows/desktop/ms633574(v=vs.85).aspx#system) for simple message only window.
pub struct Builder {
    ex_style: DWORD,
    class_name: Option<Vec<u16>>,
    window_name: Option<Vec<u16>>,
    style: DWORD,
    position: (c_int, c_int),
    width: c_int,
    height: c_int,
    parent: HWND,
    menu: HMENU,
    inst: HINSTANCE,
    param: Option<CREATESTRUCTW>
}

impl Builder {
    ///Initialize builder.
    pub fn new() -> Builder {
        Builder {
            ex_style: 0,
            class_name: None,
            window_name: None,
            style: 0,
            position: (CW_USEDEFAULT, CW_USEDEFAULT),
            width: CW_USEDEFAULT,
            height: CW_USEDEFAULT,
            parent: ptr::null_mut(),
            menu: ptr::null_mut(),
            inst: ptr::null_mut(),
            param: None
        }
    }

    ///Sets style.
    ///
    ///See possible [values](https://msdn.microsoft.com/en-us/library/ms632600(v=vs.85).aspx)
    pub fn style(&mut self, value: DWORD) -> &mut Builder {
        self.style = value;
        self
    }

    ///Sets extended style.
    ///
    ///See possible [values](https://msdn.microsoft.com/en-us/library/ff700543(v=vs.85).aspx)
    pub fn ex_style(&mut self, value: DWORD) -> &mut Builder {
        self.ex_style = value;
        self
    }

    ///Sets class name.
    pub fn class_name<T: AsRef<ffi::OsStr>>(&mut self, value: T) -> &mut Builder {
        let mut class_name: Vec<u16> = value.as_ref().encode_wide().collect();
        class_name.push(0);
        self.class_name = Some(class_name);
        self
    }

    ///Sets class name.
    pub fn window_name<T: AsRef<ffi::OsStr>>(&mut self, value: T) -> &mut Builder {
        let mut window_name: Vec<u16> = value.as_ref().encode_wide().collect();
        window_name.push(0);
        self.window_name = Some(window_name);
        self
    }

    ///Sets position. Default is `(CW_USEDEFAULT, CW_USEDEFAULT`.
    pub fn position(&mut self, x: c_int, y: c_int) -> &mut Builder {
        self.position.0 = x;
        self.position.1 = y;
        self
    }

    ///Sets size of window. Default is `CW_USEDEFAULT`.
    pub fn size(&mut self, width: c_int, height: c_int) -> &mut Builder {
        self.width = width;
        self.height = height;
        self
    }

    ///Sets parent window. Default is `null`
    pub fn parent(&mut self, value: HWND) -> &mut Builder {
        self.parent = value;
        self
    }

    ///Sets parent window to message only `HWND_MESSAGE`.
    pub fn parent_message(&mut self) -> &mut Builder {
        self.parent = HWND_MESSAGE;
        self
    }

    ///Seta module instance associated with window.
    pub fn instance(&mut self, value: HINSTANCE) -> &mut Builder {
        self.inst = value;
        self
    }

    ///Sets param which will be sent in `WM_CREATE`
    pub fn param(&mut self, value: &CREATESTRUCTW) -> &mut Builder {
        self.param = Some(*value);
        self
    }

    ///Creates window.
    pub fn create(&mut self) -> Result<HWND> {
        let param = self.param.as_mut()
                              .map(|create_struct| create_struct as *mut CREATESTRUCTW as *mut c_void)
                              .unwrap_or(ptr::null_mut());

        let result = unsafe { CreateWindowExW(self.ex_style,
                                              self.class_name.as_mut().map(|val| val.as_ptr()).unwrap_or(ptr::null()),
                                              self.window_name.as_mut().map(|val| val.as_ptr()).unwrap_or(ptr::null()),
                                              self.style,
                                              self.position.0, self.position.1,
                                              self.width, self.height,
                                              self.parent,
                                              self.menu,
                                              self.inst,
                                              param) };

        if result.is_null() {
            Err(utils::get_last_error())
        }
        else {
            Ok(result)
        }
    }
}

///Shows window
///
///[See](https://msdn.microsoft.com/en-us/library/windows/desktop/ms633548(v=vs.85).aspx) possible commands
///
///If the window was previously visible, the return value is `true`.
///If the window was previously hidden, the return value is `false`.
pub fn show(window: HWND, cmd: c_int) -> bool {
    unsafe {
        ShowWindow(window, cmd) != 0
    }
}

///Destroy window.
///
///`WM_DESTROY` and `WM_NCDESTROY` are sent after.
#[inline]
pub fn destroy(window: HWND) -> bool {
    unsafe {
        DestroyWindow(window) != 0
    }
}
