//! Provides File Management functions

use std::ffi;
use std::os::windows::ffi::{
    OsStrExt,
    OsStringExt
};
use core::{default, ptr, mem, convert};

use crate::sys::*;
use crate::utils::{self, Result};

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

///File System Entry.
pub struct Entry(WIN32_FIND_DATAW);

impl Entry {
    ///Determines whether Entry is directory or not.
    pub fn is_dir(&self) -> bool {
        (self.0.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0
    }

    ///Determines whether Entry is file or not.
    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    ///Returns size of entry
    pub fn size(&self) -> u64 {
        ((self.0.nFileSizeHigh as u64) << 32) | (self.0.nFileSizeLow as u64)
    }

    ///Returns whether Entry is read-only
    pub fn is_read_only(&self) -> bool {
        (self.0.dwFileAttributes & FILE_ATTRIBUTE_READONLY) != 0
    }

    ///Returns name of Entry.
    pub fn name(&self) -> ffi::OsString {
        ffi::OsString::from_wide(match self.0.cFileName.iter().position(|c| *c == 0) {
            Some(n) => &self.0.cFileName[..n],
            None => &self.0.cFileName
        })
    }
}

///File System Search iterator.
pub struct Search(HANDLE);

impl Search {
    ///Creates new instance of Search.
    ///
    ///Due to the way how underlying WinAPI works first entry is also returned alongside it.
    pub fn new<T: ?Sized + AsRef<ffi::OsStr>>(name: &T, level: FileInfoLevel, typ: FileSearchType, flags: DWORD) -> Result<(Search, Entry)> {
        let mut utf16_buff: Vec<u16> = name.as_ref().encode_wide().collect();
        utf16_buff.push(0);

        let mut file_data: WIN32_FIND_DATAW = unsafe { mem::zeroed() };

        let result = unsafe {
            FindFirstFileExW(utf16_buff.as_ptr(),
                             level.into(),
                             &mut file_data as *mut _ as *mut c_void,
                             typ.into(),
                             ptr::null_mut(),
                             flags)
        };

        if result == INVALID_HANDLE_VALUE {
            Err(utils::get_last_error())
        }
        else {
            Ok((Search(result), Entry(file_data)))
        }
    }

    ///Attempts to search again.
    pub fn again(&self) -> Result<Entry> {
        let mut file_data: WIN32_FIND_DATAW = unsafe { mem::zeroed() };

        unsafe {
            if FindNextFileW(self.0, &mut file_data) != 0 {
                Ok(Entry(file_data))
            }
            else {
                Err(utils::get_last_error())
            }
        }
    }

    ///Closes search.
    pub fn close(self) {
        unsafe {
            FindClose(self.0);
        }
    }
}

impl Iterator for Search {
    type Item = Result<Entry>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.again() {
            Ok(data) => Some(Ok(data)),
            Err(error) => {
                match error.raw_code() {
                    NO_MORE_FILES => None,
                    _ => Some(Err(error))
                }
            }
        }
    }
}

impl Drop for Search {
    fn drop(&mut self) {
        unsafe {
            debug_assert!(FindClose(self.0) != 0);
        }
    }
}

