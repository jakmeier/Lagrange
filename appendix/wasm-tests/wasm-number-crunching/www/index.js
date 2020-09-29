import * as wasm from "wasm-number-crunching";

function modulo_add_range_js(a, b, c) {
    let acc = 0;
    for (let i = a; i < b; i++) {
        acc = (acc + i) % c;
    }
    return acc;
}
function modulo_add_range_js_bigint(a, b, c) {
    let acc = 0n;
    for (let i = a; i < b; i++) {
        acc = (acc + i) % c;
    }
    return acc;
}

function bench(a, b, c, debug = false) {
    if (debug) {
        console.log(`Running sum(${a},${b}) % ${c}`);
    }

    const beforeJs = performance.now();
    const resultJs = modulo_add_range_js(a, b, c);
    const afterJs = performance.now();

    const beforeJs2 = performance.now();
    const resultJs2 = modulo_add_range_js_bigint(BigInt(a), BigInt(b), BigInt(c));
    const afterJs2 = performance.now();

    const beforeWasm = performance.now();
    const result = wasm.modulo_add_range_bigint(BigInt(a), BigInt(b), BigInt(c));
    const afterWasm = performance.now();

    if (resultJs != result || BigInt(result) != resultJs2) {
        console.log("DIFF", resultJs, "=", resultJs2, "=", result);
    }

    return [afterJs - beforeJs,
    afterJs2 - beforeJs2,
    afterWasm - beforeWasm];
}

function benchSweep(rangeSizes, start, name) {
    console.log(name, "JS number");
    rangeSizes.forEach(
        (x) => {
            const end = start + x;
            const before = performance.now();
            const result = modulo_add_range_js(start, end, 7);
            const after = performance.now();
            console.log(x, after - before, start, end);
        }
    )

    console.log(name, "JS BigInt");
    rangeSizes.forEach(
        (x) => {
            const end = start + x;
            const before = performance.now();
            const result = modulo_add_range_js_bigint(BigInt(start), BigInt(end), BigInt(7));
            const after = performance.now();
            console.log(x, after - before, start, end);
        }
    )

    console.log(name, "WASM i64");
    rangeSizes.forEach(
        (x) => {
            const end = start + x;
            const before = performance.now();
            const result = wasm.modulo_add_range_bigint(BigInt(start), BigInt(end), BigInt(7));
            const after = performance.now();
            console.log(x, after - before, start, end);
        }
    )
}

const u32Max = 4294967295;
const jsMax = 1007199254740991;



const testpoints = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
];

benchSweep(testpoints, 1, "Small");
benchSweep(testpoints, u32Max, "\"Above u32\"");
benchSweep(testpoints, jsMax - testpoints[testpoints.length - 1], "\"At JS max\"");


// This was used at fist.
// The data was too stable to show meaningful errorbars.
// Thus I decided rto use lineplots instead and without errorbars.

// const N = 2;
// // const N = 100;
// const hundredMillion = 100000000;
// const small = [];
// const medium = [];
// const big = [];

// This way messes with the JIT for small numbers, only the first time is fast.
// for (let i = 0; i < N; i++) {
//     small.push(bench(1, hundredMillion, 7));
//     medium.push(bench(u32Max - hundredMillion, u32Max, 7));
//     big.push(bench(jsMax - hundredMillion, jsMax, 7));
// }