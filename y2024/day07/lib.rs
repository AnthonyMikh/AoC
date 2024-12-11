use std::ops::ControlFlow;

type Num = u64;

struct Equation {
    target: Num,
    operands: Vec<Num>,
}

impl Equation {
    fn parse(input: &str) -> Self {
        let (target, rest) = input.split_once(": ").unwrap();
        let target = target.parse().unwrap();
        let operands = rest.split(' ').map(|n| n.parse().unwrap()).collect();
        Self { target, operands }
    }
}

fn num_concat(mut lhs: Num, rhs: Num) -> Num {
    let mut r = rhs;
    while r != 0 {
        lhs *= 10;
        r /= 10;
    }
    lhs + rhs
}

fn try_recursive<const CONSIDER_CONCAT: bool>(
    target: Num,
    acc: Num,
    vals: &[Num],
) -> ControlFlow<(), ()> {
    match vals.split_first() {
        None => {
            if target == acc {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        }
        Some((&next, rest)) => {
            if CONSIDER_CONCAT {
                try_recursive::<CONSIDER_CONCAT>(target, num_concat(acc, next), rest)?;
            }
            try_recursive::<CONSIDER_CONCAT>(target, acc + next, rest)?;
            try_recursive::<CONSIDER_CONCAT>(target, acc * next, rest)?;
            ControlFlow::Continue(())
        }
    }
}

enum Answer {
    No,
    Both,
    WithConcatOnly,
}

fn can_be_solved(eq: &Equation) -> Answer {
    let (&first, rest) = eq.operands.split_first().unwrap();
    if try_recursive::<false>(eq.target, first, rest).is_break() {
        Answer::Both
    } else if try_recursive::<true>(eq.target, first, rest).is_break() {
        Answer::WithConcatOnly
    } else {
        Answer::No
    }
}

pub fn solve_both(input: &str) -> (Num, Num) {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Equation::parse)
        .fold((0, 0), |acc @ (total, with_concat), eq| {
            let target = eq.target;
            match can_be_solved(&eq) {
                Answer::No => acc,
                Answer::WithConcatOnly => (total, with_concat + target),
                Answer::Both => (total + target, with_concat + target),
            }
        })
}
