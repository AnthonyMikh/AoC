#![cfg(test)]

type NCubes = u32;

#[derive(Default)]
struct DrawSet {
    red: NCubes,
    green: NCubes,
    blue: NCubes,
}

impl DrawSet {
    fn parse(s: &str) -> Self {
        let mut parts = s.split(", ");
        let [mut red, mut green, mut blue] = [None; 3];

        for part in parts.by_ref().take(3) {
            macro_rules! assign {
                ($name:ident) => {
                    if let Some(n) = part.strip_suffix(concat!(" ", stringify!($name))) {
                        if $name.is_some() {
                            panic!("`{}` is already set", stringify!($name));
                        }
                        $name = Some(n.parse().unwrap());
                        continue;
                    }
                };
            }
            assign!(red);
            assign!(green);
            assign!(blue);
            panic!("invalid part {part:?}");
        }

        assert!(parts.next().is_none());

        Self {
            red: red.unwrap_or(0),
            green: green.unwrap_or(0),
            blue: blue.unwrap_or(0),
        }
    }

    fn is_covered_by(&self, other: &Self) -> bool {
        let &Self { red, green, blue } = self;
        red <= other.red && green <= other.green && blue <= other.blue
    }

    fn merge_with(self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> NCubes {
        self.red * self.green * self.blue
    }
}

type Id = u32;

struct Game {
    id: Id,
    draw_sets: Vec<DrawSet>,
}

impl Game {
    fn parse(s: &str) -> Self {
        let (with_id, draw_sets) = s.split_once(": ").unwrap();
        let id = with_id.strip_prefix("Game ").unwrap().parse().unwrap();
        let draw_sets = draw_sets.split("; ").map(DrawSet::parse).collect();
        Self { id, draw_sets }
    }

    fn is_valid(&self, constraint: &DrawSet) -> bool {
        self.draw_sets
            .iter()
            .all(|set| set.is_covered_by(constraint))
    }
}

const CONSTRAINT: DrawSet = DrawSet {
    red: 12,
    green: 13,
    blue: 14,
};

fn solve_both(input: &str) -> (Id, NCubes) {
    let games = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Game::parse)
        .collect::<Vec<_>>();

    let first = games
        .iter()
        .filter_map(|game| game.is_valid(&CONSTRAINT).then_some(game.id))
        .sum();

    let second = games
        .iter()
        .map(|game| {
            game.draw_sets
                .iter()
                .fold(DrawSet::default(), DrawSet::merge_with)
                .power()
        })
        .sum();
    
    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (8, 2286));
}

const INPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
