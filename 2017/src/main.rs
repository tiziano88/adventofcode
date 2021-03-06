#![feature(conservative_impl_trait)]
#![feature(slice_rotate)]

use std::cell::Cell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::{Entry, RandomState};
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

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

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn day_3_1(input: usize) -> usize {
    let mut map = HashMap::<Vec2, usize>::new();

    fn get(map: &HashMap<Vec2, usize>, pos: &Vec2) -> usize {
        *map.get(pos).unwrap_or(&0)
    }
    fn set(map: &mut HashMap<Vec2, usize>, pos: &Vec2, v: usize) {
        map.insert(pos.clone(), v);
    }

    // (dx, dy)
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut current_dir_index = 0;
    let mut current_pos: Vec2 = Vec2 { x: 0, y: 0 };

    set(&mut map, &current_pos, 1);

    for i in 2..input + 1 {
        let current_dir = dirs[current_dir_index];
        current_pos.x += current_dir.0;
        current_pos.y += current_dir.1;

        set(&mut map, &current_pos, i);

        let tentative_dir = dirs[(current_dir_index + 1) % dirs.len()];
        let tentative_pos = Vec2 {
            x: current_pos.x + tentative_dir.0,
            y: current_pos.y + tentative_dir.1,
        };

        if get(&map, &tentative_pos) == 0 {
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
    let mut map = HashMap::<Vec2, usize>::new();

    fn get(map: &HashMap<Vec2, usize>, pos: &Vec2) -> usize {
        *map.get(pos).unwrap_or(&0)
    }
    fn set(map: &mut HashMap<Vec2, usize>, pos: &Vec2, v: usize) {
        map.insert(pos.clone(), v);
    }

    fn sum_adjacent(cols: &HashMap<Vec2, usize>, pos: &Vec2) -> usize {
        let mut res = 0;
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                let pos2 = Vec2 {
                    x: pos.x + dx,
                    y: pos.y + dy,
                };
                res += get(cols, &pos2);
            }
        }
        res
    }

    // (dx, dy)
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut current_dir_index = 0;
    let mut current_pos: Vec2 = Vec2 { x: 0, y: 0 };

    set(&mut map, &current_pos, 1);

    loop {
        let current_dir = dirs[current_dir_index];
        current_pos.x += current_dir.0;
        current_pos.y += current_dir.1;

        let sum = sum_adjacent(&map, &current_pos);
        set(&mut map, &current_pos, sum);
        if sum > input {
            return sum;
        }

        let tentative_dir = dirs[(current_dir_index + 1) % dirs.len()];
        let tentative_pos = Vec2 {
            x: current_pos.x + tentative_dir.0,
            y: current_pos.y + tentative_dir.1,
        };

        if get(&map, &tentative_pos) == 0 {
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
    let mut node_to_parent = HashMap::<String, String>::new();
    for l in input.lines() {
        let fields = l.split_whitespace().collect::<Vec<_>>();

        let parent = fields[0].to_string();

        let children = fields.iter().skip(3).map(|f| f.replace(",", ""));
        for child in children {
            node_to_parent.insert(child.clone(), parent.clone());
        }
    }

    let mut current = node_to_parent.keys().next().unwrap().clone();
    while let Some(n) = node_to_parent.get(&current) {
        current = n.clone();
    }

    current
}

#[test]
fn test_day_7_1() {
    let input = read_file_as_string("./input/day_7.txt");
    assert_eq!("cyrupz", &day_7_1(&input));
}

fn day_7_2(input: &str) -> String {
    let mut node_to_parent = HashMap::<String, String>::new();
    let mut node_to_weight = HashMap::<String, usize>::new();
    for l in input.lines() {
        let fields = l.split_whitespace().collect::<Vec<_>>();

        let parent = fields[0].to_string();

        let weight = fields[1]
            .replace("(", "")
            .replace(")", "")
            .parse::<usize>()
            .unwrap();
        node_to_weight.insert(parent.clone(), weight);

        let children = fields.iter().skip(3).map(|f| f.replace(",", ""));
        for child in children {
            node_to_parent.insert(child.clone(), parent.clone());
        }
    }

    fn children(node_to_parent: &HashMap<String, String>, node: &str) -> Vec<String> {
        node_to_parent
            .iter()
            .filter(|&(_, v)| v == node)
            .map(|(k, _)| k)
            .cloned()
            .collect()
    }

    fn total_weight(
        node_to_parent: &HashMap<String, String>,
        node_to_weight: &HashMap<String, usize>,
        node: &str,
    ) -> usize {
        let mut w = node_to_weight[node];
        for n in children(node_to_parent, &node) {
            w += total_weight(node_to_parent, node_to_weight, &n);
        }
        w
    }

    let mut current = node_to_parent.keys().next().unwrap().clone();
    while let Some(n) = node_to_parent.get(&current) {
        current = n.clone();
    }

    'outer: loop {
        let mut w = HashMap::<String, usize>::new();
        for c in children(&node_to_parent, &current) {
            w.insert(
                c.clone(),
                total_weight(&node_to_parent, &node_to_weight, &c),
            );
        }
        for n in w.values() {
            let ww = w.iter().filter(|&(k, v)| v == n).collect::<Vec<_>>();
            if ww.len() == 1 {
                current = ww[0].0.clone();
                continue 'outer;
            }
        }
        break 'outer;
    }

    println!(
        "t: {}",
        total_weight(&node_to_parent, &node_to_weight, &current)
    );

    current
}

#[test]
fn test_day_7_2() {
    let input = read_file_as_string("./input/day_7.txt");
    assert_eq!("cyrupz", &day_7_2(&input));
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
    assert_eq!(212, day_10_1(&input));
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

fn day_11_1(input: &str) -> usize {
    let mut x = 0isize;
    let mut y = 0isize;
    let mut z = 0isize;

    for d in input.trim().split(",") {
        match d.as_ref() {
            "n" => {
                y += 1;
                z -= 1;
            }
            "s" => {
                y -= 1;
                z += 1;
            }
            "ne" => {
                x += 1;
                z -= 1;
            }
            "sw" => {
                x -= 1;
                z += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            _ => {}
        }
    }

    std::cmp::max(std::cmp::max(x.abs(), y.abs()), z.abs()) as usize
}

#[test]
fn test_day_11_1() {
    assert_eq!(3, day_11_1("ne,ne,ne"));
    assert_eq!(0, day_11_1("ne,ne,sw,sw"));
    assert_eq!(2, day_11_1("ne,ne,s,s"));
    assert_eq!(3, day_11_1("se,sw,se,sw,sw"));
    let input = read_file_as_string("./input/day_11.txt");
    assert_eq!(796, day_11_1(&input));
}

fn day_11_2(input: &str) -> usize {
    let mut max = 0isize;

    let mut x = 0isize;
    let mut y = 0isize;
    let mut z = 0isize;

    for d in input.trim().split(",") {
        match d.as_ref() {
            "n" => {
                y += 1;
                z -= 1;
            }
            "s" => {
                y -= 1;
                z += 1;
            }
            "ne" => {
                x += 1;
                z -= 1;
            }
            "sw" => {
                x -= 1;
                z += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            _ => {}
        }
        max = std::cmp::max(max, x.abs());
        max = std::cmp::max(max, y.abs());
        max = std::cmp::max(max, z.abs());
    }

    max as usize
}

#[test]
fn test_day_11_2() {
    let input = read_file_as_string("./input/day_11.txt");
    assert_eq!(1585, day_11_2(&input));
}

fn day_12_1(input: &str) -> usize {
    let mut graph = HashMap::<usize, HashSet<usize>>::new();

    fn insert_edge(graph: &mut HashMap<usize, HashSet<usize>>, from: usize, to: usize) {
        graph
            .entry(from)
            .or_insert(HashSet::<usize>::new())
            .insert(to);
    }

    for l in input.lines() {
        let fields = l.split_whitespace().collect::<Vec<_>>();
        let left = fields[0].parse::<usize>().unwrap();
        let rights = fields
            .iter()
            .skip(2)
            .map(|f| f.replace(",", "").parse::<usize>().unwrap());
        for r in rights {
            insert_edge(&mut graph, left, r);
            insert_edge(&mut graph, r, left);
        }
    }

    let mut size = 0;

    let mut visited = HashSet::<usize>::new();
    let mut stack = Vec::<usize>::new();

    stack.push(0);

    while let Some(x) = stack.pop() {
        visited.insert(x);
        size += 1;
        if let Some(nodes) = graph.get(&x) {
            for node in nodes {
                if !visited.contains(node) {
                    stack.push(*node);
                }
            }
        }
    }

    size
}

#[test]
fn test_day_12_1() {
    //assert_eq!(
    //6,
    //day_12_1("0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5")
    //);
    let input = read_file_as_string("./input/day_12.txt");
    assert_eq!(130, day_12_1(&input));
}

fn day_12_2(input: &str) -> usize {
    let mut graph = HashMap::<usize, HashSet<usize>>::new();

    fn insert_edge(graph: &mut HashMap<usize, HashSet<usize>>, from: usize, to: usize) {
        graph
            .entry(from)
            .or_insert(HashSet::<usize>::new())
            .insert(to);
    }

    for l in input.lines() {
        let fields = l.split_whitespace().collect::<Vec<_>>();
        let left = fields[0].parse::<usize>().unwrap();
        let rights = fields
            .iter()
            .skip(2)
            .map(|f| f.replace(",", "").parse::<usize>().unwrap());
        for r in rights {
            insert_edge(&mut graph, left, r);
            insert_edge(&mut graph, r, left);
        }
    }

    let mut group = 0;

    let mut visited = HashSet::<usize>::new();
    let mut stack = Vec::<usize>::new();

    for k in graph.keys() {
        if visited.contains(k) {
            continue;
        }
        group += 1;
        stack.push(*k);
        while let Some(x) = stack.pop() {
            visited.insert(x);
            if let Some(nodes) = graph.get(&x) {
                for node in nodes {
                    if !visited.contains(node) {
                        stack.push(*node);
                    }
                }
            }
        }
    }

    group
}

#[test]
fn test_day_12_2() {
    let input = read_file_as_string("./input/day_12.txt");
    assert_eq!(189, day_12_2(&input));
}

fn day_13_1(input: &str) -> usize {
    let entries = input.lines().map(|l| {
        let vs = l.split(": ").collect::<Vec<_>>();
        (
            vs[0].parse::<usize>().unwrap(),
            vs[1].parse::<usize>().unwrap(),
        )
    });
    let m = HashMap::<_, _, RandomState>::from_iter(entries);
    let mut severity = 0usize;
    for step in 0..89 {
        if let Some(depth) = m.get(&step) {
            if step % ((depth - 1) * 2) == 0 {
                severity += step * depth;
            }
        }
    }
    severity
}

#[test]
fn test_day_13_1() {
    assert_eq!(24, day_13_1("0: 3\n1: 2\n4: 4\n6: 4"));
    let input = read_file_as_string("./input/day_13.txt");
    assert_eq!(1624, day_13_1(&input));
}

fn day_14_1(input: &str) -> usize {
    let hash = day_10_2;
    let mut res = 0;
    for i in 0..128 {
        let row = format!("{}-{}", input, i);
        let h = hash(&row);
        for c in h.chars() {
            let n = c.to_digit(16).unwrap();
            for i in 0..4 {
                if n & (1 << (3 - i)) != 0 {
                    res += 1;
                }
            }
        }
    }
    res
}

#[test]
fn test_day_14_1() {
    assert_eq!(8108, day_14_1("flqrgnkx"));
    assert_eq!(8194, day_14_1("uugsqrei"));
}

fn day_14_2(input: &str) -> usize {
    let hash = day_10_2;
    let mut map = HashMap::<(isize, isize), usize>::new();
    for y in 0..128 {
        let row = format!("{}-{}", input, y);
        let h = hash(&row);
        let mut x = 0;
        for c in h.chars() {
            let n = c.to_digit(16).unwrap();
            for i in 0..4 {
                if n & (1 << (3 - i)) != 0 {
                    map.insert((x, y), 0);
                }
                x += 1;
            }
        }
    }

    let mut stack = Vec::<(isize, isize)>::new();
    let mut region = 0;
    for y in 0..128isize {
        for x in 0..128isize {
            if map.get(&(x, y)).is_none() {
                continue;
            }
            if map[&(x, y)] != 0 {
                continue;
            }
            region += 1;
            stack.push((x, y));
            while let Some((x, y)) = stack.pop() {
                if let Entry::Occupied(mut e) = map.entry((x, y)) {
                    if *e.get() == 0 {
                        e.insert(region);
                        stack.extend(&[(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)]);
                    };
                };
            }
        }
    }

    region
}

#[test]
fn test_day_14_2() {
    assert_eq!(1242, day_14_2("flqrgnkx"));
    assert_eq!(1141, day_14_2("uugsqrei"));
}

fn day_15_1(start_a: usize, start_b: usize) -> usize {
    struct Generator {
        factor: usize,
        modulo: usize,

        current: usize,
    }

    impl Iterator for Generator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.current = self.current * self.factor % self.modulo;
            Some(self.current)
        }
    }

    let generator_a = Generator {
        factor: 16807,
        modulo: 2147483647,
        current: start_a,
    };
    let generator_b = Generator {
        factor: 48271,
        modulo: 2147483647,
        current: start_b,
    };

    fn hash(v: usize) -> usize {
        v & ((1 << 16) - 1)
    }

    let stream_a = generator_a.take(40_000_000).map(hash);
    let stream_b = generator_b.take(40_000_000).map(hash);
    stream_a.zip(stream_b).filter(|&(a, b)| a == b).count()
}

fn day_15_1_for(start_a: usize, start_b: usize) -> usize {
    let mut a = start_a;
    let mut b = start_b;
    let mut res = 0;
    let modulo = 2147483647;
    for _ in 0..40_000_000 {
        a = a * 16807 % modulo;
        b = b * 48271 % modulo;
        if a & ((1 << 16) - 1) == b & ((1 << 16) - 1) {
            res += 1;
        }
    }
    res
}

#[test]
fn test_day_15_1() {
    assert_eq!(592, day_15_1(277, 349));
}

fn day_15_2(start_a: usize, start_b: usize) -> usize {
    struct Generator {
        factor: usize,
        modulo: usize,

        current: usize,
    }

    impl Iterator for Generator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.current = self.current * self.factor % self.modulo;
            Some(self.current)
        }
    }

    let generator_a = Generator {
        factor: 16807,
        modulo: 2147483647,
        current: start_a,
    };
    let generator_b = Generator {
        factor: 48271,
        modulo: 2147483647,
        current: start_b,
    };

    fn hash(v: usize) -> usize {
        v & ((1 << 16) - 1)
    }

    fn divisible_by(v: usize, n: usize) -> bool {
        v % n == 0
    }

    let stream_a = generator_a
        .filter(|&a| divisible_by(a, 4))
        .take(5_000_000)
        .map(hash);
    let stream_b = generator_b
        .filter(|&a| divisible_by(a, 8))
        .take(5_000_000)
        .map(hash);
    stream_a.zip(stream_b).filter(|&(a, b)| a == b).count()
}

#[test]
fn test_day_15_2() {
    assert_eq!(320, day_15_2(277, 349));
}

fn day_16_1(input: &str) -> String {
    let mut current = [b' '; 16].iter().cloned().collect::<Vec<_>>();
    let len = current.len();
    for i in 0..16 {
        current[i] = b'a' + i as u8;
    }

    for m in input.trim().split(",") {
        let cs = m.chars().collect::<Vec<_>>();
        match cs[0] {
            's' => {
                let p = cs.iter().skip(1).collect::<String>();
                let x = p.parse::<usize>().unwrap();
                current.rotate(len - x as usize);
            }
            'x' => {
                let p = cs.iter().skip(1).collect::<String>();
                let d = p.split("/").collect::<Vec<_>>();
                let a = d[0].parse::<usize>().unwrap();
                let b = d[1].parse::<usize>().unwrap();
                current.swap(a, b);
            }
            'p' => {
                let p = cs.iter().skip(1).collect::<String>();
                let d = p.chars().collect::<Vec<_>>();
                let a = current.iter().position(|&c| c == d[0] as u8).unwrap();
                let b = current.iter().position(|&c| c == d[2] as u8).unwrap();
                current.swap(a, b);
            }
            _ => {}
        }
    }

    String::from_utf8(current).unwrap()
}

#[test]
fn test_day_16_1() {
    let input = read_file_as_string("./input/day_16.txt");
    assert_eq!("hmefajngplkidocb", &day_16_1(&input));
}

fn day_16_2(input: &str) -> String {
    let mut current = [b' '; 16].iter().cloned().collect::<Vec<_>>();
    let len = current.len();
    for i in 0..16 {
        current[i] = b'a' + i as u8;
    }

    let initial = current.clone();

    let mut iter = 0;
    while iter < 1_000_000_000 {
        for m in input.trim().split(",") {
            let cs = m.chars().collect::<Vec<_>>();
            match cs[0] {
                's' => {
                    let p = cs.iter().skip(1).collect::<String>();
                    let x = p.parse::<usize>().unwrap();
                    current.rotate(len - x as usize);
                }
                'x' => {
                    let p = cs.iter().skip(1).collect::<String>();
                    let d = p.split("/").collect::<Vec<_>>();
                    let a = d[0].parse::<usize>().unwrap();
                    let b = d[1].parse::<usize>().unwrap();
                    current.swap(a, b);
                }
                'p' => {
                    let p = cs.iter().skip(1).collect::<String>();
                    let d = p.chars().collect::<Vec<_>>();
                    let a = current.iter().position(|&c| c == d[0] as u8).unwrap();
                    let b = current.iter().position(|&c| c == d[2] as u8).unwrap();
                    current.swap(a, b);
                }
                _ => {}
            }
        }
        iter += 1;
        if current == initial {
            iter = iter * (1_000_000_000 / iter);
        }
    }

    String::from_utf8(current).unwrap()
}

#[test]
fn test_day_16_2() {
    let input = read_file_as_string("./input/day_16.txt");
    assert_eq!("fbidepghmjklcnoa", &day_16_2(&input));
}

fn day_17_1(input: usize) -> usize {
    let mut current = vec![0];
    let mut position = 0;
    for i in 1..2018 {
        position = (position + input) % current.len();
        let c2 = current.clone();
        current = Vec::new();
        current.extend(c2.get(0..(position + 1)).unwrap());
        current.push(i);
        current.extend(c2.get((position + 1)..c2.len()).unwrap());
        position = (position + 1) % current.len();
    }
    position = (position + 1) % current.len();
    current[position]
}

#[test]
fn test_day_17_1() {
    assert_eq!(638, day_17_1(3));
    assert_eq!(1311, day_17_1(371));
}

fn day_17_2(input: usize) -> usize {
    let mut current = vec![0];
    let mut position = 0;
    for i in 1..50_000_001 {
        position = (position + input) % current.len();
        let c2 = current.clone();
        current = Vec::new();
        current.extend(c2.get(0..(position + 1)).unwrap());
        current.push(i);
        current.extend(c2.get((position + 1)..c2.len()).unwrap());
        position = (position + 1) % current.len();
    }
    position = (position + 1) % current.len();
    current[position]
}

#[test]
fn test_day_17_2() {
    //assert_eq!(1311, day_17_2(371));
}

fn day_18_1(input: &str) -> isize {
    let mut registers = HashMap::<String, isize>::new();
    for i in 0..26 {
        registers.insert(format!("{}", char::from(b'a' + i)), 0);
    }

    let mut freq = 0;

    fn eval(registers: &HashMap<String, isize>, v: &str) -> isize {
        match v.parse::<isize>() {
            Ok(v) => v,
            Err(_) => registers[v],
        }
    }

    let instructions = input.lines().collect::<Vec<_>>();

    let mut pc = 0i32;
    while let Some(instruction) = instructions.get(pc as usize) {
        let parts = instruction.split_whitespace().collect::<Vec<_>>();
        match parts[0].as_ref() {
            "snd" => {
                let x = eval(&registers, parts[1]);
                freq = x;
                pc += 1;
            }
            "set" => {
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), y);
                pc += 1;
            }
            "add" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x + y);
                pc += 1;
            }
            "mul" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x * y);
                pc += 1;
            }
            "mod" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x % y);
                pc += 1;
            }
            "rcv" => {
                let x = eval(&registers, parts[1]);
                if x != 0 {
                    break;
                } else {
                    pc += 1;
                }
            }
            "jgz" => {
                let x = eval(&registers, parts[1]);
                if x > 0 {
                    pc += parts[2].parse::<i32>().unwrap();
                } else {
                    pc += 1;
                }
            }
            _ => panic!("error"),
        }
    }

    freq
}

#[test]
fn test_day_18_1() {
    let input = read_file_as_string("./input/day_18.txt");
    assert_eq!(8600, day_18_1(&input));
}

fn day_18_2(input: &str) -> isize {
    let mut registers_0 = HashMap::<String, isize>::new();
    for i in 0..26 {
        registers_0.insert(format!("{}", char::from(b'a' + i)), 0);
    }
    registers_0.insert("p".to_string(), 0);

    let mut registers_1 = HashMap::<String, isize>::new();
    for i in 0..26 {
        registers_1.insert(format!("{}", char::from(b'a' + i)), 0);
    }
    registers_0.insert("p".to_string(), 1);

    fn eval(registers: &HashMap<String, isize>, v: &str) -> isize {
        match v.parse::<isize>() {
            Ok(v) => v,
            Err(_) => registers[v],
        }
    }

    let instructions = input.lines().collect::<Vec<_>>();

    let mut pc_0 = Cell::new(0isize);
    let mut pc_1 = Cell::new(0isize);

    let mut send_0 = Cell::new(0isize);
    let mut send_1 = Cell::new(0isize);

    let mut queue_0 = VecDeque::new();
    let mut queue_1 = VecDeque::new();

    fn exec(
        registers: &mut HashMap<String, isize>,
        instruction: &str,
        queue_in: &mut VecDeque<isize>,
        queue_out: &mut VecDeque<isize>,
        pc: &mut Cell<isize>,
        send: &mut Cell<isize>,
    ) {
        let parts = instruction.split_whitespace().collect::<Vec<_>>();
        match parts[0].as_ref() {
            "snd" => {
                let x = eval(&registers, parts[1]);
                queue_out.push_front(x);
                pc.set(pc.get() + 1);
                send.set(send.get() + 1);
            }
            "set" => {
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), y);
                pc.set(pc.get() + 1);
            }
            "add" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x + y);
                pc.set(pc.get() + 1);
            }
            "mul" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x * y);
                pc.set(pc.get() + 1);
            }
            "mod" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x % y);
                pc.set(pc.get() + 1);
            }
            "rcv" => if let Some(v) = queue_in.pop_back() {
                registers.insert(parts[1].to_string(), v);
                pc.set(pc.get() + 1);
            },
            "jgz" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                if x > 0 {
                    pc.set(pc.get() + y);
                } else {
                    pc.set(pc.get() + 1);
                }
            }
            _ => panic!("error"),
        }
    }

    while let (Some(instruction_0), Some(instruction_1)) = (
        instructions.get(pc_0.get() as usize),
        instructions.get(pc_1.get() as usize),
    ) {
        exec(
            &mut registers_0,
            instruction_0,
            &mut queue_0,
            &mut queue_1,
            &mut pc_0,
            &mut send_0,
        );
        exec(
            &mut registers_1,
            instruction_1,
            &mut queue_1,
            &mut queue_0,
            &mut pc_1,
            &mut send_1,
        );
        if instruction_0.split_whitespace().next().unwrap() == "rcv"
            && instruction_1.split_whitespace().next().unwrap() == "rcv"
            && queue_0.is_empty() && queue_1.is_empty()
        {
            println!("DEADLOCK");
            break;
        }
    }

    send_0.get()
}

#[test]
fn test_day_18_2() {
    let input = read_file_as_string("./input/day_18.txt");
    assert_eq!(7239, day_18_2(&input));
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

fn day_20_1(input: &str) -> usize {
    fn parse_vec(i: &str) -> Vec3 {
        let i = i.split(|c| c == '<' || c == '>').collect::<Vec<_>>();
        let vs = i[1].split(",").collect::<Vec<_>>();
        Vec3 {
            x: vs[0].parse::<i64>().unwrap(),
            y: vs[1].parse::<i64>().unwrap(),
            z: vs[2].parse::<i64>().unwrap(),
        }
    }

    fn particle_over(p: &Particle) -> bool {
        p.p.x.signum() == p.v.x.signum() && p.v.x.signum() == p.a.x.signum()
            && p.p.y.signum() == p.v.y.signum() && p.v.y.signum() == p.a.y.signum()
            && p.p.z.signum() == p.v.z.signum() && p.v.z.signum() == p.a.z.signum()
    }

    fn particle_distance(p: &Particle) -> usize {
        (p.p.x.abs() + p.p.y.abs() + p.p.z.abs()) as usize
    }

    fn particle_step(p: &mut Particle) {
        p.v.x += p.a.x;
        p.v.y += p.a.y;
        p.v.z += p.a.z;

        p.p.x += p.v.x;
        p.p.y += p.v.y;
        p.p.z += p.v.z;
    }

    let mut particles = input
        .lines()
        .map(|l| {
            let s = l.split(", ").collect::<Vec<_>>();
            Particle {
                p: parse_vec(s[0]),
                v: parse_vec(s[1]),
                a: parse_vec(s[2]),
            }
        })
        .collect::<Vec<_>>();

    for i in 0..1_000_000 {
        for mut p in particles.iter_mut() {
            particle_step(&mut p);
        }
    }

    let mut min = 0;
    for i in 0..particles.len() {
        if particle_distance(&particles[i]) < particle_distance(&particles[min]) {
            min = i;
        }
    }

    min
}

#[test]
fn test_day_20_1() {
    let input = read_file_as_string("./input/day_20.txt");
    assert_eq!(91, day_20_1(&input));
}

fn day_20_2(input: &str) -> usize {
    fn parse_vec(i: &str) -> Vec3 {
        let i = i.split(|c| c == '<' || c == '>').collect::<Vec<_>>();
        let vs = i[1].split(",").collect::<Vec<_>>();
        Vec3 {
            x: vs[0].parse::<i64>().unwrap(),
            y: vs[1].parse::<i64>().unwrap(),
            z: vs[2].parse::<i64>().unwrap(),
        }
    }

    fn particle_over(p: &Particle) -> bool {
        p.p.x.signum() == p.v.x.signum() && p.v.x.signum() == p.a.x.signum()
            && p.p.y.signum() == p.v.y.signum() && p.v.y.signum() == p.a.y.signum()
            && p.p.z.signum() == p.v.z.signum() && p.v.z.signum() == p.a.z.signum()
    }

    fn particle_distance(p: &Particle) -> usize {
        (p.p.x.abs() + p.p.y.abs() + p.p.z.abs()) as usize
    }

    fn particle_step(p: &mut Particle) {
        p.v.x += p.a.x;
        p.v.y += p.a.y;
        p.v.z += p.a.z;

        p.p.x += p.v.x;
        p.p.y += p.v.y;
        p.p.z += p.v.z;
    }

    let mut particles = input
        .lines()
        .map(|l| {
            let s = l.split(", ").collect::<Vec<_>>();
            Particle {
                p: parse_vec(s[0]),
                v: parse_vec(s[1]),
                a: parse_vec(s[2]),
            }
        })
        .collect::<Vec<_>>();

    for _ in 0..1_000_000 {
        let mut pos = HashMap::<Vec3, HashSet<usize>>::new();
        for i in 0..particles.len() {
            let mut p = &mut particles[i];
            particle_step(p);
            let mut v = pos.entry(p.p.clone()).or_insert(HashSet::<usize>::new());
            v.insert(i);
        }
        {
            let mut rem = pos.values()
                .filter(|p| p.len() >= 2)
                .flat_map(|p| p)
                .collect::<Vec<_>>();
            rem.sort();
            rem.reverse();
            for p in rem {
                particles.remove(*p);
            }
        }
    }

    particles.len()
}

#[test]
fn test_day_20_2() {
    let input = read_file_as_string("./input/day_20.txt");
    assert_eq!(567, day_20_2(&input));
}

fn day_21_1(input: &str) -> usize {
    let mut map = HashMap::<Vec2, bool>::new();

    map.insert(Vec2 { x: 0, y: 0 }, false);
    map.insert(Vec2 { x: 1, y: 0 }, true);
    map.insert(Vec2 { x: 2, y: 0 }, false);
    map.insert(Vec2 { x: 0, y: 1 }, false);
    map.insert(Vec2 { x: 1, y: 1 }, false);
    map.insert(Vec2 { x: 2, y: 1 }, true);
    map.insert(Vec2 { x: 0, y: 2 }, true);
    map.insert(Vec2 { x: 1, y: 2 }, true);
    map.insert(Vec2 { x: 2, y: 2 }, true);

    fn get_square(map: &HashMap<Vec2, bool>, start: Vec2, size: usize) -> HashMap<Vec2, bool> {
        let mut out = HashMap::<Vec2, bool>::new();
        for dx in 0..size {
            let x = start.x + (dx as i32);
            for dy in 0..size {
                let y = start.y + (dy as i32);
                out.insert(
                    Vec2 {
                        x: dx as i32,
                        y: dy as i32,
                    },
                    *map.get(&Vec2 { x: x, y: y }).unwrap_or(&false),
                );
            }
        }
        out
    }

    0
}

#[test]
fn test_day_21_1() {
    let input = read_file_as_string("./input/day_21.txt");
    assert_eq!(567, day_21_1(&input));
}

fn day_23_1(input: &str) -> usize {
    let mut registers = HashMap::<String, isize>::new();
    registers.insert("a".to_string(), 0);
    registers.insert("b".to_string(), 0);
    registers.insert("c".to_string(), 0);
    registers.insert("d".to_string(), 0);
    registers.insert("e".to_string(), 0);
    registers.insert("f".to_string(), 0);
    registers.insert("g".to_string(), 0);
    registers.insert("h".to_string(), 0);

    fn eval(registers: &HashMap<String, isize>, v: &str) -> isize {
        match v.parse::<isize>() {
            Ok(v) => v,
            Err(_) => registers[v],
        }
    }

    let instructions = input.lines().collect::<Vec<_>>();

    let mut count = 0;

    let mut pc = 0i32;
    while let Some(instruction) = instructions.get(pc as usize) {
        let parts = instruction.split_whitespace().collect::<Vec<_>>();
        match parts[0].as_ref() {
            "set" => {
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), y);
                pc += 1;
            }
            "sub" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x - y);
                pc += 1;
            }
            "mul" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x * y);
                count += 1;
                pc += 1;
            }
            "jnz" => {
                let x = eval(&registers, parts[1]);
                if x != 0 {
                    pc += parts[2].parse::<i32>().unwrap();
                } else {
                    pc += 1;
                }
            }
            _ => panic!("error"),
        }
    }

    count
}

#[test]
fn test_day_23_1() {
    let input = read_file_as_string("./input/day_23.txt");
    assert_eq!(6241, day_23_1(&input));
}

fn day_23_2(input: &str) -> isize {
    let mut registers = HashMap::<String, isize>::new();
    registers.insert("a".to_string(), 1);
    registers.insert("b".to_string(), 0);
    registers.insert("c".to_string(), 0);
    registers.insert("d".to_string(), 0);
    registers.insert("e".to_string(), 0);
    registers.insert("f".to_string(), 0);
    registers.insert("g".to_string(), 0);
    registers.insert("h".to_string(), 0);

    fn eval(registers: &HashMap<String, isize>, v: &str) -> isize {
        match v.parse::<isize>() {
            Ok(v) => v,
            Err(_) => registers[v],
        }
    }

    let instructions = input.lines().collect::<Vec<_>>();

    let mut pc = 0i32;
    while let Some(instruction) = instructions.get(pc as usize) {
        let parts = instruction.split_whitespace().collect::<Vec<_>>();
        match parts[0].as_ref() {
            "set" => {
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), y);
                pc += 1;
                if parts[1] == "h" {
                    println!("set {} {}", parts[1], y);
                }
            }
            "sub" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x - y);
                pc += 1;
            }
            "mul" => {
                let x = eval(&registers, parts[1]);
                let y = eval(&registers, parts[2]);
                registers.insert(parts[1].to_string(), x * y);
                pc += 1;
            }
            "jnz" => {
                let x = eval(&registers, parts[1]);
                if x != 0 {
                    pc += parts[2].parse::<i32>().unwrap();
                } else {
                    pc += 1;
                }
            }
            _ => panic!("error"),
        }
    }

    registers["h"]
}

#[test]
fn test_day_23_2() {
    let input = read_file_as_string("./input/day_23.txt");
    assert_eq!(6241, day_23_2(&input));
}

fn read_file_as_string(name: &str) -> String {
    let mut input = String::new();
    File::open(name)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}
