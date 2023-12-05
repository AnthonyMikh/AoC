#![cfg(test)]

type Amount = u8;

const N_TRAITS: usize = 10;

#[derive(Clone, Copy)]
enum Criterion {
    Exact,
    GreaterThen,
    LessThen,
}

fn trait_to_idx(trait_: &str) -> usize {
    match trait_ {
        "children" => 0,
        "cats" => 1,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 4,
        "vizslas" => 5,
        "goldfish" => 6,
        "trees" => 7,
        "cars" => 8,
        "perfumes" => 9,
        _ => unreachable!(),
    }
}

#[rustfmt::skip]
static IDX_TO_CRITERION: [Criterion; N_TRAITS] = [
    Criterion::Exact,       // children
    Criterion::GreaterThen, // cats
    Criterion::Exact,       // samoyeds
    Criterion::LessThen,    // pomeranians
    Criterion::Exact,       // akitas
    Criterion::Exact,       // vizslas
    Criterion::LessThen,    // goldfish
    Criterion::GreaterThen, // trees
    Criterion::Exact,       // cars
    Criterion::Exact,       // perfumes
];

struct Description {
    traits: [Amount; N_TRAITS],
}

struct PartialDescription {
    traits: Description,
    known: [bool; N_TRAITS],
}

impl PartialDescription {
    fn parse(s: &str) -> Self {
        let mut traits = [0; N_TRAITS];
        let mut known = [false; N_TRAITS];

        let (_prefix, parts) = s.split_once(": ").unwrap();
        for part in parts.split(", ") {
            let (trait_, amount) = part.split_once(": ").unwrap();
            let idx = trait_to_idx(trait_);
            assert!(!known[idx]);
            traits[idx] = amount.parse().unwrap();
            known[idx] = true;
        }

        Self {
            traits: Description { traits },
            known,
        }
    }

    fn matches_plain(&self, target: &Description) -> bool {
        (0..N_TRAITS).all(|i| {
            !self.known[i] || self.traits.traits[i] == target.traits[i]
        })
    }
    
    fn matches_accurately(&self, target: &Description) -> bool {
        #[rustfmt::skip]
        let ret = (0..N_TRAITS).all(|i| {
            if !self.known[i] {
                return true;
            }
            match IDX_TO_CRITERION[i] {
                Criterion::Exact       => self.traits.traits[i] == target.traits[i],
                Criterion::GreaterThen => self.traits.traits[i]  > target.traits[i],
                Criterion::LessThen    => self.traits.traits[i]  < target.traits[i],
            }
        });
        ret
    }
}

const AUNT_SUE: Description = Description {
    traits: [3, 7, 2, 3, 0, 0, 5, 3, 2, 1],
};

fn solve_both(input: &str) -> (usize, usize) {
    let candidates = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(PartialDescription::parse)
        .collect::<Vec<_>>();
    let first = candidates
        .iter()
        .enumerate()
        .find_map(|(i, d)| d.matches_plain(&AUNT_SUE).then_some(i + 1))
        .unwrap();
    let second = candidates
        .iter()
        .enumerate()
        .find_map(|(i, d)| d.matches_accurately(&AUNT_SUE).then_some(i + 1))
        .unwrap();
    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (2, 3));
}

const INPUT: &str = "
Sue 1: children: 0
Sue 2: children: 3, cats: 7, goldfish: 5
Sue 2: children: 3, cats: 8, goldfish: 4
";
