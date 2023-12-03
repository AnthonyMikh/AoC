#![cfg(test)]

type Num = u32;
type Speed = Num;
type Time = Num;
type Distance = Num;

type Score = u32;

struct Reindeer {
    speed: Speed,
    time_fly: Time,
    time_rest: Time,
}

impl Reindeer {
    fn parse(s: &str) -> Self {
        let (_name, rest) = s.split_once(" can fly ").unwrap();
        let (speed, rest) = rest.split_once(" km/s for ").unwrap();
        let (time_fly, rest) = rest.split_once(" seconds, but then must rest for ").unwrap();
        let time_rest = rest.strip_suffix(" seconds.").unwrap();
        Self {
            speed: speed.parse().unwrap(),
            time_fly: time_fly.parse().unwrap(),
            time_rest: time_rest.parse().unwrap(),
        }
    }

    fn reach(&self, total_time: Time) -> Distance {
        let t_cycle = self.time_fly + self.time_rest;
        let reach_per_cycle = self.speed * self.time_fly;
        let whole_cycles = total_time / t_cycle;
        let additional_time = total_time % t_cycle;
        let additional_reach = self.time_fly.min(additional_time) * self.speed;
        reach_per_cycle * whole_cycles + additional_reach
    }
    
    fn kick(self) -> MovingReindeer {
        assert_ne!(self.time_fly, 0);
        assert_ne!(self.time_rest, 0);

        let time_fly = self.time_fly;
        MovingReindeer {
            reindeer: self,
            state: State::Flying(time_fly),
            pos: 0,
        }
    }
}

fn winner_distance(reindeers: &[Reindeer], total_time: Time) -> Distance {
    reindeers.iter().map(|r| r.reach(total_time)).max().unwrap_or(0)
}

enum State {
    Flying(Time),
    Resting(Time),
}

struct MovingReindeer {
    reindeer: Reindeer,
    state: State,
    pos: Distance,
}

impl MovingReindeer {
    fn tick(&mut self) -> Distance {
        match &mut self.state {
            State::Flying(time_left) => {
                self.pos += self.reindeer.speed;
                *time_left -= 1;
                if *time_left == 0 {
                    self.state = State::Resting(self.reindeer.time_rest);
                }
            }
            State::Resting(time_left) => {
                *time_left -= 1;
                if *time_left == 0 {
                    self.state = State::Flying(self.reindeer.time_fly);
                }
            }
        }
        self.pos
    }
}

fn solve_both(input: &str, race_time: Time) -> (Distance, Score) {
    let reindeers = input.lines().filter(|l| !l.is_empty()).map(Reindeer::parse).collect::<Vec<_>>();
    assert!(!reindeers.is_empty());

    let first = winner_distance(&reindeers, race_time);

    let mut reindeers = reindeers.into_iter().map(|r| (r.kick(), 0)).collect::<Vec<_>>();
    for _ in 0..race_time {
        let furthest = reindeers.iter_mut().map(|(r, _score)| r.tick()).max().unwrap();
        reindeers.iter_mut().for_each(|(r, score)| if r.pos == furthest { *score += 1 });
    }
    
    let second = reindeers.iter().map(|(_r, score)| *score).max().unwrap();

    (first, second)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT, 1000), (1120, 689));
}

const INPUT: &str = "
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";
