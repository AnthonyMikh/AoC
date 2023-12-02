#![cfg(test)]

const LEN: usize = 8;
type Bytes = [u8; LEN];

struct Password(Bytes);

impl Password {
    fn from_str(s: &str) -> Self {
        let mut bytes = <&Bytes>::try_from(s.as_bytes()).unwrap().clone();
        bytes.iter_mut().for_each(|b| *b -= b'a');
        Self(bytes)
    }

    fn increment(&mut self) {
        const MAX: u8 = b'z' - b'a';

        for digit in self.0.iter_mut().rev() {
            *digit += 1;
            if *digit > MAX {
                *digit = 0;
                continue;
            }
            break;
        }
    }
}

fn has_no_confusables(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .all(|&b| !matches!(b + b'a', b'i' | b'o' | b'l'))
}

fn has_ascend(bytes: &[u8]) -> bool {
    bytes
        .windows(3)
        .map(|w| <&[u8; 3]>::try_from(w).unwrap())
        .any(|&[a, b, c]| b.wrapping_sub(a) == 1 && c.wrapping_sub(a) == 2)
}

fn has_different_pairs(bytes: &[u8]) -> bool {
    let mut seen = [false; (b'z' - b'a' + 1) as usize];
    let mut n_seen = 0;

    for b in bytes
        .iter()
        .zip(&bytes[1..])
        .filter_map(|(&a, &b)| (a == b).then_some(a))
    {
        if !std::mem::replace(&mut seen[b as usize], true) {
            n_seen += 1;
        }
        if n_seen >= 2 {
            return true;
        }
    }

    false
}

fn is_valid(bytes: &Bytes) -> bool {
    has_no_confusables(bytes) && has_ascend(bytes) && has_different_pairs(bytes)
}

fn solve(input: &str) -> String {
    let mut password = Password::from_str(input);
    let mut answer = std::iter::from_fn(|| {
        password.increment();
        Some(password.0)
    })
    .filter(is_valid)
    .next()
    .unwrap();
    answer.iter_mut().for_each(|b| *b += b'a');
    std::str::from_utf8(&answer).unwrap().to_owned()
}

#[test]
fn example() {
    assert_eq!(solve("abcdefgh"), "abcdffaa");
    assert_eq!(solve("ghijklmn"), "ghjaabcc");
}
