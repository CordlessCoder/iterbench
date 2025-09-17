use std::time::Instant;

use iterbench::expand_iota_views;

fn main() {
    for _ in 0..50 {
        let input: Vec<i32> = (0..50).collect();

        let sample_result: Vec<i32> = expand_iota_views(&input).collect();
        println!("Rust Result count: {}", sample_result.len());

        let start = Instant::now();
        let mut total_count = 0;
        for _ in 0..1000 {
            let result = expand_iota_views(&input);
            total_count += result.max().expect("Always nonempty");
        }
        let duration = start.elapsed();

        println!("Rust Total count (1000 iterations): {}", total_count);
        println!("Rust Total time: {} microseconds", duration.as_micros());
        println!(
            "Rust Average per iteration: {:.2} microseconds",
            duration.as_micros() as f64 / 1000.0
        );
    }
}
