/*
    You are processing 10 million fixed-size messages per second. Each message is exactly 32 bytes.

    Rewrite this function so that:
        It performs zero heap allocations
        It does not copy the message
        It compiles to optimal code in --release
        It introduces no bounds checks in the hot loop
 */
mod cache_lines_false_sharing_atomics_foundational;

pub fn _process_messages(input: &[[u8; 32]]) -> u64 {
    let mut total = 0;

    for msg in input {
        let v = msg.to_vec();
        total += v[0] as u64;
    }

    total
}

pub fn process_messages(input: &[[u8; 32]]) -> u64 {
    let mut total = 0;

    for msg in input {
        total += msg[0] as u64;
    }

    total
}

fn main() {
    use std::time::Instant;

    const N: usize = 10_000_000;

    // Allocate once
    let mut data = Vec::with_capacity(N);

    for i in 0..N {
        let mut msg = [0u8; 32];
        msg[0] = (i % 256) as u8;
        data.push(msg);
    }

    // Warmup
    let _ = process_messages(&data);

    let start = Instant::now();
    let result = process_messages(&data);
    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("Elapsed: {:?}", elapsed);
    println!(
        "Throughput: {:.2} M msgs/sec",
        N as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );
}