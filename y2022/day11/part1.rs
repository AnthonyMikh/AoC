use std::cell::RefCell;

type Worry = u32;
type Monkeys<'a> = &'a [RefCell<Monkey>];

#[derive(Debug)]
struct Monkey {
    items: Vec<Worry>,
    n_inspected: usize,
    operation: Expr,
    test: Test,
}

#[derive(Debug)]
struct Expr {
    lhs: Var,
    op: Op,
    rhs: Var,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum Var {
    Old,
    Literal(Worry),
}

#[derive(Debug)]
struct Test {
    divisor: Worry,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn parse(s: &str, expected_num: usize) -> Self {
        let mut lines = s.lines();

        let num = lines.next().expect("no num");
        let num: usize = num
            .strip_prefix("Monkey ").unwrap()
            .strip_suffix(":").unwrap()
            .parse().unwrap();
        assert_eq!(num, expected_num);

        let items = lines
            .next().expect("no items")
            .strip_prefix("  Starting items: ").unwrap()
            .split(", ")
            .map(str::parse).map(Result::unwrap)
            .collect();

        let operation = lines
            .next().expect("no operation")
            .strip_prefix("  Operation: new = ").unwrap();
        let operation = Expr::parse(operation);

        let test = Test::parse(&mut lines);

        assert!(lines.next().is_none());
        assert_ne!(num, test.if_true);
        assert_ne!(num, test.if_false);

        Self {
            items,
            n_inspected: 0,
            operation,
            test,
        }
    }

    fn turn(&mut self, monkeys: Monkeys<'_>) {
        self.n_inspected += self.items.len();
        let mut items = std::mem::take(&mut self.items);
        for item in items.drain(..) {
            self.distribute(item, monkeys);
        }
        self.items = items;
    }

    fn distribute(&mut self, mut item: Worry, monkeys: Monkeys<'_>) {
        item = self.operation.eval(item);
        item /= 3;
        let target = self.test.select(item);
        monkeys[target].borrow_mut().items.push(item);
    }
}

impl Expr {
    fn parse(s: &str) -> Self {
        let mut s = s.split(' ');
        let lhs = Var::parse(s.next().unwrap());
        let op = Op::parse(s.next().unwrap());
        let rhs = Var::parse(s.next().unwrap());
        assert!(s.next().is_none());
        Self { lhs, op, rhs }
    }

    fn eval(&self, item: Worry) -> Worry {
        self.op.eval(self.lhs.eval(item), self.rhs.eval(item))
    }
}

impl Var {
    fn parse(s: &str) -> Self {
        if s == "old" {
            return Var::Old;
        }
        Var::Literal(s.parse().unwrap())
    }

    fn eval(&self, item: Worry) -> Worry {
        match self {
            Var::Old => item,
            &Var::Literal(x) => x,
        }
    }
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!(),
        }
    }

    fn eval(&self, lhs: Worry, rhs: Worry) -> Worry {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }
}

impl Test {
    fn parse(lines: &mut std::str::Lines<'_>) -> Self {
        let divisor = lines
            .next().unwrap()
            .strip_prefix("  Test: divisible by ").unwrap()
            .parse().unwrap();
        let if_true = lines
            .next().unwrap()
            .strip_prefix("    If true: throw to monkey ").unwrap()
            .parse().unwrap();
        let if_false = lines
            .next().unwrap()
            .strip_prefix("    If false: throw to monkey ").unwrap()
            .parse().unwrap();
        Self { divisor, if_true, if_false }
    }

    fn select(&self, item: Worry) -> usize {
        if item % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

fn round(monkeys: Monkeys<'_>) {
    for monkey in monkeys {
        monkey.borrow_mut().turn(monkeys);
    }
}

const N_ROUNDS: usize = 20;

fn solve(input: &str) -> usize {
    let monkeys = input
        .split("\n\n")
        .enumerate()
        .map(|(i, line)| RefCell::new(Monkey::parse(line, i)))
        .collect::<Vec<_>>();
    for _ in 0..N_ROUNDS {
        round(&monkeys);
    }
    let mut n_inspected = monkeys
        .iter()
        .map(|m| m.borrow().n_inspected)
        .collect::<Vec<_>>();
    n_inspected.sort_unstable();
    let &[.., second_max, max] = &n_inspected[..] else { panic!() };
    max * second_max
}

fn main() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    println!("{}", solve(input));
}
