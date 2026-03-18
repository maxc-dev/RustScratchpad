use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

mod atomics {
    
}

const ITERATIONS: u64 = 100_000_000;

#[repr(C)]
struct Counters {
    a: AtomicU64,
    b: AtomicU64,
}

// 64-byte aligned wrapper to isolate each counter
#[repr(align(64))]
struct PaddedAtomic(AtomicU64);

#[repr(C)]
struct CountersPadded {
    a: PaddedAtomic,
    b: PaddedAtomic,
}

fn pin_thread_to_core(core_id: usize) {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::System::Threading::{GetCurrentThread, SetThreadAffinityMask};
        let mask = 1 << core_id;
        let result = SetThreadAffinityMask(GetCurrentThread(), mask);
        if result == 0 {
            panic!("Failed to set thread affinity");
        }
    }
}

fn run_false_sharing() {
    let counters = Arc::new(Counters {
        a: AtomicU64::new(0),
        b: AtomicU64::new(0),
    });

    let start = Instant::now();

    let c1 = Arc::clone(&counters);
    let t1 = thread::spawn(move || {
        pin_thread_to_core(0);
        for _ in 0..ITERATIONS {
            c1.a.fetch_add(1, Ordering::Relaxed);
        }
    });

    let c2 = Arc::clone(&counters);
    let t2 = thread::spawn(move || {
        pin_thread_to_core(1);
        for _ in 0..ITERATIONS {
            c2.b.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let elapsed = start.elapsed();
    println!("False sharing time: {:?}", elapsed);
}

fn run_padded() {
    let counters = Arc::new(CountersPadded {
        a: PaddedAtomic(AtomicU64::new(0)),
        b: PaddedAtomic(AtomicU64::new(0)),
    });

    let start = Instant::now();

    let c1 = Arc::clone(&counters);
    let t1 = thread::spawn(move || {
        pin_thread_to_core(0);
        for _ in 0..ITERATIONS {
            c1.a.0.fetch_add(1, Ordering::Relaxed);
        }
    });

    let c2 = Arc::clone(&counters);
    let t2 = thread::spawn(move || {
        pin_thread_to_core(1);
        for _ in 0..ITERATIONS {
            c2.b.0.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let elapsed = start.elapsed();
    println!("Padded time: {:?}", elapsed);
}

fn main() {
    println!("Running false sharing benchmark...");
    run_false_sharing();

    println!("Running padded benchmark...");
    run_padded();
}