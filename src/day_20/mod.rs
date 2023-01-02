use std::cmp::Ordering;

use btreelist::BTreeList;

pub type NumVal = i16;
pub type NumId = usize;

#[derive(Copy, Clone)]
struct Num(NumId, NumVal);

pub type Input = Vec<Num>;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| Num(id, line.parse().unwrap()))
        .collect()
}

struct File {
    data: BTreeList<Num>,
}

impl File {
    fn new(input: &Input) -> Self {
        Self {
            data: BTreeList::from_iter(input.iter().copied()),
        }
    }

    fn binary_search(&self, id: NumId) -> Option<usize> {
        let (mut lo, mut hi) = (0, self.data.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.data[mid].0.cmp(&id) {
                Ordering::Less => lo = mid,
                Ordering::Greater => hi = mid,
                Ordering::Equal => return Some(mid),
            }
        }
        None
    }

    fn mix(&mut self) {
        for id in 0..self.data.len() {
            let idx = self.binary_search(id).unwrap();
            let Num(_, didx) = self.data[idx];
            if didx > 0 {
                let new_idx = idx + didx as usize;
            } else {
                let new_idx = idx.checked_sub(didx)...;
            }
        }
    }
}

#[aoc(day20, part1)]
pub fn part_1(input: &Input) -> u32 {}

#[aoc(day20, part2)]
pub fn part_2(input: &Input) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            1
            2
            -3
            3
            -2
            0
            4
            "
        });
        assert_eq!(part_1(&input),);
        assert_eq!(part_2(&input),);
    }
}
