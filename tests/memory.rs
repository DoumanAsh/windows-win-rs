extern crate windows_win;

use std::ptr;

use windows_win::{
    raw
};

use raw::memory::*;

#[test]
fn query() {
    let handle = raw::process::get_current_handle();
    let result = virtual_query_ex(handle, ptr::null());

    assert!(result.is_ok());

    let result = result.unwrap();
    //Well, most likely it should be free
    assert!(result.is_free());
}
