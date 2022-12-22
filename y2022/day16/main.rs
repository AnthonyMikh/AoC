use std::collections::HashMap;

fn main() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    println!("{:?}", solve(input));
}

type Num = u32;
type Id = usize;
type Edges = Vec<Vec<Id>>;
type Time = Num;
type Rates = Vec<Num>;
type BestSolutions = HashMap<SmallSet, Num>;

const N_MINUTES: Num = 30;
const TIME_TO_LEARN: Num = 4;

fn solve(input: &str) -> (Num, Num) {
    let mut i = Interner::default();
    let (edges, rates) = parse(input, &mut i);
    let start = i.insert("AA");
    let answer1 = solve1(&edges, &rates, start);
    let answer2 = solve2(&edges, &rates, start);
    (answer1, answer2)
}

fn solve1(edges: &Edges, rates: &Rates, start: Id) -> Num {
    let solutions = best_solutions(edges, rates, start, N_MINUTES);
    solutions.values().copied().max().unwrap()
}

fn solve2(edges: &Edges, rates: &Rates, start: Id) -> Num {
    let solutions = best_solutions(edges, rates, start, N_MINUTES - TIME_TO_LEARN);

    let all_opened = usable_valves(&rates);
    let mut best = 0;
    let mut subsets = Vec::new();

    for (&opened, &released_by_person) in &solutions {
        let complement = all_opened.sub(&opened);
        complement.all_subsets(&mut subsets);
        for &subset in subsets.iter() {
            if let Some(released_by_elephants) = solutions.get(&subset) {
                best = best.max(released_by_person + released_by_elephants);
            }
        }
        subsets.clear();
    }

    best
}

fn best_solutions(edges: &Edges, rates: &Rates, start: Id, n_minutes: Time) -> BestSolutions {
    use std::collections::hash_map::Entry;

    let all_closed = usable_valves(rates);
    let mut seen = HashMap::new();
    let initial = (start, all_closed, 0);
    let mut current = vec![initial];
    let mut next_states = Vec::new();
    for i in 0..n_minutes {
        if current.is_empty() {
            break;
        }

        let minutes_left = n_minutes - i - 1;
        for &(current, mut closed, released) in &current {
            if closed.remove(current) {
                let released = released + rates[current] * minutes_left;
                let skip = match seen.entry((current, closed)) {
                    Entry::Vacant(e) => {
                        e.insert(released);
                        closed.is_empty()
                    }
                    Entry::Occupied(mut e) => {
                        if *e.get() < released {
                            *e.get_mut() = released;
                            closed.is_empty()
                        } else {
                            true
                        }
                    }
                };
                if !skip {
                    next_states.push((current, closed, released));
                }
                closed.insert(current);
            }

            let neighbors = &edges[current];
            for &next in neighbors {
                let consider = match seen.entry((next, closed)) {
                    Entry::Vacant(e) => {
                        e.insert(released);
                        true
                    }
                    Entry::Occupied(mut e) => {
                        if *e.get() < released {
                            *e.get_mut() = released;
                            true
                        } else {
                            false
                        }
                    }
                };
                if consider {
                    next_states.push((next, closed, released));
                }
            }
        }

        std::mem::swap(&mut current, &mut next_states);
        next_states.clear();
    }

    let mut ret = HashMap::new();
    for (&(_, closed), &released) in &seen {
        let opened = all_closed.sub(&closed);
        match ret.entry(opened) {
            Entry::Vacant(e) => drop(e.insert(released)),
            Entry::Occupied(mut e) => {
                if *e.get() < released {
                    *e.get_mut() = released;
                }
            }
        }
    }

    ret
}

fn parse<'a>(s: &'a str, i: &mut Interner<'a>) -> (Edges, Rates) {
    let mut edges = vec![Vec::new(); s.lines().count()];
    let mut rates = vec![0; edges.len()];

    for l in s.lines() {
        let mut p = Lexer::of(l);
        p.literal("Valve ");
        let label = i.insert(p.before_literal(" has flow rate=").unwrap());
        let rate = p.number().unwrap();
        let next = if p.literal("; tunnels lead to valves ").is_some() {
            p.s.split(", ").map(|n| i.insert(n)).collect()
        } else {
            p.literal("; tunnel leads to valve ").unwrap();
            vec![i.insert(p.s)]
        };
        edges[label] = next;
        rates[label] = rate;
    }

    (edges, rates)
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

    fn literal(&mut self, literal: &str) -> Option<()> {
        self.s = self.s.strip_prefix(literal)?;
        Some(())
    }

    fn before_literal(&mut self, literal: &str) -> Option<&'a str> {
        let pos = self.s.find(literal)?;
        let ret = &self.s[..pos];
        self.shift(pos + literal.len());
        Some(ret)
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

#[derive(Default)]
struct Interner<'a> {
    bag: std::collections::HashMap<&'a str, usize>,
    words: Vec<&'a str>,
}

impl<'a> Interner<'a> {
    fn insert(&mut self, word: &'a str) -> Id {
        if let Some(&idx) = self.bag.get(&word) {
            return idx;
        }
        let idx = self.words.len();
        self.words.push(word);
        self.bag.insert(word, idx);
        idx
    }
}

type Bits = u64;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct SmallSet {
    bits: Bits,
}

impl SmallSet {
    const MAX_SIZE: usize = Bits::BITS as usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.bits.count_ones() as _
    }

    fn insert(&mut self, x: usize) {
        let mask = 1 << x;
        self.bits |= mask;
    }

    fn remove(&mut self, x: usize) -> bool {
        let mask = 1 << x;
        let was_present = self.bits & mask != 0;
        self.bits &= !mask;
        was_present
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }

    fn all_subsets(mut self, out: &mut Vec<Self>) {
        let shift = self.bits.trailing_zeros();
        if shift >= Bits::BITS {
            out.push(self);
            return;
        }
        let mask = 1 << shift;
        self.bits &= !mask;
        self.all_subsets(out);
        let len = out.len();
        out.extend_from_within(..);
        out[len..].iter_mut().for_each(|s| s.insert(shift as _));
    }
}

impl std::iter::FromIterator<usize> for SmallSet {
    fn from_iter<I: IntoIterator<Item = usize>>(it: I) -> Self {
        let mut ret = Self::default();
        it.into_iter().for_each(|x| ret.insert(x));
        ret
    }
}

fn usable_valves(rates: &Rates) -> SmallSet {
    assert!(rates.len() <= SmallSet::MAX_SIZE);
    rates
        .iter()
        .enumerate()
        .filter_map(|(i, &rate)| (rate != 0).then_some(i))
        .collect()
}
