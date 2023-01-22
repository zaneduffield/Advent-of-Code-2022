use std::ops::Index;
use std::process::id;

use itertools::iterate;
use itertools::Itertools;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_till1;
use nom::bytes::complete::take_while;
use nom::character::complete::alpha1;
use nom::character::complete::{char, i64, line_ending, *};
use nom::character::is_newline;
use nom::character::is_space;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use rustc_hash::FxHashMap;

pub type Id = u16;
pub type Val = i16;
pub type LargeVal = i64;

#[derive(Copy, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn eval(&self, left: LargeVal, right: LargeVal) -> LargeVal {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Job {
    Number(Val),
    Computation(Id, Operation, Id),
}

#[derive(Clone)]
pub struct Monkey {
    id: Id,
    job: Job,
    result: Option<LargeVal>,
}

#[derive(Clone)]
pub struct Input {
    monkeys: Vec<Monkey>,
    root_id: Id,
}

struct MonkeyParser<'a, 'b> {
    next_id: Id,
    id_map: &'b mut FxHashMap<&'a str, Id>,
}

impl<'a, 'b> MonkeyParser<'a, 'b> {
    fn new(id_map: &'b mut FxHashMap<&'a str, Id>) -> Self {
        Self { next_id: 0, id_map }
    }

    fn get_id(&mut self, name: &'a str) -> Id {
        *self.id_map.entry(name).or_insert_with(|| {
            let out = self.next_id;
            self.next_id += 1;
            out
        })
    }

    fn parse_job(&mut self, input: &'a str) -> nom::IResult<&'a str, Job> {
        let (input, num) = opt(delimited(space0, i16, space0))(input)?;
        if let Some(n) = num {
            return Ok((input, Job::Number(n)));
        }

        let (input, word1) = delimited(space0, alpha1, space0)(input)?;
        let (input, (op, _, word2)) = tuple((one_of("+/-*"), space0, alpha1))(input)?;
        let id1 = self.get_id(word1);
        let id2 = self.get_id(word2);
        let op = match op {
            '*' => Operation::Mul,
            '/' => Operation::Div,
            '+' => Operation::Add,
            '-' => Operation::Sub,
            _ => unreachable!(),
        };

        Ok((input, Job::Computation(id1, op, id2)))
    }
}

impl<'a, 'b> nom::Parser<&'a str, Monkey, nom::error::Error<&'a str>> for MonkeyParser<'a, 'b> {
    fn parse(
        &mut self,
        input: &'a str,
    ) -> nom::IResult<&'a str, Monkey, nom::error::Error<&'a str>> {
        let (input, (name, _, _, rest)) =
            tuple((alpha1, char(':'), space0, not_line_ending))(input)?;
        let (_, job) = self.parse_job(rest)?;
        let id = self.get_id(name);

        Ok((
            input,
            Monkey {
                id,
                job,
                result: None,
            },
        ))
    }
}

#[aoc_generator(day21)]
pub fn input_generator(data: &str) -> Input {
    let mut id_map = FxHashMap::default();

    let (unparsed_input, mut monkeys) =
        separated_list1(line_ending, MonkeyParser::new(&mut id_map))(data).unwrap();
    assert_eq!(unparsed_input.trim(), "");
    let root_id = *id_map
        .get("root")
        .expect("couldn't find monkey with name 'root'");

    monkeys.sort_by_key(|m| m.id);
    Input { monkeys, root_id }
}

impl Input {
    fn eval(&mut self, id: Id) -> LargeVal {
        let m = &self.monkeys[id as usize];
        if let Some(res) = m.result {
            return res;
        }

        let res = match m.job {
            Job::Number(n) => n as LargeVal,
            Job::Computation(left_id, op, right_id) => {
                let left = self.eval(left_id);
                let right = self.eval(right_id);
                op.eval(left, right)
            }
        };
        self.monkeys[id as usize].result = Some(res);
        res
    }
}

#[aoc(day21, part1)]
pub fn part_1(input: &Input) -> LargeVal {
    let mut input = input.clone();
    input.eval(input.root_id)
}

#[aoc(day21, part2)]
pub fn part_2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            root: pppw + sjmn
            dbpl: 5
            cczh: sllz + lgvd
            zczc: 2
            ptdq: humn - dvpt
            dvpt: 3
            lfqf: 4
            humn: 5
            ljgn: 2
            sjmn: drzm * dbpl
            sllz: 4
            pppw: cczh / lfqf
            lgvd: ljgn * ptdq
            drzm: hmdt - zczc
            hmdt: 32
            "
        });
        assert_eq!(part_1(&input), 152);
        // assert_eq!(part_2(&input),);
    }
}
