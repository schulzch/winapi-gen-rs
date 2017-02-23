fn to_wide_chars(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
}

#[test]
fn hello_world() {
    let lp_text = to_wide_chars("Hello, world!");
    let lp_caption = to_wide_chars("MessageBox Example");
    unsafe {
        MessageBoxW(std::ptr::null_mut(),
                    lp_text.as_ptr(),
                    lp_caption.as_ptr(),
                    MB_OK | MB_ICONINFORMATION);
    }
}
