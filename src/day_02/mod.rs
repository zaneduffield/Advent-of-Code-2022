#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Weapon {
    Rock,
    Paper,
    Scissors,
}

impl Weapon {
    fn value(&self) -> i8 {
        match &self {
            Weapon::Rock => 0,
            Weapon::Paper => 1,
            Weapon::Scissors => 2,
        }
    }
}

impl From<i8> for Weapon {
    fn from(v: i8) -> Self {
        match v {
            0 => Weapon::Rock,
            1 => Weapon::Paper,
            2 => Weapon::Scissors,
            _ => panic!("invalid value {}", v),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Play {
    theirs: Weapon,
    ours: Weapon,
}

pub enum Outcome {
    Win,
    Loss,
    Draw,
}
pub type Input = Vec<Play>;

impl From<char> for Weapon {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Weapon::Rock,
            'B' | 'Y' => Weapon::Paper,
            'C' | 'Z' => Weapon::Scissors,
            c => panic!("Unexpected input: {}", c),
        }
    }
}

impl From<&str> for Play {
    fn from(s: &str) -> Self {
        Play {
            theirs: s.chars().next().expect("expected line to be longer").into(),
            ours: s.chars().nth(2).expect("expected line to be longer").into(),
        }
    }
}

impl Play {
    fn outcome(&self) -> Outcome {
        if self.ours == self.theirs {
            Outcome::Draw
        } else if (self.theirs.value() + 1) % 3 == self.ours.value() {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn score_outcome(outcome: Outcome) -> u32 {
        match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn score_ours(&self) -> u32 {
        match &self.ours {
            Weapon::Rock => 1,
            Weapon::Paper => 2,
            Weapon::Scissors => 3,
        }
    }

    fn score(&self) -> u32 {
        Play::score_outcome(self.outcome()) + self.score_ours()
    }

    fn score_p2(&self) -> u32 {
        let offset: i8 = match &self.ours {
            Weapon::Rock => -1,
            Weapon::Paper => 0,
            Weapon::Scissors => 1,
        };
        let our_play = Weapon::from((offset + self.theirs.value()).rem_euclid(3));
        Play {
            ours: our_play,
            theirs: self.theirs,
        }
        .score()
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.into()).collect()
}

#[aoc(day2, part1)]
pub fn part_1(input: &Input) -> u32 {
    input.iter().map(Play::score).sum()
}

#[aoc(day2, part2)]
pub fn part_2(input: &Input) -> u32 {
    input.iter().map(Play::score_p2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator("A Y\nB X\nC Z");
        assert_eq!(part_1(&input), 15);
        assert_eq!(part_2(&input), 12);
    }
}
