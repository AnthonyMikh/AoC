use num_integer::Integer;

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse_seq(s: &str) -> Vec<Self> {
        s.as_bytes()
            .iter()
            .map(|&b| match b {
                b'L' => Self::Left,
                b'R' => Self::Right,
                _ => panic!("invalid direction {:?}", char::from(b)),
            })
            .collect()
    }

    fn iterate(dirs: &[Self]) -> impl FnMut() -> Self + '_ {
        let mut it = dirs.iter().copied().cycle();
        move || it.next().unwrap()
    }
}

struct Fork<T> {
    enter: T,
    left: T,
    right: T,
}

impl Fork<String> {
    fn parse(s: &str) -> Self {
        let (enter, paths) = s.split_once(" = (").unwrap();
        let (left, right) = paths.strip_suffix(")").unwrap().split_once(", ").unwrap();
        Self {
            enter: enter.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }
}

const START: &str = "AAA";
const END: &str = "ZZZ";
pub type Forks = HashMap<String, [String; 2]>;

fn parse_forks(input: &str) -> Forks {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let Fork { enter, left, right } = Fork::parse(l);
            (enter, [left, right])
        })
        .collect::<HashMap<_, _>>()
}

pub fn parse_input(input: &str) -> (Forks, Vec<Direction>) {
    let input = input.trim_start_matches("\n");
    let (directions, forks) = input.split_once("\n\n").unwrap();
    let directions = Direction::parse_seq(directions);
    assert!(!directions.is_empty());
    let forks = parse_forks(forks);
    (forks, directions)
}

fn step<'f>(here: &str, forks: &'f Forks, dir: Direction) -> &'f str {
    match (&forks[here], dir) {
        ([left, _], Direction::Left) => left,
        ([_, right], Direction::Right) => right,
    }
}

pub fn solve_first(forks: &Forks, directions: &[Direction]) -> usize {
    let mut next_dir = Direction::iterate(directions);
    let mut current = START;
    let mut n_steps = 0;

    while current != END {
        current = step(current, forks, next_dir());
        n_steps += 1;
    }

    n_steps
}

fn loop_len<'a>(
    mut current: &'a str,
    forks: &'a HashMap<String, [String; 2]>,
    directions: &[Direction],
) -> usize {
    let mut len = 0;
    let mut next_dir = {
        let mut it = directions.iter().cycle().copied();
        move || it.next().unwrap()
    };

    while !current.ends_with("Z") {
        current = match (&forks[current], next_dir()) {
            ([left, _], Direction::Left) => left,
            ([_, right], Direction::Right) => right,
        };
        len += 1;
    }

    // check that the loop after reaching destination has the same length
    let mut len2 = 0;
    loop {
        len2 += 1;
        current = match (&forks[current], next_dir()) {
            ([left, _], Direction::Left) => left,
            ([_, right], Direction::Right) => right,
        };
        if current.ends_with("Z") {
            break;
        }
    }

    assert_eq!(len, len2);
    len
}

pub fn solve_second(forks: &Forks, directions: &[Direction]) -> usize {
    forks
        .keys()
        .map(String::as_str)
        .filter(|e| e.ends_with("A"))
        .map(|start| loop_len(start, &forks, &directions))
        .fold(1, |total, this| total.lcm(&this))
}

#[test]
fn example_first_short() {
    let input = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    let (forks, directions) = parse_input(input);
    assert_eq!(solve_first(&forks, &directions), 2);
}

#[test]
fn example_first_longer() {
    let input = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    let (forks, directions) = parse_input(input);
    assert_eq!(solve_first(&forks, &directions), 6);
}

#[test]
fn example_second() {
    let input = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    let (forks, directions) = parse_input(input);
    assert_eq!(solve_second(&forks, &directions), 6);
}
