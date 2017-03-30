extern crate windows_win;

use windows_win::{
    raw
};

use raw::file::*;

#[test]
fn try_search() {
    let result = Search::new("non-existing.rs", FileInfoLevel::default(), FileSearchType::default(), 0);

    assert!(result.is_err());
}

#[test]
fn search_self() {
    let file_name = file!();
    let result = Search::new(file_name, FileInfoLevel::default(), FileSearchType::default(), 0);

    assert!(result.is_ok());

    let (mut search, entry) = result.unwrap();
    let name = entry.name();

    assert!(file_name.ends_with(name.as_os_str().to_str().unwrap()));
    assert!(entry.is_file());

    assert!(search.next().is_none());
}

#[test]
fn search_few_rs() {
    let path = std::path::Path::new(file!()).parent().unwrap().join("*.rs");
    let result = Search::new(&path, FileInfoLevel::default(), FileSearchType::default(), 0);

    assert!(result.is_ok());

    let (search, entry) = result.unwrap();

    assert!(entry.is_file());
    for entry in search {
        assert!(entry.unwrap().is_file());
    }
}
