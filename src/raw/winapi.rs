extern crate winapi;
extern crate user32;
extern crate kernel32;

pub use os::raw::{
    c_uint,
    c_int,
    c_void
};

pub use os::windows::raw::HANDLE;

//WinAPI types
pub use self::winapi::windef::{
    HWND,
    HMENU
};

pub use self::winapi::winerror::{
    ERROR_NO_MORE_FILES
};

pub use self::winapi::shlobj::{
    INVALID_HANDLE_VALUE
};

pub use self::winapi::minwinbase::{
    WIN32_FIND_DATAW,
    FINDEX_INFO_LEVELS,
    FindExInfoStandard,
    FindExInfoBasic,
    FindExInfoMaxInfoLevel,
    FindExSearchNameMatch,
    FindExSearchLimitToDirectories,
    FINDEX_SEARCH_OPS
};

pub use self::winapi::basetsd::{
    ULONG_PTR,
    PDWORD_PTR,
    SIZE_T
};

pub use self::winapi::winnt::{
    LPWSTR,
    FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_READONLY
};

pub use self::winapi::minwindef::{
    LPARAM,
    WPARAM,
    LRESULT,
    UINT,
    MAX_PATH,
    HINSTANCE,
    DWORD
};

//WinAPI constants
pub use self::winapi::winuser::{
    LPMSG,
    SMTO_BLOCK,
    WM_SYSCOMMAND,
    WM_GETTEXT,
    WM_GETTEXTLENGTH,
    WM_SETTEXT,
    CW_USEDEFAULT,
    CREATESTRUCTW,
    HWND_MESSAGE,
    MSG
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
    GetActiveWindow,
    CreateWindowExW,
    DestroyWindow
};

pub use self::kernel32::{
    OpenProcess,
    CloseHandle,
    ReadProcessMemory,
    WriteProcessMemory,
    QueryFullProcessImageNameW,
    GetCurrentProcess,
    GetConsoleWindow,
    FindFirstFileExW,
    FindNextFileW,
    FindClose
};


