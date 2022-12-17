pub struct Input {

}

#[aoc_generator(dayxx)]
pub fn input_generator(input: &str) -> Input {

}

#[aoc(dayxx, part1)]
pub fn part_1(input: &Input) -> u32 {

}

#[aoc(dayxx, part2)]
pub fn part_2(input: &Input) -> u32 {

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            "
        });
        assert_eq!(part_1(&input),);
        assert_eq!(part_2(&input),);
    }
}
