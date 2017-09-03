//! FFI helpers.

use std::ffi::CStr;

/// Converts str to CStr using stack allocated slice.
///
/// Will panic if this slice is too small.
pub fn str_to_cstr<'r>(bytes: &'r mut [u8], s: &str) -> &'r CStr {
    let num_bytes = s.bytes().len();
    if num_bytes > bytes.len() - 1 {
        panic!("cstr candidate too long");
    }

    for (i, byte) in s.bytes().enumerate() {
        bytes[i] = byte;
    }
    bytes[num_bytes] = '\0' as u8;
    unsafe { CStr::from_bytes_with_nul_unchecked(&bytes[..num_bytes + 1]) }
}

/// Converts [u8] to CStr using stack allocated slice.
///
/// Will panic if this slice is too small.
pub fn bytes_to_cstr<'r>(bytes: &'r mut [u8], s: &[u8]) -> &'r CStr {
    let num_bytes = s.len();
    if num_bytes > bytes.len() - 1 {
        panic!("[u8] candidate too long");
    }

    for (i, byte) in s.iter().enumerate() {
        bytes[i] = *byte;
    }
    bytes[num_bytes] = '\0' as u8;
    unsafe { CStr::from_bytes_with_nul_unchecked(&bytes[..num_bytes + 1]) }
}
