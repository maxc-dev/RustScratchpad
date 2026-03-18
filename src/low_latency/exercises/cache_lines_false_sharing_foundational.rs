use std::hint::black_box;
use std::thread;
use std::time::Instant;

mod atomics {
    
}

const ITERATIONS: u64 = 100_000_000;

#[repr(C)]
struct Counters {
    a: u64,
    b: u64,
}

#[repr(align(64))]
struct Padded64(u64);

#[repr(C)]
struct CountersPadded {
    a: u64,
    b: u64,
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
    let start = Instant::now();

    let mut c1 = Counters {
        a: 0,
        b: 0,
    };
    let t1 = thread::spawn(move || {
        pin_thread_to_core(0);
        for _ in 0..ITERATIONS {
            c1.a += 1;
        }
        black_box(c1);
    });

    let mut c2 = Counters {
        a: 0,
        b: 0,
    };
    let t2 = thread::spawn(move || {
        pin_thread_to_core(1);
        for _ in 0..ITERATIONS {
            c2.b += 1;
        }
        black_box(c2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let elapsed = start.elapsed();
    println!("False sharing time: {:?}", elapsed);
}

fn run_padded() {
    let start = Instant::now();

    let mut c1 = Counters {
        a: 0,
        b: 0,
    };
    let t1 = thread::spawn(move || {
        pin_thread_to_core(0);
        for _ in 0..ITERATIONS {
            c1.a += 1;
        }
        black_box(c1);
    });

    let mut c2 = Counters {
        a: 0,
        b: 0,
    };
    let t2 = thread::spawn(move || {
        pin_thread_to_core(1);
        for _ in 0..ITERATIONS {
            c2.b += 1;
        }
        black_box(c2);
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