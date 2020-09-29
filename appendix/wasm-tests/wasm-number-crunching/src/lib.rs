mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn modulo_add_range(a: i32, b: i32, c: i32) -> i32 {
    (a..b).fold(0, |a, b| (a + b) % c)
}

#[wasm_bindgen]
pub fn modulo_add_range_bigint(a: i64, b: i64, c: i64) -> i64 {
    (a..b).fold(0, |a, b| (a + b) % c)
}

#[wasm_bindgen]
pub fn add_range_bigint(a: i64, b: i64) -> i64 {
    (a..b).fold(0, std::ops::Add::add)
}

// RUNTIME ERROR: 
// panicked at 'failed to spawn thread: Custom { kind: Other, error: "operation not supported on wasm yet" }'

// use std::sync::atomic::{AtomicI64, Ordering};
// 
// #[wasm_bindgen]
// pub fn modulo_add_range_bigint_multi_threaded(a: i64, b: i64, c: i64, threads: i64) -> i64 {
//     utils::set_panic_hook();
//     static ATOMIC_RESULT: AtomicI64 = AtomicI64::new(0);
//     let step = (b - a) / threads;
//     let mut handles = vec![];
//     for i in 0..threads {
//         let handle = std::thread::spawn(move || {
//             let sub_a = a + i * step;
//             let sub_b = if i < threads - 1 { sub_a + step } else { b };
//             let partial_result = modulo_add_range_bigint(sub_a, sub_b, c);
//             (ATOMIC_RESULT).fetch_add(partial_result, Ordering::Relaxed);
//         });
//         handles.push(handle);
//     }
//     for h in handles {
//         h.join().expect("Deadlock?");
//     }
//     let result = ATOMIC_RESULT.load(Ordering::Relaxed) % c;
//     result
// }