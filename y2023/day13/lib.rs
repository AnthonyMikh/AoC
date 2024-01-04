trait NDiff {
    fn n_diff(&self, other: &Self) -> usize;
    fn eq(&self, other: &Self) -> bool {
        self.n_diff(other) == 0
    }
}

impl NDiff for u8 {
    fn n_diff(&self, other: &Self) -> usize {
        (*self != *other) as _
    }
}

impl<'a> NDiff for &'a str {
    fn n_diff(&self, other: &Self) -> usize {
        self.as_bytes()
            .iter()
            .zip(other.as_bytes())
            .map(|(&a, &b)| (a != b) as usize)
            .sum()
    }
}

trait Checker: Default {
    fn put<T: NDiff>(&mut self, seq: &[T]) -> Result<(), ()>;
    fn seq_is_ok<'a, T: NDiff + 'a>(seq: impl IntoIterator<Item = &'a [T]>) -> bool {
        let mut checker = Self::default();
        seq.into_iter().try_for_each(|seq| checker.put(seq)).is_ok()
    }
}

#[derive(Default)]
struct IsPalindrome;

impl Checker for IsPalindrome {
    fn put<T: NDiff>(&mut self, mut seq: &[T]) -> Result<(), ()> {
        loop {
            match seq {
                [] | [_] => break Ok(()),
                [first, middle @ .., last] => {
                    if !first.eq(last) {
                        break Err(());
                    }
                    seq = middle;
                }
            }
        }
    }
}

#[derive(Default)]
struct IsDiff1Palindrome {
    has_diff: bool,
}

impl Checker for IsDiff1Palindrome {
    fn put<T: NDiff>(&mut self, mut seq: &[T]) -> Result<(), ()> {
        loop {
            match seq {
                [] | [_] => break Ok(()),
                [first, middle @ .., last] => {
                    match first.n_diff(last) {
                        0 => (),
                        1 if !self.has_diff => self.has_diff = true,
                        _ => break Err(()),
                    }
                    seq = middle;
                }
            }
        }
    }
}

fn vert_left_offset<C: Checker>(strs: &[&str], accept: impl Fn(Summary) -> bool) -> Option<usize> {
    let width = strs[0].len();
    (0..width - 1)
        .filter(|&offset| (width - offset) % 2 == 0)
        .filter(|&offset| C::seq_is_ok(strs.iter().map(|s| s[offset..].as_bytes())))
        .map(|offset| offset + (width - offset) / 2)
        .filter(|&ret| accept(Summary::Vertical(ret)))
        .next()
}

fn vert_right_offset<C: Checker>(strs: &[&str], accept: impl Fn(Summary) -> bool) -> Option<usize> {
    let width = strs[0].len();
    (2..width)
        .filter(|&cutoff| cutoff % 2 == 0)
        .filter(|&cutoff| C::seq_is_ok(strs.iter().map(|s| s[..cutoff].as_bytes())))
        .map(|cutoff| cutoff / 2)
        .filter(|&ret| accept(Summary::Vertical(ret)))
        .next()
}

fn horiz_top_offset<C: Checker>(strs: &[&str], accept: impl Fn(Summary) -> bool) -> Option<usize> {
    let height = strs.len();
    (0..height - 1)
        .filter(|&offset| (height - offset) % 2 == 0)
        .filter(|&offset| C::seq_is_ok(Some(&strs[offset..])))
        .map(|offset| offset + (height - offset) / 2)
        .filter(|&ret| accept(Summary::Horizontal(ret)))
        .next()
}

fn horiz_bottom_offset<C: Checker>(
    strs: &[&str],
    accept: impl Fn(Summary) -> bool,
) -> Option<usize> {
    let height = strs.len();
    (2..height)
        .filter(|&cutoff| cutoff % 2 == 0)
        .filter(|&cutoff| C::seq_is_ok(Some(&strs[..cutoff])))
        .map(|cutoff| cutoff / 2)
        .filter(|&ret| accept(Summary::Horizontal(ret)))
        .next()
        
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Summary {
    Vertical(usize),
    Horizontal(usize),
}

impl Summary {
    fn into_score(self) -> usize {
        match self {
            Summary::Vertical(offset) => offset,
            Summary::Horizontal(offset) => offset * 100,
        }
    }
}

fn summarize<C: Checker>(s: &str, accept: impl Fn(Summary) -> bool) -> Summary {
    let lines = &s.lines().collect::<Vec<_>>();
    if let Some(offset) =
        vert_left_offset::<C>(lines, &accept).or_else(|| vert_right_offset::<C>(lines, &accept))
    {
        return Summary::Vertical(offset);
    }
    if let Some(offset) =
        horiz_top_offset::<C>(lines, &accept).or_else(|| horiz_bottom_offset::<C>(lines, &accept))
    {
        return Summary::Horizontal(offset);
    }
    unreachable!("pattern should be reflective:\n{lines:#?}")
}

pub fn solve_both(input: &str) -> (usize, usize) {
    input
        .trim_matches('\n')
        .split("\n\n")
        .map(|grid| {
            let first = summarize::<IsPalindrome>(grid, |_| true);
            let second = summarize::<IsDiff1Palindrome>(grid, |ret| ret != first);
            (first.into_score(), second.into_score())
        })
        .fold((0, 0), |(ff, ss), (f, s)| (ff + f, ss + s))
}

#[test]
fn example() {
    const INPUT: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    assert_eq!(solve_both(INPUT), (405, 400));
}
