use arrayvec::ArrayVec;
use lazysort::*;

pub type Input = Vec<ArrayVec<i32, 16>>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|elf_food| {
            elf_food
                .lines()
                .map(|line| line.parse().expect("Could not parse calories as int"))
                .collect()
        })
        .collect()
}

fn sum_inner(input: &Input) -> impl Iterator<Item = i32> + '_ {
    input.iter().map(|elf| elf.iter().sum())
}

#[aoc(day1, part1)]
pub fn part_1(input: &Input) -> i32 {
    sum_inner(input).max().expect("No elf data was found.")
}

#[aoc(day1, part2)]
pub fn part_2(input: &Input) -> i32 {
    sum_inner(input).sorted_by(|a, b| b.cmp(a)).take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            input_generator("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(part_1(&input), 24000);
        assert_eq!(part_2(&input), 45000);
    }
}
