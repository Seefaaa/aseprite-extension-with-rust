use libc::{FILE, c_char};

/// Provided to `lua-src` (aseprite's lua fork declares these as external,
/// normally supplied by the embedding app) so the static archive links
/// without needing local patches.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_user_fopen(fname: *const c_char, mode: *const c_char) -> *mut FILE {
    unsafe { imp::user_fopen(fname, mode) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lua_user_freopen(
    fname: *const c_char,
    mode: *const c_char,
    stream: *mut FILE,
) -> *mut FILE {
    unsafe { imp::user_freopen(fname, mode, stream) }
}

#[cfg(unix)]
mod imp {
    use super::*;

    pub unsafe fn user_fopen(fname: *const c_char, mode: *const c_char) -> *mut FILE {
        unsafe { libc::fopen(fname, mode) }
    }

    pub unsafe fn user_freopen(
        fname: *const c_char,
        mode: *const c_char,
        stream: *mut FILE,
    ) -> *mut FILE {
        unsafe { libc::freopen(fname, mode, stream) }
    }
}

#[cfg(windows)]
mod imp {
    use super::*;
    use std::ffi::CStr;

    // fopen()/freopen() on the Windows CRT decode the byte string using the
    // active ANSI codepage, so non-ASCII paths (e.g. a non-Latin Windows
    // username) fail to open. Aseprite's own lua_user_fopen works around
    // this with the wide (UTF-16) entry points; mirror that here.
    unsafe extern "C" {
        fn _wfopen(filename: *const u16, mode: *const u16) -> *mut FILE;
        fn _wfreopen(filename: *const u16, mode: *const u16, stream: *mut FILE) -> *mut FILE;
    }

    fn to_utf16(s: *const c_char) -> Vec<u16> {
        let bytes = unsafe { CStr::from_ptr(s) }.to_bytes();
        String::from_utf8_lossy(bytes)
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect()
    }

    pub unsafe fn user_fopen(fname: *const c_char, mode: *const c_char) -> *mut FILE {
        let wfname = to_utf16(fname);
        let wmode = to_utf16(mode);
        unsafe { _wfopen(wfname.as_ptr(), wmode.as_ptr()) }
    }

    pub unsafe fn user_freopen(
        fname: *const c_char,
        mode: *const c_char,
        stream: *mut FILE,
    ) -> *mut FILE {
        let wfname = to_utf16(fname);
        let wmode = to_utf16(mode);
        unsafe { _wfreopen(wfname.as_ptr(), wmode.as_ptr(), stream) }
    }
}
