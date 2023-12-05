#![cfg(test)]

use std::ops::Range;

type Num = u64;

struct Projection {
    source_range: Range<Num>,
    target_start: Num,
}

impl Projection {
    fn parse(s: &str) -> Self {
        let mut nums = s.split(' ');
        let mut next = || nums.next().unwrap().parse().unwrap();
        let target_start = next();
        let start = next();
        let len = next();
        assert!(nums.next().is_none());
        Self {
            source_range: start..start + len,
            target_start,
        }
    }

    fn project(&self, val: Num) -> Num {
        self.target_start + (val - self.source_range.start)
    }

    fn project_range(&self, range: Range<Num>) -> Range<Num> {
        self.project(range.start)..self.project(range.end)
    }
}

#[derive(Default)]
struct Map {
    ranges: Vec<Projection>,
}

impl Map {
    fn project(&self, val: Num) -> Num {
        use std::cmp::Ordering;

        match self.ranges.binary_search_by(|p| {
            if p.source_range.contains(&val) {
                Ordering::Equal
            } else {
                p.source_range.start.cmp(&val)
            }
        }) {
            Ok(i) => self.ranges[i].project(val),
            Err(_) => val,
        }
    }

    fn parse(s: &str) -> Self {
        let mut ranges = s
            .lines()
            .skip(1)
            .filter(|l| !l.is_empty())
            .map(Projection::parse)
            .collect::<Vec<_>>();
        ranges.sort_unstable_by_key(|p| p.source_range.start);
        // in real world we should have checked that ranges do not overlap

        Self { ranges }
    }
}

fn probe_range(seeds: Range<Num>, map: &Map, out: &mut Vec<Range<Num>>) {
    let mut current = seeds;

    // skip map ranges which are strictly before seeds range
    let ranges = match map
        .ranges
        .iter()
        .position(|p| !(p.source_range.end <= current.start))
    {
        Some(i) => &map.ranges[i..],
        // no overlap with map ranges, so `seeds` is mapped unchanged
        None => {
            out.push(current);
            return;
        }
    };

    for p in ranges {
        let range = p.source_range.clone();

        // current:  [...)
        // range:           [...
        if current.end <= range.start {
            if current.end != current.start {
                out.push(current);
            }
            return;
        }

        // current:  [......)
        // range:    ...)
        if range.end <= current.end {
            // current:     [......)
            // range:    [.....)
            if range.start <= current.start {
                let overlap = current.start..range.end;
                current = range.end..current.end;
                out.push(p.project_range(overlap));
            }
            // current:  [...........)
            // range:      [.....)
            else {
                let prefix = current.start..range.start;
                let overlap = range.clone();
                let suffix = range.end..current.end;

                out.push(prefix);
                out.push(p.project_range(overlap));

                current = suffix;
            }
        }
        // current:  [......)
        // range:         .....)
        else {
            // current:     [......)
            // range:    [.............)
            if range.start <= current.start {
                out.push(p.project_range(current));
                return;
            }
            // current:  [......)
            // range:       [.............)
            else {
                let prefix = current.start..range.start;
                let overlap = range.start..current.end;

                out.push(prefix);
                out.push(p.project_range(overlap));
                return;
            }
        }
    }
}

fn map_ranges(initial: Vec<Range<Num>>, maps: &[Map]) -> Vec<Range<Num>> {
    let mut current = initial;
    let mut next = Vec::new();

    for map in maps {
        for range in &current {
            probe_range(range.clone(), map, &mut next);
        }
        current.clear();
        std::mem::swap(&mut current, &mut next);
    }

    current
}

fn ranges_from_pairs(seeds: &[Num]) -> Vec<Range<Num>> {
    use itertools::Itertools;
    seeds
        .iter()
        .copied()
        .tuples()
        .map(|(start, len)| start..start + len)
        .collect()
}

fn solve_both(input: &str) -> (Num, Num) {
    let mut parts = input.split("\n\n");
    let seeds = parts
        .next()
        .unwrap()
        .trim_start_matches('\n')
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
    let maps = parts.map(Map::parse).collect::<Vec<_>>();

    let first = seeds
        .iter()
        .map(|&val| maps.iter().fold(val, |val, map| map.project(val)))
        .min()
        .unwrap();

    let second = map_ranges(ranges_from_pairs(&seeds), &maps)
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap();

    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (35, 46));
}

const INPUT: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
