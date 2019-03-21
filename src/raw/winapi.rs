pub use winapi::ctypes::{
    c_uint,
    c_int,
    c_ulong,
    c_void,
    c_uchar
};

//WinAPI types
pub use winapi::shared::windef::{
    HWND,
    HMENU
};

pub use winapi::shared::winerror::{
    ERROR_NO_MORE_FILES
};

pub use winapi::um::handleapi::INVALID_HANDLE_VALUE;

pub use winapi::um::minwinbase::{
    WIN32_FIND_DATAW,
    FINDEX_INFO_LEVELS,
    FindExInfoStandard,
    FindExInfoBasic,
    FindExInfoMaxInfoLevel,
    FindExSearchNameMatch,
    FindExSearchLimitToDirectories,
    FINDEX_SEARCH_OPS
};

pub use winapi::shared::basetsd::{
    ULONG_PTR,
    PDWORD_PTR,
    SIZE_T
};

pub use winapi::shared::ntdef::{
    LPWSTR,
    LPCWSTR,
    HANDLE
};

pub use winapi::um::winnt::{
    FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_READONLY,
    MEMORY_BASIC_INFORMATION,
    MEM_COMMIT,
    MEM_FREE,
    MEM_RESERVE,
    LARGE_INTEGER,
    WT_EXECUTEINTIMERTHREAD,
    WT_EXECUTEINPERSISTENTTHREAD,
    WT_EXECUTELONGFUNCTION,
    WT_EXECUTEONLYONCE,
    WT_TRANSFER_IMPERSONATION,
    WAITORTIMERCALLBACK
};

pub use winapi::shared::minwindef::{
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
pub use winapi::um::winuser::{
    LPMSG,
    SMTO_BLOCK,
    WM_SYSCOMMAND,
    WM_GETTEXT,
    WM_GETTEXTLENGTH,
    WM_SETTEXT,
    CW_USEDEFAULT,
    CREATESTRUCTW,
    HWND_MESSAGE,
    MSG,
    SW_SHOW,
    SW_HIDE
};

//WinAPI functions
pub use winapi::um::winuser::{
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
    DestroyWindow,
    ShowWindow
};

pub use winapi::um::processthreadsapi::{
    OpenProcess,
    GetCurrentProcess,
    GetProcessId,
    TerminateProcess,
};

pub use winapi::um::handleapi::{
    CloseHandle
};

pub use winapi::um::memoryapi::{
    ReadProcessMemory,
    WriteProcessMemory
};

pub use winapi::um::winbase::{
    QueryFullProcessImageNameW
};

pub use winapi::um::wincon::{
    GetConsoleWindow,
};

pub use winapi::um::fileapi::{
    FindFirstFileExW,
    FindNextFileW,
    FindClose,
};

pub use winapi::um::memoryapi::{
    VirtualQueryEx,
};

pub use winapi::um::libloaderapi::{
    GetModuleHandleExW,
    GetModuleFileNameW
};

pub use winapi::um::profileapi::{
    QueryPerformanceFrequency,
    QueryPerformanceCounter
};

pub use winapi::um::threadpoollegacyapiset::{
    CreateTimerQueue,
    DeleteTimerQueueEx,
    CreateTimerQueueTimer,
    DeleteTimerQueueTimer,
    ChangeTimerQueueTimer,
};
