// Originally I had a solution here that was based on the A* algorithm, but it was too
// slow for my goals, so I adapted someone elses solution using Beam Search to prune
// nodes that are unlikely to lead to the optimal solution. It's technically no longer
// an 'optimal' algorithm, but it finds the right answer for me - and very quickly.
//
// https://github.com/SvetlinZarev/advent-of-code/blob/main/2022/aoc-day-16/src/

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, VecDeque},
};

use core::hash::Hash;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

pub type ValveId = u8;

pub struct Valve {
    id: ValveId,
    flow_rate: u8,
    nbours: Vec<ValveId>,
}

pub struct Input {
    valves: Vec<Valve>,
    start_id: ValveId,
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
            .unwrap_or_else(|| panic!("line didn't match re: {value}"));
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
    let mut id_map = FxHashMap::default();
    let mut max_id = 0;
    for line in input.lines() {
        let valve = Valve::new(line, &mut id_map, &mut max_id);
        let id = valve.id;
        valves[id as usize] = Some(valve);
    }
    let start_id = str_to_id("AA", &mut id_map, &mut max_id);
    Input {
        valves: valves.into_iter().flatten().collect(),
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
    fn add(self, u: u8) -> Self {
        Self(self.0 | Self::to_bit(u))
    }

    #[inline(always)]
    fn contains(&self, u: u8) -> bool {
        self.0 & Self::to_bit(u) != 0
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct ValveState {
    released_pressure: u32,
    current_flow: u8,
    open_valves: Set64,
}

const INIT_PRESSURE: u32 = 0;
const INIT_FLOW: u8 = 0;

#[aoc(day16, part1)]
pub fn part_1(input: &Input) -> u32 {
    const ROUNDS: u32 = 30;

    // The number of most-promising positions to keep exploring.
    // adjust for speed & correctness
    const BEAM_WIDTH: usize = 100;

    let mut queue = VecDeque::new();
    queue.push_back((
        input.start_id,
        ValveState {
            released_pressure: INIT_PRESSURE,
            current_flow: INIT_FLOW,
            open_valves: Set64::new(),
        },
    ));

    let mut visited = FxHashSet::default();
    visited.insert((Set64::new(), input.start_id, INIT_PRESSURE));

    let mut beam = BinaryHeap::new();

    let mut most_pressure = 0;
    for round in 0..ROUNDS {
        for _ in 0..queue.len() {
            let (pos, state) = queue.pop_front().unwrap();
            let ValveState {
                released_pressure,
                current_flow,
                open_valves,
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

            let cur_valve = &input.valves[pos as usize];
            if !open_valves.contains(pos) && cur_valve.flow_rate > 0 {
                let new_open_set = open_valves.add(pos);

                if visited.insert((new_open_set, pos, next_pressure)) {
                    let new_state = ValveState {
                        current_flow: current_flow + cur_valve.flow_rate,
                        released_pressure: next_pressure,
                        open_valves: new_open_set,
                    };
                    queue.push_back((pos, new_state))
                }
            }

            for &nb in &cur_valve.nbours {
                if visited.insert((open_valves, nb, next_pressure)) {
                    let new_state = ValveState {
                        released_pressure: next_pressure,
                        ..state
                    };
                    queue.push_back((nb, new_state));
                }
            }
        }
    }

    most_pressure
}

#[aoc(day16, part2)]
pub fn part_2(input: &Input) -> u32 {
    // This is one of those days where for the sake of speed, I can't really generalise my solution
    // to work for both part 1 and part 2, despite them being quite similar.

    const ROUNDS: u32 = 26;

    // The number of most-promising positions to keep exploring.
    // adjust for speed & correctness
    const BEAM_WIDTH: usize = 100;

    let mut queue = VecDeque::new();
    queue.push_back((
        input.start_id,
        input.start_id,
        ValveState {
            released_pressure: INIT_PRESSURE,
            current_flow: INIT_FLOW,
            open_valves: Set64::new(),
        },
    ));

    let mut visited = FxHashSet::default();
    visited.insert((Set64::new(), input.start_id, input.start_id, INIT_PRESSURE));

    let mut beam = BinaryHeap::new();

    let mut most_pressure = 0;
    for round in 0..ROUNDS {
        for _ in 0..queue.len() {
            let (me_pos, el_pos, state) = queue.pop_front().unwrap();
            let ValveState {
                released_pressure,
                current_flow,
                open_valves,
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

            let me_valve = &input.valves[me_pos as usize];
            let el_valve = &input.valves[el_pos as usize];
            let me_can_open = !open_valves.contains(me_pos) && me_valve.flow_rate > 0;
            // let's always assume that 'me' will open the valve when we both can,
            // the situation is symmetric so it shouldn't affect the solution
            let el_can_open =
                !open_valves.contains(el_pos) && el_valve.flow_rate > 0 && me_pos != el_pos;

            if me_can_open && el_can_open {
                // we both open different valves
                let new_open_set = open_valves.add(me_pos).add(el_pos);
                if visited.insert((new_open_set, me_pos, el_pos, next_pressure)) {
                    let new_flow = current_flow + me_valve.flow_rate + el_valve.flow_rate;
                    queue.push_back((
                        me_pos,
                        el_pos,
                        ValveState {
                            open_valves: new_open_set,
                            released_pressure: next_pressure,
                            current_flow: new_flow,
                        },
                    ));
                }
            }

            if me_can_open {
                // only I open, and el moves to all neighbouring positions
                let new_open_set = open_valves.add(me_pos);
                let new_flow = current_flow + me_valve.flow_rate;
                for &el_new_pos in &el_valve.nbours {
                    if visited.insert((new_open_set, me_pos, el_new_pos, next_pressure)) {
                        queue.push_back((
                            me_pos,
                            el_new_pos,
                            ValveState {
                                open_valves: new_open_set,
                                released_pressure: next_pressure,
                                current_flow: new_flow,
                            },
                        ));
                    }
                }
            }

            if el_can_open {
                // only el opens, and I moves to all neighbouring positions
                let new_open_set = open_valves.add(el_pos);
                let new_flow = current_flow + el_valve.flow_rate;
                for &me_new_pos in &me_valve.nbours {
                    if visited.insert((new_open_set, me_new_pos, el_pos, next_pressure)) {
                        queue.push_back((
                            me_new_pos,
                            el_pos,
                            ValveState {
                                open_valves: new_open_set,
                                released_pressure: next_pressure,
                                current_flow: new_flow,
                            },
                        ));
                    }
                }
            }

            // we both move and open no valves
            for &me_new_pos in &me_valve.nbours {
                for &el_new_pos in &el_valve.nbours {
                    if visited.insert((open_valves, me_new_pos, el_new_pos, next_pressure)) {
                        queue.push_back((
                            me_new_pos,
                            el_new_pos,
                            ValveState {
                                released_pressure: next_pressure,
                                ..state
                            },
                        ));
                    }
                }
            }
        }
    }
    most_pressure
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
        assert_eq!(part_2(&input), 1707);
    }
}
