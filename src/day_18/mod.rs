use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashSet;

pub type Cubes = Vec<(usize, usize, usize)>;

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

#[derive(Clone, Copy)]
enum Block {
    Empty,
    Full,
}

struct Grid {
    data: Vec<Block>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        let data = vec![Block::Empty; width * height * depth];
        Self {
            data,
            width,
            height,
        }
    }

    fn idx(&self, (x, y, z): (usize, usize, usize)) -> usize {
        z * self.width * self.height + y * self.width + x
    }

    fn out_of_bounds(&self, (x, y, z): (usize, usize, usize)) -> bool {
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

impl From<&Cubes> for Grid {
    fn from(cubes: &Cubes) -> Self {
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
        let mut grid = Self::new(
            (max_x - min_x + 3) as usize,
            (max_y - min_y + 3) as usize,
            (max_z - min_z + 3) as usize,
        );
        cubes.iter().for_each(|&(x, y, z)| {
            *grid
                .get_mut((x - min_x + 1, y - min_y + 1, z - min_z + 1))
                .unwrap() = Block::Full
        });
        grid
    }
}

fn nbours((x, y, z): (usize, usize, usize)) -> [Option<(usize, usize, usize)>; 6] {
    let mut out = [None; 6];
    let mut len = 0;
    if x >= 1 {
        out[len] = Some((x - 1, y, z));
        len += 1;
    }
    out[len] = Some((x + 1, y, z));
    len += 1;
    if y >= 1 {
        out[len] = Some((x, y - 1, z));
        len += 1;
    }
    out[len] = Some((x, y + 1, z));
    len += 1;
    if z >= 1 {
        out[len] = Some((x, y, z - 1));
        len += 1;
    }
    out[len] = Some((x, y, z + 1));

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

fn count_visible_sides(cubes: &Cubes) -> usize {
    let grid = Grid::from(cubes);

    cubes.iter().fold(0, |count, p| {
        count
            + nbours(*p)
                .iter()
                .flatten()
                .map(|p| grid.get(*p))
                .filter(|b| matches!(b, Some(Block::Empty) | None))
                .count()
    })
}

#[aoc(day18, part1)]
pub fn part_1(input: &Input) -> usize {
    count_visible_sides(&input.cubes)
}

fn count_reachable_sides(cubes: &Cubes) -> usize {
    let grid = Grid::from(cubes);

    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize, 0usize));

    let mut count = 0;
    while let Some(p) = queue.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        for nb in nbours_saturating(p) {
            match grid.get(nb) {
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
    count_reachable_sides(&input.cubes)
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
