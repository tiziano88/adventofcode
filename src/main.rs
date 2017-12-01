#![feature(slice_rotate)]

use std::io::{self, Read};

type Run = Fn(&str) -> Result<String, String>;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input).unwrap();

    let d: &Run = match std::env::args().nth(1).unwrap().as_ref() {
        "day1" => &day1,
        _ => &err,
    };

    match d(&input) {
        Ok(o) => print!("{}", o),
        Err(e) => print!("ERROR: {}", e),
    }
}

fn err(input: &str) -> Result<String, String> {
    Err("Invalid day".to_string())
}

fn day1(input: &str) -> Result<String, String> {
    let chars: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    let chars1 = {
        let mut chars1 = chars.clone();
        chars1.rotate(chars.len() / 2);
        chars1
    };

    let out: u32 = chars
        .iter()
        .zip(chars1.iter())
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a)
        .sum();

    Ok(format!("{}", out))
}
