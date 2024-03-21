#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

fn bench_sum_impl<T>(b: &mut Bencher, len: usize) {
    b.iter(|| {
        let mut res = 0;
        for i in 0..len {
            res += black_box(i);
        }
        black_box(res)
    });
}

grid_gen::grid_gen_bench!(
    bench_sum_impl,  // the function to call
    bench_sum, // the prefix of the generated tests names
    generic T: [
        u8,
        u16,
        u32,
        u64,
        usize,
    ],
    len: [
        1,
        8,
        64,
        1024,
        8192,
    ],
);
