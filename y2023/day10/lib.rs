#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

pub type Pipe = [Direction; 2];

fn parse_pipe(ch: u8) -> Option<Pipe> {
    use Direction::*;

    #[rustfmt::skip]
    let ret = Some(match ch {
        b'.' => return None,
        b'-' => [Left, Right],
        b'|' => [Up,   Down],
        b'L' => [Up,   Right],
        b'J' => [Up,   Left],
        b'F' => [Down, Right],
        b'7' => [Down, Left],
        _ => panic!("invalid character {}", char::from(ch)),
    });

    ret
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn apply(&mut self, dir: Direction) {
        #[rustfmt::skip]
        match dir {
            Direction::Up    => self.row -= 1,
            Direction::Right => self.col += 1,
            Direction::Down  => self.row += 1,
            Direction::Left  => self.col -= 1,
        };
    }

    fn applying(&self, dir: Direction) -> Self {
        let mut ret = self.clone();
        ret.apply(dir);
        ret
    }

    fn of(&self, grid: &Grid) -> Option<Pipe> {
        grid[self.row][self.col].non_start()
    }
}

#[derive(PartialEq, Eq)]
pub enum Input {
    Start,
    Pipe(Option<Pipe>),
}

impl Input {
    fn parse(ch: u8) -> Self {
        if ch == b'S' {
            return Self::Start;
        }
        Self::Pipe(parse_pipe(ch))
    }

    fn non_start(&self) -> Option<Pipe> {
        match self {
            Input::Start => panic!("start hise the pipe"),
            &Input::Pipe(pipe) => pipe,
        }
    }
}

type Grid = Vec<Vec<Input>>;

fn parse_input(input: &str) -> (Grid, Position) {
    let mut start_pos = None;
    let grid = input
        .trim_matches('\n')
        .lines()
        .enumerate()
        .map(|(irow, l)| {
            l.bytes()
                .enumerate()
                .map(|(icol, ch)| {
                    let ret = Input::parse(ch);
                    if ret == Input::Start {
                        if start_pos.is_some() {
                            panic!("more than one start");
                        }
                        start_pos = Some(Position {
                            row: irow,
                            col: icol,
                        });
                    }
                    ret
                })
                .collect()
        })
        .collect();
    (grid, start_pos.expect("no start point"))
}

fn lookup_connected_neighbor(grid: &Grid, start: Position) -> Vec<(Position, Direction)> {
    use Direction::*;

    let mut ret = Vec::with_capacity(2);
    let mut push_if_connects = |dir: Direction| {
        let pos = start.applying(dir);
        if let Some(pipe) = pos.of(grid) {
            if pipe.contains(&dir.opposite()) {
                ret.push((pos, dir));
            }
        }
    };

    if start.row > 0 {
        push_if_connects(Up);
    }
    if start.col > 0 {
        push_if_connects(Left);
    }
    if start.row + 1 < grid[start.row].len() {
        push_if_connects(Right);
    }
    if start.row + 1 < grid.len() {
        push_if_connects(Down);
    }

    ret
}

fn visit_loop(grid: &Grid, start: Position, mut f: impl FnMut(Position, Pipe)) -> usize {
    let mut current = lookup_connected_neighbor(grid, start.clone())[0].0.clone();
    let mut prev = start.clone();
    let mut len = 1;

    while current != start {
        let pipe @ [dir1, dir2] = current.of(grid).expect("clear ground instead of pipe");
        f(current.clone(), pipe);
        let next = if current.applying(dir1) == prev {
            current.applying(dir2)
        } else {
            current.applying(dir1)
        };
        prev = current;
        current = next;
        len += 1;
    }

    len
}

pub fn solve_first(grid: &Grid, start: Position) -> usize {
    let mut len = 1;
    visit_loop(grid, start, |_, _| len += 1);
    len / 2
}

fn start_directions(grid: &Grid, start: Position) -> Pipe {
    let [(_, dir1), (_, dir2)]: [(Position, Direction); 2] = lookup_connected_neighbor(grid, start)
        .try_into()
        .ok()
        .unwrap();
    [dir1, dir2]
}

pub fn solve_second(grid: &Grid, start: Position) -> usize {
    let loop_tiles = {
        let mut map = std::collections::HashMap::new();
        #[allow(dropping_copy_types)]
        visit_loop(grid, start.clone(), |pos, pipe| drop(map.insert(pos, pipe)));
        map.insert(start.clone(), start_directions(grid, start.clone()));
        map
    };

    let mut ret = 0;
    for (irow, row) in grid.iter().enumerate() {
        let mut inside_loop = false;
        let mut has_up = false;
        let mut has_down = false;
        for icol in 0..row.len() {
            if let Some(&pipe) = loop_tiles.get(&Position {
                row: irow,
                col: icol,
            }) {
                has_up ^= pipe.contains(&Direction::Up);
                has_down ^= pipe.contains(&Direction::Down);
                if has_up && has_down {
                    inside_loop = !inside_loop;
                    has_up = false;
                    has_down = false;
                }
                continue;
            }
            if inside_loop {
                ret += 1;
            }
        }
        assert!(!inside_loop, "in loop by the end of row {irow}");
    }

    ret
}

pub fn solve_both(input: &str) -> (usize, usize) {
    let (grid, start) = parse_input(input);
    let first = solve_first(&grid, start.clone());
    let second = solve_second(&grid, start);
    (first, second)
}

#[test]
fn example_loop_length() {
    let input = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
    let (grid, start) = parse_input(input);
    assert_eq!(solve_first(&grid, start), 8);
}

#[test]
fn example_loop_area() {
    #[track_caller]
    fn assert(input: &str, expected: usize) {
        let (grid, start) = parse_input(input);
        assert_eq!(solve_second(&grid, start), expected);
    }

    let basic = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
    assert(basic, 4);

let tight_passage = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";
    assert(tight_passage, 4);

let larger = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
    assert(larger, 8);

let larger_with_junk = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
    assert(larger_with_junk, 10);
}
