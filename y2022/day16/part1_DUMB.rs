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
    println!("{}", solve(input));
}

fn solve(input: &str) -> Num {
    let mut i = Interner::default();
    let (raw_edges, rates) = parse(input, &mut i);
    assert!(raw_edges.len() <= SmallSet::MAX_SIZE);
    let edges = compress(&raw_edges, &rates);
    let start = i.insert("AA");
    let mut closed = rates
        .iter()
        .enumerate()
        .filter_map(|(s, &rate)| (rate != 0).then_some(s))
        .collect::<SmallSet>();
    let ret = descend(
        &edges,
        &rates,
        start,
        start,
        &mut closed,
        0,
        30,
    );
    ret
}

fn descend(
    edges: &CompressedEdges,
    rates: &Rates,
    current: Id,
    prev: Id,
    closed: &mut SmallSet,
    released: Num,
    minutes_left: Time,
) -> Num {
    if minutes_left == 0 || minutes_left == 1 {
        return released;
    }
    if closed.is_empty() {
        return released;
    }

    let neighbors = &edges[current];
    let mut max = released;
    for &(next, time_to_reach) in neighbors {
        if next == prev {
            continue
        }
        let minutes_left = match minutes_left.checked_sub(time_to_reach) {
            Some(left) => left,
            None => continue,
        };
        max = descend(
            edges,
            rates,
            next,
            current,
            closed,
            released,
            minutes_left,
        ).max(max);
    }
    if minutes_left > 2 && closed.remove(current) {
        let minutes_left = minutes_left - 1;
        let rate = rates[current];
        max = descend(
            edges,
            rates,
            current,
            current,
            closed,
            released + rate * minutes_left,
            minutes_left,
        ).max(max);
        closed.insert(current);
    }

    max
}

type Num = u32;
type Edges = Vec<Vec<Id>>;
type CompressedEdges = Vec<Vec<(Id, Time)>>;
type Id = usize;
type Time = Num;
type Rates = Vec<Num>;

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

fn compress(edges: &Edges, rates: &Rates) -> CompressedEdges {
    let mut ret = vec![Vec::new(); edges.len()];
    for (node, (neighbors, &rate)) in edges.iter().zip(rates).enumerate() {
        if rate == 0 && neighbors.len() <= 2 {
            continue
        }
        let compressed_neighbors = &mut ret[node];
        for &neighbor in neighbors {
            let mut prev = node;
            let mut current = neighbor;
            let mut len = 1;
            while rates[current] == 0 {
                if let &[a, b] = &edges[current][..] {
                    let next = if a == prev { b } else { a };
                    prev = current;
                    current = next;
                    len += 1;
                } else {
                    break
                }
            }
            compressed_neighbors.push((current, len));
        }
    }
    ret
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

    fn get(&self, id: Id) -> &'a str {
        self.words[id]
    }
}

#[allow(dead_code)]
fn emit(edges: &Edges, rates: &Rates, i: &Interner<'_>) {
    println!("strict graph {{");
    for (n, (neighbors, &rate)) in edges.iter().zip(rates).enumerate() {
        let label = i.get(n);
        if rate == 0 && label != "AA" {
            println!("    {label} [shape=point]");
        } else {
            println!("    {label}");
        }
        for &n in neighbors {
            println!("    {label} -- {}", i.get(n));
        }
    }
    println!("}}")
}

#[allow(dead_code)]
fn emit_compressed(edges: &CompressedEdges, rates: &Rates, i: &Interner<'_>) {
    println!("strict graph {{");
    for (n, (neighbors, &rate)) in edges.iter().zip(rates).enumerate() {
        let label = i.get(n);
        if rate == 0 && label != "AA" {
            println!("    {label} [shape=point]");
        } else {
            println!("    {label}");
        }
        for &(n, _time) in neighbors {
            println!("    {label} -- {}", i.get(n));
        }
    }
    println!("}}")
}

type Bits = u64;

#[derive(Default)]
struct SmallSet {
    bits: Bits,
    len: usize,
}

impl SmallSet {
    const MAX_SIZE: usize = Bits::BITS as usize;

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn insert(&mut self, x: usize) {
        let mask = 1 << x;
        let was_present = self.bits & mask != 0;
        self.bits |= mask;
        if !was_present {
            self.len += 1;
        }
    }

    fn remove(&mut self, x: usize) -> bool {
        let mask = 1 << x;
        let was_present = self.bits & mask != 0;
        self.bits &= !mask;
        if was_present {
            self.len -= 1;
        }
        was_present
    }
}

impl std::iter::FromIterator<usize> for SmallSet {
    fn from_iter<I: IntoIterator<Item = usize>>(it: I) -> Self {
        let mut ret = Self::default();
        it.into_iter().for_each(|x| ret.insert(x));
        ret
    }
}
