use std::cmp::Ordering;

type Num = u32;

#[derive(Clone)]
enum Packet {
    Single(Num),
    List(Vec<Self>),
}

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

    fn end(&mut self) -> Option<()> {
        if self.s.is_empty() {
            Some(())
        } else {
            None
        }
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

impl Packet {
    fn parse(s: &str) -> Self {
        let mut p = Lexer::of(s);
        let ret = Self::parse_recursive(&mut p);
        p.end().unwrap();
        ret
    }

    fn parse_recursive(p: &mut Lexer<'_>) -> Self {
        if p.literal("[").is_some() {
            let mut list = Vec::new();
            if p.literal("]").is_some() {
                return Self::List(list);
            }
            loop {
                list.push(Self::parse_recursive(p));
                if p.literal("]").is_some() {
                    return Self::List(list);
                }
                p.literal(",").unwrap();
            }
        }
        if let Some(num) = p.number() {
            return Self::Single(num);
        }
        unreachable!()
    }

    fn zip_singles<'a, R>(
        &'a self,
        rhs: &'a Self,
        op: impl FnOnce(&Num, &Num) -> R,
    ) -> Result<R, (&'a [Self], &'a [Self])> {
        use std::slice::from_ref;
        use Packet::*;

        let (lhs, rhs): (&[_], &[_]) = match (self, rhs) {
            (Single(x), Single(y)) => return Ok(op(x, y)),
            (x @ Single(_), List(yy)) => (from_ref(x), yy),
            (List(xx), y @ Single(_)) => (xx, from_ref(y)),
            (List(xx), List(yy)) => (xx, yy),
        };
        Err((lhs, rhs))
    }

    // fn array_le(lhs: &[Self], rhs: &[Self]) -> bool {
    //     let [mut lhs, mut rhs] = [lhs, rhs].map(<[_]>::iter);
    //     loop {
    //         break match (lhs.next(), rhs.next()) {
    //             (None, _) => true,
    //             (Some(_), None) => false,
    //             (Some(x), Some(y)) => {
    //                 if !x.le(y) {
    //                     false
    //                 } else {
    //                     continue;
    //                 }
    //             }
    //         };
    //     }
    // }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.zip_singles(other, PartialEq::eq)
            .unwrap_or_else(|(lhs, rhs)| lhs == rhs)
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.zip_singles(other, Ord::cmp)
            .unwrap_or_else(|(lhs, rhs)| lhs.cmp(rhs))
    }
}

fn solve1(packets: &[Packet]) -> usize {
    packets
        .chunks_exact(2)
        .enumerate()
        .filter_map(|(i, c)| (c[0] <= c[1]).then_some(i + 1))
        .sum()
}

const DIVIDER_PACKETS: [&str; 2] = ["[[2]]", "[[6]]"];

fn solve2(packets: &mut [Packet]) -> usize {
    let [first, second] = DIVIDER_PACKETS.map(Packet::parse);
    packets.sort_unstable();
    let ifirst = packets.binary_search(&first).unwrap_err() + 1;
    let isecond = packets.binary_search(&second).unwrap_err() + 1 + 1;
    ifirst * isecond
}

fn solve(input: &str) -> (usize, usize) {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::parse)
        .collect::<Vec<_>>();
    let answer1 = solve1(&packets);
    let answer2 = solve2(&mut packets);
    (answer1, answer2)
}

fn main() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    println!("{:?}", solve(input));
}
