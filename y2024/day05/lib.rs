use std::collections::{HashMap, HashSet};

type Page = u16;
type Orderings = HashMap<Page, HashSet<Page>>;

fn parse_orderings(input: &str) -> Orderings {
    let mut ret = HashMap::<_, HashSet<_>>::new();

    for pair in input.lines() {
        let (before, after) = pair.split_once('|').unwrap();
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        ret.entry(before).or_default().insert(after);
    }

    ret
}

fn is_correct_order(orderings: &Orderings, update: &[Page]) -> bool {
    update.iter().enumerate().skip(1).all(|(i, page)| {
        let Some(pages_after) = orderings.get(page) else {
            return true;
        };
        update[..i]
            .iter()
            .all(|before| !pages_after.contains(before))
    })
}

fn parse(input: &str) -> (Orderings, Vec<Vec<Page>>) {
    let (pairs, updates) = input.split_once("\n\n").unwrap();
    let orderings = parse_orderings(pairs);
    let updates = updates
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|p| p.parse().unwrap()).collect())
        .collect();
    (orderings, updates)
}

pub fn solve_both(input: &str) -> (Page, Page) {
    let (orderings, mut updates) = parse(input);
    let order = |a: &_, b: &_| {
        use std::cmp::Ordering::*;

        if orderings.get(a).map_or(false, |after| after.contains(b)) {
            Less
        } else if orderings.get(b).map_or(false, |after| after.contains(a)) {
            Greater
        } else {
            Equal
        }
    };

    updates
        .iter_mut()
        .fold((0, 0), |(mut correct, mut incorrect), update| {
            let imiddle = update.len() / 2;
            if is_correct_order(&orderings, update) {
                correct += update[imiddle];
            } else {
                // Does `orderings` give total order? In general - no, but that
                // being Advent of code it is reasonable to expect that subset
                // of all pages realizes total order (otherwise there would be no
                // unique answer)
                incorrect += *update.select_nth_unstable_by(imiddle, &order).1;
            }
            (correct, incorrect)
        })
}

#[test]
fn example() {
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    assert_eq!(solve_both(INPUT), (143, 123));
}
