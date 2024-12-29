use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day22.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (u64, u64) {
    let mut p1 = 0;

    let mut seq_bananas: HashMap<[i8; 4], u64> = HashMap::new();
    let mut seen_seq: HashSet<[i8; 4]> = HashSet::new();

    for line in input.lines() {
        let mut x = line.parse().unwrap();

        seen_seq.clear();
        let mut seq = [0; 4];
        let mut prev = 0;

        for i in 0..2000 {
            x = process(x);

            let bananas = x % 10;
            let price = bananas as i8;
            shift_in(&mut seq, price - prev);
            prev = price;

            if i >= 4 {
                if seen_seq.insert(seq) {
                    *seq_bananas.entry(seq).or_default() += bananas;
                }
            }
        }

        p1 += x;
    }

    let p2 = *seq_bananas.values().max().unwrap();

    (p1, p2)
}

fn shift_in(seq: &mut [i8; 4], x: i8) {
    let [_, a, b, c] = *seq;
    *seq = [a, b, c, x];
}

fn process(mut x: u64) -> u64 {
    const M: u64 = 16777216;
    x = (x ^ (x << 6)) % M;
    x = (x ^ (x >> 5)) % M;
    x = (x ^ (x * 2048)) % M;
    x
}

#[test]
fn day22() {
    let x = 123;
    let x = process(x);
    assert_eq!(x, 15887950);
    let x = process(x);
    assert_eq!(x, 16495136);
    let x = process(x);
    assert_eq!(x, 527345);

    let example1 = r"1
10
100
2024
";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 37327623);

    let example2 = r"1
2
3
2024
";

    let (_p1, p2) = solve(&example2);
    assert_eq!(p2, 23);
}
