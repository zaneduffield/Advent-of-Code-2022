use std::{collections::VecDeque, hash::BuildHasherDefault};

use itertools::Itertools;
use rustc_hash::FxHashSet;

pub type Cubes = Vec<(usize, usize, usize)>;

pub struct Input {
    cubes: Cubes,
    data: Vec<Block>,
    width: usize,
    height: usize,
    depth: usize,
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Input {
    Input::new(
        &input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse().expect("failed to parse as int"))
                    .collect_tuple()
                    .expect("failed to parse line as cube (x,y,z)")
            })
            .collect(),
    )
}

#[derive(Clone, Copy)]
enum Block {
    Empty,
    Full,
}

impl Input {
    fn new(cubes: &Cubes) -> Self {
        let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
        let (mut min_x, mut min_y, mut min_z) = (usize::MAX, usize::MAX, usize::MAX);
        cubes.iter().for_each(|&(x, y, z)| {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            min_z = min_z.min(z);

            max_x = max_x.max(x);
            max_y = max_y.max(y);
            max_z = max_z.max(z);
        });

        // We will rescale so all the mins become 1, and the maxes become (max - min + 1).
        // We also add a one-block buffer around all cubes, so that a DFS can count the side
        // of a block that is up against the edge of the grid.
        let width = max_x - min_x + 3;
        let height = max_y - min_y + 3;
        let depth = max_z - min_z + 3;

        let data = vec![Block::Empty; width * height * depth];
        let cubes: Cubes = cubes
            .iter()
            .map(|&(x, y, z)| (x - min_x + 1, y - min_y + 1, z - min_z + 1))
            .collect();

        let mut grid = Self {
            width,
            height,
            depth,
            data,
            cubes: vec![],
        };

        cubes
            .iter()
            .for_each(|&p| *grid.get_mut(p).unwrap() = Block::Full);
        grid.cubes = cubes;

        grid
    }

    fn idx(&self, (x, y, z): (usize, usize, usize)) -> usize {
        z * self.width * self.height + y * self.width + x
    }

    fn out_of_bounds(&self, (x, y, _z): (usize, usize, usize)) -> bool {
        x >= self.width || y >= self.height
    }

    fn get(&self, p: (usize, usize, usize)) -> Option<&Block> {
        if self.out_of_bounds(p) {
            None
        } else {
            self.data.get(self.idx(p))
        }
    }

    fn get_mut(&mut self, p: (usize, usize, usize)) -> Option<&mut Block> {
        if self.out_of_bounds(p) {
            None
        } else {
            let idx = self.idx(p);
            self.data.get_mut(idx)
        }
    }
}

fn nbours((x, y, z): (usize, usize, usize)) -> [Option<(usize, usize, usize)>; 6] {
    let mut out = [None; 6];
    let mut idx = 0;
    if x >= 1 {
        out[idx] = Some((x - 1, y, z));
        idx += 1;
    }
    out[idx] = Some((x + 1, y, z));
    idx += 1;

    if y >= 1 {
        out[idx] = Some((x, y - 1, z));
        idx += 1;
    }
    out[idx] = Some((x, y + 1, z));
    idx += 1;

    if z >= 1 {
        out[idx] = Some((x, y, z - 1));
        idx += 1;
    }
    out[idx] = Some((x, y, z + 1));

    out
}

fn nbours_saturating((x, y, z): (usize, usize, usize)) -> [(usize, usize, usize); 6] {
    [
        (x.saturating_sub(1), y, z),
        (x + 1, y, z),
        (x, y.saturating_sub(1), z),
        (x, y + 1, z),
        (x, y, z.saturating_sub(1)),
        (x, y, z + 1),
    ]
}

fn count_visible_sides(input: &Input) -> usize {
    input.cubes.iter().fold(0, |count, p| {
        count
            + nbours(*p)
                .iter()
                .flatten()
                .map(|p| input.get(*p))
                .filter(|b| matches!(b, Some(Block::Empty) | None))
                .count()
    })
}

#[aoc(day18, part1)]
pub fn part_1(input: &Input) -> usize {
    count_visible_sides(input)
}

fn count_reachable_sides(input: &Input) -> usize {
    let grid_size = input.width * input.height * input.depth;
    let mut visited = FxHashSet::with_capacity_and_hasher(grid_size, BuildHasherDefault::default());

    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize, 0usize));

    let mut count = 0;
    while let Some(p) = queue.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        for nb in nbours_saturating(p) {
            match input.get(nb) {
                Some(Block::Full) => count += 1,
                Some(Block::Empty) => queue.push_back(nb),
                None => {}
            }
        }
    }
    count
}

#[aoc(day18, part2)]
pub fn part_2(input: &Input) -> usize {
    count_reachable_sides(input)
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
        assert_eq!(part_2(&input), 58);
    }
}
