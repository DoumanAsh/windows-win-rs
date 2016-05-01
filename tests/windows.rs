extern crate windows_win;

use windows_win::get_windows_by_class;

#[test]
fn test_get_windows_by_class() {
    let result = get_windows_by_class("IME", None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.len() > 0);
}
