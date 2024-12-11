}

use std::collections::HashSet as Set;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        use Direction::*;
        [Right, Down, Left, Up][self as usize]
    }
}

struct Bounds {
    max_row: usize,
    max_col: usize,
}

#[derive(Clone)]
struct Guard {
    pos: Position,
    dir: Direction,
}

impl Guard {
    fn step(&mut self, bounds: &Bounds) -> Result<(), ()> {
        use Direction::*;

        let &Bounds { max_row, max_col } = bounds;
        let Position { row, col } = &mut self.pos;

        match self.dir {
            Up if *row > 0 => *row -= 1,
            Right if *col < max_col => *col += 1,
            Down if *row < max_row => *row += 1,
            Left if *col > 0 => *col -= 1,
            _ => return Err(()),
        }

        Ok(())
    }

    fn stepping(&self, bounds: &Bounds) -> Result<Self, ()> {
        let mut ret = self.clone();
        ret.step(bounds)?;
        Ok(ret)
    }
}

fn parse(input: &str) -> (Set<Position>, Guard, Bounds) {
    let mut max_row = 0;
    let mut max_col = 0;
    let mut obstacles = Set::new();
    let mut guard_pos = None;

    for (row, line) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        max_row = row;
        for (col, &ch) in line.as_bytes().iter().enumerate() {
            max_col = col;
            match ch {
                b'.' => {}
                b'#' => drop(obstacles.insert(Position { row, col })),
                b'^' => guard_pos = Some(Position { row, col }),
                _ => unreachable!(),
            }
        }
    }

    let guard = Guard {
        pos: guard_pos.unwrap(),
        dir: Direction::Up,
    };

    let bounds = Bounds { max_row, max_col };

    (obstacles, guard, bounds)
}

fn loops(obstacles: &Set<Position>, mut guard: Guard, bounds: &Bounds, at: Position) -> bool {
    let mut visited = Set::new();
    loop {
        let Ok(next) = guard.stepping(&bounds) else {
            break false;
        };
        if next.pos == at || obstacles.contains(&next.pos) {
            guard.dir = guard.dir.turn_right();
        } else {
            guard = next;
            if !visited.insert((guard.pos, guard.dir)) {
                break true;
            }
        }
    }
}

pub fn solve_both(input: &str) -> (usize, usize) {
    let (obstacles, start_guard, bounds) = parse(input);
    let mut visited = Set::new();
    let mut guard = start_guard.clone();
    visited.insert(guard.pos);

    while let Ok(next) = guard.stepping(&bounds) {
        if obstacles.contains(&next.pos) {
            guard.dir = guard.dir.turn_right();
        } else {
            guard = next;
            visited.insert(guard.pos);
        }
    }

    let n_turns = visited.len();
    let n_loops = visited
        .iter()
        .filter(|&&p| p != start_guard.pos)
        .filter(|&&p| loops(&obstacles, start_guard.clone(), &bounds, p))
        .count();

    (n_turns, n_loops)
}
