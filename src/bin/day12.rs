use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day12.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut regions: Vec<(u8, HashSet<(i16, i16)>)> = Vec::new();

    let mut up_line = [].as_slice();
    let mut up_regions = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let line = line.as_bytes();

        let mut left = None;
        let mut left_region: Option<usize> = None;
        let mut current_regions = Vec::new();

        for (x, c) in line.iter().enumerate() {
            let region = if Some(c) == left
                && Some(c) == up_line.get(x)
                && left_region.unwrap() != up_regions[x]
            {
                // join existing regions

                let a = left_region.unwrap();
                let b = up_regions[x];
                let from = a.max(b);
                let to = a.min(b);

                assert_eq!(regions[from].0, regions[to].0);

                let plots_it = std::mem::take(&mut regions[from].1).into_iter();

                regions[to].1.extend(plots_it);

                current_regions.iter_mut().for_each(|r| {
                    if *r == from {
                        *r = to;
                    }
                });
                up_regions.iter_mut().for_each(|r| {
                    if *r == from {
                        *r = to;
                    }
                });

                to
            } else if Some(c) == left {
                left_region.unwrap()
            } else if Some(c) == up_line.get(x) {
                up_regions[x]
            } else {
                let r = regions.len();
                regions.push((*c, HashSet::new()));
                r
            };

            regions[region].1.insert((x as i16, y as i16));

            left = Some(c);
            left_region = Some(region);
            current_regions.push(region);
        }

        up_line = line;
        up_regions = current_regions;
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for (_, region) in regions.into_iter() {
        let area = region.len();
        let mut perimiter = 0;

        let mut up_sides = HashMap::<i16, BTreeSet<i16>>::new();
        let mut down_sides = HashMap::<i16, BTreeSet<i16>>::new();
        let mut left_sides = HashMap::<i16, BTreeSet<i16>>::new();
        let mut right_sides = HashMap::<i16, BTreeSet<i16>>::new();

        for (x, y) in region.iter() {
            // up edge
            if !region.contains(&(*x, *y - 1)) {
                up_sides.entry(*y).or_default().insert(*x);
                perimiter += 1;
            }
            // down edge
            if !region.contains(&(*x, *y + 1)) {
                down_sides.entry(*y).or_default().insert(*x);
                perimiter += 1;
            }
            // left edge
            if !region.contains(&(*x - 1, *y)) {
                left_sides.entry(*x).or_default().insert(*y);
                perimiter += 1;
            }
            // right edge
            if !region.contains(&(*x + 1, *y)) {
                right_sides.entry(*x).or_default().insert(*y);
                perimiter += 1;
            }
        }

        let sides: usize = up_sides
            .into_values()
            .chain(down_sides.into_values())
            .chain(left_sides.into_values())
            .chain(right_sides.into_values())
            .map(|sides| {
                let mut chunk_count = 0;
                let mut prev = None;
                for i in sides.into_iter() {
                    if prev != Some(i - 1) {
                        chunk_count += 1;
                    }
                    prev = Some(i);
                }

                chunk_count
            })
            .sum();

        p1 += area * perimiter;
        p2 += area * sides;
    }

    (p1, p2)
}

#[test]
fn day12() {
    let example1 = r"AAAA
BBCD
BBCC
EEEC
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 140);
    assert_eq!(p2, 80);

    let example2 = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    let (p1, p2) = solve(&example2);
    assert_eq!(p1, 772);
    assert_eq!(p2, 436);

    let example3 = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    let (p1, p2) = solve(&example3);
    assert_eq!(p1, 1930);
    assert_eq!(p2, 1206);

    let example4 = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    let (_p1, p2) = solve(&example4);
    assert_eq!(p2, 236);

    let example5 = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    let (_p1, p2) = solve(&example5);
    assert_eq!(p2, 368);
}
