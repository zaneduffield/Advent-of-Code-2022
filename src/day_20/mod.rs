use std::cmp::Ordering;

use btreelist::BTreeList;

pub type NumVal = i64;
pub type NumId = usize;

#[derive(Copy, Clone, Debug)]
pub struct Num(NumId, NumVal);

pub type Input = Vec<Num>;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| Num(id, line.parse().unwrap()))
        .collect()
}

const DECRYPTION_KEY: NumVal = 811589153;

struct File {
    data: BTreeList<Num>,
}

impl File {
    fn new(input: &Input) -> Self {
        Self {
            data: BTreeList::from_iter(input.iter().copied()),
        }
    }

    fn find(&self, id: NumId) -> Option<usize> {
        self.data.iter().position(|n| n.0 == id)

        // let (mut lo, mut hi) = (0, self.data.len() - 1);
        // while lo <= hi {
        //     let mid = (hi + lo) / 2;
        //     match self.data[mid].0.cmp(&id) {
        //         Ordering::Less => lo = mid + 1,
        //         Ordering::Greater => hi = mid - 1,
        //         Ordering::Equal => return Some(mid),
        //     }
        // }
        // None
    }

    fn mix(&mut self) {
        for id in 0..self.data.len() {
            let idx = self.find(id).unwrap();
            let elm = self.data[idx];
            self.data.remove(idx);
            let new_idx =
                (idx as isize + elm.1 as isize).rem_euclid(self.data.len() as isize) as usize;
            self.data.insert(new_idx, elm).unwrap();
        }
    }

    fn groove_coordinates(&self) -> [Num; 3] {
        let zero_idx = self
            .data
            .iter()
            .position(|Num(_, val)| val == &0)
            .expect("couldn't find zero");
        [1000, 2000, 3000].map(|idx: usize| {
            let idx = (idx + zero_idx).rem_euclid(self.data.len());
            self.data[idx]
        })
    }
}

#[aoc(day20, part1)]
pub fn part_1(input: &Input) -> NumVal {
    let mut file = File::new(input);
    file.mix();
    file.groove_coordinates().iter().map(|n| n.1).sum()
}

#[aoc(day20, part2)]
pub fn part_2(input: &Input) -> i64 {
    let mut file = File::new(
        &input
            .iter()
            .copied()
            .map(|mut n| {
                n.1 *= DECRYPTION_KEY;
                n
            })
            .collect(),
    );

    (0..10).for_each(|_| file.mix());
    file.groove_coordinates().iter().map(|n| n.1).sum()
}

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
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 1623178306);
    }
}
