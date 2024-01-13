//! WinAPI related definitions
#![allow(missing_docs, non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub use core::ffi::*;

pub type BYTE = c_uchar;
pub type BOOL = c_int;
pub type BOOLEAN = BYTE;
pub type PVOID = *mut c_void;
pub type HANDLE = *mut c_void;
pub type PHANDLE = *mut HANDLE;
pub type HMENU = *mut c_void;
pub type HWND = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HMODULE = HINSTANCE;
pub type UINT = c_uint;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type ULONG = c_ulong;
pub type LONG = c_long;
pub type DWORD = c_ulong;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type WCHAR = u16;
pub type LONGLONG = i64;
pub type LPMSG = *mut MSG;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;

pub type ULONG_PTR = usize;
pub type PDWORD_PTR = *mut ULONG_PTR;
pub type SIZE_T = ULONG_PTR;

pub type LPWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum TOKEN_INFORMATION_CLASS {
    TokenUser = 1,
    TokenGroups,
    TokenPrivileges,
    TokenOwner,
    TokenPrimaryGroup,
    TokenDefaultDacl,
    TokenSource,
    TokenType,
    TokenImpersonationLevel,
    TokenStatistics,
    TokenRestrictedSids,
    TokenSessionId,
    TokenGroupsAndPrivileges,
    TokenSessionReference,
    TokenSandBoxInert,
    TokenAuditPolicy,
    TokenOrigin,
    TokenElevationType,
    TokenLinkedToken,
    TokenElevation,
    TokenHasRestrictions,
    TokenAccessInformation,
    TokenVirtualizationAllowed,
    TokenVirtualizationEnabled,
    TokenIntegrityLevel,
    TokenUIAccess,
    TokenMandatoryPolicy,
    TokenLogonSid,
    TokenIsAppContainer,
    TokenCapabilities,
    TokenAppContainerSid,
    TokenAppContainerNumber,
    TokenUserClaimAttributes,
    TokenDeviceClaimAttributes,
    TokenRestrictedUserClaimAttributes,
    TokenRestrictedDeviceClaimAttributes,
    TokenDeviceGroups,
    TokenRestrictedDeviceGroups,
    TokenSecurityAttributes,
    TokenIsRestricted,
    TokenProcessTrustLevel,
    TokenPrivateNameSpace,
    TokenSingletonAttributes,
    TokenBnoIsolation,
    MaxTokenInfoClass,
}

pub const MAX_PATH: usize = 260;

pub const MB_OK: UINT = 0x00000000;
pub const MB_OKCANCEL: UINT = 0x00000001;
pub const MB_ABORTRETRYIGNORE: UINT = 0x00000002;
pub const MB_YESNOCANCEL: UINT = 0x00000003;
pub const MB_YESNO: UINT = 0x00000004;
pub const MB_RETRYCANCEL: UINT = 0x00000005;
pub const MB_CANCELTRYCONTINUE: UINT = 0x00000006;
pub const MB_ICONHAND: UINT = 0x00000010;
pub const MB_ICONQUESTION: UINT = 0x00000020;
pub const MB_ICONEXCLAMATION: UINT = 0x00000030;
pub const MB_ICONASTERISK: UINT = 0x00000040;
pub const MB_USERICON: UINT = 0x00000080;
pub const MB_ICONWARNING: UINT = MB_ICONEXCLAMATION;
pub const MB_ICONERROR: UINT = MB_ICONHAND;
pub const MB_ICONINFORMATION: UINT = MB_ICONASTERISK;
pub const MB_ICONSTOP: UINT = MB_ICONHAND;
pub const MB_DEFBUTTON1: UINT = 0x00000000;
pub const MB_DEFBUTTON2: UINT = 0x00000100;
pub const MB_DEFBUTTON3: UINT = 0x00000200;
pub const MB_DEFBUTTON4: UINT = 0x00000300;
pub const MB_APPLMODAL: UINT = 0x00000000;
pub const MB_SYSTEMMODAL: UINT = 0x00001000;
pub const MB_TASKMODAL: UINT = 0x00002000;
pub const MB_HELP: UINT = 0x00004000;
pub const MB_NOFOCUS: UINT = 0x00008000;
pub const MB_SETFOREGROUND: UINT = 0x00010000;
pub const MB_DEFAULT_DESKTOP_ONLY: UINT = 0x00020000;
pub const MB_TOPMOST: UINT = 0x00040000;
pub const MB_RIGHT: UINT = 0x00080000;
pub const MB_RTLREADING: UINT = 0x00100000;
pub const MB_SERVICE_NOTIFICATION: UINT = 0x00200000;
pub const MB_SERVICE_NOTIFICATION_NT3X: UINT = 0x00040000;
pub const MB_TYPEMASK: UINT = 0x0000000F;
pub const MB_ICONMASK: UINT = 0x000000F0;
pub const MB_DEFMASK: UINT = 0x00000F00;
pub const MB_MODEMASK: UINT = 0x00003000;
pub const MB_MISCMASK: UINT = 0x0000C000;

pub const SMTO_BLOCK: UINT = 0x0001;
pub const WM_SYSCOMMAND: UINT = 0x0112;
pub const WM_GETTEXT: UINT = 0x000D;
pub const WM_GETTEXTLENGTH: UINT = 0x000E;
pub const WM_SETTEXT: UINT = 0x000C;
pub const WM_DEVICECHANGE: UINT = 0x0219;
pub const WM_MDICREATE: UINT = 0x0220;
pub const WM_MDIDESTROY: UINT = 0x0221;
pub const WM_MDIACTIVATE: UINT = 0x0222;
pub const WM_MDIRESTORE: UINT = 0x0223;
pub const WM_MDINEXT: UINT = 0x0224;
pub const WM_MDIMAXIMIZE: UINT = 0x0225;
pub const WM_MDITILE: UINT = 0x0226;
pub const WM_MDICASCADE: UINT = 0x0227;
pub const WM_MDIICONARRANGE: UINT = 0x0228;
pub const WM_MDIGETACTIVE: UINT = 0x0229;
pub const WM_MDISETMENU: UINT = 0x0230;
pub const WM_ENTERSIZEMOVE: UINT = 0x0231;
pub const WM_EXITSIZEMOVE: UINT = 0x0232;
pub const WM_DROPFILES: UINT = 0x0233;
pub const WM_MDIREFRESHMENU: UINT = 0x0234;
pub const WM_POINTERDEVICECHANGE: UINT = 0x238;
pub const WM_POINTERDEVICEINRANGE: UINT = 0x239;
pub const WM_POINTERDEVICEOUTOFRANGE: UINT = 0x23A;
pub const WM_TOUCH: UINT = 0x0240;
pub const WM_NCPOINTERUPDATE: UINT = 0x0241;
pub const WM_NCPOINTERDOWN: UINT = 0x0242;
pub const WM_NCPOINTERUP: UINT = 0x0243;
pub const WM_POINTERUPDATE: UINT = 0x0245;
pub const WM_POINTERDOWN: UINT = 0x0246;
pub const WM_POINTERUP: UINT = 0x0247;
pub const WM_POINTERENTER: UINT = 0x0249;
pub const WM_POINTERLEAVE: UINT = 0x024A;
pub const WM_POINTERACTIVATE: UINT = 0x024B;
pub const WM_POINTERCAPTURECHANGED: UINT = 0x024C;
pub const WM_TOUCHHITTESTING: UINT = 0x024D;
pub const WM_POINTERWHEEL: UINT = 0x024E;
pub const WM_POINTERHWHEEL: UINT = 0x024F;
pub const DM_POINTERHITTEST: UINT = 0x0250;
pub const WM_POINTERROUTEDTO: UINT = 0x0251;
pub const WM_POINTERROUTEDAWAY: UINT = 0x0252;
pub const WM_POINTERROUTEDRELEASED: UINT = 0x0253;
pub const WM_IME_SETCONTEXT: UINT = 0x0281;
pub const WM_IME_NOTIFY: UINT = 0x0282;
pub const WM_IME_CONTROL: UINT = 0x0283;
pub const WM_IME_COMPOSITIONFULL: UINT = 0x0284;
pub const WM_IME_SELECT: UINT = 0x0285;
pub const WM_IME_CHAR: UINT = 0x0286;
pub const WM_IME_REQUEST: UINT = 0x0288;
pub const WM_IME_KEYDOWN: UINT = 0x0290;
pub const WM_IME_KEYUP: UINT = 0x0291;
pub const WM_MOUSEHOVER: UINT = 0x02A1;
pub const WM_MOUSELEAVE: UINT = 0x02A3;
pub const WM_NCMOUSEHOVER: UINT = 0x02A0;
pub const WM_NCMOUSELEAVE: UINT = 0x02A2;
pub const WM_WTSSESSION_CHANGE: UINT = 0x02B1;
pub const WM_TABLET_FIRST: UINT = 0x02c0;
pub const WM_TABLET_LAST: UINT = 0x02df;
pub const WM_DPICHANGED: UINT = 0x02E0;
pub const WM_DPICHANGED_BEFOREPARENT: UINT = 0x02E2;
pub const WM_DPICHANGED_AFTERPARENT: UINT = 0x02E3;
pub const WM_GETDPISCALEDSIZE: UINT = 0x02E4;
pub const WM_CUT: UINT = 0x0300;
pub const WM_COPY: UINT = 0x0301;
pub const WM_PASTE: UINT = 0x0302;
pub const WM_CLEAR: UINT = 0x0303;
pub const WM_UNDO: UINT = 0x0304;
pub const WM_RENDERFORMAT: UINT = 0x0305;
pub const WM_RENDERALLFORMATS: UINT = 0x0306;
pub const WM_DESTROYCLIPBOARD: UINT = 0x0307;
pub const WM_DRAWCLIPBOARD: UINT = 0x0308;
pub const WM_PAINTCLIPBOARD: UINT = 0x0309;
pub const WM_VSCROLLCLIPBOARD: UINT = 0x030A;
pub const WM_SIZECLIPBOARD: UINT = 0x030B;
pub const WM_ASKCBFORMATNAME: UINT = 0x030C;
pub const WM_CHANGECBCHAIN: UINT = 0x030D;
pub const WM_HSCROLLCLIPBOARD: UINT = 0x030E;
pub const WM_QUERYNEWPALETTE: UINT = 0x030F;
pub const WM_PALETTEISCHANGING: UINT = 0x0310;
pub const WM_PALETTECHANGED: UINT = 0x0311;
pub const WM_HOTKEY: UINT = 0x0312;
pub const WM_PRINT: UINT = 0x0317;
pub const WM_PRINTCLIENT: UINT = 0x0318;
pub const WM_APPCOMMAND: UINT = 0x0319;
pub const WM_THEMECHANGED: UINT = 0x031A;
pub const WM_CLIPBOARDUPDATE: UINT = 0x031D;
pub const WM_DWMCOMPOSITIONCHANGED: UINT = 0x031E;
pub const WM_DWMNCRENDERINGCHANGED: UINT = 0x031F;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: UINT = 0x0320;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: UINT = 0x0321;
pub const WM_DWMSENDICONICTHUMBNAIL: UINT = 0x0323;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: UINT = 0x0326;
pub const WM_GETTITLEBARINFOEX: UINT = 0x033F;
pub const WM_HANDHELDFIRST: UINT = 0x0358;
pub const WM_HANDHELDLAST: UINT = 0x035F;
pub const WM_AFXFIRST: UINT = 0x0360;
pub const WM_AFXLAST: UINT = 0x037F;
pub const WM_PENWINFIRST: UINT = 0x0380;
pub const WM_PENWINLAST: UINT = 0x038F;
pub const WM_APP: UINT = 0x8000;
pub const WM_USER: UINT = 0x0400;
pub const CW_USEDEFAULT: c_int = -2147483648i32;

pub const HWND_MESSAGE: HWND = -3isize as HWND;

pub const FILE_ATTRIBUTE_DIRECTORY: DWORD = 0x00000010;
pub const FILE_ATTRIBUTE_READONLY: DWORD = 0x00000001;

pub const MEM_COMMIT: DWORD = 0x1000;
pub const MEM_FREE: DWORD = 0x10000;
pub const MEM_RESERVE: DWORD = 0x2000;

pub const WT_EXECUTEINTIMERTHREAD: ULONG = 0x00000020;
pub const WT_EXECUTEINPERSISTENTTHREAD: ULONG = 0x00000080;
pub const WT_EXECUTELONGFUNCTION: ULONG = 0x00000010;
pub const WT_EXECUTEONLYONCE: ULONG = 0x00000008;
pub const WT_TRANSFER_IMPERSONATION: ULONG = 0x00000100;

pub type WAITORTIMERCALLBACK = Option<unsafe extern "system" fn(_: PVOID, _: BOOLEAN)>;
pub type WNDENUMPROC = Option<unsafe extern "system" fn(_: HWND, _: LPARAM) -> BOOL>;

pub const TOKEN_QUERY: DWORD = 0x0008;

pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

pub const SW_SHOW: c_int = 5;
pub const SW_HIDE: c_int = 0;

pub const ERROR_NO_MORE_FILES: DWORD = 18;

pub type FINDEX_INFO_LEVELS = u32;
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = 0;
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = FindExInfoStandard + 1;
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = FindExInfoBasic + 1;

pub type FINDEX_SEARCH_OPS = u32;
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = 0;
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = FindExSearchNameMatch + 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FILETIME {
    pub dwLowDateTime: DWORD,
    pub dwHighDateTime: DWORD,
}

pub type LPWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
#[repr(C)]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: DWORD,
    pub ftCreationTime: FILETIME,
    pub ftLastAccessTime: FILETIME,
    pub ftLastWriteTime: FILETIME,
    pub nFileSizeHigh: DWORD,
    pub nFileSizeLow: DWORD,
    pub dwReserved0: DWORD,
    pub dwReserved1: DWORD,
    pub cFileName: [WCHAR; 260],
    pub cAlternateFileName: [WCHAR; 14],
}

pub type PMEMORY_BASIC_INFORMATION = *mut MEMORY_BASIC_INFORMATION;
#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: PVOID,
    pub AllocationBase: PVOID,
    pub AllocationProtect: DWORD,
    pub RegionSize: SIZE_T,
    pub State: DWORD,
    pub Protect: DWORD,
    pub Type: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LARGE_INTEGER_s {
    pub LowPart: ULONG,
    pub HighPart: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LARGE_INTEGER_u {
    pub LowPart: ULONG,
    pub HighPart: LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union LARGE_INTEGER {
    repr: i64,
    pub s: LARGE_INTEGER_s,
    pub u: LARGE_INTEGER_u,
    pub QuadPart: LONGLONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: DWORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCWSTR,
    pub lpszClass: LPCWSTR,
    pub dwExStyle: DWORD,
}

//Functions
#[link(name = "user32", kind = "dylib")]
extern "system" {
    pub fn FindWindowW(lpClassName: LPCWSTR, lpWindowName: LPCWSTR) -> HWND;
    pub fn FindWindowExW(hWndParent: HWND, hWndChildAfter: HWND, lpszClass: LPCWSTR, lpszWindow: LPCWSTR) -> HWND;
    pub fn IsWindowVisible(hWnd: HWND) -> BOOL;
    pub fn GetWindowTextW(hWnd: HWND, lpString: LPWSTR, nMaxCount: c_int) -> c_int;
    pub fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SendMessageTimeoutW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM, fuFlags: UINT, uTimeout: UINT, lpdwResult: PDWORD_PTR) -> LRESULT;
    pub fn RealGetWindowClassW(hwnd: HWND, ptszClassName: LPWSTR, cchClassNameMax: UINT) -> UINT;
    pub fn EnumChildWindows(hWndParent: HWND, lpEnumFunc: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    pub fn EnumWindows(lpEnumFunc: WNDENUMPROC, lParam: LPARAM) -> BOOL;
    pub fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: LPDWORD) -> DWORD;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn PeekMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT) -> BOOL;
    pub fn GetActiveWindow() -> HWND;
    pub fn CreateWindowExW(dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR, dwStyle: DWORD, x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> c_int;
    pub fn SetLastErrorEx(dwErrCode: DWORD, dwType: DWORD);
    pub fn AddClipboardFormatListener(hWnd: HWND) -> BOOL;
    pub fn RemoveClipboardFormatListener(hWnd: HWND) -> BOOL;
    pub fn PostMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
}

#[link(name = "kernel32", kind = "dylib")]
extern "system" {
    pub fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn GetProcessId(Process: HANDLE) -> DWORD;
    pub fn TerminateProcess(hProcess: HANDLE, uExitCode: UINT) -> BOOL;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;

    pub fn ReadProcessMemory(hProcess: HANDLE, lpBaseAddress: LPCVOID, lpBuffer: LPVOID, nSize: SIZE_T, lpNumberOfBytesRead: *mut SIZE_T) -> BOOL;
    pub fn WriteProcessMemory(hProcess: HANDLE, lpBaseAddress: LPVOID, lpBuffer: LPCVOID, nSize: SIZE_T, lpNumberOfBytesWritten: *mut SIZE_T) -> BOOL;

    pub fn QueryFullProcessImageNameW(hProcess: HANDLE, dwFlags: DWORD, lpExeName: LPWSTR, lpdwSize: PDWORD) -> BOOL;

    pub fn GetConsoleWindow() -> HWND;

    pub fn FindFirstFileExW(lpFileName: LPCWSTR, fInfoLevelId: FINDEX_INFO_LEVELS, lpFindFileData: LPVOID, fSearchOp: FINDEX_SEARCH_OPS, lpSearchFilter: LPVOID, dwAdditionalFlags: DWORD) -> HANDLE;
    pub fn FindNextFileW(hFindFile: HANDLE, lpFindFileData: LPWIN32_FIND_DATAW) -> BOOL;
    pub fn FindClose(hFindFile: HANDLE) -> BOOL;

    pub fn VirtualQueryEx(hProcess: HANDLE, lpAddress: LPCVOID, lpBuffer: PMEMORY_BASIC_INFORMATION, dwLength: SIZE_T) -> SIZE_T;

    pub fn GetModuleHandleExW(dwFlags: DWORD, lpModuleName: LPCWSTR, phModule: *mut HMODULE) -> BOOL;
    pub fn GetModuleFileNameW(hModule: HMODULE, lpFilename: LPWSTR, nSize: DWORD) -> DWORD;

    pub fn QueryPerformanceFrequency(lpFrequency: *mut LARGE_INTEGER) -> BOOL;
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut LARGE_INTEGER) -> BOOL;

    pub fn CreateTimerQueue() -> HANDLE;
    pub fn DeleteTimerQueueEx(TimerQueue: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    pub fn CreateTimerQueueTimer(phNewTimer: PHANDLE, TimerQueue: HANDLE, Callback: WAITORTIMERCALLBACK, Parameter: PVOID, DueTime: DWORD, Period: DWORD, Flags: ULONG) -> BOOL;
    pub fn DeleteTimerQueueTimer(TimerQueue: HANDLE, Timer: HANDLE, CompletionEvent: HANDLE) -> BOOL;
    pub fn ChangeTimerQueueTimer(TimerQueue: HANDLE, Timer: HANDLE, DueTime: ULONG, Period: ULONG) -> BOOL;
}

#[link(name = "advapi32", kind = "dylib")]
extern "system" {
    pub fn OpenProcessToken(ProcessHandle: HANDLE, DesiredAccess: DWORD, TokenHandle: PHANDLE) -> BOOL;
    pub fn GetTokenInformation(TokenHandle: HANDLE, TokenInformationClass: TOKEN_INFORMATION_CLASS, TokenInformation: LPVOID, TokenInformationLength: DWORD, ReturnLength: PDWORD) -> BOOL;
}
