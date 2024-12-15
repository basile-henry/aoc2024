use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/day14.txt").unwrap();
    let (p1, p2) = solve(&input, 101, 103);
    println!("{} {}", p1, p2);
}

fn parse_pv(input: &str) -> ((i64, i64), (i64, i64)) {
    let mut it = input.split(&['=', ',', ' ']);
    it.next();
    let px = it.next().unwrap().parse().unwrap();
    let py = it.next().unwrap().parse().unwrap();
    it.next();
    let vx = it.next().unwrap().parse().unwrap();
    let vy = it.next().unwrap().parse().unwrap();
    ((px, py), (vx, vy))
}

fn solve(input: &str, width: i64, height: i64) -> (usize, usize) {
    let mut nw = 0;
    let mut ne = 0;
    let mut se = 0;
    let mut sw = 0;

    let mut robots = Vec::new();

    for line in input.lines() {
        let robot @ ((px, py), (vx, vy)) = parse_pv(line);
        robots.push(robot);

        let (x, y) = (
            (px + 100 * vx).rem_euclid(width),
            (py + 100 * vy).rem_euclid(height),
        );

        if x == width / 2 || y == height / 2 {
            continue;
        }

        match (y < height / 2, x < width / 2) {
            (true, true) => nw += 1,
            (true, false) => ne += 1,
            (false, true) => sw += 1,
            (false, false) => se += 1,
        }
    }

    fn score(robots: &HashSet<(i64, i64)>) -> usize {
        let mut out = 0;

        for (x, y) in robots {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == dy {
                        continue;
                    }

                    if robots.contains(&(x + dx, y + dy)) {
                        out += 1;
                    }
                }
            }
        }

        out
    }

    let p1 = nw * ne * se * sw;
    let mut p2 = 0;

    for i in 0..10000 {
        let current: HashSet<_> = robots
            .iter()
            .map(|((px, py), (vx, vy))| {
                (
                    (px + i * vx).rem_euclid(width),
                    (py + i * vy).rem_euclid(height),
                )
            })
            .collect();

        let s = score(&current);
        if s > 1000 {
            p2 = i as usize;
            println!("{i} ({s}):");
            for y in 0..height {
                for x in 0..width {
                    if current.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }

    (p1, p2)
}

#[test]
fn day14() {
    let example1 = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    let (p1, _p2) = solve(&example1, 11, 7);
    assert_eq!(p1, 12);
}
