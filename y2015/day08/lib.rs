fn escape_excess(s: &str) -> usize {
    let mut ret = 0;
    let mut iter = s.as_bytes().iter().copied();
    while let Some(ch) = iter.next() {
        if ch != b'\\' {
            continue;
        }
        match iter.next().unwrap() {
            b'\\' => ret += 1,
            b'"' => ret += 1,
            b'x' => {
                ret += 3;
                iter.next().unwrap();
                iter.next().unwrap();
            }
            _ => unreachable!(),
        }
    }
    ret
}

fn count_escape_additional(s: &str) -> usize {
    s.matches(&['\\', '"'][..]).count()
}

fn solve_first(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| escape_excess(&l[1..l.len() - 1]) + 2)
        .sum()
}

fn solve_second(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| count_escape_additional(l) + 2)
        .sum()
}

#[test]
fn example() {
    assert_eq!(solve_first(INPUT), 12);
    assert_eq!(solve_second(INPUT), 19);
}

const INPUT: &str = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;
