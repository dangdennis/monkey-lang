use libc::{c_char, geteuid, getpwuid, passwd};
use std::ffi::CStr;

pub struct User {
    uid: u32,
    username: String,
}

impl User {
    pub fn current() -> Result<Self, String> {
        unsafe {
            let uid = geteuid();

            let pw: *mut passwd = getpwuid(uid);
            if pw.is_null() {
                return Err("Failed to get user information.".to_string());
            }

            // Get the username from the password entry
            let username = CStr::from_ptr((*pw).pw_name as *const c_char)
                .to_string_lossy()
                .into_owned();

            Ok(User { uid, username })
        }
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}
