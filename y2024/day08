use std::collections::{HashMap, HashSet};

type Pos = (isize, isize);
type Antennas = HashMap<u8, Vec<Pos>>;

struct Bounds {
    max_row: isize,
    max_col: isize,
}

impl Bounds {
    fn contains(&self, (row, col): Pos) -> bool {
        (0..self.max_row).contains(&row) && (0..self.max_col).contains(&col)
    }
}

fn parse(input: &str) -> (Antennas, Bounds) {
    let mut max_row = 0;
    let mut max_col = 0;
    let mut antennas = Antennas::new();
    
    for (irow, row) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        max_row = irow + 1;
        for (icol, &ch) in row.as_bytes().iter().enumerate() {
            max_col = icol + 1;
            if let b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' = ch {
                antennas.entry(ch).or_default().push((irow as isize, icol as isize));
            }
        }
    }
    
    let bounds = Bounds {
        max_row: max_row as isize,
        max_col: max_col as isize,
    };
    
    (antennas, bounds)
}

fn mark_antinodes(antennas: &[Pos], bounds: &Bounds, adjacent: &mut HashSet<Pos>, all: &mut HashSet<Pos>) {
    for (i, &(row1, col1)) in antennas.iter().enumerate() {
        for &(row2, col2) in &antennas[i + 1..] {
            let delta_row = row2 - row1;
            let delta_col = col2 - col1;
            {
                let pos1 = (row1 - delta_row, col1 - delta_col);
                let pos2 = (row2 + delta_row, col2 + delta_col);
                for p in [pos1, pos2] {
                    if bounds.contains(p) {
                        adjacent.insert(p);
                    }
                }
            }
            {
                let mut p = (row1, col1);
                while bounds.contains(p) {
                    all.insert(p);
                    p.0 -= delta_row;
                    p.1 -= delta_col;
                }
                
                let mut p = (row2, col2);
                while bounds.contains(p) {
                    all.insert(p);
                    p.0 += delta_row;
                    p.1 += delta_col;
                }
            }
        }
    }
}

pub fn solve_both(input: &str) -> (usize, usize) {
    let (antennas, bounds) = parse(input);
    let mut adjacent = HashSet::new();
    let mut all = HashSet::new();
    for set in antennas.values() {
        mark_antinodes(set, &bounds, &mut adjacent, &mut all);
    }
    (adjacent.len(), all.len())
}
