use std::hint;
use std::net::Ipv4Addr;
use std::num::NonZero;

#[inline(never)]
pub fn to_errno(sfx: &[u8]) -> Option<NonZero<i32>> {
    match sfx {
        b"NoMemory" => NonZero::new(1),
        _ => None,
    }
}

#[inline(never)]
pub fn is_documentation(addr: Ipv4Addr) -> bool {
    addr.is_documentation()
}

fn main() {
    assert_eq!(to_errno(b"NoMemory"), NonZero::new(1));
    assert_eq!(to_errno(b"OtherError"), None);
    assert!(is_documentation(hint::black_box(Ipv4Addr::new(
        192, 0, 2, 1
    ))));
}
