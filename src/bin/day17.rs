fn main() {
    let input = std::fs::read_to_string("inputs/day17.txt").unwrap();
    let p1 = solve_p1(&input);
    let p2 = 0; // in python
    println!("{:?} {}", &p1[..], p2);
}

fn solve_p1(input: &str) -> Vec<u8> {
    let mut lines = input.lines();
    let a: usize = lines
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let b: usize = lines
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let c: usize = lines
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    lines.next();
    let prog: Vec<u8> = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cpu = CPU { a, b, c, pc: 0 };

    let mut p1 = Vec::new();

    while let Some(out) = cpu.next(&prog) {
        p1.push(out);
    }

    p1
}

#[derive(Debug, Clone, PartialEq)]
struct CPU {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
}

impl CPU {
    fn next(&mut self, prog: &[u8]) -> Option<u8> {
        while let Some(opcode) = prog.get(self.pc) {
            let literal = prog[self.pc + 1] as usize;
            let combo = match literal {
                0..=3 => literal,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unreachable!(),
            };

            match opcode {
                // adv - Div by pow 2 - shift
                0 => {
                    self.a >>= combo;
                }
                // bxl
                1 => {
                    self.b ^= literal;
                }
                // bst
                2 => {
                    self.b = combo % 8;
                }
                // jnz
                3 => {
                    if self.a != 0 {
                        self.pc = literal;
                        continue;
                    }
                }
                // bxc
                4 => {
                    self.b ^= self.c;
                }
                // out
                5 => {
                    self.pc += 2;
                    return Some((combo % 8) as u8);
                }
                // bdv
                6 => {
                    self.b = self.a >> combo;
                }
                // cdv
                7 => {
                    self.c = self.a >> combo;
                }
                _ => unreachable!(),
            }

            self.pc += 2;
        }

        None
    }
}

#[test]
fn day17() {
    let example1 = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    let p1 = solve_p1(&example1);
    assert_eq!(&p1[..], &[4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
}
