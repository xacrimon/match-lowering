use std::{num::NonZero, ffi::CStr};

#[inline(never)]
pub fn to_errno(sfx: &[u8]) -> Option<NonZero<i32>> {
    match sfx {
        b"NoMemory" => NonZero::new(1),
        _ => None,
    }
}

fn main() {
    assert_eq!(to_errno(b"NoMemory"), NonZero::new(1));
    assert_eq!(to_errno(b"OtherError"), None);
}
