use std::collections::{hash_map::Entry, BTreeSet, HashMap};

fn main() {
    let input = std::fs::read_to_string("inputs/day20.txt").unwrap();
    let p1 = solve(&input, 100, 2);
    let p2 = solve(&input, 100, 20);
    println!("{} {}", p1, p2);
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Cheat {
    NoCheat,
    CheatStart { start: (u8, u8), dur: usize },
    CheatEnded { start: (u8, u8), end: (u8, u8) },
}

fn solve(input: &str, save: usize, max_cheat_dur: usize) -> usize {
    let mut start = None;
    let mut end = None;
    let grid: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| match *c {
                    b'S' => {
                        start = Some((x as u8, y as u8));
                        b'.'
                    }
                    b'E' => {
                        end = Some((x as u8, y as u8));
                        b'.'
                    }
                    _ => *c,
                })
                .collect()
        })
        .collect();

    let width = grid[0].len() as u8;
    let height = grid.len() as u8;

    let start = start.unwrap();
    let end = end.unwrap();

    let mut no_cheat_to_end = HashMap::new();
    no_cheat_to_end.insert(end, 0);

    let mut todo = BTreeSet::new();
    todo.insert((0, end));

    while let Some((time, (x, y))) = todo.pop_first() {
        for pos in [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ] {
            {
                let (x, y) = pos;
                if x >= width || y >= height {
                    continue;
                }

                if grid[y as usize][x as usize] == b'#' {
                    continue;
                }
            }

            let time = time + 1 as usize;
            match no_cheat_to_end.entry(pos) {
                Entry::Occupied(e) => {
                    let v = e.into_mut();

                    if *v > time {
                        *v = time;
                        todo.insert((time, pos));
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(time);
                    todo.insert((time, pos));
                }
            }
        }
    }

    let mut best = HashMap::new();
    best.insert((Cheat::NoCheat, start), 0);

    let mut todo = BTreeSet::new();
    todo.insert((0, Cheat::NoCheat, start));

    while let Some((time, cheat, (x, y))) = todo.pop_first() {
        if let Cheat::CheatStart { dur, .. } = cheat {
            if dur >= max_cheat_dur {
                continue;
            }
        } else if (x, y) == end {
            // already found the best for this configuration
            continue;
        }

        for cheat in match cheat {
            Cheat::NoCheat => vec![
                Cheat::NoCheat,
                Cheat::CheatStart {
                    start: (x, y),
                    dur: 0,
                },
            ]
            .into_iter(),
            _ => vec![cheat].into_iter(),
        } {
            for pos in [
                (x + 1, y),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_sub(1)),
            ] {
                let wall = {
                    let (x, y) = pos;
                    if x >= width || y >= height {
                        continue;
                    }

                    grid[y as usize][x as usize] == b'#'
                };

                if wall && !matches!(cheat, Cheat::CheatStart { .. }) {
                    continue;
                }

                for next_cheat in match cheat {
                    Cheat::CheatStart { start, dur } if !wall && dur < max_cheat_dur => vec![
                        Cheat::CheatStart {
                            start,
                            dur: dur + 1,
                        },
                        Cheat::CheatEnded { start, end: pos },
                    ]
                    .into_iter(),
                    Cheat::CheatStart { start, dur } if dur < max_cheat_dur => {
                        vec![Cheat::CheatStart {
                            start,
                            dur: dur + 1,
                        }]
                        .into_iter()
                    }
                    _ => vec![cheat].into_iter(),
                } {
                    let (time, p) = match next_cheat {
                        Cheat::CheatEnded { .. } => (time + 1 + no_cheat_to_end[&pos], end),
                        _ => (time + 1, pos),
                    };

                    match best.entry((next_cheat, p)) {
                        Entry::Occupied(e) => {
                            let v = e.into_mut();

                            if *v > time {
                                *v = time;
                                todo.insert((time, next_cheat, pos));
                            }
                        }
                        Entry::Vacant(e) => {
                            e.insert(time);
                            todo.insert((time, next_cheat, pos));
                        }
                    }
                }
            }
        }
    }

    let no_cheat = best[&(Cheat::NoCheat, end)];
    let mut num_paths = 0;
    let mut paths_count: HashMap<usize, usize> = HashMap::new();
    let mut paths_cheats: HashMap<usize, Vec<Cheat>> = HashMap::new();

    for ((cheat, pos), time) in best.into_iter() {
        if pos == end {
            if matches!(cheat, Cheat::CheatEnded { .. })
                && no_cheat > time
                && no_cheat - time >= save
            {
                num_paths += 1;
                (paths_cheats.entry(no_cheat - time).or_default()).push(cheat);
                *(paths_count.entry(no_cheat - time).or_default()) += 1;
            }
        }
    }

    num_paths
}

#[test]
fn day20() {
    let example1 = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    let p1 = solve(&example1, 12, 2);
    assert_eq!(p1, 3 + 1 + 1 + 1 + 1 + 1);

    let p2 = solve(&example1, 66, 20);
    assert_eq!(p2, 12 + 14 + 12 + 22 + 4 + 3);
}
