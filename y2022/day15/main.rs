use std::ops::ControlFlow;

type Coord = i64;

#[derive(Clone, Copy)]
struct Pos {
    x: Coord,
    y: Coord,
}

impl Pos {
    fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }

    fn distance_to(self, other: Self) -> Coord {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as _
    }
}

struct Sensor {
    position: Pos,
    beacon: Pos,
    beacon_dist: Coord,
}

#[derive(Clone)]
struct Lexer<'a> {
    s: &'a str,
}

impl<'a> Lexer<'a> {
    fn of(s: &'a str) -> Self {
        Self { s }
    }

    fn shift(&mut self, pos: usize) {
        self.s = &self.s[pos..];
    }

    fn end(&mut self) -> Option<()> {
        if self.s.is_empty() {
            Some(())
        } else {
            None
        }
    }

    fn literal(&mut self, literal: &str) -> Option<()> {
        self.s = self.s.strip_prefix(literal)?;
        Some(())
    }

    fn number<Num: std::str::FromStr>(&mut self) -> Option<Num> {
        let offset = self.s.starts_with('-') as usize;
        let offsetted = &self.s[offset..];
        let pos = offsetted
            .as_bytes()
            .iter()
            .position(|ch| !ch.is_ascii_digit())
            .unwrap_or(offsetted.len())
            + offset;
        let ret = self.s[..pos].parse().ok()?;
        self.shift(pos);
        Some(ret)
    }
}

impl Sensor {
    fn parse(s: &str) -> Self {
        let mut p = Lexer::of(s);
        p.literal("Sensor at x=").unwrap();
        let x = p.number().unwrap();
        p.literal(", y=").unwrap();
        let y = p.number().unwrap();
        let position = Pos { x, y };
        p.literal(": closest beacon is at x=").unwrap();
        let x = p.number().unwrap();
        p.literal(", y=").unwrap();
        let y = p.number().unwrap();
        p.end().unwrap();
        let beacon = Pos { x, y };
        Self {
            position,
            beacon,
            beacon_dist: position.distance_to(beacon),
        }
    }

    fn cover_row(&self, row: Coord) -> Option<(Segment, Option<Coord>)> {
        let vert_dist = self.position.y.abs_diff(row) as Coord;
        let half_width = if vert_dist > self.beacon_dist {
            return None;
        } else {
            self.beacon_dist - vert_dist
        };
        let center = self.position.x;
        let range = Segment::new(center - half_width, center + half_width);
        let beacon_x = (self.beacon.y == row).then_some(self.beacon.x);
        Some((range, beacon_x))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Segment {
    start: Coord,
    end: Coord,
}

impl Segment {
    fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }

    // only merges when `self` preceds `other`
    fn try_merge(self, other: Self) -> Option<Self> {
        if self.end + 1 >= other.start && self.start <= other.end {
            Some(Self {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }

    fn iter(self) -> impl Iterator<Item = Coord> {
        self.start..=self.end
    }

    fn cut(self, removed: Self) -> Split<Self> {
        if removed.start <= self.start {
            // self:         [.......
            // removed:  [.........
            if removed.end >= self.end {
                // self:         [...]
                // removed:  [.........]
                Split::Empty
            } else {
                // self:         [.....]
                // removed:  [......]
                Split::Single(Segment::new(self.start.max(removed.end + 1), self.end))
            }
        } else {
            // self:     [.......
            // removed:     [.........
            if removed.end >= self.end {
                // self:     [.......]
                // removed:     [.........]
                Split::Single(Segment::new(self.start, self.end.min(removed.start - 1)))
            } else {
                // self:     [...........]
                // removed:     [....]
                Split::Pair(
                    Segment::new(self.start, removed.start - 1),
                    Segment::new(removed.end + 1, self.end),
                )
            }
        }
    }
}

fn solve1(sensors: &[Sensor], target_row: Coord) -> usize {
    let mut covered = Vec::new();
    let mut beacons = Vec::new();
    for sensor in sensors {
        if let Some((seg, beacon)) = sensor.cover_row(target_row) {
            covered.push(seg);
            beacons.extend(beacon);
        }
    }
    beacons.sort_unstable();
    beacons.dedup();
    covered.sort_unstable();
    covered.dedup_by(|r, l| {
        if let Some(merged) = (*l).try_merge(*r) {
            *l = merged;
            true
        } else {
            false
        }
    });
    covered
        .iter()
        .flat_map(|&s| s.iter())
        .filter(|x| !beacons.contains(x))
        .count()
}

#[derive(Debug)]
enum Split<T> {
    Empty,
    Single(T),
    Pair(T, T),
}

fn locate_uncovered(mut range: Segment, mut covered: &[Segment]) -> ControlFlow<Coord> {
    while let Some((&first, rest)) = covered.split_first() {
        let split = range.cut(first);
        match split {
            Split::Empty => return ControlFlow::Continue(()),
            Split::Single(one) => range = one,
            Split::Pair(a, b) => {
                locate_uncovered(a, rest)?;
                locate_uncovered(b, rest)?;
                return ControlFlow::Continue(());
            }
        }
        covered = rest;
    }
    ControlFlow::Break(range.start)
}

struct SearchArea {
    horizontal: Segment,
    vertical: Segment,
}

fn locate(sensors: &[Sensor], search_area: &SearchArea) -> Option<Pos> {
    for y in search_area.vertical.iter() {
        let covered = sensors
            .iter()
            .filter_map(|s| Some(s.cover_row(y)?.0))
            .collect::<Vec<_>>();
        if let ControlFlow::Break(x) = locate_uncovered(search_area.horizontal, &covered) {
            return Some(Pos::new(x, y));
        }
    }
    None
}

fn solve2(sensors: &[Sensor], search_area: &SearchArea) -> Coord {
    let Pos { x, y } = locate(sensors, search_area).expect("not found");
    x * 4_000_000 + y
}

fn solve(input: &str, target_row: Coord, search_area: &SearchArea) -> (usize, Coord) {
    let sensors = input.lines().map(Sensor::parse).collect::<Vec<_>>();
    (solve1(&sensors, target_row), solve2(&sensors, search_area))
}

fn main() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    let target_row = 10;
    let search_area = SearchArea {
        horizontal: Segment::new(0, 20),
        vertical: Segment::new(0, 20),
    };
    println!("{:?}", solve(input, target_row, &search_area));
}
