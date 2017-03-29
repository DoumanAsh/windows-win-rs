//! Provides File Management functions

use ::ffi;
use ::os::windows::ffi::OsStrExt;
use ::default;
use ::ptr;
use ::mem;
use ::convert;
use ::io;
use ::inner_raw as raw;
use self::raw::winapi::*;

use ::utils;

const NO_MORE_FILES: i32 = ERROR_NO_MORE_FILES as i32;

///Level of information to store about file during search.
pub enum FileInfoLevel {
    ///Corresponds to FindExInfoStandard. Default.
    Standard,
    ///Corresponds to FindExInfoBasic.
    Basic,
    ///Corresponds to FindExInfoMaxInfoLevel.
    Max
}

impl convert::Into<FINDEX_INFO_LEVELS> for FileInfoLevel {
    fn into(self) -> FINDEX_INFO_LEVELS {
        match self {
            FileInfoLevel::Standard => FindExInfoStandard,
            FileInfoLevel::Basic => FindExInfoBasic,
            FileInfoLevel::Max => FindExInfoMaxInfoLevel
        }
    }
}

impl default::Default for FileInfoLevel {
    fn default() -> Self {
        FileInfoLevel::Standard
    }
}

///File search type
pub enum FileSearchType {
    ///Search file by name. Corresponds to FindExSearchNameMatch. Default.
    NameMatch,
    ///Ask to search directories only. Corresponds to FindExSearchLimitToDirectories.
    ///
    ///Note that this flag may be ignored by OS.
    DirectoriesOnly
}

impl convert::Into<FINDEX_SEARCH_OPS> for FileSearchType {
    fn into(self) -> FINDEX_SEARCH_OPS {
        match self {
            FileSearchType::NameMatch => FindExSearchNameMatch,
            FileSearchType::DirectoriesOnly => FindExSearchLimitToDirectories
        }
    }
}

impl default::Default for FileSearchType {
    fn default() -> Self {
        FileSearchType::NameMatch
    }
}

///Starts File search.
///
///It returns handle to continue search.
///
///For details see description of [FindFirstFileExW](https://msdn.microsoft.com/en-us/library/windows/desktop/aa364419(v=vs.85).aspx)
pub fn search<T: ?Sized + AsRef<ffi::OsStr>>(name: &T, level: FileInfoLevel, typ: FileSearchType, flags: DWORD) -> io::Result<Option<(HANDLE, WIN32_FIND_DATAW)>> {
    let mut utf16_buff: Vec<u16> = name.as_ref().encode_wide().collect();
    utf16_buff.push(0);

    let mut file_data: WIN32_FIND_DATAW = unsafe { mem::zeroed() };

    let result = unsafe { FindFirstFileExW(utf16_buff.as_ptr(), level.into(), &mut file_data as *mut _ as *mut c_void, typ.into(), ptr::null_mut(), flags) };

    if result == INVALID_HANDLE_VALUE {
        let error = utils::get_last_error();

        match error.raw_os_error() {
            Some(NO_MORE_FILES) => Ok(None),
            _ => Err(error)
        }
    }
    else {
        Ok(Some((result, file_data)))
    }
}

///Continues search.
pub fn search_next(handle: HANDLE) -> io::Result<Option<WIN32_FIND_DATAW>> {
    let mut file_data: WIN32_FIND_DATAW = unsafe { mem::zeroed() };

    unsafe {
        if FindNextFileW(handle, &mut file_data) != 0 {
            Ok(Some(file_data))
        }
        else {
            let error = utils::get_last_error();

            match error.raw_os_error() {
                Some(NO_MORE_FILES) => Ok(None),
                _ => Err(error)
            }
        }
    }
}

///Closes search.
pub fn search_close(handle: HANDLE) -> io::Result<()> {
    unsafe {
        if FindClose(handle) != 0 {
            Ok(())
        }
        else {
            Err(utils::get_last_error())
        }
    }
}
