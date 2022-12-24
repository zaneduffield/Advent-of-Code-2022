// not sure yet but this feels like A* again
// where the admisible heuristic is the current flow rate multiplied
// by the remaining time, and the 'cost' is negative released presure.
// The state would be the position and all of the values that are open,
// and the 'neighbours' would be the states that can be reached in a single move.
// ... yeah that should work quite well, there was a puzzle last year near
// the end that was very similar - start there.

use std::{
    cmp::{Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, VecDeque},
    str::FromStr,
};

use core::hash::Hash;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use rustc_hash::{FxHashMap, FxHashSet};
use slab::Slab;
use tinyset::SetUsize;

use crate::day_13::Val;

pub type ValveId = u8;

pub struct Valve {
    id: ValveId,
    flow_rate: u8,
    nbours: Vec<ValveId>,
}

pub struct Input {
    valves: Vec<Valve>,
    start_id: ValveId,
    total_flow: u32,
}

fn str_to_id(s: &str, id_map: &mut FxHashMap<String, ValveId>, max_id: &mut ValveId) -> ValveId {
    *id_map.entry(s.to_string()).or_insert_with(|| {
        let id = *max_id;
        *max_id += 1;
        id
    })
}

impl Valve {
    fn new(value: &str, id_map: &mut FxHashMap<String, ValveId>, max_id: &mut ValveId) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z]{2}(?:, )?)+)").unwrap();
        }
        let captures = RE
            .captures(value)
            .expect(&format!("line didn't match re: {value}"));
        let id = str_to_id(&captures[1], id_map, max_id);
        let flow_rate = captures[2].parse().unwrap();
        let nbours = captures[3]
            .split(", ")
            .map(|s| str_to_id(s, id_map, max_id))
            .collect();

        Self {
            id,
            flow_rate,
            nbours,
        }
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut valves = (0..26 * 26).map(|_| None).collect_vec();
    let mut total_flow = 0;
    let mut id_map = FxHashMap::default();
    let mut max_id = 0;
    for line in input.lines() {
        let valve = Valve::new(line, &mut id_map, &mut max_id);
        let id = valve.id;
        total_flow += valve.flow_rate as u32;
        valves[id as usize] = Some(valve);
    }
    let start_id = str_to_id(&"AA", &mut id_map, &mut max_id);
    Input {
        valves: valves.into_iter().flatten().collect(),
        total_flow,
        start_id,
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Set64(u64);

impl Set64 {
    fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    fn to_bit(u: u8) -> u64 {
        1 << u
    }

    #[inline(always)]
    fn add(&mut self, u: u8) {
        self.0 |= Self::to_bit(u);
    }

    #[inline(always)]
    fn contains(&self, u: u8) -> bool {
        self.0 & Self::to_bit(u) != 0
    }
}

const TOTAL_MINS: u8 = 30;

#[derive(Clone, PartialEq, Eq, Hash)]
struct ValveState {
    released_pressure: u32,
    current_flow: u8,
    open_valves: Set64,
    current_valve_id: ValveId,
}

const INIT_PRESSURE: u32 = 0;
const INIT_FLOW: u8 = 0;

#[aoc(day16, part1)]
pub fn part_1(input: &Input) -> u32 {
    // based off https://github.com/SvetlinZarev/advent-of-code/blob/main/2022/aoc-day-16/src/p1v2.rs
    const ROUNDS: u32 = 30;

    // The number of most-promising positions to keep exploring.
    // adjust for speed & correctness
    const BEAM_WIDTH: usize = 1000;

    let mut queue = VecDeque::new();
    queue.push_back(ValveState {
        released_pressure: INIT_PRESSURE,
        current_flow: INIT_FLOW,
        open_valves: Set64::new(),
        current_valve_id: input.start_id,
    });

    let mut visited = FxHashSet::default();
    visited.insert((Set64::new(), input.start_id, INIT_PRESSURE));

    let mut beam = BinaryHeap::new();

    let mut most_pressure = 0;
    for round in 0..ROUNDS {
        for _ in 0..queue.len() {
            let state = queue.pop_front().unwrap();
            let ValveState {
                released_pressure,
                current_flow,
                open_valves,
                current_valve_id,
            } = state;

            let next_pressure = released_pressure + current_flow as u32;

            if round == ROUNDS - 1 {
                most_pressure = most_pressure.max(next_pressure);
                continue;
            }

            if beam.len() < BEAM_WIDTH {
                beam.push(Reverse(next_pressure))
            } else {
                let smallest = beam.peek().unwrap().0;
                match smallest.cmp(&next_pressure) {
                    Ordering::Less => {
                        beam.pop();
                        beam.push(Reverse(next_pressure));
                    }
                    // skip because it's unlikely to catch up and become optimal
                    Ordering::Greater => continue,
                    _ => {}
                }
            }

            let cur_valve = &input.valves[current_valve_id as usize];
            if !open_valves.contains(current_valve_id) && cur_valve.flow_rate > 0 {
                let mut new_open_set = open_valves.clone();
                new_open_set.add(current_valve_id);

                if visited.insert((new_open_set, current_valve_id, next_pressure)) {
                    let new_state = ValveState {
                        current_flow: current_flow + cur_valve.flow_rate,
                        released_pressure: next_pressure,
                        open_valves: new_open_set,
                        ..state
                    };
                    queue.push_back(new_state)
                }
            }

            for &nb in &cur_valve.nbours {
                if visited.insert((open_valves, nb, next_pressure)) {
                    let new_state = ValveState {
                        current_valve_id: nb,
                        released_pressure: next_pressure,
                        ..state
                    };
                    queue.push_back(new_state);
                }
            }
        }
    }

    most_pressure
}

#[aoc(day16, part2)]
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
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
            "
        });
        assert_eq!(part_1(&input), 1651);
        // assert_eq!(part_2(&input),);
    }
}
