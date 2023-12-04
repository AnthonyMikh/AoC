#![cfg(test)]

type LotNumber = u8;
type Score = u32;

fn n_common(s: &str) -> usize {
    use std::collections::HashSet;

    let collect = |s: &str| {
        s.split(' ')
            .filter(|l| !l.is_empty())
            .map(|n| n.parse().unwrap())
            .collect::<HashSet<LotNumber>>()
    };
    let (_name, nums) = s.split_once(": ").unwrap();
    let (winning, actual) = nums.split_once(" | ").unwrap();
    collect(winning).intersection(&collect(actual)).count()
}

fn score(n_common: usize) -> Score {
    if n_common == 0 {
        return 0;
    }
    (2 as Score).pow(n_common as u32 - 1)
}

fn solve_both(input: &str) -> (Score, usize) {
    let mut cards = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| (1, n_common(l)))
        .collect::<Vec<_>>();
    let cards = &mut cards[..];

    let first = cards
        .iter()
        .map(|&(_times, n_common)| score(n_common))
        .sum();

    for i in 0..cards.len() {
        let (inc, n_common) = cards[i];
        cards[i + 1..]
            .iter_mut()
            .take(n_common)
            .for_each(|(times, _)| *times += inc);
    }
    let second = cards.iter().map(|&(times, _)| times).sum();

    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (13, 30));
}

const INPUT: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
