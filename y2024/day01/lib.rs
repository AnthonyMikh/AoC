pub type Num = u32;
const DELIMITER: &str = "   ";

fn parse(input: &str) -> (Vec<Num>, Vec<Num>) {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut iter = line.split(DELIMITER).map(|i| i.parse::<Num>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .unzip()
}

pub fn solve_both(input: &str) -> (Num, Num) {
    let (mut left, mut right) = parse(input);
    let answer1 = {
        left.sort_unstable();
        right.sort_unstable();
        left.iter().zip(&right).map(|(&l, &r)| l.abs_diff(r)).sum()
    };
    let answer2 = {
        let mut counts = std::collections::HashMap::new();
        for &n in &right {
            *counts.entry(n).or_insert(0) += 1;
        }
        left.iter()
            .map(|&n| n * counts.get(&n).copied().unwrap_or(0))
            .sum()
    };
    (answer1, answer2)
}

#[test]
fn example() {
    const INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(solve_both(INPUT), (11, 31));
}
