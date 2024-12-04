fn main() {
    let input = std::fs::read_to_string("inputs/day04.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let grid: Vec<&[u8]> = input.split_whitespace().map(|x| x.as_bytes()).collect();
    let width = grid[0].len();

    let mut p1 = 0;
    let mut p2 = 0;

    fn check_p1(word: &[u8]) -> bool {
        word == b"XMAS" || word == b"SAMX"
    }

    fn check_p2(word1: &[u8], word2: &[u8]) -> bool {
        (word1 == b"MAS" || word1 == b"SAM") && (word2 == b"MAS" || word2 == b"SAM")
    }

    // horizontal
    for line in grid.iter() {
        for window in line.windows(4) {
            if check_p1(window) {
                p1 += 1;
            }
        }
    }

    fn get_window<const N: usize>(sub_grid: &[&[u8]], x: usize, dir: isize) -> [u8; N] {
        // using a separate function because "try" isn't stable yet
        fn index(sub_grid: &[&[u8]], x: usize, y: usize, dir: isize) -> Option<u8> {
            let i = x as isize + dir * y as isize;
            let i: usize = i.try_into().ok()?;
            sub_grid[y].get(i).copied()
        }

        std::array::from_fn(|y| index(sub_grid, x, y, dir).unwrap_or(b'.'))
    }

    // vertical and diagonals
    for rows in grid.windows(4) {
        for x in 0..width {
            for dir in [-1, 0, 1] {
                let window: [u8; 4] = get_window(rows, x + if dir == -1 { 3 } else { 0 }, dir);

                if check_p1(&window) {
                    p1 += 1;
                }
            }
        }
    }

    for rows in grid.windows(3) {
        for x in 0..width {
            let word1: [u8; 3] = get_window(rows, x + 2, -1);
            let word2: [u8; 3] = get_window(rows, x, 1);

            if check_p2(&word1, &word2) {
                p2 += 1;
            }
        }
    }

    (p1, p2)
}

#[test]
fn day04() {
    let example1 = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 18);
    assert_eq!(p2, 9);
}
