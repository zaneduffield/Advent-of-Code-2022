use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

pub struct Input {
    start: Pos,
    end: Pos,
    grid: Vec<u8>,
    width: i32,
    height: i32,
}

impl Input {
    fn get_elev(&self, Pos { x, y }: Pos) -> Option<u8> {
        if x < 0 || y < 0 || x >= self.width {
            None
        } else {
            self.grid.get((y * self.width + x) as usize).copied()
        }
    }

    fn nbours(&self, pos: Pos) -> [Option<Pos>; 4] {
        let mut out = [None; 4];
        if let Some(elv) = self.get_elev(pos) {
            let max_elv = elv + 1;
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .enumerate()
                .for_each(|(i, (dx, dy))| {
                    let nb = Pos {
                        x: pos.x + dx,
                        y: pos.y + dy,
                    };
                    match self.get_elev(nb) {
                        Some(n_elv) if n_elv <= max_elv => out[i] = Some(nb),
                        _ => {}
                    }
                })
        }
        out
    }

    fn heuristic(&self, Pos { x, y }: Pos) -> u32 {
        x.abs_diff(self.end.x) + y.abs_diff(self.end.y)
    }

    fn astar_min_cost(&self, start: Pos) -> Option<u32> {
        let mut min_costs = FxHashMap::default();
        let mut min_heap = BinaryHeap::new();
        min_heap.push((Reverse(self.heuristic(start)), start, 0));

        loop {
            let (_, next, cost) = min_heap.pop()?;
            if next == self.end {
                return Some(cost);
            }
            let n_cost = cost + 1;
            for n_pos in self.nbours(next).into_iter().flatten() {
                if n_cost < *min_costs.get(&n_pos).unwrap_or(&u32::MAX) {
                    min_costs.insert(n_pos, n_cost);
                    min_heap.push((Reverse(n_cost + self.heuristic(n_pos)), n_pos, n_cost));
                }
            }
        }
    }
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

    let mut grid = vec![];
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height += 1;
        grid.extend(line.bytes().enumerate().map(|(x, b)| match b {
            b'S' => {
                start = Some((x, y).into());
                b'a'
            }
            b'E' => {
                end = Some((x, y).into());
                b'z'
            }
            _ => b,
        }));
    }

    Input {
        start: start.expect("no start found"),
        end: end.expect("no end found"),
        grid,
        width: width as i32,
        height,
    }
}

#[aoc(day12, part1)]
pub fn part_1(input: &Input) -> u32 {
    input.astar_min_cost(input.start).expect("no path found")
}

#[aoc(day12, part2)]
pub fn part_2(input: &Input) -> u32 {
    let starts = (0..input.height)
        .flat_map(|y| {
            (0..input.width)
                .map(move |x| Pos { x, y })
                .filter(|&p| input.get_elev(p) == Some(b'a'))
        })
        .collect_vec();

    starts
        .into_par_iter()
        .flat_map(|p| input.astar_min_cost(p))
        .min()
        .expect("no paths found")
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
}
