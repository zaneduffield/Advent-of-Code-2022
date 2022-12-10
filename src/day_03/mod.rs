use arrayvec::ArrayVec;
use itertools::Itertools;

pub type Rucksack = ArrayVec<u8, 48>;
pub type Input = Vec<Rucksack>;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AsciiCharset(u64);
impl<'a, T> From<T> for AsciiCharset
where
    T: Iterator<Item = &'a u8>,
{
    fn from(c: T) -> Self {
        let mut chars = 0;
        c.for_each(|b| chars |= 1 << (b - b'A'));
        Self(chars)
    }
}

impl AsciiCharset {
    fn intersect(&self, other: Self) -> Self {
        AsciiCharset(self.0 & other.0)
    }

    fn first(&self) -> Option<u8> {
        (b'A'..=b'z').find(|&b| self.0 & (1 << (b - b'A')) != 0)
    }
}

fn priority(b: u8) -> u32 {
    let priority = match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => panic!("unexpected byte {}", b),
    };
    priority as u32
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[aoc(day3, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .iter()
        .map(|r| {
            let left: AsciiCharset = r.into_iter().take(r.len() / 2).into();
            let right: AsciiCharset = r.into_iter().skip(r.len() / 2).into();
            let common = left
                .intersect(right)
                .first()
                .expect("at least one item should be in both compartments");
            priority(common)
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part_2(input: &Input) -> u32 {
    let mut sum = 0;
    for chunk in &input.iter().chunks(3) {
        sum += chunk
            .map(|r| AsciiCharset::from(r.into_iter()))
            .reduce(|acum, item| acum.intersect(item))
            .map(|c| priority(c.first().unwrap()))
            .unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator(
            "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(part_1(&input), 157);
        assert_eq!(part_2(&input), 70);
    }
}
