fn s(s: &str) -> Vec<char> {
    s.chars().collect()
}

struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
struct Lexer<'a> {
    s: &'a str,
}

impl<'a> Lexer<'a> {
    fn of(s: &'a str) -> Self {
        Self { s }
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
        self.s = &self.s[pos..];
        Some(ret)
    }
}

fn parse(s: &str) -> Option<Instruction> {
    let mut p = Lexer::of(s);
    p.literal("move ")?;
    let quantity = p.number()?;
    p.literal(" from ")?;
    let from = p.number()?;
    p.literal(" to ")?;
    let to = p.number()?;
    p.end()?;
    Some(Instruction { quantity, from, to })
}

fn interpret(state: &mut Vec<Vec<char>>, ins: Instruction) {
    let Instruction { quantity, from, to } = ins;
    println!("({from}, {to}): {:?} -({quantity})-> {:?}", state[from], state[to]);
    for _ in 0..quantity {
        let item = state[from].pop().unwrap();
        state[to].push(item);
    }
}

fn get_two_mut<T>(
    arr: &mut [T],
    idx1: usize,
    idx2: usize,
) -> (&mut T, &mut T) {
    assert_ne!(idx1, idx2);
    let a: *mut T = &mut arr[idx1];
    let b: *mut T = &mut arr[idx2];
    unsafe { (&mut *a, &mut *b) }
}

fn interpret2(state: &mut Vec<Vec<char>>, ins: Instruction) {
    let Instruction { quantity, from, to } = ins;
    let (from, to) = get_two_mut(state, from, to);
    let kept = from.len() - quantity;
    to.extend_from_slice(&from[kept..]);
    from.truncate(kept);
}

fn solve(state: &mut Vec<Vec<char>>, input: &str) -> String {
    for ins in input.lines() {
        let ins = parse(ins).unwrap();
        // interpret(state, ins);
        interpret2(state, ins);
    }
    state[1..].iter().map(|s| s.last().copied().unwrap()).collect()
}

//         [G]         [D]     [Q]    
// [P]     [T]         [L] [M] [Z]    
// [Z] [Z] [C]         [Z] [G] [W]    
// [M] [B] [F]         [P] [C] [H] [N]
// [T] [S] [R]     [H] [W] [R] [L] [W]
// [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
// [C] [N] [H] [R] [N] [H] [D] [J] [Q]
// [N] [D] [M] [G] [Z] [F] [W] [S] [S]
//  1   2   3   4   5   6   7   8   9 

fn main() {
    // yep, I am too lazy to actually parse this
    // translating all this by hand took less time than I would need
    // to write a parser
    let mut state = vec![
        s(""),
        s("NCRTMZP"),
        s("DNTSBZ"),
        s("MHQRFCTG"),
        s("GRZ"),
        s("ZNRH"),
        s("FHSWPZLD"),
        s("WDZRCGM"),
        s("SJFLHWZQ"),
        s("SQPWN"),
    ];
    let input = "move 7 from 6 to 8
move 5 from 2 to 6
move 2 from 4 to 1
move 1 from 4 to 5
move 5 from 7 to 6
move 7 from 6 to 3
move 5 from 9 to 2
move 6 from 2 to 3
move 2 from 7 to 9
move 20 from 3 to 1
move 11 from 1 to 6
move 1 from 9 to 8
move 3 from 8 to 2
move 8 from 1 to 5
move 10 from 8 to 4
move 7 from 6 to 4
move 1 from 8 to 3
move 8 from 1 to 7
move 16 from 4 to 8
move 1 from 9 to 8
move 1 from 5 to 2
move 4 from 7 to 4
move 5 from 6 to 7
move 1 from 6 to 1
move 8 from 7 to 4
move 1 from 6 to 9
move 12 from 4 to 5
move 3 from 2 to 5
move 1 from 6 to 2
move 1 from 3 to 7
move 1 from 3 to 2
move 1 from 9 to 3
move 1 from 7 to 8
move 1 from 7 to 5
move 1 from 3 to 2
move 4 from 5 to 7
move 5 from 5 to 7
move 1 from 4 to 3
move 1 from 3 to 9
move 3 from 1 to 8
move 1 from 9 to 1
move 2 from 2 to 1
move 2 from 2 to 7
move 8 from 8 to 1
move 3 from 5 to 2
move 8 from 7 to 5
move 7 from 1 to 3
move 3 from 1 to 7
move 1 from 1 to 5
move 1 from 3 to 7
move 7 from 5 to 8
move 2 from 2 to 8
move 1 from 3 to 2
move 1 from 2 to 4
move 1 from 4 to 8
move 13 from 8 to 1
move 13 from 5 to 9
move 2 from 5 to 2
move 7 from 9 to 3
move 12 from 8 to 3
move 4 from 9 to 3
move 1 from 3 to 4
move 2 from 2 to 3
move 1 from 1 to 6
move 1 from 2 to 3
move 1 from 5 to 9
move 7 from 7 to 4
move 10 from 1 to 8
move 1 from 1 to 4
move 1 from 9 to 5
move 2 from 5 to 1
move 1 from 6 to 5
move 3 from 8 to 9
move 5 from 4 to 3
move 4 from 4 to 1
move 7 from 1 to 6
move 2 from 5 to 7
move 35 from 3 to 4
move 4 from 9 to 1
move 19 from 4 to 8
move 1 from 7 to 6
move 1 from 9 to 2
move 10 from 4 to 5
move 2 from 4 to 7
move 3 from 4 to 3
move 1 from 2 to 8
move 1 from 1 to 9
move 3 from 3 to 6
move 4 from 8 to 6
move 4 from 5 to 2
move 2 from 8 to 3
move 3 from 5 to 9
move 12 from 6 to 1
move 8 from 8 to 6
move 2 from 9 to 1
move 1 from 4 to 1
move 1 from 3 to 8
move 3 from 7 to 8
move 2 from 9 to 7
move 1 from 6 to 7
move 10 from 6 to 8
move 4 from 2 to 5
move 1 from 3 to 7
move 7 from 5 to 7
move 13 from 8 to 1
move 29 from 1 to 4
move 8 from 7 to 8
move 1 from 1 to 3
move 3 from 7 to 6
move 1 from 1 to 9
move 15 from 4 to 1
move 1 from 3 to 6
move 10 from 1 to 6
move 10 from 6 to 7
move 1 from 4 to 9
move 1 from 9 to 1
move 1 from 9 to 7
move 6 from 7 to 8
move 1 from 1 to 6
move 5 from 6 to 5
move 21 from 8 to 9
move 5 from 1 to 9
move 2 from 9 to 5
move 3 from 5 to 6
move 3 from 7 to 9
move 4 from 4 to 6
move 6 from 8 to 7
move 6 from 6 to 3
move 2 from 7 to 9
move 1 from 7 to 2
move 6 from 3 to 2
move 1 from 6 to 4
move 4 from 5 to 9
move 1 from 4 to 5
move 9 from 4 to 6
move 7 from 6 to 4
move 10 from 9 to 2
move 5 from 7 to 5
move 10 from 2 to 7
move 2 from 5 to 4
move 2 from 5 to 9
move 4 from 9 to 4
move 1 from 8 to 6
move 7 from 7 to 2
move 1 from 5 to 4
move 2 from 7 to 1
move 1 from 5 to 7
move 3 from 6 to 2
move 4 from 4 to 5
move 1 from 2 to 7
move 10 from 4 to 7
move 3 from 7 to 3
move 17 from 9 to 4
move 1 from 1 to 4
move 1 from 1 to 5
move 5 from 2 to 7
move 1 from 9 to 2
move 5 from 4 to 8
move 2 from 9 to 7
move 4 from 8 to 1
move 3 from 4 to 8
move 1 from 2 to 5
move 1 from 9 to 2
move 6 from 4 to 8
move 3 from 7 to 5
move 1 from 4 to 9
move 1 from 9 to 1
move 3 from 1 to 9
move 4 from 8 to 5
move 2 from 9 to 8
move 4 from 2 to 5
move 8 from 7 to 2
move 5 from 8 to 5
move 2 from 7 to 8
move 1 from 3 to 5
move 1 from 1 to 2
move 1 from 1 to 6
move 2 from 3 to 6
move 5 from 2 to 8
move 4 from 7 to 1
move 7 from 8 to 5
move 1 from 1 to 5
move 3 from 8 to 3
move 1 from 9 to 3
move 7 from 2 to 3
move 2 from 2 to 8
move 2 from 4 to 8
move 1 from 8 to 5
move 1 from 1 to 4
move 2 from 4 to 7
move 2 from 7 to 1
move 3 from 2 to 3
move 3 from 5 to 2
move 1 from 8 to 3
move 3 from 3 to 2
move 5 from 2 to 1
move 17 from 5 to 8
move 9 from 8 to 1
move 11 from 3 to 5
move 8 from 8 to 5
move 2 from 8 to 5
move 16 from 1 to 4
move 13 from 4 to 7
move 6 from 5 to 2
move 2 from 4 to 8
move 5 from 7 to 9
move 2 from 1 to 2
move 7 from 7 to 1
move 1 from 1 to 4
move 1 from 9 to 8
move 7 from 2 to 8
move 1 from 4 to 7
move 2 from 9 to 4
move 1 from 4 to 1
move 1 from 3 to 5
move 2 from 9 to 8
move 11 from 8 to 7
move 2 from 6 to 5
move 1 from 6 to 9
move 1 from 1 to 9
move 1 from 9 to 1
move 4 from 1 to 4
move 2 from 1 to 8
move 1 from 1 to 2
move 1 from 9 to 5
move 2 from 4 to 3
move 2 from 2 to 7
move 2 from 3 to 9
move 1 from 9 to 1
move 1 from 9 to 1
move 5 from 5 to 1
move 19 from 5 to 6
move 5 from 1 to 4
move 1 from 2 to 9
move 1 from 1 to 3
move 7 from 5 to 8
move 1 from 3 to 6
move 8 from 7 to 3
move 7 from 4 to 8
move 3 from 8 to 5
move 1 from 4 to 1
move 1 from 9 to 4
move 1 from 4 to 9
move 1 from 5 to 2
move 2 from 5 to 6
move 2 from 8 to 2
move 7 from 8 to 1
move 1 from 1 to 7
move 3 from 6 to 9
move 2 from 3 to 2
move 1 from 2 to 1
move 1 from 8 to 7
move 2 from 9 to 6
move 2 from 9 to 5
move 1 from 5 to 6
move 1 from 2 to 8
move 2 from 1 to 7
move 1 from 4 to 3
move 3 from 2 to 5
move 7 from 1 to 3
move 10 from 3 to 4
move 3 from 5 to 4
move 1 from 3 to 8
move 3 from 3 to 2
move 1 from 8 to 1
move 1 from 1 to 3
move 3 from 8 to 3
move 5 from 4 to 6
move 1 from 2 to 3
move 4 from 6 to 4
move 1 from 5 to 7
move 4 from 3 to 4
move 1 from 2 to 8
move 12 from 7 to 6
move 1 from 8 to 2
move 2 from 2 to 7
move 1 from 8 to 4
move 23 from 6 to 3
move 14 from 3 to 6
move 15 from 4 to 6
move 1 from 8 to 6
move 10 from 3 to 7
move 2 from 4 to 2
move 11 from 7 to 8
move 2 from 2 to 6
move 44 from 6 to 9
move 21 from 9 to 3
move 12 from 3 to 6
move 1 from 7 to 4
move 1 from 4 to 7
move 9 from 3 to 2
move 2 from 8 to 6
move 3 from 2 to 4
move 17 from 9 to 1
move 3 from 4 to 6
move 2 from 2 to 9
move 4 from 9 to 2
move 10 from 6 to 9
move 1 from 7 to 6
move 4 from 9 to 5
move 4 from 2 to 4
move 14 from 1 to 5
move 4 from 4 to 3
move 3 from 2 to 9
move 9 from 9 to 7
move 1 from 2 to 5
move 9 from 8 to 5
move 8 from 7 to 2
move 4 from 3 to 8
move 5 from 6 to 2
move 3 from 1 to 6
move 1 from 7 to 1
move 4 from 2 to 4
move 3 from 6 to 4
move 3 from 8 to 3
move 13 from 5 to 2
move 2 from 3 to 5
move 12 from 5 to 9
move 1 from 3 to 5
move 1 from 5 to 9
move 1 from 8 to 3
move 4 from 9 to 5
move 6 from 4 to 5
move 12 from 9 to 7
move 1 from 9 to 3
move 1 from 3 to 2
move 12 from 5 to 6
move 12 from 7 to 2
move 1 from 3 to 7
move 1 from 4 to 8
move 33 from 2 to 8
move 1 from 7 to 5
move 1 from 1 to 2
move 4 from 5 to 4
move 3 from 2 to 5
move 34 from 8 to 6
move 1 from 4 to 3
move 1 from 5 to 7
move 1 from 7 to 5
move 3 from 4 to 9
move 2 from 9 to 7
move 1 from 9 to 4
move 1 from 3 to 7
move 1 from 5 to 8
move 1 from 5 to 1
move 1 from 5 to 7
move 1 from 4 to 8
move 1 from 1 to 4
move 1 from 4 to 2
move 3 from 7 to 5
move 2 from 8 to 5
move 1 from 2 to 8
move 4 from 6 to 2
move 1 from 8 to 6
move 1 from 7 to 9
move 29 from 6 to 7
move 4 from 2 to 3
move 2 from 5 to 8
move 1 from 9 to 5
move 2 from 8 to 1
move 23 from 7 to 5
move 2 from 6 to 1
move 23 from 5 to 6
move 1 from 3 to 6
move 4 from 5 to 9
move 2 from 1 to 3
move 5 from 3 to 8
move 2 from 6 to 5
move 2 from 1 to 4
move 1 from 9 to 8
move 1 from 9 to 1
move 1 from 4 to 6
move 2 from 5 to 6
move 6 from 7 to 8
move 2 from 9 to 2
move 18 from 6 to 5
move 21 from 6 to 4
move 1 from 1 to 6
move 2 from 6 to 7
move 2 from 7 to 9
move 2 from 2 to 8
move 7 from 4 to 3
move 12 from 5 to 3
move 1 from 9 to 5
move 1 from 9 to 4
move 6 from 5 to 2
move 17 from 3 to 4
move 3 from 4 to 3
move 1 from 2 to 4
move 5 from 2 to 8
move 1 from 5 to 8
move 19 from 8 to 7
move 1 from 3 to 6
move 1 from 8 to 4
move 1 from 6 to 1
move 15 from 4 to 6
move 1 from 1 to 4
move 3 from 3 to 5
move 4 from 6 to 7
move 1 from 4 to 7
move 10 from 6 to 7
move 16 from 4 to 5
move 24 from 7 to 2
move 8 from 7 to 8
move 1 from 4 to 2
move 6 from 8 to 7
move 1 from 8 to 7
move 1 from 6 to 9
move 14 from 5 to 4
move 9 from 7 to 8
move 4 from 5 to 1
move 2 from 1 to 5
move 3 from 8 to 6
move 2 from 6 to 9
move 2 from 2 to 8
move 6 from 2 to 7
move 3 from 4 to 6
move 1 from 3 to 4
move 3 from 5 to 7
move 1 from 6 to 9
move 5 from 7 to 2
move 4 from 9 to 1
move 1 from 7 to 9
move 9 from 8 to 4
move 5 from 1 to 2
move 2 from 6 to 1
move 6 from 4 to 7
move 1 from 7 to 3
move 1 from 3 to 9
move 1 from 9 to 7
move 1 from 6 to 7
move 9 from 4 to 5
move 7 from 7 to 9
move 3 from 7 to 5
move 1 from 9 to 2
move 6 from 9 to 8
move 4 from 4 to 5
move 1 from 4 to 2
move 1 from 4 to 2
move 2 from 1 to 2
move 1 from 9 to 8
move 10 from 2 to 4
move 8 from 2 to 7
move 12 from 2 to 9
move 6 from 7 to 4
move 1 from 1 to 2
move 8 from 9 to 8
move 7 from 5 to 1
move 9 from 4 to 3
move 14 from 8 to 4
move 1 from 8 to 4
move 1 from 1 to 5
move 1 from 5 to 2
move 3 from 2 to 4
move 1 from 7 to 1
move 1 from 7 to 3
move 2 from 1 to 7
move 3 from 5 to 7
move 2 from 7 to 6
move 1 from 6 to 5
move 3 from 7 to 1
move 1 from 6 to 8
move 1 from 8 to 7
move 1 from 3 to 6
move 1 from 7 to 1
move 4 from 1 to 4
move 6 from 3 to 2
move 3 from 1 to 2
move 3 from 3 to 6
move 3 from 2 to 6
move 6 from 6 to 5
move 1 from 1 to 4
move 1 from 9 to 6
move 5 from 2 to 1
move 3 from 1 to 2
move 2 from 9 to 8
move 3 from 1 to 5
move 1 from 9 to 7
move 25 from 4 to 1
move 1 from 1 to 7
move 2 from 8 to 3
move 13 from 1 to 9
move 2 from 3 to 5
move 8 from 5 to 9
move 4 from 2 to 1
move 2 from 6 to 7
move 10 from 5 to 9
move 4 from 7 to 2
move 2 from 2 to 3
move 9 from 9 to 2
move 4 from 4 to 5
move 4 from 5 to 4
move 5 from 1 to 4
move 10 from 4 to 5
move 22 from 9 to 1
move 2 from 2 to 7
move 3 from 2 to 1
move 6 from 2 to 6
move 1 from 7 to 1
move 10 from 5 to 7
move 15 from 1 to 4
move 13 from 1 to 5
move 3 from 6 to 8
move 1 from 8 to 9";
    println!("{}", solve(&mut state, input));
}
