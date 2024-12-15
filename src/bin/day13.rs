fn main() {
    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn parse_xy(input: &str, split: &[char]) -> (i64, i64) {
    let mut it = input.split(split);
    it.next();
    let x = it.next().unwrap().parse().unwrap();
    it.next();
    let y = it.next().unwrap().parse().unwrap();
    (x, y)
}

/*
x = xa * a + xb * b
y = ya * a + yb * b
*/

/*
a = (x - xb * b) / xa
a = (y - yb * b) / ya
*/

/*
(x - xb * b) * ya = (y - yb * b) * xa
x * ya - xb * b * ya = y * xa - yb * b * xa
x * ya - y * xa = xb * b * ya - yb * b * xa
x * ya - y * xa = (xb * ya - yb * xa) * b
b = (x * ya - y * xa) / (xb * ya - yb * xa)
*/
fn find_tokens((xa, ya): (i64, i64), (xb, yb): (i64, i64), (x, y): (i64, i64)) -> Option<i64> {
    if xa * yb == xb * ya {
        // a and b are co-linear
        if xa * y == x * ya {
            unimplemented!()
        } else {
            None
        }
    } else {
        let b = (x * ya - y * xa) / (xb * ya - yb * xa);
        let a = (x - xb * b) / xa;

        if x == xa * a + xb * b && y == ya * a + yb * b {
            Some(3 * a + b)
        } else {
            None
        }
    }
}

fn solve(input: &str) -> (i64, i64) {
    let mut line_it = input.lines();

    let mut p1 = 0;
    let mut p2 = 0;

    while let Some(a) = line_it.next() {
        let b = line_it.next().unwrap();
        let g = line_it.next().unwrap();
        line_it.next();

        let a = parse_xy(a, &['+', ',']);
        let b = parse_xy(b, &['+', ',']);
        let (x, y) = parse_xy(g, &['=', ',']);

        if let Some(t) = find_tokens(a, b, (x, y)) {
            p1 += t;
        }

        if let Some(t) = find_tokens(a, b, (10000000000000 + x, 10000000000000 + y)) {
            p2 += t;
        }
    }

    (p1, p2)
}

#[test]
fn day13() {
    let example1 = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 480);
}
