use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day08.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

type V2 = (i16, i16);

fn solve(input: &str) -> (usize, usize) {
    let mut antennas = HashMap::<u8, Vec<V2>>::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        let line = line.as_bytes();
        width = line.len() as i16;
        height += 1;

        for (x, c) in line.iter().enumerate() {
            if *c != b'.' {
                antennas.entry(*c).or_default().push((x as i16, y as i16));
            }
        }
    }

    let inside = |(x, y)| x >= 0 && x < width && y >= 0 && y < height;

    let mut antinodes_p1 = HashSet::<V2>::new();
    let mut antinodes_p2 = HashSet::<V2>::new();

    for (_, ps) in antennas.iter() {
        for a in ps.iter() {
            for b in ps.iter() {
                if a == b {
                    continue;
                }

                let diff = (a.0 - b.0, a.1 - b.1);

                // part 1
                {
                    let c = (a.0 + diff.0, a.1 + diff.1);
                    let d = (b.0 - diff.0, b.1 - diff.1);

                    if inside(c) {
                        antinodes_p1.insert(c);
                    }
                    if inside(d) {
                        antinodes_p1.insert(d);
                    }
                }

                // part 2
                {
                    let mut p = *a;
                    while inside(p) {
                        antinodes_p2.insert(p);
                        p = (p.0 + diff.0, p.1 + diff.1);
                    }
                    let mut p = *b;
                    while inside(p) {
                        antinodes_p2.insert(p);
                        p = (p.0 - diff.0, p.1 - diff.1);
                    }
                }
            }
        }
    }

    let p1 = antinodes_p1.len();
    let p2 = antinodes_p2.len();

    (p1, p2)
}

#[test]
fn day08() {
    let example1 = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 14);
    assert_eq!(p2, 34);
}
