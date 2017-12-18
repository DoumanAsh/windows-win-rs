extern crate winapi;

pub use self::winapi::ctypes::{
    c_uint,
    c_int,
    c_ulong,
    c_void
};

//WinAPI types
pub use self::winapi::shared::windef::{
    HWND,
    HMENU
};

pub use self::winapi::shared::winerror::{
    ERROR_NO_MORE_FILES
};

pub use self::winapi::um::handleapi::INVALID_HANDLE_VALUE;

pub use self::winapi::um::minwinbase::{
    WIN32_FIND_DATAW,
    FINDEX_INFO_LEVELS,
    FindExInfoStandard,
    FindExInfoBasic,
    FindExInfoMaxInfoLevel,
    FindExSearchNameMatch,
    FindExSearchLimitToDirectories,
    FINDEX_SEARCH_OPS
};

pub use self::winapi::shared::basetsd::{
    ULONG_PTR,
    PDWORD_PTR,
    SIZE_T
};

pub use self::winapi::shared::ntdef::{
    LPWSTR,
    LPCWSTR,
    HANDLE
};

pub use self::winapi::um::winnt::{
    FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_READONLY,
    MEMORY_BASIC_INFORMATION,
    MEM_COMMIT,
    MEM_FREE,
    MEM_RESERVE
};

pub use self::winapi::shared::minwindef::{
    LPARAM,
    WPARAM,
    LRESULT,
    UINT,
    MAX_PATH,
    HINSTANCE,
    DWORD,
    HMODULE
};

//WinAPI constants
pub use self::winapi::um::winuser::{
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
pub use self::winapi::um::winuser::{
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

pub use self::winapi::um::processthreadsapi::{
    OpenProcess,
    GetCurrentProcess,
    GetProcessId,
    TerminateProcess,
};

pub use self::winapi::um::handleapi::{
    CloseHandle
};

pub use self::winapi::um::memoryapi::{
    ReadProcessMemory,
    WriteProcessMemory
};

pub use self::winapi::um::winbase::{
    QueryFullProcessImageNameW
};

pub use self::winapi::um::wincon::{
    GetConsoleWindow,
};

pub use self::winapi::um::fileapi::{
    FindFirstFileExW,
    FindNextFileW,
    FindClose,
};

pub use self::winapi::um::memoryapi::{
    VirtualQueryEx,
};

pub use self::winapi::um::libloaderapi::{
    GetModuleHandleExW,
    GetModuleFileNameW
};
