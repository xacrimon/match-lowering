use std::{num::NonZero, ffi::CStr};

const STANDARD_BASE: &[u8] = b"org.freedesktop.DBus.Error.";

pub fn to_errno(s: &CStr) -> Option<NonZero<i32>> {
    let (pfx, sfx) = s.to_bytes().split_at_checked(STANDARD_BASE.len())?;
    match sfx {
        b"NoMemory" => NonZero::new(1),
        _ => None,
    }
}
