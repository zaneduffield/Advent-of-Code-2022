use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::{Debug, Write},
    iter::repeat,
};

use itertools::{izip, Itertools};
use rustc_hash::FxHashMap;

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Input {
    start: (isize, isize),
    goal: (isize, isize),
    width: isize,
    height: isize,
    // TODO optimise this by merging into one vector
    north: Vec<u128>,
    south: Vec<u128>,
    east: Vec<u128>,
    west: Vec<u128>,
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
            .map(|(i, b)| (1u128 << i, b))
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

fn rotate_north_winds(winds: &mut [u128]) {
    winds.rotate_left(1);
}

fn rotate_south_winds(winds: &mut [u128]) {
    winds.rotate_right(1);
}

fn rotate_east_winds(winds: &mut [u128], width: isize) {
    winds.iter_mut().for_each(|x| {
        *x = (*x << 1) | ((*x >> (width - 1)) & 1);
        // isn't actually necessary
        *x &= !(1 << width);
    });
}

fn rotate_west_winds(winds: &mut [u128], width: isize) {
    winds.iter_mut().for_each(|x| {
        *x = (*x >> 1) | ((*x & 1) << (width - 1));
    });
}

impl Input {
    fn step(&self) -> Self {
        let mut out = self.clone();
        rotate_north_winds(&mut out.north);
        rotate_south_winds(&mut out.south);
        rotate_east_winds(&mut out.east, self.width);
        rotate_west_winds(&mut out.west, self.width);
        out
    }

    fn available(&self, pos: (isize, isize)) -> bool {
        if (pos == self.start) || (pos == self.goal) {
            return true;
        }
        if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
            return false;
        }

        let idx = pos.1 as usize;
        let bit = 1 << (pos.0 as usize);
        ((self.north[idx] & bit)
            | (self.east[idx] & bit)
            | (self.south[idx] & bit)
            | (self.west[idx] & bit))
            == 0
    }
}

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct State {
    input: Input,
    pos: (isize, isize),
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = &self.input;

        let wall = |p: usize| {
            let mut s = String::new();
            s.push('#');
            s.extend(repeat('#').take(p));
            s.push('.');
            s.extend(repeat('#').take((input.width as usize) - p - 1));
            s.push('\n');
            s
        };

        f.write_char('\n')?;
        f.write_str(&wall(input.start.0 as usize))?;

        for (y, (n, s, e, w)) in
            izip!(&input.north, &input.south, &input.east, &input.west).enumerate()
        {
            f.write_char('#')?;

            for x in 0..input.width {
                if (x, y as isize) == self.pos {
                    f.write_char('E')?;
                    continue;
                }

                let mut ch = '.';
                let mask = 1u128 << x;

                let bits = [n & mask, e & mask, s & mask, w & mask];
                let chars = ['^', '>', 'v', '<'];
                let mut count = 0;
                bits.iter()
                    .zip(chars)
                    .filter(|(b, _)| **b != 0)
                    .for_each(|(_, c)| {
                        ch = c;
                        count += 1
                    });

                if count > 1 {
                    f.write_fmt(format_args!("{count}"))?;
                } else {
                    f.write_char(ch)?;
                }
            }

            f.write_str("#\n")?;
        }

        f.write_str(&wall(input.goal.0 as usize))
    }
}

impl State {
    fn heuristic(&self) -> usize {
        self.input.goal.1.abs_diff(self.pos.1) + self.input.goal.0.abs_diff(self.pos.0)
    }

    fn neighbours(&self) -> [Option<State>; 5] {
        let next_input = self.input.step();

        #[cfg(debug_assertions)]
        dbg!(&self);

        [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)]
            .map(|(dx, dy)| (self.pos.0 + dx, self.pos.1 + dy))
            .map(|pos| {
                // TODO inline the available function and reuse the masked bitmaps
                if next_input.available(pos) {
                    Some(State {
                        input: next_input.clone(),
                        pos,
                    })
                } else {
                    None
                }
            })
    }
}

fn astar_min_cost(start: State) -> Option<(usize, State)> {
    let mut min_costs = FxHashMap::default();
    let mut min_heap = BinaryHeap::new();
    min_heap.push((Reverse(start.heuristic()), start, 0));

    loop {
        let (_, next, cost) = min_heap.pop()?;
        if next.pos == next.input.goal {
            return Some((cost, next));
        }
        let n_cost = cost + 1;
        for n_state in next.neighbours().into_iter().flatten() {
            if n_cost < *min_costs.get(&n_state).unwrap_or(&usize::MAX) {
                min_heap.push((
                    Reverse(n_cost + n_state.heuristic()),
                    n_state.clone(),
                    n_cost,
                ));
                min_costs.insert(n_state, n_cost);
            }
        }
    }
}
fn init_state(input: &Input) -> State {
    State {
        input: input.clone(),
        pos: input.start,
    }
}

#[aoc(day24, part1)]
pub fn part_1(input: &Input) -> usize {
    astar_min_cost(init_state(input)).unwrap().0
}

#[aoc(day24, part2)]
pub fn part_2(input: &Input) -> usize {
    let (cost1, state) = astar_min_cost(init_state(input)).unwrap();

    let input = &Input {
        start: state.input.goal,
        goal: state.input.start,
        ..state.input
    };
    let (cost2, state) = astar_min_cost(init_state(input)).unwrap();

    let input = &Input {
        start: state.input.goal,
        goal: state.input.start,
        ..state.input
    };
    let (cost3, _state) = astar_min_cost(init_state(input)).unwrap();

    cost1 + cost2 + cost3
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

    fn cmp(a: u128, b: u128) {
        if a != b {
            eprintln!("left:  {:0128b}", a);
            eprintln!("right: {:0128b}", b);
            panic!();
        }
    }

    #[test]
    fn test_wrapping() {
        const WIDTH: isize = 5;
        let mut winds = [0b10100u128];

        rotate_west_winds(&mut winds, WIDTH);
        cmp(winds[0], 0b01010u128);

        rotate_west_winds(&mut winds, WIDTH);
        cmp(winds[0], 0b00101u128);

        rotate_west_winds(&mut winds, WIDTH);
        cmp(winds[0], 0b10010u128);

        rotate_east_winds(&mut winds, WIDTH);
        cmp(winds[0], 0b00101u128);

        rotate_east_winds(&mut winds, WIDTH);
        cmp(winds[0], 0b01010u128);

        rotate_east_winds(&mut winds, WIDTH);
        cmp(winds[0], 0b10100u128);

        rotate_east_winds(&mut winds, WIDTH);
        cmp(winds[0], 0b01001u128);
    }
}
