struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn distance_to(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    fn space_expansion_impact(
        &self,
        other: &Self,
        empty: &Empty,
        expansion_factor: usize,
    ) -> usize {
        let compute = |a, b, excluded: &[_]| {
            min_max_range(a, b)
                .filter(|&row| excluded.binary_search(&row).is_ok())
                .count()
                * (expansion_factor - 1)
        };
        compute(self.row, other.row, &empty.rows) + compute(self.col, other.col, &empty.cols)
    }

    fn distance_considering_expansion(
        &self,
        other: &Self,
        empty: &Empty,
        expansion_factor: usize,
    ) -> usize {
        self.distance_to(other) + self.space_expansion_impact(other, empty, expansion_factor)
    }
}

struct Empty {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

fn min_max_range<T: Ord>(a: T, b: T) -> std::ops::RangeInclusive<T> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn parse_input(input: &str) -> (Vec<Position>, Empty) {
    let input = input.trim_matches('\n');
    let width = input.lines().next().unwrap().len();
    let mut used_cols = vec![false; width];
    let mut empty_rows = Vec::new();
    let mut galaxies = Vec::new();

    for (irow, row) in input.lines().enumerate() {
        let mut is_empty_row = true;
        for (icol, &ch) in row.as_bytes().iter().enumerate() {
            match ch {
                b'#' => {
                    galaxies.push(Position {
                        row: irow,
                        col: icol,
                    });
                    used_cols[icol] = true;
                    is_empty_row = false;
                }
                b'.' => (),
                _ => panic!(
                    "invalid character {:?} at line {}, column {}",
                    char::from(ch),
                    irow + 1,
                    icol + 1
                ),
            }
        }
        if is_empty_row {
            empty_rows.push(irow);
        }
    }
    let empty_cols = (0..width).filter(|&i| !used_cols[i]).collect();
    (
        galaxies,
        Empty {
            rows: empty_rows,
            cols: empty_cols,
        },
    )
}

fn solve(galaxies: &[Position], empty_space: &Empty, expansion_factor: usize) -> usize {
    let mut ret = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1..] {
            ret += g1.distance_considering_expansion(g2, empty_space, expansion_factor);
        }
    }
    ret
}

pub fn solve_with_input(input: &str, expansion_factor: usize) -> usize {
    let (galaxies, empty_space) = parse_input(input);
    solve(&galaxies, &empty_space, expansion_factor)
}

#[test]
fn example() {
    assert_eq!(solve_with_input(INPUT, 2), 374);
    assert_eq!(solve_with_input(INPUT, 10), 1030);
    assert_eq!(solve_with_input(INPUT, 100), 8410);
}

#[cfg(test)]
const INPUT: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
