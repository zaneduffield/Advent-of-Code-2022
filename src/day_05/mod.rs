use arrayvec::ArrayVec;
use itertools::Itertools;

pub type Stack = ArrayVec<u8, 64>;
pub type Stacks = Vec<Stack>;

#[derive(Clone, Copy)]
pub struct Instruction {
    n: u8,
    from: u8,
    to: u8,
}

#[derive(Clone)]
pub struct Input {
    stacks: Stacks,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut stacks = Stacks::new();
    input
        .lines()
        .take_while(|line| !line.starts_with(" 1"))
        .for_each(|line| {
            for (i, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let c = chunk.into_iter().nth(1).unwrap() as u8;
                if c == b' ' {
                    continue;
                }
                let stack: &mut Stack = match stacks.get_mut(i) {
                    Some(stack) => stack,
                    None => {
                        stacks.extend((1..=(i + 1 - stacks.len())).map(|_| Stack::new()));
                        &mut stacks[i]
                    }
                };
                stack.push(c);
            }
        });
    stacks.iter_mut().for_each(|s| s.reverse());

    let instructions = input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|line| {
            let (n, from, to) = line
                .split_ascii_whitespace()
                .filter_map(|part| part.parse::<u8>().ok())
                .collect_tuple()
                .expect("Couldn't parse instruction");
            Instruction {
                n,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect();

    Input {
        stacks,
        instructions,
    }
}

fn read_stacks(stacks: &Stacks) -> String {
    stacks
        .iter()
        .filter_map(|s| s.last())
        .map(|c| *c as char)
        .collect()
}

#[aoc(day5, part1)]
pub fn part_1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for Instruction { n, from, to } in &input.instructions {
        (0..*n).for_each(|_| {
            let from_stack = stacks
                .get_mut(*from as usize)
                .expect("from stack not found");
            let elm = from_stack.pop().expect("stack is empty!");
            let to_stack = stacks.get_mut(*to as usize).expect("to stack not found");
            to_stack.push(elm);
        });
    }
    read_stacks(&stacks)
}

#[aoc(day5, part2)]
pub fn part_2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for Instruction { n, from, to } in &input.instructions {
        let from_stack = stacks
            .get_mut(*from as usize)
            .expect("from stack not found");
        let elms: ArrayVec<_, 32> = from_stack
            .drain((from_stack.len() - *n as usize)..(from_stack.len()))
            .collect();
        let to_stack = stacks.get_mut(*to as usize).expect("to stack not found");
        to_stack.extend(elms);
    }
    read_stacks(&stacks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {"
                    [D]    
                [N] [C]    
                [Z] [M] [P]
                 1   2   3 

                move 1 from 2 to 1
                move 3 from 1 to 3
                move 2 from 2 to 1
                move 1 from 1 to 2
            "});
        assert_eq!(part_1(&input), "CMZ");
        assert_eq!(part_2(&input), "MCD");
    }
}
