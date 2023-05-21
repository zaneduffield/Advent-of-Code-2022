use std::{cmp::Reverse, collections::BinaryHeap};

const BASE: i64 = 5;
const POWS: [i64; 23] = [
    1,
    BASE.pow(1),
    BASE.pow(2),
    BASE.pow(3),
    BASE.pow(4),
    BASE.pow(5),
    BASE.pow(6),
    BASE.pow(7),
    BASE.pow(8),
    BASE.pow(9),
    BASE.pow(10),
    BASE.pow(11),
    BASE.pow(12),
    BASE.pow(13),
    BASE.pow(14),
    BASE.pow(15),
    BASE.pow(16),
    BASE.pow(17),
    BASE.pow(18),
    BASE.pow(19),
    BASE.pow(20),
    BASE.pow(21),
    BASE.pow(22),
];

fn parse_snafu(num: &str) -> i64 {
    num.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            POWS[i]
                * match c {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    c => panic!("unexpected char: {c}"),
                }
        })
        .sum()
}

fn format_digit(d: i64) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("unexpected digit {d}"),
    }
}

fn search_snafu(target: i64) -> Option<String> {
    let mut queue = BinaryHeap::default();
    queue.push(Reverse((target, POWS.len(), target, String::new())));

    while let Some(Reverse((abs_diff, last_pow, remaining_target, mut s))) = queue.pop() {
        if abs_diff == 0 {
            s.extend(std::iter::repeat(format_digit(0)).take(last_pow));
            return Some(s);
        } else if last_pow == 0 {
            continue;
        }

        let new_pow = last_pow - 1;
        for digit in -2..=2 {
            let new_target = remaining_target - (digit * POWS[new_pow]);
            let mut s2 = s.to_owned();
            s2.push(format_digit(digit));
            queue.push(Reverse((new_target.abs(), new_pow, new_target, s2)));
        }
    }

    None
}

fn format_snafu(num: i64) -> String {
    search_snafu(num)
        .unwrap()
        .trim_start_matches('0')
        .to_owned()
}

#[aoc(day25, part1)]
pub fn part_1(input: &str) -> String {
    format_snafu(input.lines().map(parse_snafu).sum())
}

#[aoc(day25, part2)]
pub fn part_2(_input: &str) -> String {
    "DONE!".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {
            "
            1=-0-2
            12111
            2=0=
            21
            2=01
            111
            20012
            112
            1=-1=
            1-12
            12
            1=
            122
            "
        };
        assert_eq!(part_1(input), "2=-1=0");
    }
}
