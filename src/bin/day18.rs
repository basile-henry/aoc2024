use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day18.txt").unwrap();
    let (p1, p2) = solve(&input, 1024, (70, 70));
    println!("{} {:?}", p1, p2);
}

fn solve(input: &str, start_bytes: usize, goal: (u8, u8)) -> (usize, (u8, u8)) {
    let mut bytes = Vec::new();

    for line in input.lines() {
        let (x, y) = line.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        bytes.push((x, y));
    }

    let start: (u8, u8) = (0, 0);
    let mut p1 = 0;
    let mut p2 = start;

    for n in start_bytes..bytes.len() {
        let mut best = HashMap::new();
        best.insert(start, 0);

        let mut todo = BTreeSet::new();
        todo.insert((0, start));

        let walls: HashSet<_> = bytes[..n].iter().copied().collect();
        let mut solution = false;

        'outer: while let Some((steps, (x, y))) = todo.pop_first() {
            for pos in [
                (x + 1, y),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_sub(1)),
            ] {
                if pos.0 > goal.0 || pos.1 > goal.1 || walls.contains(&pos) {
                    continue;
                }

                let steps = steps + 1;

                let v = best.entry(pos).or_insert(usize::MAX);
                if *v > steps {
                    *v = steps;
                    todo.insert((steps, pos));
                    if pos == goal {
                        if n == start_bytes {
                            p1 = steps;
                        }
                        solution = true;
                        break 'outer;
                    }
                }
            }
        }

        if !solution {
            p2 = bytes[n - 1];
            break;
        }
    }

    (p1, p2)
}

#[test]
fn day18() {
    let example1 = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    let (p1, p2) = solve(&example1, 12, (6, 6));
    assert_eq!(p1, 22);
    assert_eq!(p2, (6, 1));
}
