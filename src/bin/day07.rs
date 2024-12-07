fn main() {
    let input = std::fs::read_to_string("inputs/day07.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let (goal, rest) = line.split_once(": ").unwrap();
        let goal: usize = goal.parse().unwrap();
        let xs = rest.split_whitespace().map(|x| x.parse().unwrap());

        let (p1_sat, p2_sat) = satisty(goal, xs);
        if p1_sat {
            p1 += goal;
        }
        if p2_sat {
            p2 += goal;
        }
    }

    (p1, p2)
}

fn satisty(goal: usize, mut xs: impl Iterator<Item = usize>) -> (bool, bool) {
    let mut so_far = Vec::new();
    let mut so_far_with_concat = Vec::new();

    {
        let x = xs.next().unwrap();
        so_far.push(x);
        so_far_with_concat.push(x);
    }

    for x in xs {
        for y in std::mem::take(&mut so_far).into_iter() {
            so_far.push(y + x);
            so_far.push(y * x);
        }

        for y in std::mem::take(&mut so_far_with_concat).into_iter() {
            so_far_with_concat.push(y + x);
            so_far_with_concat.push(y * x);

            let shift = x.ilog10() + 1;
            let y = y * 10usize.pow(shift);
            so_far_with_concat.push(y + x);
        }
    }

    return (
        so_far.iter().any(|x| *x == goal),
        so_far_with_concat.iter().any(|x| *x == goal),
    );
}

#[test]
fn day07() {
    let example1 = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 3749);
    assert_eq!(p2, 11387);
}
