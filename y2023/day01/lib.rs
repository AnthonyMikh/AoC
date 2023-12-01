type Num = u32;

fn extract1(s: &str) -> Num {
    let mut iter = s.as_bytes().iter().copied().filter(u8::is_ascii_digit);
    let a = iter.clone().next().unwrap() - b'0';
    let b = iter.next_back().unwrap() - b'0';
    a as Num * 10 + b as Num
}

fn extract2(s: &str) -> Num {
    const WORDS: &[(&str, Num)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let by_idx = |&(idx, _): &_| idx;

    let from_word = WORDS
        .iter()
        .filter_map(|&(w, val)| Some((s.find(w)?, val)))
        .min_by_key(by_idx);
    let from_digit = s
        .as_bytes()
        .iter()
        .copied()
        .enumerate()
        .find(|&(_idx, n)| n.is_ascii_digit())
        .map(|(i, n)| (i, (n - b'0') as Num));
    let (_, first) = match (from_word, from_digit) {
        (None, None) => unreachable!(),
        (Some(v), None) | (None, Some(v)) => v,
        (Some(a), Some(b)) => std::cmp::min_by_key(a, b, by_idx),
    };

    let from_word = WORDS
        .iter()
        .filter_map(|&(w, val)| Some((s.rfind(w)?, val)))
        .max_by_key(by_idx);
    let from_digit = s
        .as_bytes()
        .iter()
        .copied()
        .enumerate()
        .rev()
        .find(|&(_idx, n)| n.is_ascii_digit())
        .map(|(i, n)| (i, (n - b'0') as Num));
    let (_, second) = match (from_word, from_digit) {
        (None, None) => unreachable!(),
        (Some(v), None) | (None, Some(v)) => v,
        (Some(a), Some(b)) => std::cmp::max_by_key(a, b, by_idx),
    };

    first * 10 + second
}

fn solve(s: &str, extract: impl Fn(&str) -> Num) -> Num {
    s.lines().filter(|line| !line.is_empty()).map(extract).sum()
}

#[test]
fn first() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let answer = solve(input, extract1);
    assert_eq!(answer, 142);
}

#[test]
fn second() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let answer = solve(input, extract2);
    assert_eq!(answer, 281);
}
