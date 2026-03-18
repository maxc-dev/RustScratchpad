use std::hint::black_box;
use std::time::Instant;

const N: usize = 100_000_000;

#[inline(always)]
fn process(flag: bool, x: u64) -> u64 {
    if flag {
        x.wrapping_mul(3)
    } else {
        x.wrapping_mul(7)
    }
}

fn run(flags: &[bool]) -> u64 {
    let mut acc = 0u64;

    for i in 0..flags.len() {
        let f = unsafe { *flags.get_unchecked(i) };
        acc = acc.wrapping_add(process(f, i as u64));
    }

    black_box(acc)
}

fn main() {
    // Scenario 1: always true
    let flags_all_true = vec![true; N];

    // Scenario 2: 99% true
    let mut flags_99 = vec![true; N];
    for i in (0..N).step_by(100) {
        flags_99[i] = false;
    }

    // Scenario 3: 50/50
    let mut flags_half = vec![false; N];
    for i in 0..N {
        flags_half[i] = i % 2 == 0;
    }

    // Warmup
    run(&flags_all_true);

    let start = Instant::now();
    run(&flags_all_true);
    println!("All true: {:?}", start.elapsed());

    let start = Instant::now();
    run(&flags_99);
    println!("99% true: {:?}", start.elapsed());

    let start = Instant::now();
    run(&flags_half);
    println!("50/50: {:?}", start.elapsed());
}