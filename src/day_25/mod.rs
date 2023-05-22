const BASE: i64 = 5;
const DIGITS: [char; BASE as usize] = ['=', '-', '0', '1', '2'];
const DIGIT_ROTATION: i64 = BASE / 2;

fn parse_snafu(num: &str) -> i64 {
    num.chars()
        .map(parse_digit)
        .fold(0, |sum, digit| sum * BASE + digit)
}

fn parse_digit(c: char) -> i64 {
    DIGITS
        .iter()
        .position(|d| *d == c)
        .unwrap_or_else(|| panic!("unexpected char: {c}")) as i64
        - DIGIT_ROTATION
}

fn format_digit(d: i64) -> char {
    *DIGITS
        .get((d + DIGIT_ROTATION) as usize)
        .unwrap_or_else(|| panic!("unexpected digit: {d}"))
}

fn format_snafu(num: i64) -> String {
    // first convert to the new base
    let mut digits = vec![];
    let mut remainder = num;
    loop {
        digits.push(remainder % BASE);
        remainder /= BASE;
        if remainder == 0 {
            break;
        }
    }

    // then carry and wrap the digits
    let mut i = 0;
    let mut carry = 0;
    while carry != 0 || i < digits.len() {
        match digits.get_mut(i) {
            None => digits.push(carry),
            Some(v) => {
                let dig = *v + carry;
                let wrapped = (dig > DIGIT_ROTATION) as i64;
                (*v, carry) = (dig - BASE * wrapped, wrapped);
            }
        }
        i += 1;
    }

    digits.into_iter().rev().map(format_digit).collect()
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
