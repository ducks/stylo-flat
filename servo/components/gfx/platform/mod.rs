/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(any(target_os = "linux", target_os = "android", all(target_os = "windows", target_env = "gnu")))]
pub use platform::freetype::{font, font_context, font_list, font_template};

#[cfg(target_os = "macos")]
pub use platform::macos::{font, font_context, font_list, font_template};

#[cfg(all(target_os = "windows", target_env = "msvc"))]
pub use platform::dummy::{font, font_context, font_list, font_template};

#[cfg(any(target_os = "linux", target_os = "android", all(target_os = "windows", target_env = "gnu")))]
mod freetype {
    use libc::c_char;
    use std::ffi::CStr;
    use std::str;

    /// Creates a String from the given null-terminated buffer.
    /// Panics if the buffer does not contain UTF-8.
    unsafe fn c_str_to_string(s: *const c_char) -> String {
        str::from_utf8(CStr::from_ptr(s).to_bytes()).unwrap().to_owned()
    }

    pub mod font;
    pub mod font_context;
    pub mod font_list;
    pub mod font_template;
}

#[cfg(target_os = "macos")]
mod macos {
    pub mod font;
    pub mod font_context;
    pub mod font_list;
    pub mod font_template;
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
mod dummy {
    pub mod font;
    pub mod font_context;
    pub mod font_list;
    pub mod font_template;
}