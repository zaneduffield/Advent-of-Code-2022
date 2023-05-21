use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use rustc_hash::FxHashMap;

type Bits = u128;

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Input {
    start: (isize, isize),
    goal: (isize, isize),
    width: isize,
    height: isize,
    north: Vec<Bits>,
    south: Vec<Bits>,
    east: Vec<Bits>,
    west: Vec<Bits>,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Input {
    fn gap_pos(s: &str) -> usize {
        s.bytes().position(|c| c == b'.').map(|p| p - 1).unwrap()
    }

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let width = first_line.len() - 2;
    let start_x = gap_pos(first_line);

    let mut north = vec![];
    let mut east = vec![];
    let mut south = vec![];
    let mut west = vec![];

    for line in lines.take_while_ref(|line| !line.contains("##")) {
        let mut n = 0;
        let mut e = 0;
        let mut s = 0;
        let mut w = 0;

        line.bytes()
            .skip(1)
            .take(width)
            .enumerate()
            .map(|(i, b)| (1 << i, b))
            .for_each(|(bit, b)| match b {
                b'^' => n |= bit,
                b'>' => e |= bit,
                b'v' => s |= bit,
                b'<' => w |= bit,
                b'.' => {}
                _ => panic!("unexpected byte {b}"),
            });

        north.push(n);
        east.push(e);
        south.push(s);
        west.push(w);
    }

    let goal_x = gap_pos(lines.next().unwrap());
    let height = north.len() as isize;

    Input {
        start: (start_x as isize, -1),
        goal: (goal_x as isize, height),
        width: width as isize,
        height,
        north,
        south,
        east,
        west,
    }
}

impl Input {
    fn in_range(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }
}

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct State {
    pos: (isize, isize),
    elapsed: usize,
}

fn rotate_east_wind(x: Bits, step: isize, width: isize) -> Bits {
    let mut out = x << step;
    out |= x >> (width - step);

    // we could skip masking the result of this shift because our garbage is
    // being rotated AWAY FROM the important bits, but it's easier to test this way
    // and doesn't make much of a difference to the performance
    out &= Bits::MAX >> (Bits::BITS as isize - width);

    out
}

fn rotate_west_wind(x: Bits, step: isize, width: isize) -> Bits {
    let mut out = x >> step;
    out |= x << (width - step);

    // we can't skip masking the result of this shift because our garbage is
    // being rotated TOWARDS the important bits
    out &= Bits::MAX >> (Bits::BITS as isize - width);

    out
}

impl State {
    fn heuristic(&self, goal: (isize, isize)) -> usize {
        goal.1.abs_diff(self.pos.1) + goal.0.abs_diff(self.pos.0)
    }

    fn rotate_or_ones<F>(&self, input: &Input, f: F, bits: &[Bits], y: isize, step: isize) -> Bits
    where
        F: Fn(Bits, isize, isize) -> Bits,
    {
        if y >= 0 && y < input.height {
            if let Some(x) = bits.get(y as usize) {
                return f(*x, step, input.width);
            }
        }
        Bits::MAX
    }

    fn neighbours(&self, input: &Input) -> [Option<State>; 5] {
        let rotation = self.elapsed as isize + 1;

        let len = input.north.len() as isize;

        let new_ys = [-1, 0, 1].map(|dy| self.pos.1 + dy);

        let north_bitmaps_by_dy = new_ys.map(|y| {
            if y < 0 || y >= input.height {
                Bits::MAX
            } else {
                input.north[(y + rotation).rem_euclid(len) as usize]
            }
        });

        let len = input.south.len() as isize;
        let south_bitmaps_by_dy = new_ys.map(|y| {
            if y < 0 || y >= input.height {
                Bits::MAX
            } else {
                input.south[(y - rotation).rem_euclid(len) as usize]
            }
        });

        let hor_rotation = rotation % input.width;
        let east_bitmaps_by_dy = new_ys
            .map(|y| self.rotate_or_ones(input, rotate_east_wind, &input.east, y, hor_rotation));
        let west_bitmaps_by_dy = new_ys
            .map(|y| self.rotate_or_ones(input, rotate_west_wind, &input.west, y, hor_rotation));

        [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)].map(|(dx, dy)| {
            let pos = (self.pos.0 + dx, self.pos.1 + dy);

            let mut available = false;
            if (pos == input.start) || (pos == input.goal) {
                available = true;
            } else if input.in_range(pos) {
                let dy_idx = (dy + 1) as usize;
                let n = north_bitmaps_by_dy[dy_idx];
                let e = east_bitmaps_by_dy[dy_idx];
                let s = south_bitmaps_by_dy[dy_idx];
                let w = west_bitmaps_by_dy[dy_idx];

                let bit = 1 << pos.0;
                available = ((n & bit) | (e & bit) | (s & bit) | (w & bit)) == 0;
            }

            if available {
                Some(State {
                    pos,
                    elapsed: self.elapsed + 1,
                })
            } else {
                None
            }
        })
    }
}

fn astar_min_cost(start: State, input: &Input) -> Option<usize> {
    let mut min_costs = FxHashMap::default();
    let mut min_heap = BinaryHeap::new();
    min_heap.push((Reverse(start.heuristic(input.goal)), start, 0));

    loop {
        let (_, next, cost) = min_heap.pop()?;
        if next.pos == input.goal {
            return Some(cost);
        }
        for n_state in next.neighbours(input).into_iter().flatten() {
            let n_cost = n_state.elapsed;
            if n_cost < *min_costs.get(&n_state).unwrap_or(&usize::MAX) {
                min_heap.push((
                    Reverse(n_cost + n_state.heuristic(input.goal)),
                    n_state.clone(),
                    n_cost,
                ));
                min_costs.insert(n_state, n_cost);
            }
        }
    }
}
fn init_state(input: &Input, elapsed: usize) -> State {
    State {
        pos: input.start,
        elapsed,
    }
}

#[aoc(day24, part1)]
pub fn part_1(input: &Input) -> usize {
    astar_min_cost(init_state(input, 0), input).unwrap()
}

#[aoc(day24, part2)]
pub fn part_2(input: &Input) -> usize {
    let cost = astar_min_cost(init_state(input, 0), input).unwrap();

    let reverse_input = &Input {
        start: input.goal,
        goal: input.start,
        ..input.clone()
    };
    let cost = astar_min_cost(init_state(reverse_input, cost), reverse_input).unwrap();

    astar_min_cost(init_state(input, cost), input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            #.######
            #>>.<^<#
            #.<..<<#
            #>v.><>#
            #<^v^^>#
            ######.#
            "
        });
        assert_eq!(part_1(&input), 18);
        assert_eq!(part_2(&input), 54);
    }

    fn cmp(a: Bits, b: Bits) {
        if a != b {
            eprintln!("left:  {:0128b}", a);
            eprintln!("right: {:0128b}", b);
            panic!();
        }
    }

    #[test]
    fn test_wrapping() {
        const WIDTH: isize = 5;

        cmp(rotate_west_wind(0b10100, 1, WIDTH), 0b01010);
        cmp(rotate_west_wind(0b01010, 1, WIDTH), 0b00101);
        cmp(rotate_west_wind(0b00101, 1, WIDTH), 0b10010);
        cmp(rotate_west_wind(0b00101, 2, WIDTH), 0b01001);

        cmp(rotate_east_wind(0b10010, 1, WIDTH), 0b00101);
        cmp(rotate_east_wind(0b00101, 1, WIDTH), 0b01010);
        cmp(rotate_east_wind(0b01010, 1, WIDTH), 0b10100);
        cmp(rotate_east_wind(0b10100, 1, WIDTH), 0b01001);
        cmp(rotate_east_wind(0b10100, 2, WIDTH), 0b10010);
    }
}
