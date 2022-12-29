use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, VecDeque},
};

use arrayvec::ArrayVec;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, EnumString};

#[derive(Copy, Clone, EnumString, EnumCount, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

pub struct Recipe {
    goal: Resource,
    ingredients: ArrayVec<(u8, Resource), { Resource::COUNT }>,
}

pub struct Blueprint {
    recipes: ArrayVec<Recipe, 4>,
}

pub type Input = Vec<Blueprint>;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let re = Regex::new(r"Each (\w+) robot costs (?:(\d+) (\w+)(?: and)?)+.").unwrap();
    input
        .lines()
        .map(|line| Blueprint {
            recipes: re
                .captures_iter(line)
                .map(|caps| Recipe {
                    goal: caps
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("failed to parse as resource"),
                    ingredients: caps
                        .iter()
                        .skip(2)
                        .tuples()
                        .map(|(count, res)| {
                            (
                                count.unwrap().as_str().parse().unwrap(),
                                res.unwrap().as_str().parse().unwrap(),
                            )
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect()
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    resources: [u8; Resource::COUNT],
    production: [u8; Resource::COUNT],
}

impl State {
    fn new() -> Self {
        State {
            resources: [0; Resource::COUNT],
            production: [0; Resource::COUNT],
        }
    }

    fn get_res_count(&self, r: Resource) -> u8 {
        self.resources[r as usize]
    }

    fn get_res_count_mut(&mut self, r: Resource) -> &mut u8 {
        &mut self.resources[r as usize]
    }

    fn get_prod_count(&self, r: Resource) -> u8 {
        self.production[r as usize]
    }

    fn get_prod_count_mut(&mut self, r: Resource) -> &mut u8 {
        &mut self.production[r as usize]
    }

    fn nbours(&self, blueprint: &Blueprint) -> ArrayVec<State, { Resource::COUNT + 1 }> {
        let mut next = self.clone();
        for r in Resource::iter() {
            let c = next.get_prod_count(r);
            *next.get_res_count_mut(r) += c;
        }

        let mut out = ArrayVec::new();

        out.push(next.clone());
        for r in &blueprint.recipes {
            if r.ingredients
                .iter()
                .all(|(count, res)| self.get_res_count(*res) >= *count)
            {
                let mut new = next.clone();
                for (count, res) in &r.ingredients {
                    *new.get_res_count_mut(*res) -= *count;
                }
                *new.get_prod_count_mut(r.goal) += 1;
                out.push(new);
            }
        }

        out
    }
}

impl Blueprint {
    fn solve(&self) -> u8 {
        const ROUNDS: usize = 24;
        const BEAM_WIDTH: usize = 1000;

        let mut queue = VecDeque::new();
        let mut init = State::new();
        *init.get_prod_count_mut(Resource::Ore) += 1;
        queue.push_back(init);

        let mut visited = FxHashSet::default();

        let mut beam = BinaryHeap::new();

        let mut most_geodes = 0;
        for round in 0..ROUNDS {
            for _ in 0..queue.len() {
                let state = queue.pop_front().unwrap();
                if !visited.insert(state.clone()) {
                    continue;
                }

                let next_geodes =
                    state.get_res_count(Resource::Geode) + state.get_prod_count(Resource::Geode);

                if round == ROUNDS - 1 {
                    most_geodes = most_geodes.max(next_geodes);
                    continue;
                }

                if beam.len() < BEAM_WIDTH {
                    beam.push(Reverse(next_geodes))
                } else {
                    let smallest = beam.peek().unwrap().0;
                    match smallest.cmp(&next_geodes) {
                        Ordering::Less => {
                            beam.pop();
                            beam.push(Reverse(next_geodes));
                        }
                        // skip because it's unlikely to catch up and become optimal
                        Ordering::Greater => continue,
                        _ => {}
                    }
                }

                queue.extend(state.nbours(self));
            }
        }

        most_geodes
    }
}

#[aoc(day19, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as u32 * b.solve() as u32)
        .sum()
}

#[aoc(day19, part2)]
pub fn part_2(input: &Input) -> u64 {
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
            Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
            Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
            "
        });
        assert_eq!(part_1(&input), 33);
        // assert_eq!(part_2(&input),);
    }
}
