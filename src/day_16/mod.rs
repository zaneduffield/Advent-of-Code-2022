// not sure yet but this feels like A* again
// where the admisible heuristic is the current flow rate multiplied
// by the remaining time, and the 'cost' is negative released presure.
// The state would be the position and all of the values that are open,
// and the 'neighbours' would be the states that can be reached in a single move.
// ... yeah that should work quite well, there was a puzzle last year near
// the end that was very similar - start there.

pub struct Input {

}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {

}

#[aoc(day16, part1)]
pub fn part_1(input: &Input) -> u32 {

}

#[aoc(day16, part2)]
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
