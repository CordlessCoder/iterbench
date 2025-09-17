use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use iterbench::{cpp_max_ptr, rust_max_ptr};
use std::{fmt::Display, hint::black_box};

type CFunc = unsafe extern "C" fn(*const i32, usize) -> i32;

fn teardown<T: Display, CB>((_, val): (CB, T)) {
    println!("Computed {val}")
}

fn setup(func: CFunc, len: usize) -> impl FnMut() -> i32 {
    let input: Vec<i32> = (0..len as i32).collect();
    move || unsafe { func(black_box(input.as_ptr()), black_box(input.len())) }
}

#[library_benchmark]
#[bench::short(args = (cpp_max_ptr, 10), setup = setup, teardown = teardown)]
#[bench::long(args = (cpp_max_ptr, 50), setup = setup, teardown = teardown)]
fn bench_cpp<T: Display, CB: FnMut() -> T>(mut callback: CB) -> (CB, T) {
    let max = callback();
    (callback, max)
}

#[library_benchmark]
#[bench::short(args = (rust_max_ptr, 10), setup = setup, teardown = teardown)]
#[bench::long(args = (rust_max_ptr, 50), setup = setup, teardown = teardown)]
fn bench_rust<T: Display, CB: FnMut() -> T>(mut callback: CB) -> (CB, T) {
    let max = black_box(callback());
    (callback, max)
}

library_benchmark_group!(
    name = bench_group;
    compare_by_id = true;
    benchmarks =
    bench_cpp,
    bench_rust,
);

main!(library_benchmark_groups = bench_group);
