#![cfg(test)]

type Volume = u8;
type Num = usize;

fn n_different_minimal_ways(target: Volume, containers: &[Volume]) -> (Num, Num) {
    use std::cmp::Ordering;

    assert_ne!(target, 0);

    let mut n_different = 0;
    let mut n_different_minimal = 0;
    let mut n_minimal = Num::MAX;

    let mut volumes = vec![(0 as Volume, 0)];
    let mut next = Vec::new();

    for &c in containers {
        for &(v, n_containers) in &volumes {
            next.push((v, n_containers));
            let Some(inc) = v.checked_add(c).filter(|&inc| inc <= target) else {
                continue;
            };
            let n_containers = n_containers + 1;
            if inc == target {
                n_different += 1;
                match n_containers.cmp(&n_minimal) {
                    Ordering::Less => {
                        n_minimal = n_containers;
                        n_different_minimal = 1;
                    }
                    Ordering::Equal => n_different_minimal += 1,
                    Ordering::Greater => (),
                }
                continue;
            }
            next.push((inc, n_containers));
        }

        volumes.clear();
        std::mem::swap(&mut volumes, &mut next);
    }

    (n_different, n_different_minimal)
}

fn solve_both(input: &str, target: Volume) -> (Num, Num) {
    let containers = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|n| n.parse::<Volume>().unwrap())
        .collect::<Vec<_>>();
    n_different_minimal_ways(target, &containers)
}

#[test]
fn example() {
    assert_eq!(solve_both(INPUT, 25), (4, 3));
}

const INPUT: &str = "
20
15
10
5
5
";
