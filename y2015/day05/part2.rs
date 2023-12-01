fn to_key(a: u8, b: u8) -> u16 {
    (u16::from(a) << u8::BITS) | u16::from(b)
}

fn is_nice(s: &str) -> bool {
    use std::collections::hash_map::{Entry, HashMap};

    let s = s.as_bytes();
    let mut has_repeating_pair = false;
    let mut has_triple = false;
    let mut positions = HashMap::with_capacity(s.len());

    for (i, triple) in s.windows(3).enumerate() {
        let &[a, b, c] = triple else { unreachable!() };
        has_triple |= a == c;
        if !has_repeating_pair {
            match positions.entry(to_key(a, b)) {
                Entry::Vacant(e) => drop(e.insert(i)),
                Entry::Occupied(e) => {
                    if i - *e.get() > 1 {
                        has_repeating_pair = true;
                    }
                }
            }
        }

        if has_repeating_pair && has_triple {
            return true;
        }
    }

    if !has_triple {
        return false;
    }

    // windows(3) does not cover the last pair, handle it separately
    if let &[.., x, y] = s {
        return positions
            .get(&to_key(x, y))
            .map_or(false, |&pos| s.len() - 2 - pos > 1);
    }

    false
}

#[test]
fn examples() {
    assert!(is_nice("qjhvhtzxzqqjkmpb"));
    assert!(is_nice("xxyxx"));
    assert!(!is_nice("uurcxstgmygtbstg"));
    assert!(!is_nice("ieodomkazucvgmuy"));
}
