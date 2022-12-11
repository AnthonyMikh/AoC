use std::collections::{BTreeMap, HashSet};

type Height = u8;
type Pos = (usize, usize);

fn add_visible(it: impl Iterator<Item = (Height, Pos)>, seen: &mut HashSet<Pos>) {
    let mut max = 0;
    seen.extend(it.filter_map(move |(h, pos)| {
        let ret = (h > max).then_some(pos);
        max = max.max(h);
        ret
    }));
}

const MAX_HEIGHT: Height = b'9';

fn distances_along<'a>(
    it: impl Iterator<Item = (Height, Pos)> + 'a,
    last_pos: &'a mut BTreeMap<Height, usize>,
) -> impl Iterator<Item = (usize, Pos)> + 'a {
    last_pos.clear();
    it.enumerate().map(move |(current, (height, pos))| {
        let closest = last_pos.range(height..=MAX_HEIGHT).map(|(_h, &pos)| pos).max();
        let ret = current - closest.unwrap_or(0);
        last_pos.insert(height, current);
        (ret, pos)
    })
}

fn solve1(input: &str) -> usize {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let ref mut seen = HashSet::with_capacity(n_rows * n_cols);

    for (irow, row) in grid.iter().enumerate() {
        let with_positions = row
            .iter()
            .enumerate()
            .map(move |(icol, &height)| (height, (irow, icol)));
        add_visible(with_positions.clone(), seen);
        add_visible(with_positions.rev(), seen);
    }

    for icol in 0..n_cols {
        let with_positions = grid
            .iter()
            .enumerate()
            .map(move |(irow, row)| (row[icol], (irow, icol)));
        add_visible(with_positions.clone(), seen);
        add_visible(with_positions.rev(), seen);
    }

    seen.len()
}

fn solve2(input: &str) -> usize {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let ref mut pos = BTreeMap::new();
    let mut distances = vec![vec![[0; 4]; n_cols]; n_rows];

    for (irow, row) in grid.iter().enumerate() {
        let with_positions = row
            .iter()
            .enumerate()
            .map(move |(icol, &height)| (height, (irow, icol)));
        for (dist, (irow, icol)) in distances_along(with_positions.clone(), pos) {
            distances[irow][icol][0] = dist;
        }
        for (dist, (irow, icol)) in distances_along(with_positions.rev(), pos) {
            distances[irow][icol][1] = dist;
        }
    }

    for icol in 0..n_cols {
        let with_positions = grid
            .iter()
            .enumerate()
            .map(move |(irow, row)| (row[icol], (irow, icol)));
        for (dist, (irow, icol)) in distances_along(with_positions.clone(), pos) {
            distances[irow][icol][2] = dist;
        }
        for (dist, (irow, icol)) in distances_along(with_positions.rev(), pos) {
            distances[irow][icol][3] = dist;
        }
    }

    distances.iter().flatten().map(|d| d.iter().product::<usize>()).max().unwrap()
}

fn main() {
    let input = "\
30373
25512
65332
33549
35390";
    println!("{}", solve1(input));
    println!("{}", solve2(input));
}
