pub type Grid<const SIZE: usize> = [[bool; SIZE]; SIZE];

fn fill_corners<const SIZE: usize>(grid: &mut Grid<SIZE>) {
    assert_ne!(SIZE, 0);
    grid[0][0] = true;
    grid[0][SIZE - 1] = true;
    grid[SIZE - 1][0] = true;
    grid[SIZE - 1][SIZE - 1] = true;
}

fn step<const SIZE: usize>(grid: &Grid<SIZE>, out: &mut Grid<SIZE>) {
    assert_ne!(SIZE, 0);

    for (irow, row) in grid.iter().enumerate() {
        for (icol, &alive) in row.iter().enumerate() {
            let mut n_neighbors = 0;
            let mut inc = |alive: bool| n_neighbors += alive as usize;
            if irow > 0 {
                let above = grid[irow - 1];
                if icol > 0 {
                    inc(above[icol - 1]);
                }
                inc(above[icol]);
                if icol < SIZE - 1 {
                    inc(above[icol + 1]);
                }
            }
            if icol > 0 {
                inc(row[icol - 1]);
            }
            if icol < SIZE - 1 {
                inc(row[icol + 1]);
            }
            if irow < SIZE - 1 {
                let below = grid[irow + 1];
                if icol > 0 {
                    inc(below[icol - 1]);
                }
                inc(below[icol]);
                if icol < SIZE - 1 {
                    inc(below[icol + 1]);
                }
            }
            out[irow][icol] = match (alive, n_neighbors) {
                (false, 3) => true,
                (true, 2 | 3) => true,
                _ => false,
            };
        }
    }
}

fn fill_grid<const SIZE: usize>(out: &mut Grid<SIZE>, input: &str) {
    let n_rows = input
        .lines()
        .zip(&mut out[..])
        .map(|(line, row)| {
            assert_eq!(line.len(), SIZE);
            line.as_bytes().iter().zip(row).for_each(|(&ch, cell)| {
                *cell = match ch {
                    b'#' => true,
                    b'.' => false,
                    _ => panic!("wrong character {}", char::from(ch)),
                };
            })
        })
        .count();
    assert_eq!(n_rows, SIZE);
}

pub fn solve<'a, const SIZE: usize, const KEEP_CORNERS: bool>(
    mut current: &'a mut Grid<SIZE>,
    mut next: &'a mut Grid<SIZE>,
    n_steps: usize,
) -> usize {
    if KEEP_CORNERS {
        fill_corners(current);
    }
    for _ in 0..n_steps {
        step(current, next);
        if KEEP_CORNERS {
            fill_corners(next);
        }
        std::mem::swap(&mut current, &mut next);
    }
    current
        .iter()
        .flat_map(|row| row.iter())
        .map(|&alive| alive as usize)
        .sum()
}

pub fn solve_both<const SIZE: usize>(input: &str, n_steps: usize) -> (usize, usize) {
    let mut initial = [[false; SIZE]; SIZE];
    let current = &mut [[false; SIZE]; SIZE];
    let next = &mut [[false; SIZE]; SIZE];
    fill_grid(&mut initial, input.trim_matches('\n'));

    current.clone_from(&initial);
    let first = solve::<SIZE, false>(current, next, n_steps);

    current.clone_from(&initial);
    let second = solve::<SIZE, true>(current, next, n_steps);

    (first, second)
}

#[cfg(test)]
const fn grid_size(grid: &str) -> usize {
    let grid = grid.as_bytes();
    let mut i = 0;
    let mut len = 0;
    while grid[i] == b'\n' {
        i += 1;
    }
    while grid[i] != b'\n' {
        i += 1;
        len += 1;
    }
    len
}

#[test]
fn example() {
    const EXAMPLE_INPUT: &str = "
.#.#.#
...##.
#....#
..#...
#.#..#
####..
";
    assert_eq!(
        solve_both::<{ grid_size(EXAMPLE_INPUT) }>(EXAMPLE_INPUT, 5),
        (4, 17)
    );
}
