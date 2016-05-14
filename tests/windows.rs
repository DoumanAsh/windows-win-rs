extern crate windows_win;

use windows_win::{
    get_windows_by_class,
    get_windows_by_title,
    get_window_by_pid,
    send_get_text,
    send_set_text,
    send_sys_command,
    is_window_visible,
    get_window_text,
    open_process,
    close_process,
    get_process_exe_path,
};

fn start_prog(name: &str) -> std::process::Child {
    let res = std::process::Command::new(name).spawn().unwrap();
    //Give a bit of time for window to appear
    sleep(100);

    res
}

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms))
}

#[test]
fn test_get_windows_by_class() {
    let result = get_windows_by_class("IME", None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.len() > 0);
}

#[test]
fn test_interact_notepad() {
    let mut notepad = start_prog("notepad");

    test_open_close(notepad.id());
    test_query_process_exe(notepad.id());
    test_get_windows_by_title(notepad.id());
    test_window_set_text_message(notepad.id());
    //This test should be last as it closes notepad
    test_window_sys_command_close(notepad.id());

    notepad.wait().expect("Failed to wait of closing");
}

fn test_query_process_exe(notepad_id: u32) {
    let result = open_process(notepad_id, 0x0400);
    assert!(result.is_ok());
    let notepad = result.unwrap();

    let result = get_process_exe_path(notepad);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.starts_with("C:\\Windows\\"));
    assert!(result.ends_with("\\notepad.exe"));

    let result = close_process(notepad);
    assert!(result.is_ok());
}

fn test_open_close(notepad_id: u32) {
    let result = open_process(notepad_id, 0x0038);
    assert!(result.is_ok());

    let result = close_process(result.unwrap());
    assert!(result.is_ok());
}

fn test_get_windows_by_title(notepad_id: u32) {
    let notepad_window = get_window_by_pid(notepad_id);
    assert!(notepad_window.is_ok());
    let notepad_window = notepad_window.unwrap();
    assert!(notepad_window.is_some());
    let notepad_window = notepad_window.unwrap();

    let result = send_get_text(notepad_window);
    assert!(result.is_some());
    let notepad_orig_title = result.unwrap();

    let result = get_windows_by_title(&notepad_orig_title, None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.len() > 0);
    let result = result[0];

    let result = get_window_text(result);
    assert!(result.is_ok());
    let result = result.unwrap();

    assert_eq!(notepad_orig_title, result);
}

fn test_window_set_text_message(notepad_id: u32) {
    let notepad_window = get_window_by_pid(notepad_id);
    assert!(notepad_window.is_ok());
    let notepad_window = notepad_window.unwrap();
    assert!(notepad_window.is_some());
    let notepad_window = notepad_window.unwrap();

    let result = send_get_text(notepad_window);
    assert!(result.is_some());
    let notepad_orig_title = result.unwrap();

    let new_title = "OLOLO notepad";
    let result = send_set_text(notepad_window, new_title);
    assert!(result);
    let result = send_get_text(notepad_window);
    assert!(result.is_some());
    let notepad_new_title = result.unwrap();

    assert!(notepad_new_title != notepad_orig_title);
    assert_eq!(notepad_new_title, new_title);
}

fn test_window_sys_command_close(notepad_id: u32) {
    let notepad_window = get_window_by_pid(notepad_id);
    assert!(notepad_window.is_ok());
    let notepad_window = notepad_window.unwrap();
    assert!(notepad_window.is_some());
    let notepad_window = notepad_window.unwrap();

    assert!(is_window_visible(notepad_window) == true);
    assert!(send_sys_command(notepad_window, 0xF060, 0));
    assert!(is_window_visible(notepad_window) == false);
}
