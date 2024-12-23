use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BTreeSet, HashMap, HashSet},
};

fn main() {
    let input = std::fs::read_to_string("inputs/day16.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut start = None;
    let mut end = None;
    let grid: Vec<Vec<u8>> = input
        .split_whitespace()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| match *c {
                    b'S' => {
                        start = Some((x as i16, y as i16));
                        b'.'
                    }
                    b'E' => {
                        end = Some((x as i16, y as i16));
                        b'.'
                    }
                    _ => *c,
                })
                .collect()
        })
        .collect();

    let start = start.unwrap();
    let end = end.unwrap();

    let mut best = HashMap::new();
    best.insert((start, (1, 0)), (0, Vec::new()));

    let mut todo = BTreeSet::new();
    todo.insert((0, start, (1, 0), false));

    let mut candidates = Vec::new();

    let mut p1 = usize::MAX;
    let mut p1_key = ((0, 0), (0, 0));

    while let Some((score, (x, y), (dx, dy), just_turned)) = todo.pop_first() {
        candidates.clear();
        candidates.push((score + 1, (x + dx, y + dy), (dx, dy), false));
        if !just_turned {
            if dx == 0 {
                candidates.push((score + 1000, (x, y), (-1, 0), true));
                candidates.push((score + 1000, (x, y), (1, 0), true));
            } else {
                candidates.push((score + 1000, (x, y), (0, -1), true));
                candidates.push((score + 1000, (x, y), (0, 1), true));
            }
        }

        let cur_key = ((x, y), (dx, dy));

        for next @ (score, (x, y), (dx, dy), _) in candidates.iter().copied() {
            if x < 0 || y < 0 || grid[y as usize][x as usize] == b'#' {
                continue;
            }

            if score > p1 {
                continue;
            }

            let new_key = ((x, y), (dx, dy));
            match best.entry(new_key) {
                Entry::Occupied(e) => {
                    let (best_score, prevs) = e.into_mut();

                    match score.cmp(best_score) {
                        Ordering::Less => {
                            *best_score = score;
                            prevs.clear();
                            prevs.push(cur_key);
                        }
                        Ordering::Equal => {
                            prevs.push(cur_key);
                        }
                        Ordering::Greater => {
                            continue;
                        }
                    }
                }
                Entry::Vacant(e) => {
                    e.insert((score, vec![cur_key]));
                }
            }

            if (x, y) == end && score < p1 {
                p1 = score;
                p1_key = new_key;
            }

            todo.insert(next);
        }
    }

    let mut best_visited = HashSet::new();
    best_visited.insert(end);

    let mut todo_prev = BTreeSet::new();
    todo_prev.insert(p1_key);

    while let Some(prev_key) = todo_prev.pop_first() {
        for key in best[&prev_key].1.iter() {
            best_visited.insert(key.0);
            todo_prev.insert(*key);
        }
    }

    let p2 = best_visited.len();

    (p1, p2)
}

#[test]
fn day16() {
    let example1 = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 7036);
    assert_eq!(p2, 45);

    let example2 = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    let (p1, p2) = solve(&example2);
    assert_eq!(p1, 11048);
    assert_eq!(p2, 64);
}
