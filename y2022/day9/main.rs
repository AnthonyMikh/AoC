use std::ops::ControlFlow;

const ROPE_LEN: usize = 9;

type Coord = i32;
type Knots = [Pos; ROPE_LEN];

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: Coord,
    y: Coord,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Pos {
    fn step(&mut self, dir: Direction) {
        use Direction::*;

        #[rustfmt::skip]
        match dir {
            Up    => self.y += 1,
            Right => self.x += 1,
            Down  => self.y -= 1,
            Left  => self.x -= 1,
        };
    }

    fn stepping(mut self, dir: Direction) -> Self {
        self.step(dir);
        self
    }

    fn is_close_to(self, other: Self) -> bool {
        std::cmp::max(self.x.abs_diff(other.x), self.y.abs_diff(other.y)) <= 1
    }

    fn catch_up(&mut self, leader: Self) -> ControlFlow<(), Self> {
        let dx = leader.x - self.x;
        let dy = leader.y - self.y;
        if let (-1..=1, -1..=1) = (dx, dy) {
            return ControlFlow::Break(());
        }
        self.x += dx.clamp(-1, 1);
        self.y += dy.clamp(-1, 1);
        ControlFlow::Continue(*self)
    }
}

fn parse(s: &str) -> (Direction, Coord) {
    use Direction::*;

    let (dir, offset) = s.split_once(' ').unwrap();
    let dir = match dir {
        "U" => Up,
        "R" => Right,
        "D" => Down,
        "L" => Left,
        _ => panic!("invalid direction {dir}"),
    };
    let offset = offset.parse().unwrap();
    (dir, offset)
}

fn solve(input: &str) -> (usize, usize) {
    use std::collections::HashSet;

    let mut head = Pos::default();
    let mut knots = Knots::default();
    let mut visited_first = HashSet::new();
    let mut visited_tail = HashSet::new();
    visited_first.insert(head);
    visited_tail.insert(head);

    for (dir, offset) in input.lines().map(parse) {
        for _ in 0..offset {
            head.step(dir);
            knots
                .iter_mut()
                .try_fold(head, |prev, current| current.catch_up(prev));
            visited_first.insert(knots[0]);
            visited_tail.insert(knots[ROPE_LEN - 1]);
        }
    }

    (visited_first.len(), visited_tail.len())
}

#[test]
fn first_example() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(solve(input), (13, 1));
}

#[test]
fn second_example() {
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(solve(input).1, 36);
}
