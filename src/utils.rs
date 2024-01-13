//! Various useful utilities for working with winapi
pub use error_code::ErrorCode;
///IO result
pub type Result<T> = core::result::Result<T, ErrorCode>;

#[inline(always)]
///Alias to `std::io::Error::last_os_error()`
pub fn get_last_error() -> ErrorCode {
    ErrorCode::last_system()
}
