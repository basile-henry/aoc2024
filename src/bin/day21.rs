use std::{collections::HashMap, iter::repeat};

fn main() {
    let input = std::fs::read_to_string("inputs/day21.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    for code in input.lines() {
        let num: usize = code.strip_suffix("A").unwrap().parse().unwrap();

        let mut x = shortest(&NUM_POS, code);

        for _ in 0..2 {
            for (k, v) in std::mem::take(&mut x).drain() {
                for (k_, v_) in shortest(&DIR_POS, &k).drain() {
                    *x.entry(k_).or_default() += v * v_;
                }
            }
        }

        let l: usize = x.iter().map(|(k, v)| k.len() * v).sum();
        p1 += num * l;

        for _ in 2..25 {
            for (k, v) in std::mem::take(&mut x).drain() {
                for (k_, v_) in shortest(&DIR_POS, &k).drain() {
                    *x.entry(k_).or_default() += v * v_;
                }
            }
        }

        let l: usize = x.into_iter().map(|(k, v)| k.len() * v).sum();
        p2 += num * l;
    }

    (p1, p2)
}

const NUM_POS: [(i8, i8); 128] = {
    let mut arr = [(0, 0); 128];
    arr['7' as usize] = (0, 0);
    arr['8' as usize] = (1, 0);
    arr['9' as usize] = (2, 0);
    arr['4' as usize] = (0, 1);
    arr['5' as usize] = (1, 1);
    arr['6' as usize] = (2, 1);
    arr['1' as usize] = (0, 2);
    arr['2' as usize] = (1, 2);
    arr['3' as usize] = (2, 2);
    arr['G' as usize] = (0, 3);
    arr['0' as usize] = (1, 3);
    arr['A' as usize] = (2, 3);
    arr
};

const DIR_POS: [(i8, i8); 128] = {
    let mut arr = [(0, 0); 128];
    arr['G' as usize] = (0, 0);
    arr['^' as usize] = (1, 0);
    arr['A' as usize] = (2, 0);
    arr['<' as usize] = (0, 1);
    arr['v' as usize] = (1, 1);
    arr['>' as usize] = (2, 1);
    arr
};

fn shortest(mapping: &[(i8, i8)], code: &str) -> HashMap<String, usize> {
    let mut cur = mapping['A' as usize];
    let mut out = HashMap::new();

    let gap = mapping['G' as usize];

    for x in code.chars() {
        let mut steps = String::new();
        let to = mapping[x as usize];

        let dx = to.0 - cur.0;
        let dy = to.1 - cur.1;

        let would_go_via_gap =
            (to.0 == gap.0 && cur.1 == gap.1) || (to.1 == gap.1 && cur.0 == gap.0);

        if would_go_via_gap {
            // prioritise going right, then up/down, then left to avoid gap
            if dx > 0 {
                steps.extend(repeat('>').take(dx as usize));
            }

            if dy > 0 {
                steps.extend(repeat('v').take(dy as usize));
            } else {
                steps.extend(repeat('^').take((-dy) as usize));
            }

            if dx < 0 {
                steps.extend(repeat('<').take((-dx) as usize));
            }
        } else {
            if dx < 0 {
                steps.extend(repeat('<').take((-dx) as usize));
            }

            if dy > 0 {
                steps.extend(repeat('v').take(dy as usize));
            } else {
                steps.extend(repeat('^').take((-dy) as usize));
            }

            if dx > 0 {
                steps.extend(repeat('>').take(dx as usize));
            }
        }

        steps.push('A');

        *out.entry(steps).or_default() += 1;

        cur = to;
    }

    out
}

#[test]
fn day21() {
    let example1 = r"029A
980A
179A
456A
379A
";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 126384);
}
