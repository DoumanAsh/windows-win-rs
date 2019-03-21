//! Various useful utilities for working with winapi
use std::io;

#[inline(always)]
///Alias to `std::io::Error::last_os_error()`
pub fn get_last_error() -> io::Error {
    io::Error::last_os_error()
}
