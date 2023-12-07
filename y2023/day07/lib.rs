#![cfg(test)]

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    const N_CARDS: usize = 13;

    fn parse(ch: u8) -> Self {
        use Card::*;

        match ch {
            b'2' => N2,
            b'3' => N3,
            b'4' => N4,
            b'5' => N5,
            b'6' => N6,
            b'7' => N7,
            b'8' => N8,
            b'9' => N9,
            b'T' => T,
            b'J' => J,
            b'Q' => Q,
            b'K' => K,
            b'A' => A,
            _ => panic!("invalid card character: {}", char::from(ch)),
        }
    }
}

#[derive(PartialEq, Eq)]
struct CardWithJoker(Card);

impl CardWithJoker {
    fn cmp_key(&self) -> impl Ord {
        (self.0 != Card::J, self.0)
    }
}

impl std::cmp::Ord for CardWithJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_key().cmp(&other.cmp_key())
    }
}

impl std::cmp::PartialOrd for CardWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Combination {
    HighCard,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}

type Counts = [u8; Card::N_CARDS];

impl Combination {
    fn from_counts(counts: &mut Counts) -> Self {
        counts.sort_unstable_by_key(|&n| std::cmp::Reverse(n));
        match &counts[..] {
            [5, ..] => Self::Five,
            [4, ..] => Self::Four,
            [3, 2, ..] => Self::FullHouse,
            [3, ..] => Self::Three,
            [2, 2, ..] => Self::TwoPairs,
            [2, ..] => Self::Pair,
            _ => Self::HighCard,
        }
    }

    fn of(hand: [Card; 5]) -> Self {
        let mut counts = [0u8; Card::N_CARDS];
        for &card in &hand {
            counts[card as usize] += 1;
        }
        Self::from_counts(&mut counts)
    }

    fn of_with_jokers(hand: [Card; 5]) -> Self {
        let mut counts = [0u8; Card::N_CARDS];
        let mut n_jokers = 0;
        for &card in &hand {
            if card == Card::J {
                n_jokers += 1;
                continue;
            }
            counts[card as usize] += 1;
        }
        Self::from_counts(&mut counts).up(n_jokers)
    }

    fn up(self, n_jokers: usize) -> Self {
        use Combination::*;
        
        if n_jokers == 0 {
            return self;
        }
        
        match self {
            HighCard => match n_jokers {
                1 => Pair,
                2 => Three,
                3 => Four,
                _ => Five,
            },
            Pair => match n_jokers {
                1 => Three,
                2 => Four,
                _ => Five,
            },
            TwoPairs => FullHouse,
            Three => if n_jokers == 1 { Four } else { Five },
            FullHouse => FullHouse,
            Four => Five,
            Five => Five,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    combination: Combination,
    cards: [Card; 5],
}

impl Hand {
    fn parse(s: &str) -> Self {
        let cards = <[_; 5]>::try_from(s.as_bytes()).unwrap().map(Card::parse);
        let combination = Combination::of(cards);
        Self { combination, cards }
    }
}

type Num = u32;
type Bid = Num;

fn winnings(hands: &[(Hand, Bid)]) -> Num {
    hands.iter()
        .zip(1..)
        .map(|((_hand, bid), rank)| *bid * rank)
        .sum()
}

fn solve_both(input: &str) -> (Num, Num) {
    let mut hands = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            (Hand::parse(hand), bid.parse::<Bid>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by_key(|(hand, _bid)| hand.clone());
    let first = winnings(&hands);
    
    hands.iter_mut().for_each(|(h, _bid)| h.combination = Combination::of_with_jokers(h.cards));
    hands.sort_unstable_by_key(|(hand, _bid)| (hand.combination, hand.cards.map(CardWithJoker)));
    let second = winnings(&hands);

    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (6440, 5905));
}

const INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
