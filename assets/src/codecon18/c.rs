#![feature(libc)]
extern crate libc;

fn c_decode(input: &str, key: u8) -> String {
    let mut output = String::new();
    for i in 0..(input.len()/2) {
        let hexdigit = &input[2*i..2*i+2]; 
        let character = u8::from_str_radix(hexdigit, 16).unwrap();
        output.push((character ^ key) as char);
    }
    output
}

/*
fn c_decode(input: &str, key: u8) -> String {
    let mut output = String::new();
    for i in 0..(input.len()/2) {
        let hexdigit = &input[2*i..2*i+2]; 
        let character = libc_strtol(&hexdigit, 16) as u8;
        output.push((character ^ key) as char);
    }
    output
}

fn strtol(s: &str, radix: u32 ) -> u64 {
    s.chars().fold(0, |acc, c| {
        (radix as u64 * acc) + c.to_digit(radix).unwrap() as u64
    })
}
*/

use std::os::raw::c_char;
fn clib_decode(input: &str, key: u8) -> String {
    let input_vec = input.as_bytes();
    let mut output = String::new();
    for i in 0..(input_vec.len()/2) {
        let hexdigit = vec![input_vec[2*i], input_vec[2*i+1], 0]; 
        let character = unsafe {
            ffi::strtol(hexdigit.as_ptr() as *const c_char, ptr::null_mut(), 16) as u8
        };
        output.push((character ^ key) as char);
    }
    output
}

// Declaration of the external function strtol in libc
mod ffi {
    use std::ffi::CString;
    use std::os::raw::c_char;
    extern {
        pub fn strtol(s: *const c_char, endptr: *const *mut c_char, base: u32 ) -> u64;
    }
}