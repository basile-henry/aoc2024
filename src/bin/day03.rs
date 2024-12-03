fn main() {
    let input = std::fs::read_to_string("inputs/day03.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

// returns (num_digits, value)
fn parse_1_to_3_digit_num(input: &[u8]) -> Option<(usize, usize)> {
    let mut num_digits = 0;
    let mut value = 0;

    while num_digits < 3 && num_digits < input.len() {
        if input[num_digits].is_ascii_digit() {
            value = value * 10 + (input[num_digits] - b'0') as usize;
            num_digits += 1;
        } else {
            break;
        }
    }

    if num_digits > 0 {
        return Some((num_digits, value));
    }

    None
}

fn solve(input: &str) -> (usize, usize) {
    let input = input.as_bytes(); // simpler parsing

    let mut enabled = true;
    let mut p1 = 0;
    let mut p2 = 0;

    let mut offset = 0;
    while offset < input.len() - 8 {
        if &input[offset..offset + 4] == b"mul(" {
            offset += 4;
            let Some((n, x)) = parse_1_to_3_digit_num(&input[offset..]) else {
                continue;
            };
            offset += n;

            if input[offset] != b',' {
                continue;
            }
            offset += 1;

            let Some((n, y)) = parse_1_to_3_digit_num(&input[offset..]) else {
                continue;
            };
            offset += n;

            if input[offset] != b')' {
                continue;
            }
            offset += 1;

            p1 += x * y;

            if enabled {
                p2 += x * y;
            }
        } else if &input[offset..offset + 4] == b"do()" {
            offset += 4;
            enabled = true;
        } else if &input[offset..offset + 7] == b"don't()" {
            offset += 7;
            enabled = false;
        } else {
            offset += 1;
        }
    }

    (p1, p2)
}

#[test]
fn day03() {
    let example1 = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 161);

    let example2 = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let (_p1, p2) = solve(&example2);
    assert_eq!(p2, 48);
}
