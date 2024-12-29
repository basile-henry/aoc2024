use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day23.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, String) {
    let mut connections: HashMap<&str, BTreeSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();

        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    let mut sets = HashSet::new();
    let mut triples_with_t = HashSet::new();
    for (k, v) in connections.iter() {
        for x in v.iter() {
            let mut set = BTreeSet::new();
            set.insert(x);
            for y in v.iter() {
                if x == y {
                    continue;
                }

                if set.iter().copied().all(|z| connections[y].contains(z)) {
                    set.insert(y);
                }
            }

            set.insert(k);
            sets.insert(set);
        }

        if !k.starts_with("t") {
            continue;
        }

        for x in v.iter() {
            for y in v.iter() {
                if x <= y {
                    continue;
                }

                if connections[x].contains(y) {
                    let mut triple = vec![k, x, y];
                    triple.sort();
                    triples_with_t.insert(triple);
                }
            }
        }
    }

    let p1 = triples_with_t.len();
    let mut comp_iter = sets
        .into_iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .into_iter();
    let mut p2 = comp_iter.next().unwrap().to_string();
    for comp in comp_iter {
        p2.push(',');
        p2.push_str(comp);
    }

    (p1, p2)
}

#[test]
fn day23() {
    let example1 = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 7);
    assert_eq!(&p2, "co,de,ka,ta");
}
