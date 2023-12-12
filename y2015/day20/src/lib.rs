pub use primal::Sieve;

pub type Num = usize;

fn sum_of_divisors(sieve: &Sieve, n: Num) -> Num {
    sieve
        .factor(n)
        .unwrap()
        .into_iter()
        .map(|(p, k)| (0..=k).map(|k| p.pow(k as _)).sum::<Num>())
        .product::<Num>()
}

fn n_presents(sieve: &Sieve, n_house: Num) -> Num {
    sum_of_divisors(sieve, n_house) * 10
}

pub fn solve_first(sieve: &Sieve, n_target: Num) -> Num {
    (1..)
        .find(|&n_house| n_presents(sieve, n_house) >= n_target)
        .unwrap()
}

fn n_presents_modified(n_house: Num) -> Num {
    (1..=50)
        .filter(|elf| n_house % elf == 0)
        .map(|elf| n_house / elf)
        .sum::<Num>() * 11
}

pub fn solve_second(n_target: Num) -> Num {
    (1..).find(|&n_house| n_presents_modified(n_house) >= n_target).unwrap()
}

pub fn solve_both(input: Num) -> (Num, Num) {
    let sieve = Sieve::new(input / 10);
    let first = solve_first(&sieve, input);
    let second = solve_second(input);
    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(34000000), (786240, 831600));
}
