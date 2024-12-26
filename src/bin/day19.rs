use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day19.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solvable_count<'a>(
    cache: &mut HashMap<&'a str, usize>,
    patterns: &[&str],
    design: &'a str,
) -> usize {
    if let Some(n) = cache.get(design) {
        return *n;
    }

    let mut count = 0;

    for pat in patterns.iter() {
        if let Some(rest) = design.strip_prefix(pat) {
            if rest.is_empty() {
                count += 1;
            } else {
                count += solvable_count(cache, patterns, rest);
            }
        }
    }

    cache.insert(design, count);

    count
}

fn solve(input: &str) -> (usize, usize) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.split(", ").collect();

    let mut cache = HashMap::new();

    let mut p1 = 0;
    let mut p2 = 0;

    for design in designs.lines() {
        let c = solvable_count(&mut cache, &patterns, design);

        if c > 0 {
            p1 += 1;
        }

        p2 += c;
    }

    (p1, p2)
}

#[test]
fn day19() {
    let example1 = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 6);
    assert_eq!(p2, 16);
}
