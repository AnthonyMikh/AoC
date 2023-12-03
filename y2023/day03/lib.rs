#![cfg(test)]

use std::ops::{Bound, Range, RangeBounds};

type Num = u64;

struct SymbolLocations {
    locations: Vec<Vec<(u8, usize)>>,
}

fn has_adjacent_in_line(symbols: &[(u8, usize)], span: Range<usize>) -> bool {
    let span = (
        Bound::Included(span.start.saturating_sub(1)),
        Bound::Included(span.end),
    );
    symbols.iter().any(|(_, pos)| span.contains(pos))
}

impl SymbolLocations {
    fn has_adjacent(&self, line: usize, pos: Range<usize>) -> bool {
        let locs = &self.locations;

        has_adjacent_in_line(&locs[line], pos.clone())
            || (line > 0 && has_adjacent_in_line(&locs[line - 1], pos.clone()))
            || (line + 1 < locs.len() && has_adjacent_in_line(&locs[line + 1], pos))
    }
}

struct NumLocations {
    locations: Vec<Vec<(Num, Range<usize>)>>,
}

fn adjacent_nums_in_line(nums: &[(Num, Range<usize>)], pos: usize) -> Vec<Num> {
    nums.iter()
        .cloned()
        .filter_map(|(num, span)| {
            (
                Bound::Included(span.start.saturating_sub(1)),
                Bound::Included(span.end),
            )
                .contains(&pos)
                .then_some(num)
        })
        .collect()
}

impl NumLocations {
    fn adjacent_nums(&self, line: usize, pos: usize) -> Vec<Num> {
        let locs = &self.locations;
        let mut ret;

        ret = adjacent_nums_in_line(&locs[line], pos);
        if line > 0 {
            ret.extend(adjacent_nums_in_line(&locs[line - 1], pos));
        }
        if let Some(loc_line) = locs.get(line + 1) {
            ret.extend(adjacent_nums_in_line(loc_line, pos));
        }

        ret
    }
}

fn parse(s: &str) -> (SymbolLocations, NumLocations) {
    let mut sym_locations = Vec::new();
    let mut num_locations = Vec::new();

    for row in s.lines().filter(|l| !l.is_empty()) {
        let mut num = None;
        let mut num_line = Vec::new();
        let mut sym_line = Vec::new();

        for (col, &b) in row.as_bytes().iter().enumerate() {
            match b {
                b'0'..=b'9' => {
                    let digit = (b - b'0') as Num;
                    num = match num {
                        None => Some((digit, col..col + 1)),
                        Some((n, range)) => Some((10 * n + digit, range.start..col + 1)),
                    };
                }
                _ => {
                    num_line.extend(num.take());
                    if b != b'.' {
                        sym_line.push((b, col));
                    }
                }
            }
        }

        num_line.extend(num.take());
        num_locations.push(num_line);
        sym_locations.push(sym_line);
    }

    (
        SymbolLocations {
            locations: sym_locations,
        },
        NumLocations {
            locations: num_locations,
        },
    )
}

fn solve_both(input: &str) -> (Num, Num) {
    let (sym_locs, num_locs) = &parse(input);
    let first = num_locs
        .locations
        .iter()
        .enumerate()
        .flat_map(|(irow, row)| {
            row.iter().cloned().filter_map(move |(num, num_span)| {
                sym_locs.has_adjacent(irow, num_span).then_some(num)
            })
        })
        .sum();
    let second = sym_locs
        .locations
        .iter()
        .enumerate()
        .flat_map(|(irow, row)| row.iter().filter_map(move |&(sym, pos)| (sym == b'*').then_some((irow, pos))))
        .filter_map(|(irow, pos)| {
            let around = num_locs.adjacent_nums(irow, pos);
            match &around[..] {
                &[a, b] => Some(a * b),
                _ => None,
            }
        })
        .sum();
    
    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (4361, 467835));
}

const INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
