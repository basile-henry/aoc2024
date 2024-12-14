use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid = Vec::new();
    let mut zeroes = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let line = line.as_bytes();
        grid.push(line);

        for (x, c) in line.iter().enumerate() {
            if *c == b'0' {
                zeroes.push((x, y));
            }
        }
    }

    let width = grid[0].len();
    let height = grid.len();

    let neighbours = |(x, y): (usize, usize)| {
        let mut out = Vec::with_capacity(4);
        if x > 0 {
            out.push((x - 1, y));
        }
        if x < width - 1 {
            out.push((x + 1, y));
        }
        if y > 0 {
            out.push((x, y - 1));
        }
        if y < height - 1 {
            out.push((x, y + 1));
        }

        out.into_iter()
    };

    let mut p1 = 0;
    let mut p2 = 0;
    for zero in zeroes.iter() {
        let mut todo = vec![(*zero, b'0')];
        let mut reached_9: HashMap<(usize, usize), usize> = HashMap::new();

        while let Some((pos, level)) = todo.pop() {
            if level == b'9' {
                *reached_9.entry(pos).or_default() += 1;
            } else {
                for n @ (x, y) in neighbours(pos) {
                    if grid[y][x] == level + 1 {
                        todo.push((n, level + 1));
                    }
                }
            }
        }

        p1 += reached_9.len();
        p2 += reached_9.values().sum::<usize>();
    }

    (p1, p2)
}

#[test]
fn day10() {
    let example1 = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 36);
    assert_eq!(p2, 81);
}
