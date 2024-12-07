use std::collections::{hash_map::Entry, HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day06.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut obstacles = HashSet::new();
    let mut start = (0i16, 0i16);

    let mut height = 0;
    let mut width = 0;
    for (y, line) in input.lines().enumerate() {
        let line = line.as_bytes();
        width = line.len() as i16;
        height += 1;

        for (x, c) in line.iter().enumerate() {
            let pos = (x as i16, y as i16);

            match c {
                b'#' => {
                    obstacles.insert(pos);
                }
                b'^' => {
                    start = pos;
                }
                _ => {}
            }
        }
    }

    let mut visited = HashMap::new();
    guard_path_loops(&mut visited, &obstacles, start, width, height);
    let p1 = visited.len();

    let mut obstacles_with_extra = obstacles.clone();
    let mut p2 = 0;
    // only worth putting obstacles in the original path since we're only adding 1 obstacle
    for pos in std::mem::take(&mut visited).keys() {
        if *pos == start {
            continue;
        }

        visited.clear();
        obstacles_with_extra.insert(*pos);

        if guard_path_loops(&mut visited, &obstacles_with_extra, start, width, height) {
            p2 += 1;
        }

        obstacles_with_extra.remove(pos);
    }

    (p1, p2)
}

type V2 = (i16, i16);

fn guard_path_loops(
    visited: &mut HashMap<V2, u8>,
    obstacles: &HashSet<V2>,
    start: V2,
    width: i16,
    height: i16,
) -> bool {
    let mut pos = start;
    let mut dir = (0, -1); // up
    let mut dir_mask = 0b0001;

    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        match visited.entry(pos) {
            Entry::Occupied(mut e) => {
                let prev = e.get_mut();

                if *prev & dir_mask == dir_mask {
                    // direction already used at this previous location, loop detected
                    return true;
                }

                *prev |= dir_mask;
            }
            Entry::Vacant(e) => {
                e.insert(dir_mask);
            }
        }

        let next = (pos.0 + dir.0, pos.1 + dir.1);

        if obstacles.contains(&next) {
            (dir, dir_mask) = match dir {
                (0, -1) => ((1, 0), 0b0010),  // up -> right
                (1, 0) => ((0, 1), 0b0100),   // right -> down
                (0, 1) => ((-1, 0), 0b1000),  // down -> left
                (-1, 0) => ((0, -1), 0b0001), // left -> up
                _ => unreachable!(),
            };
        } else {
            pos = next;
        }
    }

    false
}

#[test]
fn day06() {
    let example1 = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 41);
    assert_eq!(p2, 6);
}
