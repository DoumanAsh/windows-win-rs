use windows_win::sys::{AddClipboardFormatListener, SetLastErrorEx};

use clipboard_win::set_clipboard_string;

use windows_win::Window;

use windows_win::raw::window::{
    get_by_class,
    get_by_title,
    get_by_pid,
    is_visible,
    get_text,
    send_get_text,
    send_set_text,
    send_sys_command,
    Builder,
    destroy
};

use windows_win::raw::process::{
    open,
    close,
    get_exe_path,
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
    let result = get_by_class("IME", None);
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
    test_get_window_by_pid_after_error(notepad.id());
    test_window_set_text_message(notepad.id());
    //This test should be last as it closes notepad
    test_window_sys_command_close(notepad.id());

    notepad.wait().expect("Failed to wait for notepad to close");
}

fn test_query_process_exe(notepad_id: u32) {
    let result = open(notepad_id, 0x0400);
    assert!(result.is_ok());
    let notepad = result.unwrap();

    let result = get_exe_path(notepad);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.starts_with("C:\\Windows\\"));
    assert!(result.ends_with("\\notepad.exe"));

    let result = close(notepad);
    assert!(result.is_ok());
}

fn test_open_close(notepad_id: u32) {
    let result = open(notepad_id, 0x0038);
    assert!(result.is_ok());

    let result = close(result.unwrap());
    assert!(result.is_ok());
}

fn test_get_windows_by_title(notepad_id: u32) {
    let notepad_window = get_by_pid(notepad_id);
    assert!(notepad_window.is_ok());
    let notepad_window = notepad_window.unwrap();
    assert!(notepad_window.is_some());
    let notepad_window = notepad_window.unwrap();

    let result = send_get_text(notepad_window);
    assert!(result.is_some());
    let notepad_orig_title = result.unwrap();

    let result = get_by_title(&notepad_orig_title, None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.len() > 0);
    let result = result[0];

    let result = get_text(result);
    assert!(result.is_ok());
    let result = result.unwrap();

    assert_eq!(notepad_orig_title, result);
}

fn test_get_window_by_pid_after_error(notepad_id: u32) {
    unsafe { SetLastErrorEx(5, 0) };
    let notepad_window = get_by_pid(notepad_id);
    assert!(notepad_window.is_ok());
}

fn test_window_set_text_message(notepad_id: u32) {
    let notepad_window = get_by_pid(notepad_id);
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
    let notepad_window = get_by_pid(notepad_id);
    assert!(notepad_window.is_ok());
    let notepad_window = notepad_window.unwrap();
    assert!(notepad_window.is_some());
    let notepad_window = notepad_window.unwrap();

    assert!(is_visible(notepad_window) == true);
    assert!(send_sys_command(notepad_window, 0xF060, 0));
    assert!(is_visible(notepad_window) == false);
}

#[cfg(target_env="msvc")]
#[test]
fn test_window_create() {
    let window = Window::from_builder(Builder::new().class_name("BUTTON").parent_message());
    assert!(window.is_ok());
    let window = window.unwrap();

    unsafe { AddClipboardFormatListener(window.inner()); }

    assert!(set_clipboard_string("Test").is_ok());
    let msg = windows_win::Messages::new().window(Some(window.inner())).next();
    assert!(msg.is_some());
    let msg = msg.unwrap();
    assert!(msg.is_ok());
    let msg = msg.unwrap();

    assert_eq!(msg.id(), 797); //Clipboard update

    assert!(destroy(window.into()));
}

#[test]
fn test_window_create_dummy() {
    let window = Builder::new().class_name("BUTTON").create();
    assert!(window.is_ok());
    let window = window.unwrap();
    assert!(destroy(window));
}

#[test]
fn check_enum_by_with_last_error_will_not_fail() {
    unsafe {
        SetLastErrorEx(1, 0)
    }

    let result = windows_win::raw::window::enum_by_until(None, |_| {
        0
    });

    assert!(result.is_ok());

    let result = windows_win::raw::window::enum_by_until(None, |_| {
        1
    });

    assert!(result.is_ok());
}
