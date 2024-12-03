fn main() {
    let input = std::fs::read_to_string("inputs/day02.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

#[derive(Clone)]
struct Checker {
    increasing: bool,
    decreasing: bool,
    valid_diff: bool,
    last: Option<usize>,
}

impl Checker {
    fn new() -> Self {
        Self {
            increasing: true,
            decreasing: true,
            valid_diff: true,
            last: None,
        }
    }

    fn check(&mut self, x: usize) {
        if let Some(l) = self.last {
            self.increasing &= x > l;
            self.decreasing &= x < l;
            self.valid_diff &= (1..=3).contains(&x.abs_diff(l));
        }

        self.last = Some(x);
    }

    fn valid(&self) -> bool {
        (self.increasing || self.decreasing) && self.valid_diff
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let mut p1_checker = Checker::new();
        let mut p2_checkers = vec![(false, Checker::new())];

        for x in line.split_whitespace().map(|x| x.parse().unwrap()) {
            p1_checker.check(x);

            for (has_skipped, mut checker) in std::mem::take(&mut p2_checkers).into_iter() {
                if has_skipped {
                    checker.check(x);
                    p2_checkers.push((true, checker));
                } else {
                    p2_checkers.push((true, checker.clone())); // skipping x
                    checker.check(x);
                    p2_checkers.push((false, checker));
                }
            }
        }

        if p1_checker.valid() {
            p1 += 1;
        }
        if p2_checkers.iter().any(|(_, c)| c.valid()) {
            p2 += 1;
        }
    }

    (p1, p2)
}

#[test]
fn day02() {
    let example1 = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 2);
    assert_eq!(p2, 4);
}
