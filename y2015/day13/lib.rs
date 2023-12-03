#![cfg(test)]

use itertools::Itertools;

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

type Happiness = i32;

fn parse_line(s: &str) -> (&str, &str, Happiness) {
    let (first, rest) = s.split_once(" would ").unwrap();
    let (multiplier, rest) = if let Some(rest) = rest.strip_prefix("gain ") {
        (1, rest)
    } else if let Some(rest) = rest.strip_prefix("lose ") {
        (-1, rest)
    } else {
        panic!("invalid string {s}");
    };
    let (amount, second_dot) = rest
        .split_once(" happiness units by sitting next to ")
        .unwrap();
    let second = second_dot.strip_suffix(".").unwrap();
    (
        first,
        second,
        amount.parse::<Happiness>().unwrap() * multiplier,
    )
}

fn parse_deltas(s: &str) -> Vec<Vec<Happiness>> {
    let mut int = Interner::default();
    let deltas = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .map(|(fst, snd, delta)| (int.insert(fst), int.insert(snd), delta))
        .collect::<Vec<_>>();
    let len = deltas.iter().map(|x| x.0).max().unwrap() + 1;
    let mut ret = vec![vec![0; len]; len];
    for &(i, j, delta) in &deltas {
        ret[i][j] = delta;
    }
    ret
}

fn best_arrangement(deltas: &[Vec<Happiness>]) -> Happiness {
    let n_guests = deltas.len();
    (0..n_guests)
        .permutations(n_guests)
        .map(|p| {
            p.iter()
                .cycle()
                .tuple_windows()
                .take(n_guests)
                .map(|(&i, &j)| deltas[i][j] + deltas[j][i])
                .sum()
        })
        .max()
        .unwrap()
}

fn solve_both(input: &str) -> (Happiness, Happiness) {
    let mut deltas = parse_deltas(input);
    let n_guests = deltas.len();
    let first = best_arrangement(&deltas);
    deltas.iter_mut().for_each(|dd| dd.push(0));
    deltas.push(vec![0; n_guests + 1]);
    let second = best_arrangement(&deltas);
    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (330, 286));
}

const INPUT: &str = "
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
