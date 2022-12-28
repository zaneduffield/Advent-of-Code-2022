use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashSet;

pub type Cubes = Vec<(i8, i8, i8)>;

pub struct Input {
    cubes: Cubes,
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Input {
    let cubes = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().expect("failed to parse as int"))
                .collect_tuple()
                .expect("failed to parse line as cube (x,y,z)")
        })
        .collect();
    Input { cubes }
}

struct Grid {
    data: Vec<bool>,
    width: usize,
    height: usize,
    depth: usize,
}

impl Grid {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        let data = vec![false; width * height * depth];
        Self {
            data,
            width,
            height,
            depth,
        }
    }

    fn idx(&self, (x, y, z): (i8, i8, i8)) -> usize {
        (z as usize * self.width * self.height + y as usize * self.width + x as usize) as usize
    }

    fn get(&self, p: (i8, i8, i8)) -> Option<&bool> {
        self.data.get(self.idx(p))
    }

    fn get_mut(&mut self, p: (i8, i8, i8)) -> Option<&mut bool> {
        let idx = self.idx(p);
        self.data.get_mut(idx)
    }
}

impl From<&Cubes> for Grid {
    fn from(cubes: &Cubes) -> Self {
        let (mut width, mut height, mut depth) = (0, 0, 0);
        cubes.iter().for_each(|(x, y, z)| {
            width = width.max(*x + 1);
            height = height.max(*y + 1);
            depth = depth.max(*z + 1);
        });

        let mut grid = Self::new(width as usize, height as usize, depth as usize);
        cubes.iter().for_each(|p| *grid.get_mut(*p).unwrap() = true);
        grid
    }
}

fn count_visible_sides(cubes: &Cubes) -> usize {
    let grid = Grid::from(cubes);

    cubes
        .iter()
        .map(|(x, y, z)| {
            [
                (-1, 0, 0),
                (1, 0, 0),
                (0, -1, 0),
                (0, 1, 0),
                (0, 0, -1),
                (0, 0, 1),
            ]
            .iter()
            .filter(|(dx, dy, dz)| matches!(grid.get((x + dx, y + dy, z + dz)), Some(false) | None))
            .count()
        })
        .sum()
}

#[aoc(day18, part1)]
pub fn part_1(input: &Input) -> usize {
    count_visible_sides(&input.cubes)
}

#[aoc(day18, part2)]
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
            2,2,2
            1,2,2
            3,2,2
            2,1,2
            2,3,2
            2,2,1
            2,2,3
            2,2,4
            2,2,6
            1,2,5
            3,2,5
            2,1,5
            2,3,5
            "
        });
        assert_eq!(part_1(&input), 64);
        // assert_eq!(part_2(&input), 58);
    }
}
