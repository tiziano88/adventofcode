extern crate chrono;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use chrono::TimeZone;

fn main() {
    println!("Hello, world!");
}

#[test]
fn day_01_1() {
    let f = File::open("input/input_01").expect("could not open file");
    let reader = BufReader::new(f);
    let xs = reader.lines().map(|l| {
        l.expect("could not read line")
            .parse::<i32>()
            .expect("could not parse value")
    });
    assert_eq!(416, xs.sum())
}

#[test]
fn day_01_2() {
    let f = File::open("input/input_01").expect("could not open file");
    let reader = BufReader::new(f);
    let xs = reader
        .lines()
        .map(|l| {
            l.expect("could not read line")
                .parse::<i32>()
                .expect("could not parse value")
        })
        .collect::<Vec<_>>();
    let mut s = HashSet::new();
    let mut c = 0;
    for x in xs.iter().cycle() {
        c += x;
        if s.contains(&c) {
            break;
        }
        s.insert(c);
    }
    assert_eq!(56752, c);
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: String,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s1 = s.split(" @ ").collect::<Vec<_>>();
        let s2 = s1[1].split(": ").collect::<Vec<_>>();
        let s3 = s2[0].split(",").collect::<Vec<_>>();
        let s4 = s2[1].split("x").collect::<Vec<_>>();
        Ok(Claim {
            id: s1[0].to_string(),
            left: s3[0].parse::<u32>().unwrap(),
            top: s3[1].parse::<u32>().unwrap(),
            width: s4[0].parse::<u32>().unwrap(),
            height: s4[1].parse::<u32>().unwrap(),
        })
    }
}

#[test]
fn parse_claim() {
    let c = "#1 @ 2,3: 4x5"
        .parse::<Claim>()
        .expect("could not parse claim");
    assert_eq!(
        Claim {
            id: "#1".to_string(),
            left: 2,
            top: 3,
            width: 4,
            height: 5,
        },
        c
    );
}

#[test]
fn day_03_1() {
    let f = File::open("input/input_03").expect("could not open file");
    let reader = BufReader::new(f);
    let xs = reader
        .lines()
        .map(|l| {
            l.expect("could not read line")
                .parse::<Claim>()
                .expect("could not parse value")
        })
        .collect::<Vec<_>>();
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();
    for x in xs.iter() {
        for a in x.left..x.left + x.width {
            for b in x.top..x.top + x.height {
                map.entry((a, b)).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    let c = map.iter().filter(|(_, n)| **n >= 2).count();
    assert_eq!(121259, c);
}

#[test]
fn day_03_2() {
    let f = File::open("input/input_03").expect("could not open file");
    let reader = BufReader::new(f);
    let xs = reader
        .lines()
        .map(|l| {
            l.expect("could not read line")
                .parse::<Claim>()
                .expect("could not parse value")
        })
        .collect::<Vec<_>>();
    let mut map: HashMap<(u32, u32), HashSet<String>> = HashMap::new();
    for x in xs.iter() {
        for a in x.left..x.left + x.width {
            for b in x.top..x.top + x.height {
                map.entry((a, b))
                    .and_modify(|s| {
                        s.insert(x.id.clone());
                    })
                    .or_insert_with(|| {
                        let mut s = HashSet::new();
                        s.insert(x.id.clone());
                        s
                    });
            }
        }
    }
    let mut singles: HashSet<String> = HashSet::new();
    for (_, s) in map.iter() {}
    let (_, s) = map.iter()
        .filter(|(_, s)| s.len() == 1)
        .next()
        .expect("no claim found");

    // 474 too high
    // 32 no
    //assert_eq!("#801", s.iter().next().unwrap());
}

#[derive(Debug)]
enum Action {
    Begin(u32),
    Asleep,
    Awake,
}

#[derive(Debug)]
struct Record {
    time: chrono::DateTime<chrono::offset::Utc>,
    action: Action,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s1 = s.split(" ").collect::<Vec<_>>();
        let a = match s1[2] {
            "Guard" => Action::Begin(
                s1[3][1..]
                    .to_string()
                    .parse::<u32>()
                    .expect("could not parse id"),
            ),
            "falls" => Action::Asleep,
            "wakes" => Action::Awake,
            _ => panic!("invalid record"),
        };
        let year = s[1..5].parse::<i32>().expect("could not parse year");
        let month = s[6..8].parse::<u32>().expect("could not parse month");
        let day = s[9..11].parse::<u32>().expect("could not parse day");
        let hour = s[12..14].parse::<u32>().expect("could not parse hour");
        let minute = s[15..17].parse::<u32>().expect("could not parse minute");
        Ok(Record {
            time: chrono::Utc.ymd(year, month, day).and_hms(hour, minute, 0),
            action: a,
        })
    }
}

#[test]
fn day_04_1() {
    let f = File::open("input/input_04").expect("could not open file");
    let reader = BufReader::new(f);
    let mut xs = reader
        .lines()
        .map(|l| {
            l.expect("could not read line")
                .parse::<Record>()
                .expect("could not parse value")
        })
        .collect::<Vec<_>>();
    xs.sort_unstable_by_key(|x| x.time);

    let mut current_id = 0;
    let mut start_time = chrono::Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);

    let mut min_asleep: HashMap<u32, i64> = HashMap::new();
    for x in xs {
        match x.action {
            Action::Begin(id) => current_id = id.clone(),
            Action::Asleep => start_time = x.time,
            Action::Awake => {
                let delta_min = (x.time - start_time).num_minutes();
                min_asleep
                    .entry(current_id.clone())
                    .and_modify(|v| *v += delta_min)
                    .or_insert(delta_min);
            }
        }
    }

    let max_asleep_id = min_asleep.iter().max_by_key(|(_, &v)| v).expect("no max");
    assert_eq!(1823, *max_asleep_id.0);
}

//#[test]
fn day_09_1() {
    let mut v = VecDeque::new();
    v.push_back(0);
    let mut current = 0;
    print!("\n");
    for i in 1..10 {
        print!("cur: {}   ", current);
        for j in 0..v.len() {
            print!("{} ", v[j]);
        }
        let next = current + 1 % v.len();
        print!("next: {}   ", next);
        if next + 1 == v.len() {
            v.push_back(i);
        } else {
            v.insert(next, i);
        }
        current = next;

        print!("\n");
    }
    assert_eq!(1, 2);
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Record10 {
    pos: Vec2,
    vel: Vec2,
}

impl FromStr for Record10 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let px = s[10..16].trim().parse::<i32>().expect("could not parse px");
        let py = s[18..24].trim().parse::<i32>().expect("could not parse py");
        let vx = s[36..38].trim().parse::<i32>().expect("could not parse vx");
        let vy = s[40..42].trim().parse::<i32>().expect("could not parse vy");
        Ok(Record10 {
            pos: Vec2 { x: px, y: py },
            vel: Vec2 { x: vx, y: vy },
        })
    }
}

fn print_at_time(rs: &Vec<Record10>, t: i32) -> String {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();

    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();

    let mut set = HashSet::new();
    for r in rs.iter() {
        let x = r.pos.x + r.vel.x * t;
        let y = r.pos.y + r.vel.y * t;
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
        set.insert((x, y));
    }

    let mut ret = "".to_string();
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if set.contains(&(x, y)) {
                ret.push('#');
            } else {
                ret.push('.');
            }
        }
        ret.push('\n');
    }
    ret
}

fn find_delta_y(rs: &Vec<Record10>, t: i32) -> i32 {
    let mut min_y = 0;
    let mut max_y = 0;
    for r in rs.iter() {
        let x = r.pos.x + r.vel.x * t;
        let y = r.pos.y + r.vel.y * t;
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    max_y - min_y
}

#[test]
fn day_10_1() {
    let f = File::open("input/input_10").expect("could not open file");
    let reader = BufReader::new(f);
    let mut xs = reader
        .lines()
        .map(|l| {
            l.expect("could not read line")
                .parse::<Record10>()
                .expect("could not parse value")
        })
        .collect::<Vec<_>>();
    let t = (10000..15000)
        .min_by_key(|t| find_delta_y(&xs, *t))
        .expect("could not find min key");
    assert_eq!(10710, t);
    let p = print_at_time(&xs, t);
    assert_eq!(
        "\
######..#####.....##....#####...#....#..#....#.....###...####.
.....#..#....#...#..#...#....#..#....#..#....#......#...#....#
.....#..#....#..#....#..#....#...#..#....#..#.......#...#.....
....#...#....#..#....#..#....#...#..#....#..#.......#...#.....
...#....#####...#....#..#####.....##......##........#...#.....
..#.....#..#....######..#....#....##......##........#...#.....
.#......#...#...#....#..#....#...#..#....#..#.......#...#.....
#.......#...#...#....#..#....#...#..#....#..#...#...#...#.....
#.......#....#..#....#..#....#..#....#..#....#..#...#...#....#
######..#....#..#....#..#####...#....#..#....#...###.....####.
",
        p
    );
}

#[test]
fn day_22_1() {
    const DEPTH: u32 = 3879;
    let target = Vec2{x:8, y:713};
    let mut geologic_index: HashMap<Vec2, u32> = HashMap::new();
    fn erosion_level(i: u32) -> u32 {
        (i + DEPTH) % 20183
    }
    for x in 0..(target.x+1) {
        for y in 0..(target.y+1) {
            let index = if x == 0 && y == 0 {
                0u32
            } else if x == target.x && y ==  target.y {
                0u32
            } else if x == 0 {
                y as u32 * 48271
            } else if y == 0 {
                x as u32 * 16807
            } else {
                erosion_level(geologic_index[&Vec2{x:x-1, y}]) * erosion_level(geologic_index[&Vec2{x, y:y-1}])
            };
            let v = Vec2{x,y};
            geologic_index.insert(v, index);
        }
    }
    let mut total_risk = 0;
    for x in 0..(target.x+1) {
        for y in 0..(target.y+1) {
            total_risk += erosion_level(geologic_index[&Vec2{x, y}]) % 3
        }
    }
    assert_eq!(6323, total_risk);
}

#[derive(Debug)]
struct Record23 {
    pos: Vec3,
    radius: u32,
}

impl FromStr for Record23 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(|c: char| !(c.is_numeric() || c == '-'))
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
        Ok(Record23 {
            pos: Vec3 { x: v[0], y: v[1], z: v[2] },
            radius: v[3] as u32,
        })
    }
}

fn manhattan_distance(a: &Vec3, b: &Vec3) -> u32 {
    ((a.x-b.x).abs() + (a.y-b.y).abs() + (a.z-b.z).abs()) as u32
}

#[test]
fn day_23_1() {
    let f = File::open("input/input_23").expect("could not open file");
    let reader = BufReader::new(f);
    let mut xs = reader
        .lines()
        .map(|l| {
            l.expect("could not read line")
                .parse::<Record23>()
                .expect("could not parse value")
        })
        .collect::<Vec<_>>();
        println!("{:?}", xs);
    let max = xs.iter().max_by_key(|x| x.radius).expect("could not find maximum");
        println!("{:?}", max);
    let in_range = xs.iter().filter(|x| manhattan_distance(&x.pos, &max.pos) <= max.radius).count();
    assert_eq!(737, in_range);
}
