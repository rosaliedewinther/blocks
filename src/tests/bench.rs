#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_fib_10(b: &mut Bencher) {
    b.iter(|| {
        let mut var = 0u64;
        for _ in 0..100000000 {
            var += 10;
        }
        var
    });
}
