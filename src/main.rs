#![feature(slice_rotate)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn day1_1(input: &str) -> String {
    let chars: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    let chars1 = {
        let mut chars1 = chars.clone();
        chars1.rotate(1);
        chars1
    };

    let out: u32 = chars
        .iter()
        .zip(chars1.iter())
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a)
        .sum();

    format!("{:?}", out)
}

#[test]
fn test_day_1_1() {
    let input = read_file_as_string("./input/day_1.txt");
    assert_eq!("1203", &day1_1(&input));
}

fn day1_2(input: &str) -> String {
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

    format!("{:?}", out)
}

#[test]
fn test_day_1_2() {
    let input = read_file_as_string("./input/day_1.txt");
    assert_eq!("1146", &day1_2(&input));
}

fn day2_1(input: &str) -> String {
    let rows: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect();

    let res: u32 = rows.iter()
        .map(|r| {
            r.iter().cloned().fold(u32::min_value(), u32::max)
                - r.iter().cloned().fold(u32::max_value(), u32::min)
        })
        .sum();

    format!("{:?}", res)
}

#[test]
fn test_day_2_1() {
    let input = read_file_as_string("./input/day_2.txt");
    assert_eq!("48357", &day2_1(&input));
}

fn day2_2(input: &str) -> String {
    let rows: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect();

    let res: u32 = rows.iter()
        .map(|r| {
            for i in r.iter() {
                for j in r.iter() {
                    if i != j && i % j == 0 {
                        return i / j;
                    }
                }
            }
            // To make compiler happy.
            return 0;
        })
        .sum();

    format!("{:?}", res)
}

#[test]
fn test_day_2_2() {
    let input = read_file_as_string("./input/day_2.txt");
    assert_eq!("351", &day2_2(&input));
}

fn day_4_1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            l.split_whitespace().collect::<HashSet<_>>().len()
                == l.split_whitespace().collect::<Vec<_>>().len()
        })
        .count()
}

#[test]
fn test_day_4_1() {
    let input = read_file_as_string("./input/day_4.txt");
    assert_eq!(386, day_4_1(&input));
}

fn day_4_2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            l.split_whitespace()
                .map(|w| {
                    let mut c = w.bytes().collect::<Vec<_>>();
                    c.sort();
                    c
                })
                .collect::<HashSet<_>>()
                .len()
                == l.split_whitespace()
                    .map(|w| {
                        let mut c = w.bytes().collect::<Vec<_>>();
                        c.sort();
                        c
                    })
                    .collect::<Vec<_>>()
                    .len()
        })
        .count()
}

#[test]
fn test_day_4_2() {
    let input = read_file_as_string("./input/day_4.txt");
    assert_eq!(208, day_4_2(&input));
}

fn day_5_1(input: &str) -> usize {
    let mut instructions = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut n = 0;
    let mut i = 0i32;
    while i >= 0 && i < (instructions.len() as i32) {
        n += 1;
        let instruction = instructions[i as usize];
        instructions[i as usize] += 1;
        i += instruction;
    }
    n
}

#[test]
fn test_day_5_1() {
    let input = read_file_as_string("./input/day_5.txt");
    assert_eq!(318883, day_5_1(&input));
}

fn day_5_2(input: &str) -> usize {
    let mut instructions = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut n = 0;
    let mut i = 0i32;
    while i >= 0 && i < (instructions.len() as i32) {
        n += 1;
        let instruction = instructions[i as usize];
        if instruction >= 3 {
            instructions[i as usize] -= 1;
        } else {
            instructions[i as usize] += 1;
        }
        i += instruction;
    }
    n
}

#[test]
fn test_day_5_2() {
    let input = read_file_as_string("./input/day_5.txt");
    assert_eq!(23948711, day_5_2(&input));
}

fn day_6_1(input: &str) -> usize {
    let mut banks = input
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let num_banks = banks.len();

    let mut seen = HashSet::<Vec<usize>>::new();
    let mut n = 0;

    while !seen.contains(&banks) {
        seen.insert(banks.clone());
        n += 1;

        let mut max_index = 0usize;
        for i in 0..banks.len() {
            if banks[i] > banks[max_index] {
                max_index = i;
            }
        }
        let blocks = banks[max_index];
        banks[max_index] = 0;
        for i in 0..blocks {
            banks[(max_index + 1 + i) % num_banks] += 1;
        }
    }
    n
}

#[test]
fn test_day_6_1() {
    let input = read_file_as_string("./input/day_6.txt");
    assert_eq!(11137, day_6_1(&input));
}


fn day_6_2(input: &str) -> usize {
    let mut banks = input
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let num_banks = banks.len();

    let mut seen = HashMap::<Vec<usize>, usize>::new();
    let mut n = 0;

    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), n);
        n += 1;

        let mut max_index = 0usize;
        for i in 0..banks.len() {
            if banks[i] > banks[max_index] {
                max_index = i;
            }
        }
        let blocks = banks[max_index];
        banks[max_index] = 0;
        for i in 0..blocks {
            banks[(max_index + 1 + i) % num_banks] += 1;
        }
    }
    n - seen[&banks]
}

#[test]
fn test_day_6_2() {
    let input = read_file_as_string("./input/day_6.txt");
    assert_eq!(1037, day_6_2(&input));
}

fn read_file_as_string(name: &str) -> String {
    let mut input = String::new();
    File::open(name)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}
