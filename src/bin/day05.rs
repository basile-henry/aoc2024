use bitvec::vec::BitVec;

fn main() {
    let input = std::fs::read_to_string("inputs/day05.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn valid_update(rule_lt: &[BitVec], update: &[u8]) -> bool {
    for w in update.windows(2) {
        let [x, y] = w else { panic!() };

        // check if a rule exist that says otherwise
        if rule_lt[*y as usize][*x as usize] {
            // failed a rule
            return false;
        }
    }

    return true;
}

fn fix_update(rule_lt: &[BitVec], update: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();

    for x in update {
        // find the first index where the new update needs to happen before
        let ix = out
            .iter()
            .position(|y| rule_lt[*x as usize][*y as usize])
            .unwrap_or(out.len());

        out.insert(ix, *x);
    }

    out
}

fn solve(input: &str) -> (usize, usize) {
    let mut rule_lt: Vec<BitVec> = (0..100).map(|_| BitVec::repeat(false, 100)).collect();

    let mut updates: Vec<Vec<u8>> = Vec::new();

    let mut it = input.lines();
    while let Some(line) = it.next() {
        if line.is_empty() {
            // ordering rules over
            break;
        }

        let (l, g) = line.split_once('|').unwrap();
        let l: usize = l.parse().unwrap();
        let g: usize = g.parse().unwrap();

        rule_lt[l].set(g, true);
    }

    while let Some(line) = it.next() {
        let mut update = Vec::new();
        for x in line.split(',') {
            update.push(x.parse().unwrap());
        }
        updates.push(update);
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for update in updates.iter() {
        if valid_update(&rule_lt, update) {
            p1 += update[update.len() / 2] as usize;
        } else {
            let fixed_update = fix_update(&rule_lt, update);
            p2 += fixed_update[fixed_update.len() / 2] as usize;
        }
    }

    (p1, p2)
}

#[test]
fn day05() {
    let example1 = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 143);
    assert_eq!(p2, 123);
}
