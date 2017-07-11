#[macro_use]
extern crate windows_win;

use windows_win::{
    raw
};

use raw::module::*;

#[test]
fn get_module_handle() {
    let result = get_module_handle_from_addr(module_to_addr!(get_module_handle));

    assert!(result.is_ok());

    let result = get_module_name(result.unwrap());

    assert!(result.is_ok());

    let result = result.unwrap();

    //Should be equal to current exe path
    let expected_path = std::env::current_exe().unwrap().into_os_string().into_string().unwrap();

    assert_eq!(result, expected_path);
}
