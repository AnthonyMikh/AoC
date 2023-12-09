pub type Num = i32;

fn parse_seq(s: &str) -> Vec<Num> {
    s.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn extrapolate(original: Vec<Num>) -> (Num, Num) {
    let mut seqs = Vec::new();
    let mut current = original;

    loop {
        let mut all_zeroes = true;
        let next = current
            .iter()
            .zip(&current[1..])
            .map(|(&a, &b)| b - a)
            .inspect(|&diff| all_zeroes &= diff == 0)
            .collect::<Vec<_>>();
        seqs.push(current);
        if all_zeroes {
            break;
        }
        current = next;
    }

    seqs.iter()
        .rev()
        .fold((0, 0), |(diff_first, diff_last), seq| {
            (
                *seq.first().unwrap() - diff_first,
                *seq.last().unwrap() + diff_last,
            )
        })
}

pub fn solve_both(input: &str) -> (Num, Num) {
    let (second, first) = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_seq)
        .map(extrapolate)
        .fold((0, 0), |(xs, ys), (x, y)| (xs + x, ys + y));
    (first, second)
}

#[test]
fn example() {
    let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(solve_both(input), (114, 2))
}
