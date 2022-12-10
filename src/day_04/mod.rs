use std::ops::RangeInclusive;

use itertools::Itertools;

pub struct Assignment {
    range: RangeInclusive<u8>,
}
pub struct Pair {
    left: Assignment,
    right: Assignment,
}
pub type Input = Vec<Pair>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split(',')
                .map(|p| {
                    let (min, max) = p
                        .split('-')
                        .map(|n| n.parse::<u8>().expect("couldn't parse as u8"))
                        .collect_tuple()
                        .expect("two '-' separated ints are expected");
                    min..=max
                })
                .collect_tuple()
                .expect("two comma-separated values are expected");
            Pair {
                left: Assignment { range: left },
                right: Assignment { range: right },
            }
        })
        .collect()
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.range.start() <= other.range.start() && self.range.end() >= other.range.end()
    }

    fn intersects(&self, other: &Self) -> bool {
        !(self.range.start() > other.range.end() || self.range.end() < other.range.start())
    }
}

#[aoc(day4, part1)]
pub fn part_1(input: &Input) -> usize {
    input
        .iter()
        .filter(|p| p.left.contains(&p.right) || p.right.contains(&p.left))
        .count()
}

#[aoc(day4, part2)]
pub fn part_2(input: &Input) -> usize {
    input.iter().filter(|p| p.left.intersects(&p.right)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator(
            "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );
        assert_eq!(part_1(&input), 2);
        assert_eq!(part_2(&input), 4);
    }
}
