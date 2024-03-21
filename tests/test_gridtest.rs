fn test_vec_impl<T>(len: usize) {
    let _ = <Vec<T>>::with_capacity(len);
}

grid_gen::grid_gen_test!(
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
