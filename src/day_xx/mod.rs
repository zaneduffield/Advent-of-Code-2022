pub type Input = ...;

#[aoc_generator(dayxx)]
pub fn input_generator(input: &str) -> Input {
}

#[aoc(dayxx, part1)]
pub fn part_1(input: &Input) ->  {
}

#[aoc(dayxx, part2)]
pub fn part_2(input: &Input) ->  {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator("");
        assert_eq!(part_1(&input), );
        assert_eq!(part_2(&input), );
    }
}
