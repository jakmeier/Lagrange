use std::io;
use std::io::prelude::*;

// Sample input: 023f6c386b3f39222820326b3f246b392428206b2a6b392332262e6b3f246b392428206b2a6b392332262e6b3f232a3f6c386b39222c233f6b24256b3f22262e

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input= line.unwrap();
        let best_guess = 
         (0..128).map(|key| { decode(&input, key) } )
          .max_by_key( |msg| { evaluate_credibility(msg) } );
    println!("{}", best_guess.unwrap());
    }
}

fn decode(input: &str, key: u8) -> String {
    input.chars()
        .map(|c| { c.to_digit(16).unwrap() } )
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| { ( chunk[0] << 4 | chunk[1] ) as u8 })
        .map(|character| { (character ^ key) as char} ) // XOR
        .collect()
}

fn evaluate_credibility(msg: &String) -> u64 {
    let mut score = 0;
    let words = msg.split(|c: char|{ c.is_whitespace() || c == '-'});
    for word in words {
        if word.chars().all(char::is_alphanumeric) {
            score += 5;
            if word.starts_with(char::is_uppercase) 
                && word.ends_with(char::is_lowercase)
            {
                score += 20;
            }
        }
    }
    score
}

/* Some helper function I used
use std::collections::BinaryHeap;

fn helper(input: &str) {
     (0..128).map(|key| { decode(&input, key) })
        .map(|msg| (evaluate_credibility(&msg), msg))
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
        .iter()
        .for_each(|(score, msg)| {println!("Score {}  {}", score, msg)});
}

*/