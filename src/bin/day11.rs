use std::collections::{BTreeSet, HashMap};

fn main() {
    let input = std::fs::read_to_string("inputs/day11.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut memoized: HashMap<(usize, u64), usize> = HashMap::new();
    let mut todo: BTreeSet<(usize, u64)> = stones
        .iter()
        .flat_map(|stone| [(25, *stone), (75, *stone)].into_iter())
        .collect();

    while let Some(current @ (blink_left, stone)) = todo.pop_first() {
        if memoized.contains_key(&current) {
            continue;
        }

        if blink_left == 0 {
            memoized.insert(current, 1);
            continue;
        }

        if stone == 0 {
            let next = (blink_left - 1, 1);
            if let Some(count) = memoized.get(&next) {
                memoized.insert(current, *count);
            } else {
                todo.insert(next);
                todo.insert(current);
            }
            continue;
        }

        let digits = 1 + stone.ilog10();
        if digits % 2 == 0 {
            let d = 10u64.pow(digits / 2);
            let a = (blink_left - 1, stone / d);
            let b = (blink_left - 1, stone % d);

            if let Some((a, b)) = memoized.get(&a).zip(memoized.get(&b)) {
                memoized.insert(current, *a + *b);
            } else {
                todo.insert(a);
                todo.insert(b);
                todo.insert(current);
            }
        } else {
            let next = (blink_left - 1, stone * 2024);
            if let Some(count) = memoized.get(&next) {
                memoized.insert(current, *count);
            } else {
                todo.insert(next);
                todo.insert(current);
            }
        }
    }

    let p1 = stones.iter().map(|stone| memoized[&(25, *stone)]).sum();
    let p2 = stones.iter().map(|stone| memoized[&(75, *stone)]).sum();

    (p1, p2)
}

#[test]
fn day11() {
    let example1 = "125 17\n";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 55312);
}
