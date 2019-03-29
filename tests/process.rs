#[test]
fn test_is_eval() {
    //Let's hope we don't run in evaluated shell it :)
    assert!(!windows_win::raw::process::is_self_elevated());
}
