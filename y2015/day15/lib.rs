#![cfg(test)]

type Quality = i32;

const N_QUALITIES: usize = 4;

struct Ingredient {
    qualities: [Quality; N_QUALITIES],
    calories: Quality,
}

impl Ingredient {
    fn parse(s: &str) -> Self {
        let (_name, amounts) = s.split_once(": ").unwrap();
        let mut amounts = amounts.split(", ");
        let [qualities @ .., calories]: [_; N_QUALITIES + 1] =
            std::array::from_fn(|_| amounts.next().unwrap());
        assert!(amounts.next().is_none());
        let qualities = qualities.map(|s| s.split_once(' ').unwrap().1.parse().unwrap());
        let calories = calories.split_once(' ').unwrap().1.parse().unwrap();
        Self {
            qualities,
            calories,
        }
    }
}

#[derive(Default)]
struct Cookie {
    qualities: [Quality; N_QUALITIES],
    calories: Quality,
}

impl Cookie {
    fn add(&self, amount: Quality, ingredient: &Ingredient) -> Self {
        let mut qualities = self.qualities;
        qualities
            .iter_mut()
            .zip(&ingredient.qualities)
            .for_each(|(current, &delta)| *current += delta * amount);
        let calories = self.calories + amount * ingredient.calories;
        Self {
            qualities,
            calories,
        }
    }

    fn score(&self) -> Quality {
        self.qualities.iter().map(|&q| q.max(0)).product()
    }
}

fn choose_best(
    ingredients: &[Ingredient],
    total_spoons: Quality,
    allow: impl Fn(&Cookie) -> bool,
) -> Option<Quality> {
    choose_best_recursive(Cookie::default(), ingredients, total_spoons, &allow)
}

fn choose_best_recursive(
    cookie: Cookie,
    ingredients: &[Ingredient],
    total_spoons: Quality,
    allow: &impl Fn(&Cookie) -> bool,
) -> Option<Quality> {
    match ingredients {
        [] | [_] => unreachable!(),
        [first, second] => (0..total_spoons + 1)
            .filter_map(|take_first| {
                let take_second = total_spoons - take_first;
                let cookie = cookie.add(take_first, first).add(take_second, second);
                allow(&cookie).then(|| cookie.score())
            })
            .max(),
        [first, rest @ ..] => (0..total_spoons + 1)
            .filter_map(|take_first| {
                let take_rest = total_spoons - take_first;
                choose_best_recursive(cookie.add(take_first, first), rest, take_rest, allow)
            })
            .max(),
    }
}

fn solve_both(input: &str) -> (Quality, Quality) {
    let ingredients = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Ingredient::parse)
        .collect::<Vec<_>>();
    let first = choose_best(&ingredients, 100, |_| true).unwrap();
    let second = choose_best(&ingredients, 100, |c| c.calories == 500).unwrap();
    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (62842880, 57600000));
}

const INPUT: &str = "
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
