use std::iter::repeat_n;

use common::prelude::*;

use chumsky::prelude::*;
use itertools::Itertools;

register_solver!(2024, 9, Solver);
pub struct Solver;

impl DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let mut disk = input
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(i, len)| {
                let is_file = i % 2 == 0;
                let len = len as usize;
                if is_file {
                    let file_id = i / 2;
                    repeat_n(DiskSector::File(file_id), len)
                } else {
                    repeat_n(DiskSector::Empty, len)
                }
            })
            .collect_vec();

        // Fragment
        let mut write = 0;
        let mut read = disk.len() - 1;
        'l: loop {
            while matches!(disk[write], DiskSector::File(_)) {
                write += 1;
                if write >= read {
                    break 'l;
                }
            }
            disk.swap(write, read);

            while !matches!(disk[read], DiskSector::File(_)) {
                read -= 1;
            }
        }

        // Reorder
        let res = disk
            .into_iter()
            .enumerate()
            .filter_map(|(i, s)| match s {
                DiskSector::Empty => None,
                DiskSector::File(id) => Some(i * id),
            })
            .sum::<usize>();

        PartResult::new(res)
    }

    fn solve_2(&self, input: &str) -> PartResult {
        let input = parser().parse(input).unwrap();

        let mut disk = input
            .iter()
            .copied()
            .enumerate()
            .map(|(i, len)| {
                let is_file = i % 2 == 0;
                let len = len as usize;
                if is_file {
                    let file_id = i / 2;
                    (len, DiskSector::File(file_id))
                } else {
                    (len, DiskSector::Empty)
                }
            })
            .collect_vec();

        // Reorder
        for read in (0..disk.len()).rev() {
            let (len, DiskSector::File(_)) = disk[read] else {
                continue;
            };

            let Some((available_slot, space)) = disk
                .iter()
                .enumerate()
                .take_while(|(i, _)| *i < read)
                .find_map(|(i, (space, s))| match s {
                    DiskSector::Empty if (len <= *space) => Some((i, *space)),
                    _ => None,
                })
            else {
                continue;
            };

            let merge_if_needed = |disk: &mut Vec<(usize, DiskSector)>, a: usize, b: usize| {
                if let (Some((len_a, DiskSector::Empty)), Some((len_b, DiskSector::Empty))) =
                    (disk.get(a), disk.get(b))
                {
                    disk[a].0 = len_a + len_b;
                    disk.remove(b);
                }
            };

            disk[available_slot] = disk[read];
            disk[read] = (len, DiskSector::Empty);
            merge_if_needed(&mut disk, read, read + 1);
            if space > len {
                disk.insert(available_slot + 1, (space - len, DiskSector::Empty));
                merge_if_needed(&mut disk, available_slot + 1, available_slot + 2);
            }
        }

        // Checksum
        let mut index = 0;
        let res = disk
            .into_iter()
            .map(move |(len, s)| {
                let i = index;
                index += len;
                (i, (len, s))
            })
            .filter_map(|(i, (len, s))| match s {
                DiskSector::Empty => None,
                DiskSector::File(id) => Some((i..i + len).map(|i| i * id).sum::<usize>()),
            })
            .sum::<usize>();

        PartResult::new(res)
    }
}

#[derive(Debug, Clone, Copy)]
enum DiskSector {
    Empty,
    File(usize),
}

fn parser() -> impl Parser<char, Vec<u8>, Error = Simple<char>> {
    let digit = any().map(|c: char| c.to_digit(10).unwrap() as u8);

    digit.repeated()
}
