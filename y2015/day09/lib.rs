#![cfg(test)]

use std::collections::HashMap;

#[derive(Default)]
struct Interner<'i> {
    bag: HashMap<&'i str, usize>,
    strs: Vec<&'i str>,
}

impl<'i> Interner<'i> {
    fn insert(&mut self, s: &'i str) -> usize {
        if let Some(&ret) = self.bag.get(s) {
            return ret;
        }
        let ret = self.strs.len();
        self.strs.push(s);
        self.bag.insert(s, ret);
        ret
    }
}

type Distance = u32;

struct Line<'a> {
    from: &'a str,
    to: &'a str,
    distance: Distance,
}

impl<'a> Line<'a> {
    fn parse(s: &'a str) -> Self {
        let (cities, dist) = s.split_once(" = ").unwrap();
        let (from, to) = cities.split_once(" to ").unwrap();
        let distance = dist.parse().unwrap();
        Self { from, to, distance }
    }
}

fn recurse_min(
    distances: &HashMap<(usize, usize), Distance>,
    visited: &mut [bool],
    current: usize,
) -> Distance {
    if visited.iter().all(bool::clone) {
        return 0;
    }

    let n_cities = visited.len();
    let mut ret = Distance::MAX;
    for city in 0..n_cities {
        if visited[city] {
            continue;
        }
        visited[city] = true;
        let distance = distances[&(current, city)];
        ret = ret.min(recurse_min(distances, visited, city) + distance);
        visited[city] = false;
    }

    ret
}

fn recurse_max(
    distances: &HashMap<(usize, usize), Distance>,
    visited: &mut [bool],
    current: usize,
) -> Distance {
    if visited.iter().all(bool::clone) {
        return 0;
    }

    let n_cities = visited.len();
    let mut ret = 0;
    for city in 0..n_cities {
        if visited[city] {
            continue;
        }
        visited[city] = true;
        let distance = distances[&(current, city)];
        ret = ret.max(recurse_max(distances, visited, city) + distance);
        visited[city] = false;
    }

    ret
}

fn solve_both(input: &str) -> (Distance, Distance) {
    let mut distances = HashMap::new();
    let mut int = Interner::default();
    for Line { from, to, distance } in input.lines().filter(|l| !l.is_empty()).map(Line::parse) {
        let from = int.insert(from);
        let to = int.insert(to);
        distances.insert((from, to), distance);
        distances.insert((to, from), distance);
    }

    let n_cities = int.strs.len();
    let mut visited = vec![false; n_cities];
    let mut best = Distance::MAX;
    let mut worst = 0;

    for city in 0..n_cities {
        visited[city] = true;
        best = recurse_min(&distances, &mut visited, city).min(best);
        visited[city] = false;
    }

    for city in 0..n_cities {
        visited[city] = true;
        worst = recurse_max(&distances, &mut visited, city).max(worst);
        visited[city] = false;
    }

    (best, worst)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (605, 982));
}

const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
