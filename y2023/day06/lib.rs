#![cfg(test)]

type Num = u64;

type Time = Num;
type Distance = Num;

fn parse(s: &str) -> Vec<(Time, Distance)> {
    let mut lines = s.trim_matches('\n').lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    assert!(lines.next().is_none());
    
    let mut times = times
        .strip_prefix("Time:      ")
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|t| t.parse().unwrap());
    let mut distances = distances
        .strip_prefix("Distance:  ")
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|d| d.parse().unwrap());

    let ret = times.by_ref().zip(distances.by_ref()).collect();
    assert!(times.next().is_none() && distances.next().is_none());

    ret
}

fn parse_dense(s: &str) -> (Time, Distance) {
    let mut lines = s.trim_matches('\n').lines();
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    assert!(lines.next().is_none());
    
    let mut time = time.strip_prefix("Time:      ")
        .unwrap().to_owned();
    time.retain(|ch| ch != ' ');
    let time = time.parse().unwrap();
    
    let mut distance = distance.strip_prefix("Distance:  ")
        .unwrap().to_owned();
    distance.retain(|ch| ch != ' ');
    let distance = distance.parse().unwrap();
    
    (time, distance)
}

fn n_ways(time_limit: Time, record_distance: Distance) -> usize {
    (0..time_limit)
        .filter(|t| {
            let speed = t;
            let t_race = time_limit - t;
            speed * t_race > record_distance
        })
        .count()
}

fn solve_both(input: &str) -> (usize, usize) {
    let separate = parse(input);
    let first = separate.iter().map(|&(t, d)| n_ways(t, d)).product();
    let (time, distance) = parse_dense(input);
    let second = n_ways(time, distance);
    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT), (288, 71503));
}

const INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200
";
