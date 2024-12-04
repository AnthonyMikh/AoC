#[rustfmt::skip]
static DELTAS: &[(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0,  -1),          (0,  1),
    (1,  -1), (1,  0), (1,  1),
];

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn has_in_direction(
    grid: &[impl AsRef<[char]>],
    target: &str,
    (row, col): (usize, usize),
    (row_delta, col_delta): (isize, isize),
) -> bool {
    target.chars().zip(0_isize..).all(|(target_ch, mult)| {
        let row = row.wrapping_add_signed(row_delta * mult);
        let col = col.wrapping_add_signed(col_delta * mult);
        grid.get(row)
            .and_then(|row| row.as_ref().get(col))
            .map_or(false, |&ch| ch == target_ch)
    })
}

pub fn solve1(grid: &[impl AsRef<[char]>]) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].as_ref().len();
    let mut ret = 0;

    for row in 0..nrows {
        for col in 0..ncols {
            ret += DELTAS
                .iter()
                .filter(|&&delta| has_in_direction(grid, "XMAS", (row, col), delta))
                .count()
        }
    }

    ret
}

fn has_x_mas_at(grid: &[impl AsRef<[char]>], (row, col): (usize, usize)) -> bool {
    if grid[row].as_ref()[col] != 'A' {
        return false;
    }

    // check for
    // 
    // M..      S..
    // .A.  or  .A.
    // ..S      ..M
    let over = grid[row - 1].as_ref()[col - 1];
    let under = grid[row + 1].as_ref()[col + 1];
    match (over, under) {
        ('M', 'S') | ('S', 'M') => {/* correct diagonal */}
        _ => return false,
    }

    // check for
    // 
    // ..M      ..S
    // .A.  or  .A.
    // S..      M..
    let over = grid[row - 1].as_ref()[col + 1];
    let under = grid[row + 1].as_ref()[col - 1];
    match (over, under) {
        ('M', 'S') | ('S', 'M') => {/* correct diagonal */}
        _ => return false,
    }
    
    // Center is 'A', both diagonals are 'SAM'/'MAS' - it is x-mas
    true
}

pub fn solve2(grid: &[impl AsRef<[char]>]) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].as_ref().len();
    let mut ret = 0;
    
    // The first and the last rows, as well as the first and the last columns,
    // can not be a center of a 3x3 structure, so do not consider them
    for row in 1..nrows - 1 {
        for col in 1..ncols - 1 {
            if has_x_mas_at(grid, (row, col)) {
                ret += 1;
            }
        }
    }

    ret
}

#[test]
fn example() {
    const INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let grid = parse(INPUT);
    assert_eq!(solve1(&grid), 18);
    assert_eq!(solve2(&grid), 9);
}
