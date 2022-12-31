use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, VecDeque},
};

use arrayvec::ArrayVec;
use rayon::prelude::*;
use regex::Regex;
use rustc_hash::FxHashSet;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, EnumString};

#[derive(Copy, Clone, EnumString, EnumCount, EnumIter, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

pub struct Ingredient {
    count: u8,
    resource: Resource,
}

pub struct Recipe {
    goal: Resource,
    ingredients: ArrayVec<Ingredient, { Resource::COUNT }>,
}

pub struct Blueprint {
    recipes: ArrayVec<Recipe, 4>,
    max_ingredient_counts: [u8; Resource::COUNT],
}

pub type Input = Vec<Blueprint>;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let re_blueprint = Regex::new(r"Each (\w+) robot costs ([^.]+).").unwrap();
    let re_item = Regex::new(r"(\d+) (\w+)").unwrap();
    input
        .lines()
        .map(|line| {
            let recipes: ArrayVec<Recipe, { Resource::COUNT }> = re_blueprint
                .captures_iter(line)
                .map(|caps| Recipe {
                    goal: caps[1].parse().expect("failed to parse as resource"),
                    ingredients: re_item
                        .captures_iter(&caps[2])
                        .map(|caps| Ingredient {
                            count: caps[1].parse().unwrap(),
                            resource: caps[2].parse().unwrap(),
                        })
                        .collect(),
                })
                .collect();
            let mut max_ingredient_counts = [0; Resource::COUNT];
            recipes.iter().flat_map(|r| &r.ingredients).for_each(
                |&Ingredient { count, resource }| {
                    max_ingredient_counts[resource as usize] =
                        max_ingredient_counts[resource as usize].max(count)
                },
            );

            Blueprint {
                recipes,
                max_ingredient_counts,
            }
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

    fn next(&self) -> Self {
        let mut next = self.clone();
        for r in Resource::iter() {
            let c = next.get_prod_count(r);
            *next.get_res_count_mut(r) += c;
        }
        next
    }

    fn nbours(&self, blueprint: &Blueprint) -> ArrayVec<State, { Resource::COUNT + 1 }> {
        let next = self.next();
        let mut out = ArrayVec::new();

        for r in &blueprint.recipes {
            // you never need to produce more than the maximum consumption
            // if self.get_prod_count(r.goal) > blueprint.max_ingredient_counts[r.goal as usize] {
            //     continue;
            // }

            if r.ingredients
                .iter()
                .all(|Ingredient { count, resource }| self.get_res_count(*resource) >= *count)
            {
                let mut new = next.clone();
                for Ingredient { count, resource } in &r.ingredients {
                    *new.get_res_count_mut(*resource) -= *count;
                }
                *new.get_prod_count_mut(r.goal) += 1;
                out.push(new);
            }
        }

        // if we can't make every robot, then we should consider waiting
        if out.len() < Resource::COUNT {
            out.push(next.clone());
        }

        out
    }
}

impl Blueprint {
    fn solve(&self, rounds: usize) -> u8 {
        const BEAM_WIDTH: usize = 10000;

        let mut queue = VecDeque::new();
        let mut init = State::new();
        *init.get_prod_count_mut(Resource::Ore) += 1;
        queue.push_back(init);

        let mut visited = FxHashSet::default();

        let mut beam = BinaryHeap::new();

        let mut most_geodes = 0;
        for round in 0..rounds {
            for _ in 0..queue.len() {
                let state = queue.pop_front().unwrap();
                if !visited.insert(state.clone()) {
                    continue;
                }

                let next_geodes =
                    state.get_res_count(Resource::Geode) + state.get_prod_count(Resource::Geode);

                if round == rounds - 1 {
                    most_geodes = most_geodes.max(next_geodes);
                    continue;
                }

                let heuristic = next_geodes;
                if beam.len() < BEAM_WIDTH {
                    beam.push(Reverse(heuristic))
                } else {
                    let smallest = beam.peek().unwrap().0;
                    match smallest.cmp(&heuristic) {
                        Ordering::Less => {
                            beam.pop();
                            beam.push(Reverse(heuristic));
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
        // .iter()
        .par_iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as u32 * b.solve(24) as u32)
        .sum()
}

#[aoc(day19, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        // .iter()
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
        assert_eq!(part_2(&input), 62);
    }
}
