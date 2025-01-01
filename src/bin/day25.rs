fn main() {
    let input = std::fs::read_to_string("inputs/day25.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for block in input.split("\n\n") {
        let mut cols = vec![-1; 5];

        for line in block.lines() {
            for (x, c) in line.as_bytes().iter().enumerate() {
                if *c == b'#' {
                    cols[x] += 1;
                }
            }
        }

        if block.as_bytes()[0] == b'#' {
            locks.push(cols);
        } else {
            keys.push(cols);
        }
    }

    let mut p1 = 0;

    for k in keys.iter() {
        for l in locks.iter() {
            if k.iter().zip(l.iter()).all(|(a, b)| a + b < 6) {
                p1 += 1;
            }
        }
    }

    let p2 = 0;

    (p1, p2)
}

#[test]
fn day25() {
    let example1 = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 3);
}
