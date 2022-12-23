fn main() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    println!("{}", solve(input, 2022));
    println!("{}", solve(input, 1000000000000));
}

fn solve(input: &str, n_fall: usize) -> usize {
    let (
        Loop {
            fell_starting_loop,
            loop_size,
            height_before_loop,
            height_per_loop,
        },
        history,
    ) = loop_size(input);
    let fell_looping = n_fall - fell_starting_loop;
    let n_loops = fell_looping / loop_size;
    let fell_incomplete_loop = fell_looping % loop_size;
    let height_incomplete_loop =
        history[fell_starting_loop + fell_incomplete_loop] - height_before_loop;
    height_before_loop + height_per_loop * n_loops + height_incomplete_loop
}

#[derive(Clone, Copy, Debug)]
struct Loop {
    fell_starting_loop: usize,
    height_before_loop: usize,
    loop_size: usize,
    height_per_loop: usize,
}

fn loop_size(input: &str) -> (Loop, Vec<usize>) {
    use std::collections::hash_map::{Entry, HashMap};

    let moves = input.bytes().map(Move::parse).collect::<Vec<_>>();
    let mut moves = moves.iter().enumerate().cycle();
    let mut rocks = ROCKS.iter().cycle().cloned();
    let mut chamber = Chamber::default();
    let mut height = 0;
    let mut history = vec![0];
    let mut seen = HashMap::new();
    seen.insert(<_>::default(), 0);

    for i in 1.. {
        let mut rock = rocks.next().unwrap().spawn(height + 3);
        let move_idx = loop {
            let (idx, &move_) = moves.next().unwrap();
            rock.try_move(move_, &mut chamber);
            rock = match rock.fall(&mut chamber) {
                Fell::Down(rock) => rock,
                Fell::ToRest => break idx,
            };
        };
        let hh = chamber.heights();
        height = chamber.height();

        #[allow(clippy::drop_ref)]
        match seen.entry((hh, move_idx)) {
            Entry::Vacant(e) => drop(e.insert(i)),
            Entry::Occupied(e) => {
                let fell_starting_loop = *e.get();
                let loop_size = i - fell_starting_loop;
                let height_before_loop = history[fell_starting_loop];
                let height_per_loop = height - height_before_loop;
                return (
                    Loop {
                        fell_starting_loop,
                        loop_size,
                        height_before_loop,
                        height_per_loop,
                    },
                    history,
                );
            }
        }
        history.push(height);
    }

    unreachable!("loop not found")
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn parse(ch: u8) -> Self {
        match ch {
            b'<' => Move::Left,
            b'>' => Move::Right,
            _ => panic!("invalid `Move` character: {:?}", char::from(ch)),
        }
    }
}

#[derive(Clone)]
struct Row {
    bits: u8,
}

impl Row {
    const WIDTH: Size = 7;

    const fn empty() -> Self {
        Self { bits: 0 }
    }

    fn is_empty(&self) -> bool {
        self.bits == 0
    }

    const fn from_bits(bits: u8) -> Self {
        let bits = bits & ((1 << Self::WIDTH) - 1);
        Self { bits }
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.bits & other.bits != 0
    }
}

type Size = u8;

#[derive(Clone)]
struct Rock {
    parts: [Row; 4],
    width: Size,
    height: Size,
}

impl Rock {
    fn spawn(&self, y: usize) -> FallingRock {
        FallingRock {
            rock: self.clone(),
            x: 2,
            y,
        }
    }
}

#[derive(Clone)]
struct FallingRock {
    rock: Rock,
    x: Size,
    y: usize,
}

enum Fell {
    Down(FallingRock),
    ToRest,
}

impl FallingRock {
    fn try_move_right(&self, chamber: &mut Chamber) -> Option<Self> {
        if self.x + self.rock.width >= Row::WIDTH {
            return None;
        }

        let mut ret = self.clone();
        ret.rock.parts.iter_mut().for_each(|r| r.bits >>= 1);
        ret.x += 1;
        if !chamber.can_place(&ret) {
            return None;
        }

        Some(ret)
    }

    fn try_move_left(&self, chamber: &mut Chamber) -> Option<Self> {
        if self.x == 0 {
            return None;
        }

        let mut ret = self.clone();
        ret.rock.parts.iter_mut().for_each(|r| r.bits <<= 1);
        ret.x -= 1;
        if !chamber.can_place(&ret) {
            return None;
        }

        Some(ret)
    }

    fn try_move(&mut self, move_: Move, chamber: &mut Chamber) {
        if let Some(moved) = match move_ {
            Move::Left => self.try_move_left(chamber),
            Move::Right => self.try_move_right(chamber),
        } {
            *self = moved;
        }
    }

    fn fall(&self, chamber: &mut Chamber) -> Fell {
        if self.y == 0 {
            chamber.put(self);
            return Fell::ToRest;
        }
        let mut ret = self.clone();
        ret.y -= 1;
        let place = chamber.get_range(ret.occupied());
        if place
            .iter()
            .zip(ret.iter())
            .any(|(row, p)| row.overlaps_with(p))
        {
            chamber.put(self);
            return Fell::ToRest;
        }
        Fell::Down(ret)
    }

    fn occupied(&self) -> std::ops::Range<usize> {
        let end = self.y.checked_add(usize::from(self.rock.height)).unwrap();
        self.y..end
    }

    fn iter(&self) -> impl Iterator<Item = &Row> + '_ {
        self.rock.parts.iter().rev()
    }
}

// I want const array::map
macro_rules! rows {
    ($($row:literal),* $(,)?) => {
        [$(Row::from_bits($row)),*]
    }
}

#[rustfmt::skip]
const ROCKS: [Rock; 5] = [
    Rock {
        parts: rows![
            0b0000000,
            0b0000000,
            0b0000000,
            0b0011110,
        ],
        width: 4,
        height: 1,
    },
    Rock {
        parts: rows![
            0b0000000,
            0b0001000,
            0b0011100,
            0b0001000,
        ],
        width: 3,
        height: 3,
    },
    Rock {
        parts: rows![
            0b0000000,
            0b0000100,
            0b0000100,
            0b0011100,
        ],
        width: 3,
        height: 3,
    },
    Rock {
        parts: rows![
            0b0010000,
            0b0010000,
            0b0010000,
            0b0010000,
        ],
        width: 1,
        height: 4,
    },
    Rock {
        parts: rows![
            0b0000000,
            0b0000000,
            0b0011000,
            0b0011000,
        ],
        width: 2,
        height: 2,
    },
];

#[derive(Default)]
struct Chamber {
    rows: Vec<Row>,
}

type Heights = [usize; Row::WIDTH as _];

impl Chamber {
    fn get_range(&mut self, range: std::ops::Range<usize>) -> &mut [Row] {
        assert!(range.start <= range.end);
        if range.end > self.rows.len() {
            self.rows.resize(range.end, Row::empty());
        }
        &mut self.rows[range]
    }

    fn can_place(&mut self, rock: &FallingRock) -> bool {
        self.get_range(rock.occupied())
            .iter()
            .zip(rock.iter())
            .all(|(row, p)| !row.overlaps_with(p))
    }

    fn put(&mut self, rock: &FallingRock) {
        let place = self.get_range(rock.occupied());
        for (row, rock_part) in place.iter_mut().zip(rock.iter()) {
            row.bits |= rock_part.bits;
        }
    }

    fn height(&self) -> usize {
        self.rows
            .iter()
            .rposition(|r| !r.is_empty())
            .map_or(0, |i| i + 1)
    }

    fn heights(&self) -> Heights {
        let mut ret = Heights::default();
        let mut n_zero = ret.len();
        for (height, row) in self.rows.iter().enumerate().rev() {
            let height = height + 1;
            for (i, h) in ret.iter_mut().enumerate() {
                if row.bits & (1 << i) != 0 && *h < height {
                    *h = height;
                    n_zero -= 1;
                }
            }
            if n_zero == 0 {
                break;
            }
        }
        let min = ret.iter().copied().min().unwrap_or(0);
        ret.iter_mut().for_each(|h| *h -= min);
        ret
    }
}
