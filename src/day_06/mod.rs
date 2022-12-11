use std::iter;

use arrayvec::ArrayVec;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LowerAsciiCharset(u32);
impl<'a, T> From<T> for LowerAsciiCharset
where
    T: Iterator<Item = &'a u8>,
{
    fn from(c: T) -> Self {
        let mut chars = 0;
        c.for_each(|b| chars |= 1 << (b - b'a'));
        Self(chars)
    }
}

impl LowerAsciiCharset {
    fn bit(b: u8) -> u32 {
        1 << (b - b'a')
    }

    fn add(&mut self, b: u8) {
        self.0 |= Self::bit(b);
    }

    fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

fn start_marker_pos<const BUF_LEN: usize>(input: &str) -> usize {
    let mut elms: ArrayVec<u8, BUF_LEN> = input.bytes().take(BUF_LEN).collect();

    let mut oldest_idx = 0;
    input
        .as_bytes()
        .iter()
        .skip(BUF_LEN)
        .position(|&b| {
            let count = LowerAsciiCharset::from(elms.iter().chain(iter::once(&b))).count();
            if count > BUF_LEN as u32 {
                true
            } else {
                *elms.get_mut(oldest_idx).unwrap() = b;
                oldest_idx = (oldest_idx + 1) % elms.len();
                false
            }
        })
        .expect("marker not found")
        + BUF_LEN
        + 1
}

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> usize {
    start_marker_pos::<3>(input)
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> usize {
    start_marker_pos::<13>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_1(input), 7);
        assert_eq!(part_2(input), 19);
    }
}
