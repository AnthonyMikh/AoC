pub type Num = u32;
const LIMIT: Num = 3;

const INCREASING: bool = true;
const DECREASING: bool = false;

fn parse(input: &str) -> Vec<Vec<Num>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_safe(nums: &[Num]) -> bool {
    let Some((_, rest)) = nums.split_first() else {
        return false;
    };
    let mut pairs = nums.iter().zip(rest);
    let Some((&first, &second)) = pairs.next() else {
        return false;
    };

    if first == second || first.abs_diff(second) > LIMIT {
        return false;
    }
    if first < second {
        pairs.all(|(&a, &b)| is_safe_step::<INCREASING>(a, b))
    } else {
        pairs.all(|(&a, &b)| is_safe_step::<DECREASING>(a, b))
    }
}

fn is_safe_step<const IS_INCREASING: bool>(a: Num, b: Num) -> bool {
    if a == b {
        return false;
    }
    if IS_INCREASING {
        a < b && b - a <= LIMIT
    } else {
        a > b && a - b <= LIMIT
    }
}

fn is_safe_dampened_case<const IS_INCREASING: bool>(nums: &[Num]) -> bool {
    let is_safe_step = is_safe_step::<IS_INCREASING>;
    let is_safe_seq = |nums: &[Num]| {
        nums.iter()
            .zip(&nums[1..])
            .all(|(&a, &b)| is_safe_step(a, b))
    };
    if nums.len() <= 2 {
        return true;
    }

    let mut nums = nums.to_vec();
    for i in 0..nums.len() {
        let removed = nums.remove(i);
        if is_safe_seq(&nums) {
            return true;
        }
        nums.insert(i, removed);
    }

    false
}

fn is_safe_dampened(nums: &[Num]) -> bool {
    is_safe_dampened_case::<INCREASING>(nums) || is_safe_dampened_case::<DECREASING>(nums)
}

pub fn solve_both(input: &str) -> (usize, usize) {
    let reports = parse(input);
    (
        reports.iter().filter(|r| is_safe(r)).count(),
        reports.iter().filter(|r| is_safe_dampened(r)).count(),
    )
}

#[test]
fn example() {
    const INPUT: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(solve_both(INPUT), (2, 4));
}
