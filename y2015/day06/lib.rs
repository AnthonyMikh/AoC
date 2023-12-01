use std::ops::Range;

type Dim = usize;
const SIZE: Dim = 1000;
type Grid<T> = [[T; SIZE]; SIZE];

enum Action {
    On,
    Off,
    Toggle,
}

impl Action {
    fn parse_from_prefix(s: &str) -> (Self, &str) {
        if let Some(s) = s.strip_prefix("turn on ") {
            return (Self::On, s);
        }
        if let Some(s) = s.strip_prefix("turn off ") {
            return (Self::Off, s);
        }
        if let Some(s) = s.strip_prefix("toggle ") {
            return (Self::Toggle, s);
        }
        
        unreachable!("invalid prefix")
    }
}

trait Apply<T> {
    fn apply(&self, value: &mut T);
}

impl Apply<bool> for Action {
    fn apply(&self, value: &mut bool) {
        match self {
            Self::On => *value = true,
            Self::Off => *value = false,
            Self::Toggle => *value ^= true,
        }
    }
}

type Brightness = u32;

impl Apply<Brightness> for Action {
    fn apply(&self, value: &mut Brightness) {
        match self {
            Self::On => *value += 1,
            Self::Off => *value = (*value).saturating_sub(1),
            Self::Toggle => *value += 2,
        }
    }
}

fn parse_pair(s: &str) -> (Dim, Dim) {
    let (first, second) = s.split_once(",").unwrap();
    (first.parse().unwrap(), second.parse().unwrap())
}

struct Rect {
    row_range: Range<Dim>,
    col_range: Range<Dim>,
}

impl Rect {
    fn contains(&self, (row, col): (Dim, Dim)) -> bool {
        self.row_range.contains(&row) && self.col_range.contains(&col)
    }
}

struct Command {
    action: Action,
    area: Rect,
}

impl Command {
    fn parse(s: &str) -> Self {
        let (action, s) = Action::parse_from_prefix(s);
        let (start, end) = s.split_once(" through ").unwrap();
        let (left, top) = parse_pair(start);
        let (right, bottom) = parse_pair(end);
        Self {
            action,
            area: Rect {
                row_range: left..right + 1,
                col_range: top..bottom + 1,
            }
        }
    }
}

fn parse_commands(s: &str) -> Vec<Command> {
    s.lines().filter(|l| !l.is_empty()).map(Command::parse).collect()
}

// fn solve(input: &str) -> Dim {
//     let commands = parse_commands(input);
//     let mut grid = [[false; SIZE]; SIZE];
    
//     grid.iter().flat_map(|row| row.iter()).filter(|&&is_lit| is_lit as Dim).sum()
// }

fn apply_to_grid<T>(grid: &mut Grid<T>, commands: &[Command])
where
    Action: Apply<T>,
{
    for (irow, row) in grid.iter_mut().enumerate() {
        for (icol, val) in row.iter_mut().enumerate() {
            for c in commands {
                if c.area.contains((irow, icol)) {
                    c.action.apply(val);
                }
            }
        }
    }
}

fn solve_both(input: &str) -> (usize, Brightness) {
    let commands = parse_commands(input);

    let first = {
        let mut grid = [[false; SIZE]; SIZE];
        apply_to_grid(&mut grid, &commands);
        grid.iter().flat_map(|row| row.iter()).map(|&lit| lit as usize).sum()
    };
    
    let second = {
        let mut grid = [[0; SIZE]; SIZE];
        apply_to_grid(&mut grid, &commands);
        grid.iter().flat_map(|row| row.iter()).sum()
    };
    
    (first, second)
}
