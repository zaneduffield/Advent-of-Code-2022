// not sure yet but this feels like A* again
// where the admisible heuristic is the current flow rate multiplied
// by the remaining time, and the 'cost' is negative released presure.
// The state would be the position and all of the values that are open,
// and the 'neighbours' would be the states that can be reached in a single move.
// ... yeah that should work quite well, there was a puzzle last year near
// the end that was very similar - start there.

use std::{
    collections::{BTreeMap, BinaryHeap},
    str::FromStr,
};

use core::hash::Hash;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Match, Regex};
use rustc_hash::FxHashMap;
use slab::Slab;
use tinyset::SetUsize;

use crate::day_13::Val;

pub type ValveId = usize;

pub struct Valve {
    id: ValveId,
    flow_rate: u8,
    nbours: Vec<ValveId>,
}

pub struct Input {
    valves: Vec<Option<Valve>>,
    total_flow: u32,
}

impl Valve {
    fn id_from_str(s: &str) -> ValveId {
        s.chars()
            .take(2)
            .fold(0, |sum, c| sum * 26 + (c as u8 - b'A') as ValveId)
    }
}

impl From<&str> for Valve {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z]{2}(?:, )?)+)").unwrap();
        }
        let captures = RE
            .captures(value)
            .expect(&format!("line didn't match re: {value}"));
        let id = Self::id_from_str(&captures[1]);
        let flow_rate = captures[2].parse().unwrap();
        let nbours = captures[3].split(", ").map(Self::id_from_str).collect();

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
    for line in input.lines() {
        let valve = Valve::from(line);
        let id = valve.id;
        total_flow += valve.flow_rate as u32;
        valves[id] = Some(valve);
    }
    Input { valves, total_flow }
}

const TOTAL_MINS: u8 = 30;

#[derive(Clone, PartialEq, Eq)]
struct ValveState {
    elapsed_mins: u8,
    released_pressure: u32,
    current_flow: u32,
    open_valves: SetUsize,
    current_valve_id: ValveId,
    heuristic: u32,
}

impl ValveState {
    fn heuristic(&self, input: &Input) -> u32 {
        self.released_pressure + (TOTAL_MINS - self.elapsed_mins) as u32 * input.total_flow
    }

    fn nbours(&self, input: &Input, nbs: &mut Vec<ValveState>) {
        let current_valve = input.valves[self.current_valve_id].as_ref().unwrap();
        let valve_nbours = &current_valve.nbours;

        let mut next = self.clone();
        next.elapsed_mins += 1;
        next.released_pressure += next.current_flow as u32;

        if current_valve.flow_rate > 0 && !self.open_valves.contains(self.current_valve_id) {
            let mut nb = next.clone();
            nb.open_valves.insert(self.current_valve_id);
            nb.current_flow += current_valve.flow_rate as u32;
            nb.heuristic = nb.heuristic(input);
            nbs.push(nb)
        }

        for v in valve_nbours {
            let mut nb = next.clone();
            nb.current_valve_id = *v;
            nb.heuristic = nb.heuristic(input);
            nbs.push(nb);
        }
    }
}

impl Hash for ValveState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.elapsed_mins.hash(state);
        self.released_pressure.hash(state);
        self.current_flow.hash(state);
        // needs to be in a deterministic order, but it already is
        self.open_valves.iter().for_each(|v| v.hash(state));
        self.current_valve_id.hash(state);
        // derived property
        // self.heuristic.hash(state);
    }
}

impl PartialOrd for ValveState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.heuristic.cmp(&other.heuristic))
    }
}

impl Ord for ValveState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

impl Input {
    fn astar_min_cost(&self, start: ValveState) -> Option<u32> {
        let mut max_pressures = FxHashMap::default();
        let mut heap = BinaryHeap::new();
        heap.push(start);

        let mut nbours = vec![];
        loop {
            let next = heap.pop()?;
            if next.elapsed_mins == TOTAL_MINS {
                return Some(next.released_pressure);
            }
            nbours.clear();
            next.nbours(self, &mut nbours);
            for nb in nbours.iter() {
                let best = max_pressures.get(nb);
                if best.is_none() || Some(&nb.released_pressure) > best {
                    max_pressures.insert(nb.clone(), nb.released_pressure);
                    heap.push(nb.clone());
                }
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .astar_min_cost(ValveState {
            elapsed_mins: 0,
            released_pressure: 0,
            current_flow: 0,
            open_valves: SetUsize::new(),
            current_valve_id: Valve::id_from_str(&"AA"),
            heuristic: 0,
        })
        .expect("no solution found!")
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
