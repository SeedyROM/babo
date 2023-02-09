use std::ffi::CString;

use snafu::Snafu;

/// Creates a null-terminated C string with the specified length.
#[inline(always)]
pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

#[derive(Debug, Snafu)]
#[snafu(display("Graphics error \"{}\" (code {}): {}", method, code, message))]
pub struct GlError {
    pub method: String,
    pub code: u32,
    pub message: String,
}

/// Create a gl error string from the error code.
#[inline(always)]
pub fn gl_error_string(error: u32) -> &'static str {
    match error {
        gl::INVALID_ENUM => "Invalid enum",
        gl::INVALID_VALUE => "Invalid value",
        gl::INVALID_OPERATION => "Invalid operation",
        gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid framebuffer operation",
        gl::OUT_OF_MEMORY => "Out of memory",
        _ => "Unknown error",
    }
}

#[macro_export]
/// Call an OpenGL function and check for errors.
macro_rules! gl {
    ( $func:tt, $($arg:tt)* ) => {{
        use $crate::utils::{GlError, gl_error_string};

        unsafe {
            gl::$func($($arg)*);
            let err = unsafe { gl::GetError() };
            if err != gl::NO_ERROR {
                let reason = gl_error_string(err);
                Err(GlError { method: "$func".to_string(), code: err, message: reason.to_string() })
            } else {
                Ok(())
            }
        }
    }};
}
