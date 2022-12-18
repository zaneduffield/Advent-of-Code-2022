use std::{cmp::Reverse, str::FromStr};

use itertools::Itertools;
use num::integer::lcm;

pub type Item = u64;

#[derive(Clone, Copy)]
pub enum UnaryOperation {
    Add(Item),
    Sub(Item),
    Mul(Item),
    Square,
}
impl UnaryOperation {
    fn apply(&self, other: Item) -> Item {
        match self {
            UnaryOperation::Add(x) => other + x,
            UnaryOperation::Sub(x) => other - x,
            UnaryOperation::Mul(x) => other * x,
            UnaryOperation::Square => other * other,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Divisor(Item);

#[derive(Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<Item>,
    inspection_count: u64,
    operation: UnaryOperation,
    divisor: Divisor,
    true_monk: usize,
    false_monk: usize,
}

#[derive(Clone)]
pub struct Input {
    monkeys: Vec<Monkey>,
    lcm: Item,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseUnaryOperationError;

impl FromStr for UnaryOperation {
    type Err = ParseUnaryOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(("old", op, x)) = s.split_ascii_whitespace().collect_tuple() {
            if ("*", "old") == (op, x) {
                Ok(UnaryOperation::Square)
            } else {
                let operand = x.parse().map_err(|_| ParseUnaryOperationError)?;
                match op {
                    "*" => Ok(UnaryOperation::Mul(operand)),
                    "+" => Ok(UnaryOperation::Add(operand)),
                    "-" => Ok(UnaryOperation::Sub(operand)),
                    _ => Err(ParseUnaryOperationError),
                }
            }
        } else {
            Err(ParseUnaryOperationError)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseUnaryPredicateError;

impl FromStr for Divisor {
    type Err = ParseUnaryPredicateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_ascii_whitespace().collect_tuple() {
            Some(("divisible", "by", x)) => {
                Ok(Self(x.parse().expect("couldn't parse test operand as int")))
            }
            _ => Err(ParseUnaryPredicateError),
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            let id = lines
                .next()
                .unwrap()
                .trim_end_matches(':')
                .split_once(' ')
                .unwrap()
                .1
                .parse()
                .expect("couldn't parse monkey id as int");

            let items = lines
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split(',')
                .map(|item| item.trim().parse().expect("couldn't parse item as int"))
                .collect();

            let operation = lines
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .trim()
                .parse()
                .unwrap();

            let divisor = lines
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .parse()
                .unwrap();

            let true_monk = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let false_monk = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                id,
                items,
                inspection_count: 0,
                operation,
                divisor,
                true_monk,
                false_monk,
            }
        })
        // let's not assume the input is in sorted order already
        .sorted_by_key(|m| m.id)
        .collect();

    let lcm = monkeys
        .iter()
        .map(|m| m.divisor)
        .fold(1, |x, y| lcm(x, y.0));
    Input { monkeys, lcm }
}

impl Input {
    fn round<F>(&mut self, worry_fn: F)
    where
        F: Fn(Item) -> Item,
    {
        for idx in 0..self.monkeys.len() {
            for item_idx in 0..self.monkeys[idx].items.len() {
                let (item, divisor, true_m, false_m) = {
                    let m = &mut self.monkeys[idx];
                    m.inspection_count += 1;
                    let mut item = m.items[item_idx];
                    item = worry_fn(m.operation.apply(item)) % self.lcm;
                    (item, m.divisor, m.true_monk, m.false_monk)
                };

                let divides = item % divisor.0 == 0;
                self.monkeys[if divides { true_m } else { false_m }]
                    .items
                    .push(item);
            }
            self.monkeys[idx].items.clear();
        }
    }

    fn monkey_business(&self) -> u64 {
        self.monkeys
            .iter()
            .map(|m| m.inspection_count)
            .sorted_by_key(|x| Reverse(*x))
            .take(2)
            .product()
    }
}

#[aoc(day11, part1)]
pub fn part_1(input: &Input) -> u64 {
    let input = &mut input.clone();
    for _ in 0..20 {
        input.round(|i| i / 3);
    }

    input.monkey_business()
}

#[aoc(day11, part2)]
pub fn part_2(input: &Input) -> u64 {
    let input = &mut input.clone();
    for _ in 0..10000 {
        input.round(|i| i);
    }

    input.monkey_business()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
                Monkey 0:
                Starting items: 79, 98
                Operation: new = old * 19
                Test: divisible by 23
                    If true: throw to monkey 2
                    If false: throw to monkey 3

                Monkey 1:
                Starting items: 54, 65, 75, 74
                Operation: new = old + 6
                Test: divisible by 19
                    If true: throw to monkey 2
                    If false: throw to monkey 0

                Monkey 2:
                Starting items: 79, 60, 97
                Operation: new = old * old
                Test: divisible by 13
                    If true: throw to monkey 1
                    If false: throw to monkey 3

                Monkey 3:
                Starting items: 74
                Operation: new = old + 3
                Test: divisible by 17
                    If true: throw to monkey 0
                    If false: throw to monkey 1
                "
        });
        assert_eq!(part_1(&input), 10605);
        assert_eq!(part_2(&input), 2713310158);
    }
}
