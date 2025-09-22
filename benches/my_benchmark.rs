#![feature(ip)]

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::net::{Ipv4Addr};
use std::num::NonZero;

fn is_documentation(addr: Ipv4Addr) -> bool {
    addr.is_documentation()
}

 fn to_errno(s: &[u8]) -> Option<NonZero<i32>> {
    match s {
        b"NoMemory" => NonZero::new(1),
        b"PermissionDenied" => NonZero::new(2),
        b"ConnectionRefused" => NonZero::new(3),
        b"ConnectionReset" => NonZero::new(4),
        b"HostUnreachable" => NonZero::new(5),
        b"NetworkUnreachable" => NonZero::new(6),
        b"TimedOut" => NonZero::new(7),
        b"Interrupted" => NonZero::new(8),
        b"InvalidInput" => NonZero::new(9),
        b"WouldBlock" => NonZero::new(10),
        b"NotConnected" => NonZero::new(11),
        b"AddrInUse" => NonZero::new(12),
        _ => None,
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let addr_v4 = Ipv4Addr::new(192, 0, 2, 1);
    let errno = b"InvalidInput";
    c.bench_function("Ipv4Addr::is_documentation", |b| {
        b.iter(|| is_documentation(black_box(addr_v4)))
    });
    c.bench_function("to_errno", |b| {
        b.iter(|| to_errno(black_box(errno)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
