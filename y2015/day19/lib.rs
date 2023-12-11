use std::collections::HashMap;

#[derive(Default)]
pub struct Interner<'i> {
    bag: HashMap<&'i str, usize>,
    strs: Vec<&'i str>,
}

impl<'i> Interner<'i> {
    fn insert(&mut self, s: &'i str) -> usize {
        if let Some(&ret) = self.bag.get(s) {
            return ret;
        }
        let ret = self.strs.len();
        self.strs.push(s);
        self.bag.insert(s, ret);
        ret
    }

    fn len(&self) -> usize {
        self.strs.len()
    }
}

mod trie {
    use super::Token;

    struct TrieNode {
        has_end: bool,
        next: Box<[Option<Self>]>,
    }

    impl TrieNode {
        fn new(node_size: usize) -> Self {
            Self {
                has_end: false,
                next: (0..node_size)
                    .map(|_| None)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            }
        }
    }

    struct TrieProperties {
        len: usize,
        node_size: usize,
    }

    pub struct Trie {
        props: TrieProperties,
        head: TrieNode,
    }

    impl Trie {
        pub fn new(node_size: usize) -> Self {
            Self {
                props: TrieProperties { len: 0, node_size },
                head: TrieNode::new(node_size),
            }
        }

        pub fn cursor(&mut self) -> Cursor<'_> {
            Cursor {
                props: &mut self.props,
                node: &mut self.head,
            }
        }

        pub fn len(&self) -> usize {
            self.props.len
        }

        pub fn insert_seq(&mut self, seq: &[Token]) -> bool {
            self.cursor().insert_seq(seq).complete()
        }
    }

    pub struct Cursor<'a> {
        props: &'a mut TrieProperties,
        node: &'a mut TrieNode,
    }

    impl<'a> Cursor<'a> {
        pub fn reborrow<'r>(&'r mut self) -> Cursor<'r> {
            Cursor {
                props: &mut self.props,
                node: &mut self.node,
            }
        }

        pub fn insert(self, token: Token) -> Self {
            let node =
                self.node.next[token].get_or_insert_with(|| TrieNode::new(self.props.node_size));
            Self {
                props: self.props,
                node,
            }
        }

        pub fn insert_seq(mut self, tokens: &[Token]) -> Self {
            for &token in tokens {
                self = self.insert(token);
            }
            self
        }

        pub fn complete(self) -> bool {
            let is_new = !self.node.has_end;
            if is_new {
                self.props.len += 1;
            }
            self.node.has_end = true;
            is_new
        }
    }
}

fn split_off_atom<'a>(s: &mut &'a str) -> Option<&'a str> {
    if s.is_empty() {
        return None;
    }
    let len = if s.as_bytes().get(1).map_or(false, u8::is_ascii_lowercase) {
        2
    } else {
        1
    };
    let (ret, rest) = s.split_at(len);
    *s = rest;
    Some(ret)
}

fn atom_tokenize(mut s: &str) -> impl Iterator<Item = &str> + '_ {
    std::iter::from_fn(move || split_off_atom(&mut s))
}

pub type Token = usize;

struct Rule {
    head: Token,
    subts: Vec<Token>,
}

impl Rule {
    fn parse_interning<'a>(input: &'a str, i: &mut Interner<'a>) -> Self {
        let (head, rest) = input.split_once(" => ").unwrap();
        let subts = atom_tokenize(rest).map(|symbol| i.insert(symbol)).collect();
        Self {
            head: i.insert(head),
            subts,
        }
    }
}

pub fn solve_first(
    mut molecule: &[Token],
    rules_map: &[Vec<Vec<Token>>],
    n_components: usize,
) -> usize {
    let mut trie = trie::Trie::new(n_components);
    let mut seqs = trie.cursor();

    while let Some((&first, rest)) = molecule.split_first() {
        for substitution in &rules_map[first] {
            seqs.reborrow()
                .insert_seq(&substitution)
                .insert_seq(rest)
                .complete();
        }
        if rest.is_empty() {
            break;
        }
        seqs = seqs.insert(first);
        molecule = rest;
    }

    trie.len()
}

fn reverse_rules(rules_map: &[Vec<Vec<Token>>]) -> Vec<(Vec<Token>, Token)> {
    rules_map
        .iter()
        .cloned()
        .enumerate()
        .flat_map(|(atom, substs)| substs.into_iter().map(move |subst| (subst, atom)))
        .collect()
}

pub fn solve_second(
    initial: Token,
    rules_map: &[Vec<Vec<Token>>],
    target: Vec<Token>,
    n_atoms: usize,
) -> usize {
    let reverse_rules_map = reverse_rules(rules_map);
    let mut trie = trie::Trie::new(n_atoms);
    trie.insert_seq(&target);

    let mut seqs = vec![(target, 0)];

    while let Some((unreduced, n_steps)) = seqs.pop() {
        let n_steps = n_steps + 1;
        for i in 0..unreduced.len() {
            for &(ref subst, atom) in reverse_rules_map.iter() {
                let Some(tail) = unreduced[i..].strip_prefix(&subst[..]) else {
                    continue;
                };

                if atom == initial {
                    if unreduced.len() == subst.len() {
                        return n_steps;
                    } else {
                        continue;
                    }
                }

                let new = unreduced[..i]
                    .iter()
                    .chain(Some(&atom))
                    .chain(tail)
                    .copied()
                    .collect::<Vec<_>>();
                if !trie.insert_seq(&new) {
                    continue;
                }

                seqs.push((new, n_steps));
            }
        }
    }

    unreachable!()
}

pub fn parse_input<'a>(input: &'a str, i: &mut Interner<'a>) -> (Vec<Token>, Vec<Vec<Vec<Token>>>) {
    let (rules, molecule) = input.trim_matches('\n').split_once("\n\n").unwrap();
    let mut rules_map = HashMap::<_, Vec<_>>::new();
    for Rule { head, subts } in rules.lines().map(|l| Rule::parse_interning(l, i)) {
        rules_map.entry(head).or_default().push(subts);
    }
    let n_atoms = i.len();
    let rules = {
        let mut map = vec![Vec::new(); n_atoms];
        for (head, substs) in rules_map {
            map[head] = substs;
        }
        map
    };
    let molecule = atom_tokenize(molecule)
        .map(|atom| i.insert(atom))
        .collect::<Vec<_>>();

    (molecule, rules)
}

pub const INITIAL: &str = "e";

pub fn solve_both(input: &str) -> (usize, usize) {
    let mut i = Interner::default();
    let (molecule, rules) = parse_input(input, &mut i);
    let n_atoms = i.len();

    let first = solve_first(&molecule, &rules, n_atoms);
    let second = solve_second(i.insert(INITIAL), &rules, molecule.clone(), n_atoms);

    (first, second)
}

#[test]
fn example1() {
    let input = "
H => HO
H => OH
O => HH

HOH
";
    let mut i = Interner::default();
    let (molecule, rules) = parse_input(input, &mut i);
    let n_atoms = i.len();

    assert_eq!(solve_first(&molecule, &rules, n_atoms), 4);
}

#[test]
fn example2() {
    let input = "
e => H
e => O
H => HO
H => OH
O => HH

HOHOHO
";
    let mut i = Interner::default();
    let (molecule, rules) = parse_input(input, &mut i);
    let initial = i.insert(INITIAL);
    let n_atoms = i.len();

    assert_eq!(solve_second(initial, &rules, molecule, n_atoms), 6);
}
