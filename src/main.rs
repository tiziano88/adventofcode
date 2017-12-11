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
        .map(|l| {
            l.split_whitespace().map(|c| c.parse().unwrap()).collect()
        })
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
        .map(|l| {
            l.split_whitespace().map(|c| c.parse().unwrap()).collect()
        })
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

fn day_3_1(input: usize) -> usize {
    const MAX_SIZE: usize = 1000;
    let mut cols = Vec::<Vec<usize>>::with_capacity(MAX_SIZE);
    for _ in 0..MAX_SIZE {
        let mut col = Vec::<usize>::with_capacity(MAX_SIZE);
        for _ in 0..MAX_SIZE {
            col.push(0);
        }
        cols.push(col);
    }

    struct Pos {
        x: i32,
        y: i32,
    }

    fn get(cols: &Vec<Vec<usize>>, pos: &Pos) -> usize {
        cols[(pos.x + MAX_SIZE as i32 / 2) as usize][(pos.y + MAX_SIZE as i32 / 2) as usize]
    }
    fn set(cols: &mut Vec<Vec<usize>>, pos: &Pos, v: usize) {
        cols[(pos.x + MAX_SIZE as i32 / 2) as usize][(pos.y + MAX_SIZE as i32 / 2) as usize] = v
    }

    // (dx, dy)
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut current_dir_index = 0;
    let mut current_pos: Pos = Pos { x: 0, y: 0 };

    set(&mut cols, &current_pos, 1);

    for i in 2..input + 1 {
        let current_dir = dirs[current_dir_index];
        current_pos.x += current_dir.0;
        current_pos.y += current_dir.1;

        set(&mut cols, &current_pos, i);

        let tentative_dir = dirs[(current_dir_index + 1) % dirs.len()];
        let tentative_pos = Pos {
            x: current_pos.x + tentative_dir.0,
            y: current_pos.y + tentative_dir.1,
        };

        if get(&cols, &tentative_pos) == 0 {
            current_dir_index = (current_dir_index + 1) % dirs.len();
        }
    }

    (current_pos.x.abs() + current_pos.y.abs()) as usize
}

#[test]
fn test_day_3_1() {
    assert_eq!(480, day_3_1(347991));
}

fn day_3_2(input: usize) -> usize {
    const MAX_SIZE: usize = 1000;
    let mut cols = Vec::<Vec<usize>>::with_capacity(MAX_SIZE);
    for _ in 0..MAX_SIZE {
        let mut col = Vec::<usize>::with_capacity(MAX_SIZE);
        for _ in 0..MAX_SIZE {
            col.push(0);
        }
        cols.push(col);
    }

    struct Pos {
        x: i32,
        y: i32,
    }

    fn get(cols: &Vec<Vec<usize>>, pos: &Pos) -> usize {
        cols[(pos.x + MAX_SIZE as i32 / 2) as usize][(pos.y + MAX_SIZE as i32 / 2) as usize]
    }
    fn sum_adjacent(cols: &Vec<Vec<usize>>, pos: &Pos) -> usize {
        let mut res = 0;
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                let pos2 = Pos {
                    x: pos.x + dx,
                    y: pos.y + dy,
                };
                res += get(cols, &pos2);
            }
        }
        res
    }
    fn set(cols: &mut Vec<Vec<usize>>, pos: &Pos, v: usize) {
        cols[(pos.x + MAX_SIZE as i32 / 2) as usize][(pos.y + MAX_SIZE as i32 / 2) as usize] = v
    }

    // (dx, dy)
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut current_dir_index = 0;
    let mut current_pos: Pos = Pos { x: 0, y: 0 };

    set(&mut cols, &current_pos, 1);

    loop {
        let current_dir = dirs[current_dir_index];
        current_pos.x += current_dir.0;
        current_pos.y += current_dir.1;

        let sum = sum_adjacent(&cols, &current_pos);
        set(&mut cols, &current_pos, sum);
        if sum > input {
            return sum;
        }

        let tentative_dir = dirs[(current_dir_index + 1) % dirs.len()];
        let tentative_pos = Pos {
            x: current_pos.x + tentative_dir.0,
            y: current_pos.y + tentative_dir.1,
        };

        if get(&cols, &tentative_pos) == 0 {
            current_dir_index = (current_dir_index + 1) % dirs.len();
        }
    }
}

#[test]
fn test_day_3_2() {
    assert_eq!(349975, day_3_2(347991));
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

fn day_7_1(input: &str) -> String {
    struct Node {
        parent: Option<String>,
    }

    let mut nodes = HashMap::<String, Node>::new();
    for l in input.lines() {
        let fields = l.split_whitespace().collect::<Vec<_>>();

        let parent = fields[0].to_string();
        if !nodes.contains_key(&parent) {
            nodes.insert(parent.clone(), Node { parent: None });
        }

        let children = fields.iter().skip(3).map(|f| f.replace(",", ""));
        for child in children {
            nodes.insert(
                child.clone(),
                Node {
                    parent: Some(parent.clone()),
                },
            );
        }
    }

    let mut current = nodes.keys().next().unwrap().clone();
    loop {
        match nodes[&current].parent {
            Some(ref n) => {
                current = n.clone();
            }
            None => {
                break;
            }
        }
    }

    current
}

#[test]
fn test_day_7_1() {
    let input = read_file_as_string("./input/day_7.txt");
    assert_eq!("cyrupz", &day_7_1(&input));
}

fn day_8_1(input: &str) -> i32 {
    struct Instruction {
        target: String,
        operation: String,
        amount: String,

        left: String,
        comparison: String,
        right: String,
    }

    impl std::str::FromStr for Instruction {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let fields = s.split_whitespace().collect::<Vec<_>>();

            Ok(Instruction {
                target: fields[0].to_string(),
                operation: fields[1].to_string(),
                amount: fields[2].to_string(),
                left: fields[4].to_string(),
                comparison: fields[5].to_string(),
                right: fields[6].to_string(),
            })
        }
    }

    let mut env = HashMap::<String, i32>::new();

    fn eval(env: &HashMap<String, i32>, s: &str) -> i32 {
        match s.parse::<i32>() {
            Ok(v) => v,
            Err(_) => *env.get(s).unwrap_or(&0),
        }
    }

    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap());
    for instruction in instructions {
        let eval_left = eval(&env, &instruction.left);
        let eval_right = eval(&env, &instruction.right);
        let ok = match instruction.comparison.as_ref() {
            "==" => eval_left == eval_right,
            "!=" => eval_left != eval_right,
            "<" => eval_left < eval_right,
            "<=" => eval_left <= eval_right,
            ">" => eval_left > eval_right,
            ">=" => eval_left >= eval_right,
            _ => false,
        };
        if ok {
            let cur = eval(&env, &instruction.target);
            let amount = instruction.amount.parse::<i32>().unwrap();
            let res = match instruction.operation.as_ref() {
                "inc" => cur + amount,
                "dec" => cur - amount,
                _ => cur,
            };
            env.insert(instruction.target, res);
        };
    }

    env.values().cloned().fold(i32::min_value(), i32::max)
}

#[test]
fn test_day_8_1() {
    let input = read_file_as_string("./input/day_8.txt");
    assert_eq!(5075, day_8_1(&input));
}

fn day_8_2(input: &str) -> i32 {
    struct Instruction {
        target: String,
        operation: String,
        amount: String,

        left: String,
        comparison: String,
        right: String,
    }

    impl std::str::FromStr for Instruction {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let fields = s.split_whitespace().collect::<Vec<_>>();

            Ok(Instruction {
                target: fields[0].to_string(),
                operation: fields[1].to_string(),
                amount: fields[2].to_string(),
                left: fields[4].to_string(),
                comparison: fields[5].to_string(),
                right: fields[6].to_string(),
            })
        }
    }

    let mut env = HashMap::<String, i32>::new();

    fn eval(env: &HashMap<String, i32>, s: &str) -> i32 {
        match s.parse::<i32>() {
            Ok(v) => v,
            Err(_) => *env.get(s).unwrap_or(&0),
        }
    }

    let mut max = i32::min_value();

    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap());
    for instruction in instructions {
        let eval_left = eval(&env, &instruction.left);
        let eval_right = eval(&env, &instruction.right);
        let ok = match instruction.comparison.as_ref() {
            "==" => eval_left == eval_right,
            "!=" => eval_left != eval_right,
            "<" => eval_left < eval_right,
            "<=" => eval_left <= eval_right,
            ">" => eval_left > eval_right,
            ">=" => eval_left >= eval_right,
            _ => false,
        };
        if ok {
            let cur = eval(&env, &instruction.target);
            let amount = instruction.amount.parse::<i32>().unwrap();
            let res = match instruction.operation.as_ref() {
                "inc" => cur + amount,
                "dec" => cur - amount,
                _ => cur,
            };
            if res > max {
                max = res
            }
            env.insert(instruction.target, res);
        };
    }

    max
}

#[test]
fn test_day_8_2() {
    let input = read_file_as_string("./input/day_8.txt");
    assert_eq!(7310, day_8_2(&input));
}

fn day_9_1(input: &str) -> usize {
    let mut score = 0;
    enum State {
        InGroup(usize),
        InGarbage(usize),
        Escape(usize),
    }
    let mut state = State::InGroup(0);
    for c in input.chars() {
        match state {
            State::InGroup(n) => match c {
                '{' => state = State::InGroup(n + 1),
                '}' => {
                    state = State::InGroup(n - 1);
                    score += n;
                }
                '<' => state = State::InGarbage(n),
                _ => (),
            },
            State::InGarbage(n) => match c {
                '!' => state = State::Escape(n),
                '>' => state = State::InGroup(n),
                _ => (),
            },
            State::Escape(n) => state = State::InGarbage(n),
        };
    }
    score
}

#[test]
fn test_day_9_1() {
    assert_eq!(1, day_9_1("{}"));
    assert_eq!(6, day_9_1("{{{}}}"));
    assert_eq!(5, day_9_1("{{}{}}"));
    assert_eq!(16, day_9_1("{{{},{},{{}}}}"));
    assert_eq!(1, day_9_1("{<a>,<a>,<a>,<a>}"));
    assert_eq!(9, day_9_1("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!(9, day_9_1("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!(3, day_9_1("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    let input = read_file_as_string("./input/day_9.txt");
    assert_eq!(14212, day_9_1(&input));
}

fn day_9_2(input: &str) -> usize {
    let mut score = 0;
    enum State {
        InGroup(usize),
        InGarbage(usize),
        Escape(usize),
    }
    let mut state = State::InGroup(0);
    for c in input.chars() {
        match state {
            State::InGroup(n) => match c {
                '{' => state = State::InGroup(n + 1),
                '}' => state = State::InGroup(n - 1),
                '<' => state = State::InGarbage(n),
                _ => (),
            },
            State::InGarbage(n) => match c {
                '!' => state = State::Escape(n),
                '>' => state = State::InGroup(n),
                _ => score += 1,
            },
            State::Escape(n) => state = State::InGarbage(n),
        };
    }
    score
}

#[test]
fn test_day_9_2() {
    let input = read_file_as_string("./input/day_9.txt");
    assert_eq!(6569, day_9_2(&input));
}

fn day_10_1(input: &str) -> usize {
    let mut current = (0..256).collect::<Vec<usize>>();
    let mut position = 0;
    let mut skip_size = 0;
    for l in input.trim().split(",").map(|v| v.parse::<usize>().unwrap()) {
        for s in 0..(l / 2) {
            let a = (position + s) % current.len();
            let b = (position + (l - 1 - s)) % current.len();
            let t = current[b];
            current[b] = current[a];
            current[a] = t;
        }
        position = (position + l + skip_size) % current.len();
        skip_size += 1;
    }
    current[0] * current[1]
}

#[test]
fn test_day_10_1() {
    let input = read_file_as_string("./input/day_10.txt");
    assert_eq!(6569, day_10_1(&input));
}

fn day_10_2(input: &str) -> String {
    let mut current = (0..256).collect::<Vec<usize>>();
    let mut position = 0;
    let mut skip_size = 0;
    let mut lengths = input.trim().bytes().map(|v| v as usize).collect::<Vec<_>>();
    lengths.extend([17, 31, 73, 47, 23].iter());

    for _ in 0..64 {
        for l in lengths.iter() {
            for s in 0..(l / 2) {
                let a = (position + s) % current.len();
                let b = (position + (l - 1 - s)) % current.len();
                let t = current[b];
                current[b] = current[a];
                current[a] = t;
            }
            position = (position + l + skip_size) % current.len();
            skip_size += 1;
        }
    }

    let mut out = String::new();
    for c in current.chunks(16).map(|xs| xs.iter().fold(0, |a, b| a ^ b)) {
        out += &format!("{:02x}", c);
    }
    out
}

#[test]
fn test_day_10_2() {
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", day_10_2(""));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", day_10_2("AoC 2017"));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", day_10_2("1,2,3"));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", day_10_2("1,2,4"));
    let input = read_file_as_string("./input/day_10.txt");
    assert_eq!("96de9657665675b51cd03f0b3528ba26", day_10_2(&input));
}

fn read_file_as_string(name: &str) -> String {
    let mut input = String::new();
    File::open(name)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}
