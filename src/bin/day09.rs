use std::{cmp::Ordering, collections::VecDeque};

fn main() {
    let input = std::fs::read_to_string("inputs/day09.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (usize, usize) {
    let mut files: VecDeque<(usize, usize)> = VecDeque::new();
    let mut free: Vec<usize> = Vec::new();

    let mut files_total_len = 0;
    for (i, c) in input.as_bytes().iter().enumerate() {
        if !c.is_ascii_digit() {
            continue;
        }

        let n = c - b'0';
        let l = n as usize;

        if i % 2 == 0 {
            files.push_back((i / 2, l));
            files_total_len += l;
        } else {
            free.push(l);
        }
    }

    fn count(pos: usize, id: usize, n: usize) -> usize {
        (pos..pos + n).map(|i| i * id as usize).sum()
    }

    let mut p1 = 0;
    {
        let mut files = files.clone();
        let mut free = free.clone();
        free.reverse();

        let mut pos = 0;
        let mut free_slot = false;
        while pos < files_total_len {
            if free_slot {
                free_slot = false;

                let Some(slot) = free.pop() else {
                    break;
                };
                let Some((file_id, file)) = files.pop_back() else {
                    break;
                };

                let len = match file.cmp(&slot) {
                    Ordering::Less => {
                        free.push(slot - file);
                        free_slot = true;
                        file
                    }
                    Ordering::Equal => file,
                    Ordering::Greater => {
                        files.push_back((file_id, file - slot));
                        slot
                    }
                };

                p1 += count(pos, file_id, len);
                pos += len;
            } else {
                free_slot = true;
                let Some((file_id, file)) = files.pop_front() else {
                    break;
                };

                p1 += count(pos, file_id, file);
                pos += file;
            }
        }
    }

    let mut p2 = 0;
    {
        let mut files: Vec<(usize, Result<(usize, usize), usize>)> =
            files.into_iter().map(Ok).enumerate().collect();
        let mut filled: Vec<(Vec<(usize, usize)>, usize)> = free
            .into_iter()
            .map(|free_amount| (Vec::new(), free_amount))
            .collect();

        let mut files_it = files.iter_mut().rev();

        while let Some((ix, x)) = files_it.next() {
            let Ok((file_id, file)) = x else {
                continue;
            };

            if let Some((fill, free_amount)) = filled[0..*ix]
                .iter_mut()
                .find(|(_, free_amount)| *free_amount >= *file)
            {
                fill.push((*file_id, *file));
                *free_amount -= *file;
                *x = Err(*file);
            }
        }

        let mut pos = 0;
        let mut files_it = files.iter();
        let filled_it = filled.iter();

        // Handle first file by itself
        let (file_id, file) = files_it.next().unwrap().1.unwrap();
        p2 += count(pos, file_id, file);
        pos += file;

        for ((fill, free_amount), (_, r)) in filled_it.zip(files_it) {
            for (file_id, file) in fill.iter() {
                p2 += count(pos, *file_id, *file);
                pos += file;
            }

            pos += free_amount;

            match r {
                Ok((file_id, file)) => {
                    p2 += count(pos, *file_id, *file);
                    pos += file;
                }
                Err(free_amount) => {
                    pos += free_amount;
                }
            }
        }
    }

    (p1, p2)
}

#[test]
fn day09() {
    let example1 = "2333133121414131402\n";

    let (p1, p2) = solve(&example1);
    assert_eq!(p1, 1928);
    assert_eq!(p2, 2858);
}
