use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    process::Command,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day24.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Input {
    X(u8),
    Y(u8),
    O(u8),
}
impl Input {
    fn parse<'a>(next_o: &mut u8, interning: &mut HashMap<&'a str, u8>, input: &'a str) -> Input {
        if let Some(n) = input.strip_prefix("x") {
            Input::X(n.parse().unwrap())
        } else if let Some(n) = input.strip_prefix("y") {
            Input::Y(n.parse().unwrap())
        } else {
            match interning.entry(input) {
                Entry::Occupied(e) => Input::O(*e.get()),
                Entry::Vacant(e) => {
                    e.insert(*next_o);
                    let r = Input::O(*next_o);
                    *next_o += 1;
                    r
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Output {
    O(u8),
    Z(u8),
}

impl Output {
    fn parse<'a>(next_o: &mut u8, interning: &mut HashMap<&'a str, u8>, input: &'a str) -> Output {
        if let Some(n) = input.strip_prefix("z") {
            Output::Z(n.parse().unwrap())
        } else {
            match interning.entry(input) {
                Entry::Occupied(e) => Output::O(*e.get()),
                Entry::Vacant(e) => {
                    e.insert(*next_o);
                    let r = Output::O(*next_o);
                    *next_o += 1;
                    r
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    ty: GateType,
    a: Input,
    b: Input,
    c: Output,
}

impl Gate {
    fn parse<'a>(next_o: &mut u8, interning: &mut HashMap<&'a str, u8>, line: &'a str) -> Gate {
        let (inputs, c) = line.split_once(" -> ").unwrap();

        if let Some((a, b)) = inputs.split_once(" AND ") {
            Gate {
                ty: GateType::And,
                a: Input::parse(next_o, interning, a),
                b: Input::parse(next_o, interning, b),
                c: Output::parse(next_o, interning, c),
            }
        } else if let Some((a, b)) = inputs.split_once(" OR ") {
            Gate {
                ty: GateType::Or,
                a: Input::parse(next_o, interning, a),
                b: Input::parse(next_o, interning, b),
                c: Output::parse(next_o, interning, c),
            }
        } else if let Some((a, b)) = inputs.split_once(" XOR ") {
            Gate {
                ty: GateType::Xor,
                a: Input::parse(next_o, interning, a),
                b: Input::parse(next_o, interning, b),
                c: Output::parse(next_o, interning, c),
            }
        } else {
            panic!("unexpected");
        }
    }
}

struct Circuit {
    n_o: usize,
    n_z: usize,
    gates: Vec<Gate>,
}

impl Circuit {
    fn solve(&self, x_bits: Vec<bool>, y_bits: Vec<bool>) -> Vec<bool> {
        let mut o_bits: Vec<Option<bool>> = vec![None; self.n_o];
        let mut z_bits: Vec<Option<bool>> = vec![None; self.n_z];

        let mut todo: VecDeque<Gate> = self.gates.iter().cloned().collect();

        while let Some(gate) = todo.pop_front() {
            let Some(a) = (match gate.a {
                Input::X(i) => Some(x_bits[i as usize]),
                Input::Y(i) => Some(y_bits[i as usize]),
                Input::O(i) => o_bits[i as usize],
            }) else {
                todo.push_back(gate);
                continue;
            };

            let Some(b) = (match gate.b {
                Input::X(i) => Some(x_bits[i as usize]),
                Input::Y(i) => Some(y_bits[i as usize]),
                Input::O(i) => o_bits[i as usize],
            }) else {
                todo.push_back(gate);
                continue;
            };

            let r = match gate.ty {
                GateType::And => a && b,
                GateType::Or => a || b,
                GateType::Xor => a ^ b,
            };

            match gate.c {
                Output::O(i) => o_bits[i as usize] = Some(r),
                Output::Z(i) => z_bits[i as usize] = Some(r),
            }
        }

        z_bits.into_iter().map(|b| b.unwrap()).collect()
    }

    fn solve_num(&self, x: u64, y: u64, bits: usize) -> u64 {
        let x_bits = (0..bits).map(|b| ((x >> b) & 1) == 1).collect();
        let y_bits = (0..bits).map(|b| ((y >> b) & 1) == 1).collect();
        let z_bits = self.solve(x_bits, y_bits);
        let mut z = 0;
        for b in z_bits.into_iter().rev() {
            z <<= 1;
            z += if b { 1 } else { 0 };
        }
        z
    }

    fn svg(&self, n_bits: usize, name: &str) {
        let dot_path = &format!("{name}.dot");
        self.dot(n_bits, dot_path);

        let svg_file = std::fs::File::create(&format!("{name}.svg")).unwrap();
        Command::new("dot")
            .args(["-Tsvg", dot_path])
            .stdout(svg_file)
            .output()
            .unwrap();
    }

    fn dot(&self, n_bits: usize, path: &str) {
        use std::io::Write;

        let mut file = std::fs::File::create(path).unwrap();

        fn input_dot(x: &Input) -> String {
            match x {
                Input::X(i) => format!("X{}", i),
                Input::Y(i) => format!("Y{}", i),
                Input::O(i) => format!("O{}", i),
            }
        }

        fn output_dot(x: &Output) -> String {
            match x {
                Output::Z(i) => format!("Z{}", i),
                Output::O(i) => format!("O{}", i),
            }
        }

        writeln!(&mut file, "digraph {{").unwrap();

        for i in 0..n_bits {
            writeln!(&mut file, "  X{} [label=\"X{}\"]", i, i).unwrap();
            writeln!(&mut file, "  Y{} [label=\"Y{}\"]", i, i).unwrap();
        }
        for i in 0..self.n_o {
            writeln!(&mut file, "  O{} [label=\"O{}\"]", i, i).unwrap();
        }
        for i in 0..self.n_z {
            writeln!(&mut file, "  Z{} [label=\"Z{}\"]", i, i).unwrap();
        }

        for (i, gate) in self.gates.iter().enumerate() {
            writeln!(
                &mut file,
                "  G{} [label=\"Gate {}: {}\" color=\"red\" shape=\"box\"]",
                i,
                i,
                match gate.ty {
                    GateType::And => "AND",
                    GateType::Or => "OR",
                    GateType::Xor => "XOR",
                }
            )
            .unwrap();
            writeln!(&mut file, "  {} -> G{}", input_dot(&gate.a), i).unwrap();
            writeln!(&mut file, "  {} -> G{}", input_dot(&gate.b), i).unwrap();
            writeln!(&mut file, "  G{} -> {}", i, output_dot(&gate.c)).unwrap();
        }

        writeln!(&mut file, "}}").unwrap();
    }
}

fn solve(input: &str) -> (usize, String) {
    let (inputs, circuit) = input.split_once("\n\n").unwrap();

    let mut next_o = 0;
    let mut interning = HashMap::new();

    let mut x_bits = Vec::new();
    let mut y_bits = Vec::new();

    for line in inputs.lines() {
        if let Some(line) = line.strip_prefix(&format!("x{:02}: ", x_bits.len())) {
            x_bits.push(line == "1");
        } else if let Some(line) = line.strip_prefix(&format!("y{:02}: ", y_bits.len())) {
            y_bits.push(line == "1");
        } else {
            panic!("unexpected");
        }
    }

    let gates: Vec<Gate> = circuit
        .lines()
        .map(|line| Gate::parse(&mut next_o, &mut interning, line))
        .collect();

    let n_z = gates
        .iter()
        .map(|g| match g.c {
            Output::O(_) => 0,
            Output::Z(z) => z as usize,
        })
        .max()
        .unwrap()
        + 1;

    let mut circuit = Circuit {
        n_o: next_o as usize,
        n_z,
        gates,
    };

    // part 1
    let z_bits = circuit.solve(x_bits, y_bits);

    let mut z = 0;
    for b in z_bits.into_iter().rev() {
        z <<= 1;
        z += if b { 1 } else { 0 };
    }

    // part 2 (somewhat manual)
    let n_bits = n_z - 1;

    circuit.svg(n_bits, "circuit_orig");

    let reference = adder(n_bits);
    reference.svg(n_bits, "reference");

    // Just staring at SVGs and running tests ^^
    let swaps = [(199, 187), (21, 182), (194, 91), (193, 58)];

    for (i, j) in swaps.iter().copied() {
        let t = circuit.gates[i].c.clone();
        circuit.gates[i].c = circuit.gates[j].c.clone();
        circuit.gates[j].c = t;
    }

    circuit.svg(n_bits, "circuit_swapped");

    for a in 0..n_bits {
        for b in 0..n_bits {
            let x = 1 << a;
            let y = 1 << b;
            assert_eq!(
                x + y,
                circuit.solve_num(x, y, n_bits),
                "(1 << {a}) + (1 << {b})"
            );
        }
    }

    let r_interning: HashMap<u8, &str> = interning.into_iter().map(|(k, v)| (v, k)).collect();

    let mut swapped_outputs: Vec<String> = swaps
        .into_iter()
        .flat_map(|(i, j)| [i, j].into_iter())
        .map(|i| match circuit.gates[i].c {
            Output::O(i) => r_interning[&i].to_string(),
            Output::Z(i) => format!("z{i:02}"),
        })
        .collect();
    swapped_outputs.sort();

    let p1 = z;
    let p2 = swapped_outputs.join(",");

    (p1, p2)
}

fn half_adder(offset: u8, next_o: &mut u8, gates: &mut Vec<Gate>) -> u8 {
    let cout = *next_o;
    *next_o += 1;

    gates.push(Gate {
        ty: GateType::Xor,
        a: Input::X(offset),
        b: Input::Y(offset),
        c: Output::Z(offset),
    });
    gates.push(Gate {
        ty: GateType::And,
        a: Input::X(offset),
        b: Input::Y(offset),
        c: Output::O(cout),
    });

    cout
}

fn full_adder(offset: u8, cin: u8, next_o: &mut u8, gates: &mut Vec<Gate>) -> u8 {
    let t0 = *next_o;
    *next_o += 1;
    let t1 = *next_o;
    *next_o += 1;
    let t2 = *next_o;
    *next_o += 1;
    let cout = *next_o;
    *next_o += 1;

    gates.push(Gate {
        ty: GateType::Xor,
        a: Input::X(offset),
        b: Input::Y(offset),
        c: Output::O(t0),
    });
    gates.push(Gate {
        ty: GateType::Xor,
        a: Input::O(t0),
        b: Input::O(cin),
        c: Output::Z(offset),
    });
    gates.push(Gate {
        ty: GateType::And,
        a: Input::O(t0),
        b: Input::O(cin),
        c: Output::O(t1),
    });
    gates.push(Gate {
        ty: GateType::And,
        a: Input::X(offset),
        b: Input::Y(offset),
        c: Output::O(t2),
    });
    gates.push(Gate {
        ty: GateType::Or,
        a: Input::O(t1),
        b: Input::O(t2),
        c: Output::O(cout),
    });

    cout
}

fn adder(n_bits: usize) -> Circuit {
    let mut gates = Vec::new();
    let mut next_o = 0;
    let mut carry = half_adder(0, &mut next_o, &mut gates);
    for i in 1..n_bits as u8 {
        carry = full_adder(i, carry, &mut next_o, &mut gates);
    }
    gates.last_mut().unwrap().c = Output::Z(n_bits as u8);
    next_o -= 1;

    let n_o = next_o as usize;

    Circuit {
        gates,
        n_o,
        n_z: n_bits + 1,
    }
}

#[test]
fn day24() {
    let example1 = r"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 4);

    let example2 = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    let (p1, _p2) = solve(&example2);
    assert_eq!(p1, 2024);

    let c = adder(45);
    assert!(all_connected_inputs(&c.gates));

    for x in 0..10 {
        for y in 0..10 {
            assert_eq!(c.solve_num(x, y, 45), x + y);
        }
    }
}
