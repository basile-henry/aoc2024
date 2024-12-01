use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut xs: Vec<usize> = Vec::new();
    let mut ys: Vec<usize> = Vec::new();

    let mut ys_count: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let mut line = line.split_whitespace();
        let x = line.next().unwrap().parse().unwrap();
        let y = line.next().unwrap().parse().unwrap();

        xs.push(x);
        ys.push(y);

        *ys_count.entry(y).or_default() += 1;
    }

    xs.sort();
    ys.sort();

    let p1 = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| usize::abs_diff(*x, *y))
        .sum();

    let p2 = xs.iter().map(|x| *x * *ys_count.get(x).unwrap_or(&0)).sum();

    (p1, p2)
}

#[test]
fn day01() {
    let example1 = r"3   4
4   3
2   5
1   3
3   9
3   3
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 11);
    assert_eq!(p2, 31);
}
