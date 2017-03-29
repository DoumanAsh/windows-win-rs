extern crate windows_win;

use windows_win::{
    raw
};

use raw::file::*;

#[test]
fn try_search() {
    let result = search("non-existing.rs", FileInfoLevel::default(), FileSearchType::default(), 0);

    assert!(result.is_err());

    let error = result.err().unwrap();
    println!("error={:?}", error);
}

#[test]
fn search_and_find() {
    let result = search(file!(), FileInfoLevel::default(), FileSearchType::default(), 0);

    assert!(result.is_ok());

    let (handle, _) = result.unwrap().unwrap();

    assert!(search_close(handle).is_ok());
}

#[test]
fn search_few_rs() {
    let path = std::path::Path::new(file!()).parent().unwrap().join("*.rs");
    let result = search(&path, FileInfoLevel::default(), FileSearchType::default(), 0);

    assert!(result.is_ok());

    let (handle, _) = result.unwrap().unwrap();

    let result = search_next(handle);

    assert!(result.is_ok());

    assert!(search_close(handle).is_ok());
}
