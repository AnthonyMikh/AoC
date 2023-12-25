use std::num::NonZeroUsize;

pub type Num = u64;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Blank,
    Filled,
    Arbitrary,
}

impl Tile {
    fn parse_seq(s: &str) -> Vec<Self> {
        s.as_bytes()
            .iter()
            .map(|&ch| match ch {
                b'.' => Self::Blank,
                b'#' => Self::Filled,
                b'?' => Self::Arbitrary,
                _ => panic!("invalid character {}", char::from(ch)),
            })
            .collect()
    }
}

pub struct Spec {
    groups: Vec<NonZeroUsize>,
}

impl Spec {
    fn parse(s: &str) -> Self {
        Self {
            groups: s.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

pub type Task = (Vec<Tile>, Spec);

fn parse_line(s: &str) -> Task {
    let (tiles, spec) = s.split_once(' ').unwrap();
    (Tile::parse_seq(tiles), Spec::parse(spec))
}

pub fn parse_input(input: &str) -> Vec<Task> {
    input.trim_matches('\n').lines().map(parse_line).collect()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum State {
    Blank,
    Group {
        current: usize,
        expected: NonZeroUsize,
    },
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SubSolution<'a> {
    spec: &'a [NonZeroUsize],
    state: State,
}

impl<'a> SubSolution<'a> {
    fn from_spec(spec: &'a Spec) -> Self {
        Self {
            spec: &spec.groups,
            state: State::Blank,
        }
    }

    fn try_add_blank(&self) -> Result<Self, ()> {
        match self.state {
            State::Blank => Ok(self.clone()),
            State::Group { current, expected } => {
                if current != expected.get() {
                    return Err(());
                }
                Ok(Self {
                    state: State::Blank,
                    ..*self
                })
            }
        }
    }

    fn try_add_filled(&self) -> Result<Self, ()> {
        match self.state {
            State::Blank => {
                let (&first, rest) = self.spec.split_first().ok_or(())?;
                Ok(Self {
                    spec: rest,
                    state: State::Group {
                        current: 1,
                        expected: first,
                    },
                })
            }
            State::Group { current, expected } => {
                let incremented = current + 1;
                if incremented > expected.get() {
                    return Err(());
                }
                Ok(Self {
                    state: State::Group {
                        current: incremented,
                        expected,
                    },
                    ..*self
                })
            }
        }
    }

    fn add(&self, tile: Tile) -> [Option<Self>; 2] {
        let mut ret = [None, None];
        match tile {
            Tile::Blank => ret[0] = self.try_add_blank().ok(),
            Tile::Filled => ret[0] = self.try_add_filled().ok(),
            Tile::Arbitrary => {
                ret[0] = self.try_add_blank().ok();
                ret[1] = self.try_add_filled().ok();
            }
        }
        ret
    }

    fn complete(&self) -> Result<(), ()> {
        if !self.spec.is_empty() {
            return Err(());
        }
        match self.state {
            State::Blank => Ok(()),
            State::Group { current, expected } => {
                if current == expected.get() {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }
}

#[allow(dead_code)]
fn n_task_solutions(tiles: &[Tile], spec: &Spec) -> Num {
    let mut subtasks = vec![(SubSolution::from_spec(spec), tiles)];
    let mut n_solutions = 0;

    while let Some((mut sub, mut tiles)) = subtasks.pop() {
        loop {
            match tiles.split_first() {
                None => {
                    if sub.complete().is_ok() {
                        n_solutions += 1;
                    }
                    break;
                }
                Some((&first, rest)) => {
                    match sub.add(first) {
                        [None, None] => break,
                        [None, Some(new)] | [Some(new), None] => {
                            sub = new;
                        }
                        [Some(new1), Some(new2)] => {
                            sub = new1;
                            subtasks.push((new2, rest));
                        }
                    }
                    tiles = rest
                }
            }
        }
    }

    n_solutions
}

fn n_task_solutions_memoized<'a>(tiles: &'a [Tile], spec: &'a Spec) -> Num {
    let original = (SubSolution::from_spec(spec), tiles);
    let mut subtasks = vec![original.clone()];
    let mut memo = std::collections::HashMap::new();

    while let Some((sub, tiles)) = subtasks.pop() {
        if memo.contains_key(&(sub.clone(), tiles)) {
            continue;
        } else {
            subtasks.push((sub.clone(), tiles));
        }
        let subanswer = match tiles.split_first() {
            None => {
                if sub.complete().is_ok() {
                    1
                } else {
                    0
                }
            }
            Some((&first, rest)) => match sub.add(first) {
                [None, None] => 0,
                [None, Some(new)] | [Some(new), None] => {
                    if let Some(&answer) = memo.get(&(new.clone(), rest)) {
                        answer
                    } else {
                        subtasks.push((new, rest));
                        continue;
                    }
                }
                [Some(new1), Some(new2)] => {
                    if let Some(&subanswer1) = memo.get(&(new1.clone(), rest)) {
                        if let Some(&subanswer2) = memo.get(&(new2.clone(), rest)) {
                            subanswer1 + subanswer2
                        } else {
                            subtasks.push((new2, rest));
                            continue;
                        }
                    } else {
                        // pushing the other way around seemingly causes
                        // more hashmap lookups
                        subtasks.push((new1, rest));
                        subtasks.push((new2, rest));
                        continue;
                    }
                }
            },
        };
        memo.insert((sub, tiles), subanswer);
    }

    memo[&original]
}

pub fn solve_first(tasks: &[Task]) -> Num {
    tasks
        .iter()
        .map(|(tiles, spec)| n_task_solutions_memoized(tiles, spec))
        .sum()
}

const UNFOLD_FACTOR: usize = 5;

fn unfold(task: &Task) -> Task {
    let (tiles, spec) = task;
    let mut ret_tiles = Vec::new();
    let mut ret_spec = Vec::new();
    for _ in 0..UNFOLD_FACTOR {
        ret_tiles.extend(tiles);
        ret_tiles.push(Tile::Arbitrary);
        ret_spec.extend(&spec.groups);
    }
    ret_tiles.pop();
    (ret_tiles, Spec { groups: ret_spec })
}

pub fn solve_second(tasks: &[Task]) -> Num {
    tasks
        .iter()
        .map(|task| {
            let (tiles, spec) = unfold(task);
            n_task_solutions_memoized(&tiles, &spec)
        })
        .fold(0, |total, sub| {
            total.checked_add(sub).expect("n_solutions overflow")
        })
}

pub fn solve(input: &str) -> (Num, Num) {
    let tasks = parse_input(input);
    let first = solve_first(&tasks);
    let second = solve_second(&tasks);
    (first, second)
}

#[test]
fn example() {
    let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    assert_eq!(solve(input), (21, 525152));
}
