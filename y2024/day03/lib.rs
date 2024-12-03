type Num = u32;

// https://gist.github.com/AnthonyMikh/d582ff0ba1cde2987ab107ec0b05772d
#[derive(Clone)]
struct Lexer<'a> {
    s: &'a str,
}

impl<'a> Lexer<'a> {
    fn of(s: &'a str) -> Self {
        Self { s }
    }

    fn shift(&mut self, pos: usize) {
        self.s = &self.s[pos..];
    }

    fn literal(&mut self, literal: &str) -> Option<()> {
        self.s = self.s.strip_prefix(literal)?;
        Some(())
    }

    fn number<Num: std::str::FromStr>(&mut self) -> Option<Num> {
        let pos = self
            .s
            .as_bytes()
            .iter()
            .position(|ch| !ch.is_ascii_digit())
            .unwrap_or(self.s.len());
        let ret = self.s[..pos].parse().ok()?;
        self.shift(pos);
        Some(ret)
    }
}

enum Instruction {
    Do,
    Dont,
    Mul(Num, Num),
}

#[derive(Clone, Copy)]
enum TokenStart {
    Do,
    Dont,
    Mul,
}

impl TokenStart {
    fn as_str(self) -> &'static str {
        match self {
            Self::Do => "do()",
            Self::Dont => "don't()",
            Self::Mul => "mul(",
        }
    }
}

impl Instruction {
    fn locate_next(s: &str) -> Option<(usize, TokenStart)> {
        use TokenStart::*;

        let pos_of = |tok: TokenStart| s.find(tok.as_str()).map(|i| (i, tok));

        // Something like Aho-Corasick for simultaneous string search would be
        // better, but I don't want to bother with external dependencies.
        IntoIterator::into_iter(pos_of(Do))
            .chain(pos_of(Dont))
            .chain(pos_of(Mul))
            .min_by_key(|&(pos, _)| pos)
    }

    fn lex(l: &mut Lexer<'_>) -> Option<Self> {
        while let Some((pos, tok)) = Self::locate_next(&l.s) {
            l.shift(pos + tok.as_str().len());
            match tok {
                TokenStart::Do => return Some(Self::Do),
                TokenStart::Dont => return Some(Self::Dont),
                TokenStart::Mul => {
                    // try block would be very handy here
                    let mul = (|| {
                        let a = l.number()?;
                        l.literal(",")?;
                        let b = l.number()?;
                        l.literal(")")?;
                        Some((a, b))
                    })();
                    if let Some((a, b)) = mul {
                        return Some(Self::Mul(a, b));
                    }
                }
            }
        }

        None
    }
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    let mut l = Lexer::of(input);
    std::iter::from_fn(move || Instruction::lex(&mut l))
}

pub fn solve_both(input: &str) -> (Num, Num) {
    let instructions = parse(input).collect::<Vec<_>>();

    let first = instructions
        .iter()
        .filter_map(|i| match i {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum();

    let second = {
        let mut enabled = true;
        let mut sum = 0;

        for i in &instructions {
            match i {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                &Instruction::Mul(a, b) => {
                    if enabled {
                        sum += a * b;
                    }
                }
            }
        }

        sum
    };

    (first, second)
}

#[test]
fn example() {
    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(solve_both(INPUT), (161, 48));
}
