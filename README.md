# gridgen
A procedural macro to make it easier to write combinatorial tests and benches on 
arguments and generics.

Generics and arguments will be passed to the impl function in the order in which
they are defined in the procedural macro invocation.

```rust
fn test_vec_impl<T>(len: usize) {
    let _ = <Vec<T>>::with_capacity(len);
}

gen_grid::gen_grid_test!(
    test_vec_impl,
    test_vec,
    generic T: [
        u8,
        u16,
        u32,
        u64,
        usize,
    ],
    len: [
        0 => "empty",
        1 => "one",
        10000 => "max",
    ],
);

```
It will generate the following functions:
```
running 15 tests
test test_vec_u16_empty ... ok
test test_vec_u16_max ... ok
test test_vec_u32_max ... ok
test test_vec_u16_one ... ok
test test_vec_u32_empty ... ok
test test_vec_u32_one ... ok
test test_vec_u64_empty ... ok
test test_vec_u64_max ... ok
test test_vec_u8_empty ... ok
test test_vec_u64_one ... ok
test test_vec_u8_max ... ok
test test_vec_u8_one ... ok
test test_vec_usize_empty ... ok
test test_vec_usize_max ... ok
test test_vec_usize_one ... ok
```
which were expanded to:
```rust
#[test]
pub fn test_vec_u8_one() {
    test_vec_impl::<u8>(1);
}
#[test]
pub fn test_vec_u8_max() {
    test_vec_impl::<u8>(10000);
}
...
```

You can also generate benches:

```rust
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

gen_grid::gen_grid_bench!(
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

```
which will print:
```shell
$ cargo bench
running 25 tests
test bench_sum_u16_1      ... bench:           0 ns/iter (+/- 0)
test bench_sum_u16_1024   ... bench:         542 ns/iter (+/- 136)
test bench_sum_u16_64     ... bench:          33 ns/iter (+/- 5)
test bench_sum_u16_8      ... bench:           4 ns/iter (+/- 0)
test bench_sum_u16_8192   ... bench:       4,230 ns/iter (+/- 342)
test bench_sum_u32_1      ... bench:           0 ns/iter (+/- 0)
test bench_sum_u32_1024   ... bench:         546 ns/iter (+/- 58)
test bench_sum_u32_64     ... bench:          33 ns/iter (+/- 8)
test bench_sum_u32_8      ... bench:           4 ns/iter (+/- 0)
test bench_sum_u32_8192   ... bench:       4,243 ns/iter (+/- 320)
test bench_sum_u64_1      ... bench:           0 ns/iter (+/- 0)
test bench_sum_u64_1024   ... bench:         525 ns/iter (+/- 25)
test bench_sum_u64_64     ... bench:          32 ns/iter (+/- 4)
test bench_sum_u64_8      ... bench:           4 ns/iter (+/- 0)
test bench_sum_u64_8192   ... bench:       4,259 ns/iter (+/- 263)
test bench_sum_u8_1       ... bench:           0 ns/iter (+/- 0)
test bench_sum_u8_1024    ... bench:         553 ns/iter (+/- 84)
test bench_sum_u8_64      ... bench:          33 ns/iter (+/- 6)
test bench_sum_u8_8       ... bench:           4 ns/iter (+/- 0)
test bench_sum_u8_8192    ... bench:       4,503 ns/iter (+/- 895)
test bench_sum_usize_1    ... bench:           0 ns/iter (+/- 0)
test bench_sum_usize_1024 ... bench:         538 ns/iter (+/- 52)
test bench_sum_usize_64   ... bench:          33 ns/iter (+/- 4)
test bench_sum_usize_8    ... bench:           4 ns/iter (+/- 0)
test bench_sum_usize_8192 ... bench:       4,374 ns/iter (+/- 261)
```