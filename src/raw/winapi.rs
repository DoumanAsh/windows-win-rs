extern crate winapi;
extern crate user32;
extern crate kernel32;

pub use os::raw::{
    c_uint,
    c_void
};

pub use os::windows::raw::HANDLE;

//WinAPI types
pub use self::winapi::windef::{
    HWND
};

pub use self::winapi::basetsd::{
    ULONG_PTR,
    PDWORD_PTR,
    SIZE_T
};

pub use self::winapi::winnt::{
    LPWSTR
};

pub use self::winapi::minwindef::{
    LPARAM,
    WPARAM,
    LRESULT,
    UINT,
    MAX_PATH
};

//WinAPI constants
pub use self::winapi::winuser::{
    LPMSG,
    SMTO_BLOCK,
    WM_SYSCOMMAND,
    WM_GETTEXT,
    WM_GETTEXTLENGTH,
    WM_SETTEXT
};

//WinAPI functions
pub use self::user32::{
    FindWindowW,
    FindWindowExW,
    IsWindowVisible,
    GetWindowTextW,
    SendMessageW,
    SendMessageTimeoutW,
    RealGetWindowClassW,
    EnumChildWindows,
    EnumWindows,
    GetWindowThreadProcessId,
    GetMessageW,
    TranslateMessage,
    DispatchMessageW,
    PeekMessageW,
    GetActiveWindow
};

pub use self::kernel32::{
    OpenProcess,
    CloseHandle,
    ReadProcessMemory,
    WriteProcessMemory,
    QueryFullProcessImageNameW,
    GetCurrentProcess,
    GetConsoleWindow
};


