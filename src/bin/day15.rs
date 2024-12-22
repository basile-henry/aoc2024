fn main() {
    let input = std::fs::read_to_string("inputs/day15.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let mut robot1 = None;
    let mut grid1: Vec<Vec<u8>> = Vec::new();

    let mut robot2 = None;
    let mut grid2: Vec<Vec<u8>> = Vec::new();

    for (y, line) in grid.split_whitespace().enumerate() {
        let mut line1 = Vec::new();
        let mut line2 = Vec::new();
        for (x, c) in line.as_bytes().iter().enumerate() {
            if *c == b'@' {
                line1.push(b'.');
                robot1 = Some((x as i8, y as i8));
                robot2 = Some((2 * x as i8, y as i8));
            } else {
                line1.push(*c);
            }

            line2.extend_from_slice(match *c {
                b'.' => &[b'.', b'.'],
                b'#' => &[b'#', b'#'],
                b'O' => &[b'[', b']'],
                b'@' => &[b'.', b'.'],
                _ => unreachable!(),
            });
        }

        grid1.push(line1);
        grid2.push(line2);
    }

    let mut robot1 = robot1.unwrap();
    let mut robot2 = robot2.unwrap();

    fn index(grid: &mut [Vec<u8>], (x, y): (i8, i8)) -> &mut u8 {
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        &mut grid[y][x]
    }

    for m in moves.as_bytes().iter().copied() {
        let (dx, dy) = match m {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, -1),
            b'v' => (0, 1),
            _ => continue,
        };

        // part 1
        {
            let next = (robot1.0 + dx, robot1.1 + dy);

            match *index(&mut grid1, next) {
                b'.' => robot1 = next,
                b'#' => {}
                b'O' => {
                    let mut pos = (next.0 + dx, next.1 + dy);
                    while *index(&mut grid1, pos) == b'O' {
                        pos = (pos.0 + dx, pos.1 + dy);
                    }

                    if *index(&mut grid1, pos) != b'#' {
                        robot1 = next;
                        *index(&mut grid1, next) = b'.';
                        *index(&mut grid1, pos) = b'O';
                    }
                }
                _ => unreachable!(),
            }
        }

        // part 2
        {
            let next = (robot2.0 + dx, robot2.1 + dy);

            match *index(&mut grid2, next) {
                b'.' => robot2 = next,
                b'#' => {}
                b'[' | b']' => {
                    let mut pos = next;
                    let mut moving_boxes = Vec::new(); // only [
                    let mut apply_move = true;

                    match m {
                        b'<' | b'>' => loop {
                            let c = *index(&mut grid2, pos);

                            match c {
                                b'[' => moving_boxes.push(pos),
                                b']' => {}
                                b'#' => {
                                    apply_move = false;
                                    break;
                                }
                                b'.' => {
                                    break;
                                }
                                _ => unreachable!(),
                            }

                            pos = (pos.0 + dx, pos.1 + dy);
                        },
                        b'^' | b'v' => {
                            let mut to_check = vec![pos];

                            while let Some(pos) = to_check.pop() {
                                let c = *index(&mut grid2, pos);

                                match c {
                                    b'[' => {
                                        moving_boxes.push(pos);
                                        to_check.push((pos.0, pos.1 + dy));
                                        to_check.push((pos.0 + 1, pos.1 + dy));
                                    }
                                    b']' => {
                                        moving_boxes.push((pos.0 - 1, pos.1));
                                        to_check.push((pos.0 - 1, pos.1 + dy));
                                        to_check.push((pos.0, pos.1 + dy));
                                    }
                                    b'#' => {
                                        apply_move = false;
                                        break;
                                    }
                                    b'.' => {}
                                    _ => unreachable!(),
                                }
                            }
                        }
                        _ => unreachable!(),
                    }

                    if apply_move {
                        robot2 = next;

                        for (x, y) in moving_boxes.iter().copied() {
                            *index(&mut grid2, (x, y)) = b'.';
                            *index(&mut grid2, (x + 1, y)) = b'.';
                        }
                        for (x, y) in moving_boxes.into_iter() {
                            *index(&mut grid2, (x + dx, y + dy)) = b'[';
                            *index(&mut grid2, (x + dx + 1, y + dy)) = b']';
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let p1 = grid1
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| if *c == b'O' { 100 * y + x } else { 0 })
        })
        .sum();
    let p2 = grid2
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| if *c == b'[' { 100 * y + x } else { 0 })
        })
        .sum();
    (p1, p2)
}

#[test]
fn day15() {
    let example1 = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    let (p1, _p2) = solve(&example1);
    assert_eq!(p1, 2028);

    let example2 = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    let (p1, p2) = solve(&example2);
    assert_eq!(p1, 10092);
    assert_eq!(p2, 9021);
}
