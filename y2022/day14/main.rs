use std::collections::HashSet;

type Coord = u32;
type Pos = (Coord, Coord);

fn positions(s: &str) -> impl Iterator<Item = Pos> + '_ {
    s.split(" -> ").map(|xy| {
        let (x, y) = xy.split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    })
}

fn range(a: Coord, b: Coord) -> impl Iterator<Item = Coord> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn strike(wall: &str, occupied: &mut HashSet<Pos>, max_y: &mut Coord) {
    let mut points = positions(wall);
    let mut prev = points.next().unwrap();
    *max_y = (*max_y).max(prev.1);

    for point @ (x, y) in points {
        *max_y = (*max_y).max(y);
        if x == prev.0 {
            for y in range(y, prev.1) {
                occupied.insert((x, y));
            }
        } else {
            for x in range(x, prev.0) {
                occupied.insert((x, y));
            }
        }
        prev = point;
    }
}

fn draw(s: &str) -> (HashSet<Pos>, Coord) {
    let mut max_y = 0;
    let mut occupied = HashSet::new();
    for wall in s.lines() {
        strike(wall, &mut occupied, &mut max_y);
    }
    (occupied, max_y)
}

const DROP_START: Pos = (500, 0);

#[derive(PartialEq)]
enum Rest {
    Floor,
    Middle,
    Start,
}

fn drop_single(occupied: &mut HashSet<Pos>, max_y: Coord) -> Rest {
    let (mut x, mut y) = DROP_START;

    while y <= max_y {
        let next_y = y + 1;

        if !occupied.contains(&(x, next_y)) {
            y = next_y;
            continue;
        }
        if !occupied.contains(&(x - 1, next_y)) {
            y = next_y;
            x -= 1;
            continue;
        }
        if !occupied.contains(&(x + 1, next_y)) {
            y = next_y;
            x += 1;
            continue;
        }

        occupied.insert((x, y));
        return if (x, y) == DROP_START { Rest::Start } else { Rest::Middle };
    }

    occupied.insert((x, y));
    Rest::Floor
}

fn solve(input: &str) -> (usize, usize) {
    let (mut occupied, max_y) = draw(input);
    let mut n_rest = 0;

    let answer1 = loop {
        n_rest += 1;
        match drop_single(&mut occupied, max_y) {
            Rest::Start => return (n_rest, n_rest),
            Rest::Middle => (),
            Rest::Floor => break n_rest - 1,
        }
    };

    while drop_single(&mut occupied, max_y) != Rest::Start {
        n_rest += 1;
    }
    n_rest += 1; // account for last iteration

    (answer1, n_rest)
}

fn main() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    println!("{:?}", solve(input));
}
