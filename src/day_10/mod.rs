pub enum Instruction {
    Noop,
    Addx(i8),
}

pub type Input = Vec<Instruction>;

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s.split_once(' ') {
            Some(("addx", x)) => {
                Instruction::Addx(x.parse().expect("couldn't parse operand as int"))
            }
            None if s == "noop" => Instruction::Noop,
            _ => panic!("couldn't parse '{s}' as an instruction"),
        }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(Instruction::from).collect()
}

pub struct Machine<'a, I: Iterator<Item = &'a Instruction>> {
    reg: i32,
    clock: i32,
    instructions: I,
    cur: Option<&'a Instruction>,
}

impl<'a, I: Iterator<Item = &'a Instruction>> Machine<'a, I> {
    fn new(instructions: I) -> Self {
        Self {
            reg: 1,
            clock: 1,
            instructions,
            cur: None,
        }
    }

    fn tick(&mut self) {
        if let Some(Instruction::Addx(x)) = self.cur {
            self.reg += *x as i32;
            self.cur = None;
        } else if let Some(i) = self.instructions.next() {
            match i {
                Instruction::Addx(_) => self.cur = Some(i),
                Instruction::Noop => {}
            }
        }
        self.clock += 1;
    }

    fn signal_strength(&self) -> i32 {
        self.reg * self.clock
    }
}

#[aoc(day10, part1)]
pub fn part_1(input: &Input) -> i32 {
    let mut machine = Machine::new(input.iter());
    let key_cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];

    key_cycles
        .iter()
        .map(|&key_cycle| {
            (machine.clock..key_cycle).for_each(|_| machine.tick());
            machine.signal_strength()
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part_2(input: &Input) -> String {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut machine = Machine::new(input.iter());
    let mut screen = String::with_capacity(1 + (WIDTH + 1) * HEIGHT);
    screen.push('\n');
    for _ in 0..HEIGHT {
        for col in 0i32..(WIDTH as i32) {
            screen.push(if (machine.reg - col).abs() < 2 {
                '#'
            } else {
                '.'
            });
            machine.tick();
        }
        screen.push('\n');
    }
    screen
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
                "
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
            "

        });
        // assert_eq!(part_1(&input), 13140);
        assert_eq!(
            part_2(&input),
            indoc! {
                "

                ##..##..##..##..##..##..##..##..##..##..
                ###...###...###...###...###...###...###.
                ####....####....####....####....####....
                #####.....#####.....#####.....#####.....
                ######......######......######......####
                #######.......#######.......#######.....
                "
            }
        );
    }
}
