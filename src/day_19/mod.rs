// mostly a copy of https://github.com/Crazytieguy/advent-of-code/blob/ae35e9347ee3fd2737f6c1ce291b74186cde11a1/2022/src/bin/day19/main.rs

use std::ops::{Add, Index, IndexMut, Mul};

use arrayvec::ArrayVec;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, space1, u8},
    multi::{count, many0, separated_list1},
    sequence::{delimited, pair, tuple},
};
use rayon::prelude::*;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, EnumString};

#[derive(Debug, Copy, Clone, EnumString, EnumCount, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
#[strum(ascii_case_insensitive)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const ALL_RESOURCES: [Resource; Resource::COUNT] = [
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

#[derive(Debug, Copy, Clone)]
pub struct Resources {
    counts: [u8; Resource::COUNT],
}

impl Resources {
    fn new() -> Self {
        Self {
            counts: [0; Resource::COUNT],
        }
    }
}

impl Index<Resource> for Resources {
    type Output = u8;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.counts[index as usize]
    }
}

impl IndexMut<Resource> for Resources {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        &mut self.counts[index as usize]
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.counts
            .iter_mut()
            .zip(rhs.counts)
            .for_each(|(l, r)| *l += r);
        self
    }
}

impl Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self::Output {
            counts: self.counts.map(|c| c * rhs),
        }
    }
}

impl Resources {
    fn add_resource(mut self, r: Resource) -> Self {
        self[r] += 1;
        self
    }

    fn checked_sub(mut self, rhs: &Self) -> Option<Self> {
        for (l, r) in self.counts.iter_mut().zip(rhs.counts) {
            *l = l.checked_sub(r)?;
        }
        Some(self)
    }
}

#[derive(Debug)]
pub struct Recipe {
    goal: Resource,
    cost: Resources,
}

pub struct Blueprint {
    recipes: [Recipe; Resource::COUNT],
    max_ingredient_counts: Resources,
}

impl Blueprint {
    fn new(recipes: [Recipe; Resource::COUNT]) -> Blueprint {
        let mut max_ingredient_counts = Resources::new();
        recipes.iter().for_each(|r| {
            Resource::iter().for_each(|res| {
                max_ingredient_counts[res] = max_ingredient_counts[res].max(r.cost[res])
            })
        });

        Blueprint {
            recipes,
            max_ingredient_counts,
        }
    }
}

pub type Input = Vec<Blueprint>;
pub type IResult<'a, T> = nom::IResult<&'a str, T>;

fn parse_blueprint(input: &str) -> IResult<Blueprint> {
    let (input, _id) = delimited(tag("Blueprint "), u8, tag(": "))(input)?;

    let (input, v) = count(
        tuple((
            delimited(tag("Each "), alpha1, tag(" robot costs ")),
            separated_list1(tag(" and "), tuple((u8, space1, alpha1))),
            pair(tag("."), many0(char(' '))),
        )),
        Resource::COUNT,
    )(input)?;

    let mut recipes = ALL_RESOURCES.map(|goal| Recipe {
        goal,
        cost: Resources::new(),
    });

    v.into_iter().for_each(|(goal, ingredients, _)| {
        let goal: Resource = goal.parse().unwrap();
        for (count, _, res) in ingredients {
            let res = res.parse().unwrap();
            recipes[goal as usize].cost[res] = count;
        }
    });

    Ok((input, Blueprint::new(recipes)))
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    separated_list1(line_ending, parse_blueprint)(input)
        .unwrap()
        .1
}

#[derive(Copy, Clone)]
struct State {
    resources: Resources,
    production: Resources,
    secured_geodes: u8,
    remaining_steps: u8,
}

impl State {
    fn new(steps: u8) -> Self {
        State {
            resources: Resources::new(),
            production: Resources::new(),
            secured_geodes: 0,
            remaining_steps: steps,
        }
    }

    fn choose_robot(&self, recipe: &Recipe) -> Option<Self> {
        (1..self.remaining_steps)
            .rev()
            .zip(0..)
            .find_map(|(remaining_steps, steps_passed)| {
                let resources = self.resources + self.production * steps_passed;

                resources.checked_sub(&recipe.cost).map(|res| {
                    let secured_geodes = self.secured_geodes
                        + if recipe.goal == Resource::Geode {
                            remaining_steps
                        } else {
                            0
                        };

                    Self {
                        remaining_steps,
                        resources: res + self.production,
                        production: self.production.add_resource(recipe.goal),
                        secured_geodes,
                    }
                })
            })
    }

    fn branch(&self, blueprint: &Blueprint) -> ArrayVec<State, { Resource::COUNT }> {
        let mut out = ArrayVec::new();

        if self.remaining_steps == 0 {
            return out;
        }

        out.extend(
            blueprint
                .recipes
                .iter()
                .filter(|r| {
                    (r.goal == Resource::Geode)
                        | (self.production[r.goal] <= blueprint.max_ingredient_counts[r.goal])
                })
                .filter_map(|r| self.choose_robot(r)),
        );

        out
    }

    // assuming unlimited ore and clay, how many geodes could we collect if we
    // just build geode robots as soon as possible
    fn bound(&self, blueprint: &Blueprint) -> u8 {
        let obsidian_cost_for_geode =
            blueprint.recipes[Resource::Geode as usize].cost[Resource::Obsidian];

        let (_, _, geodes) = (0..self.remaining_steps).rev().fold(
            (
                self.resources[Resource::Obsidian],
                self.production[Resource::Obsidian],
                self.secured_geodes,
            ),
            |(obsidian, obsidian_rate, geodes), steps_remaining| {
                if obsidian >= obsidian_cost_for_geode {
                    (
                        obsidian + obsidian_rate - obsidian_cost_for_geode,
                        obsidian_rate,
                        geodes + steps_remaining,
                    )
                } else {
                    (obsidian + obsidian_rate, obsidian_rate + 1, geodes)
                }
            },
        );
        geodes
    }
}

impl Blueprint {
    fn branch_and_bound(&self, state: State, best: &mut u8) {
        *best = state.secured_geodes.max(*best);
        for next in state.branch(self) {
            if next.bound(self) > *best {
                self.branch_and_bound(next, best);
            }
        }
    }

    fn solve(&self, rounds: u8) -> u8 {
        let mut best = 0;
        let mut init = State::new(rounds);
        init.production[Resource::Ore] = 1;
        self.branch_and_bound(init, &mut best);
        best
    }
}

#[aoc(day19, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .par_iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as u32 * b.solve(24) as u32)
        .sum()
}

#[aoc(day19, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        .par_iter()
        .take(3)
        .map(|b| b.solve(32) as u32)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
            Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
            "
        });
        assert_eq!(part_1(&input), 33);
        assert_eq!(part_2(&input), 56 * 62);
    }
}
