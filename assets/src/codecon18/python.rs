/* Iterator transformations in Python list comprehension style */
macro_rules! list_comprehension(
    ($r:expr; for $x:pat in $J:expr; if $pred:expr) => (
        ($J).filter_map(|$x| if $pred { Some($r) } else { None })
    );
    ($r:expr; for $x:pat in $J:expr) => (
        ($J).map(|$x| $r)
    )
);

fn p_decode(msg: &str, key: u8) -> String {
    list_comprehension![
        p_decode_digit(&msg[i..i+2], key); 
        for i in (0..msg.len()).step_by(2)
        ].collect()
}

fn p_decode_digit(s: &str, key: u8) -> char {
    (key ^ u8::from_str_radix(s, 16).unwrap()) as char
}

/*
fn p_decode_digit(s: &str, key: u8) -> char {
    (key ^ s.chars().fold(0, |acc, c| {
        16 * acc + c.to_digit(16).unwrap()
    }) as u8) as char
}/