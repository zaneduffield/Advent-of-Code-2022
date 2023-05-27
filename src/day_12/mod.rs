use std::{
    fmt::{Debug, Write},
    ops::{Shl, Shr},
    simd::u32x8,
};

use itertools::Itertools;
use num::Integer;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

// u32x8 is the fastest by a slim margin on my machine, but this should work with any SIMD type.
type Lane = u32x8;
type LaneElm = u32;
const LANE_WIDTH: usize = LaneElm::BITS as usize;

#[derive(Default, Clone, Copy)]
struct Steps {
    left: Lane,
    right: Lane,
    up: Lane,
    down: Lane,
}

pub struct Input {
    start: Pos,
    alt_starts: Vec<Pos>,
    end: Pos,
    grid: Vec<Steps>,
    width: usize,
    height: usize,
}

fn set_lane_bit(lane: &mut Lane, x: usize) {
    let (quot, rem) = x.div_rem(&LANE_WIDTH);
    lane.as_mut_array()[quot] |= 1 << rem;
}

fn get_lane_bit(lane: Lane, x: usize) -> LaneElm {
    let (quot, rem) = x.div_rem(&LANE_WIDTH);
    lane.as_array()[quot] & (1 << rem)
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let mut start = None;
    let mut end = None;
    let width = input
        .lines()
        .next()
        .expect("at least one line is required")
        .len();

    let mut height_grid = vec![];
    let mut alt_starts = vec![];
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height += 1;
        height_grid.extend(line.bytes().enumerate().map(|(x, b)| match b {
            b'S' => {
                start = Some((x, y).into());
                alt_starts.push((x, y).into());
                b'a'
            }
            b'a' => {
                alt_starts.push((x, y).into());
                b'a'
            }
            b'E' => {
                end = Some((x, y).into());
                b'z'
            }
            _ => b,
        }));
    }

    let mut grid = vec![Steps::default(); height];
    grid.iter_mut().enumerate().for_each(|(i, row)| {
        height_grid
            .iter()
            .skip(i * width)
            .take(width)
            .tuple_windows()
            .enumerate()
            .for_each(|(x, (&a, &b))| {
                if b <= a + 1 {
                    set_lane_bit(&mut row.right, x);
                }
                if a <= b + 1 {
                    set_lane_bit(&mut row.left, x + 1);
                }
            });
    });

    for (a, b) in (0..height).tuple_windows() {
        (0..width)
            .map(|x| (x, height_grid[a * width + x], height_grid[b * width + x]))
            .for_each(|(x, ha, hb)| {
                if hb <= ha + 1 {
                    set_lane_bit(&mut grid[a].down, x);
                }
                if ha <= hb + 1 {
                    set_lane_bit(&mut grid[b].up, x);
                }
            });
    }

    Input {
        start: start.expect("no start found"),
        alt_starts,
        end: end.expect("no end found"),
        grid,
        width,
        height,
    }
}

impl Input {
    fn write_grid(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        ch: char,
        accessor: impl Fn(Steps) -> Lane,
    ) -> std::fmt::Result {
        f.write_char('\n')?;
        for y in 0..self.height {
            let elm = accessor(self.grid[y]);
            for x in 0..self.width {
                f.write_char(match get_lane_bit(elm, x) {
                    0 => '.',
                    _ => ch,
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_grid(f, '>', |s| s.right)?;
        self.write_grid(f, '<', |s| s.left)?;
        self.write_grid(f, '^', |s| s.up)?;
        self.write_grid(f, 'v', |s| s.down)?;
        Ok(())
    }
}

const ALL_BUT_LAST_LANE: Lane = {
    let mut lanes = [LaneElm::MAX; Lane::LANES];
    lanes[Lane::LANES - 1] = 0;
    Lane::from_array(lanes)
};
const ALL_BUT_FIRST_LANE: Lane = {
    let mut lanes = [LaneElm::MAX; Lane::LANES];
    lanes[0] = 0;
    Lane::from_array(lanes)
};

fn solve(mut reachable: Vec<Lane>, input: &Input) -> usize {
    let shift = Lane::splat(LANE_WIDTH as LaneElm - 1);

    let mut steps = 0;
    let mut new_reachable = reachable.clone();
    while 0 == get_lane_bit(reachable[input.end.y], input.end.x) {
        steps += 1;

        for i in 0..reachable.len() {
            let new_right_reachable = {
                let to_step = reachable[i] & input.grid[i].right;
                let stepped = to_step.shl(Lane::splat(1));
                let overflow = (to_step & ALL_BUT_LAST_LANE)
                    .rotate_lanes_right::<1>()
                    .shr(shift);

                stepped | overflow
            };
            let new_left_reachable = {
                let to_step = reachable[i] & input.grid[i].left;
                let stepped = to_step.shr(Lane::splat(1));
                let overflow = (to_step & ALL_BUT_FIRST_LANE)
                    .rotate_lanes_left::<1>()
                    .shl(shift);

                stepped | overflow
            };

            let new_up_reachable = {
                reachable.get(i + 1).copied().unwrap_or_default()
                    & input.grid.get(i + 1).map(|s| s.up).unwrap_or_default()
            };
            let new_down_reachable = {
                let index = i.wrapping_sub(1);
                reachable.get(index).copied().unwrap_or_default()
                    & input.grid.get(index).map(|s| s.down).unwrap_or_default()
            };

            new_reachable[i] =
                new_right_reachable | new_left_reachable | new_up_reachable | new_down_reachable;
        }

        std::mem::swap(&mut reachable, &mut new_reachable);
    }
    steps
}

fn init_reachable(input: &Input, starts: &[Pos]) -> Vec<Lane> {
    let mut reachable = vec![Lane::default(); input.grid.len()];
    for start in starts {
        set_lane_bit(&mut reachable[start.y], start.x);
    }

    reachable
}

#[aoc(day12, part1)]
pub fn part_1(input: &Input) -> usize {
    #[cfg(debug_assertions)]
    dbg!(input);

    solve(init_reachable(input, &[input.start]), input)
}

#[aoc(day12, part2)]
pub fn part_2(input: &Input) -> usize {
    solve(init_reachable(input, &input.alt_starts), input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
            "
        });
        assert_eq!(part_1(&input), 31);
        assert_eq!(part_2(&input), 29);
    }

    #[test]
    fn test_big() {
        let input = input_generator(include_str!("../../input/2022/day12.txt"));
        assert_eq!(part_1(&input), 456);
        assert_eq!(part_2(&input), 454);
    }
}
