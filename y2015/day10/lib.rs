#![cfg(test)]

struct Prefix {
    elem: u8,
    times: usize,
}

impl Prefix {
    fn of(s: &mut &str) -> Option<Self> {
        let elem = *s.as_bytes().first()?;
        let times = s
            .as_bytes()
            .iter()
            .position(|&ch| ch != elem)
            .unwrap_or(s.len());
        *s = &s[times..];
        Some(Self { elem, times })
    }
}

fn step(mut from: &str, to: &mut String) {
    use std::fmt::Write;

    while let Some(Prefix { elem, times }) = Prefix::of(&mut from) {
        let elem = elem - b'0';
        _ = write!(to, "{times}{elem}");
    }
}

fn solve_both(input: &str, n_steps_first: usize, n_steps_second: usize) -> (usize, usize) {
    let mut from = String::from(input);
    let mut to = String::new();

    for _ in 0..n_steps_first {
        step(&from, &mut to);
        std::mem::swap(&mut from, &mut to);
        to.clear();
    }
    let first = from.len();

    for _ in n_steps_first..n_steps_second {
        step(&from, &mut to);
        std::mem::swap(&mut from, &mut to);
        to.clear();
    }
    let second = from.len();

    (first, second)
}

#[test]
fn user_input() {
    assert_eq!(solve_both("1113122113", 40, 50), (360154, 5103798));
}
