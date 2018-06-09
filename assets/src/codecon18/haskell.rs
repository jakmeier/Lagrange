fn h_decode(input: &[u8], key: u8) -> String {
    match input {
        &[.., a, b] => push(h_decode(&input[..input.len()-2], key), (key ^ (16*a + b)) as char),
        _ => String::new()
    }
}

fn push(mut s: String, c: char) -> String {
    s.push(c);
    s
}

fn decode_h_wrapper(input: &str, key: u8) -> String {
    h_decode(input.chars()
    .map(|c| { c.to_digit(16).unwrap() as u8})
    .collect::<Vec<_>>().as_slice(), key)
}