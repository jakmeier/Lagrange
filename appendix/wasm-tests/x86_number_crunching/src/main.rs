use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{Duration, Instant};
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut numbers = buf
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Not a number"));
    let a = numbers.next().expect("No range start");
    let b = numbers.next().expect("No range end");
    let c = numbers.next().expect("No modulo");
    let debug = numbers.next().unwrap_or(0) > 0;

    if debug {
        println!("Computing sum([{},{}]) % {} on one thread", a, b, c);
    }
    let single = single_threaded_test(a, b, c, debug);
    print!("{} ", single.as_micros());

    // let threads = 12;
    // if debug {
    //     println!(
    //         "Computing sum([{},{}]) % {} on {} threads",
    //         a, b, c, threads
    //     );
    // }
    // let multi = multi_threaded_test(a, b, c, threads, debug);
    // print!("{} ", multi.as_micros());

    // if debug {
    //     println!("Single threaded: {:?}", single);
    //     println!("{} threads: {:?}", threads, multi);
    // } else {
    //     print!("{} {}", single.as_millis(), multi.as_millis());
    // }
}

fn single_threaded_test(a: i64, b: i64, c: i64, debug: bool) -> Duration {
    let now = Instant::now();
    let result = modulo_add_range(a, b, c);
    let elapsed = now.elapsed();
    if debug {
        println!("Result is {}", result);
    }
    elapsed
}

fn multi_threaded_test(a: i64, b: i64, c: i64, threads: i64, debug: bool) -> Duration {
    let now = Instant::now();
    static ATOMIC_RESULT: AtomicI64 = AtomicI64::new(0);
    let step = (b - a) / threads;
    let mut handles = vec![];
    for i in 0..threads {
        let handle = std::thread::spawn(move || {
            let sub_a = a + i * step;
            let sub_b = if i < threads - 1 { sub_a + step } else { b };
            let partial_result = modulo_add_range(sub_a, sub_b, c);
            (ATOMIC_RESULT).fetch_add(partial_result, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().expect("Deadlock?");
    }
    let result = ATOMIC_RESULT.load(Ordering::Relaxed) % c;
    let elapsed = now.elapsed();
    if debug {
        println!("Result is {}", result);
    }
    elapsed
}

// fn add_range(a: i64, b: i64) -> i64 {
// (a..b).fold(0, std::ops::Add::add)
// }

fn modulo_add_range(a: i64, b: i64, c: i64) -> i64 {
    (a..b).fold(0, |a, b| (a + b) % c)
}
