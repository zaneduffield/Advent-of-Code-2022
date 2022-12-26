use std::{
    collections::VecDeque,
    hash::{Hash, Hasher},
};

use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

pub enum Wind {
    Left,
    Right,
}

pub type Input = Vec<Wind>;

pub struct Shape {
    height: isize,
    offsets: Vec<(isize, isize)>,
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Input {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Wind::Left,
            '>' => Wind::Right,
            _ => panic!("unexpected char: {c}"),
        })
        .collect()
}

const SHAPES: [&[(isize, isize)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
];

const WIDTH: usize = 7;
const INIT_X: isize = 2;
const INIT_Y_BUFF: isize = 3;

fn shapes() -> [Shape; 5] {
    SHAPES.map(|s| Shape {
        offsets: s.iter().copied().collect(),
        height: 1 + *s.iter().map(|(_, dy)| dy).max().unwrap_or(&0),
    })
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Block {
    Empty,
    Filled,
}

struct Cave<'a> {
    wind: &'a Input,
    wind_idx: usize,
    blocks: Vec<Block>,
    width: isize,
    height: isize,
}

impl<'a> Cave<'a> {
    fn new(input: &'a Input) -> Cave<'a> {
        Cave {
            wind: input,
            wind_idx: 0,
            blocks: vec![],
            width: WIDTH as isize,
            height: 0,
        }
    }

    fn idx(&self, (x, y): (isize, isize)) -> usize {
        (y * self.width + x) as usize
    }

    fn get(&self, (x, y): (isize, isize)) -> Block {
        if x < 0 || x >= self.width || y < 0 {
            Block::Filled
        } else {
            *self.blocks.get(self.idx((x, y))).unwrap_or(&Block::Filled)
        }
    }

    fn set(&mut self, (x, y): (isize, isize), new: Block) -> bool {
        if x < 0 || x >= self.width || y < 0 {
            false
        } else {
            let idx = self.idx((x, y));
            if let Some(b) = self.blocks.get_mut(idx) {
                Some(*b = new);
                true
            } else {
                false
            }
        }
    }

    fn place_at(&mut self, shape: &Shape, (px, py): (isize, isize)) {
        shape.offsets.iter().for_each(|(ox, oy)| {
            self.set((px + *ox, py + *oy), Block::Filled);
        })
    }

    fn can_fit_at(&self, shape: &Shape, (px, py): (isize, isize)) -> bool {
        shape
            .offsets
            .iter()
            .all(|(ox, oy)| self.get((px + *ox, py + *oy)) == Block::Empty)
    }

    fn fall(&mut self, shape: &Shape) {
        let required_height = self.height + INIT_Y_BUFF + shape.height;
        let blocks_to_add =
            (required_height * self.width).saturating_sub_unsigned(self.blocks.len());
        self.blocks.extend((0..blocks_to_add).map(|_| Block::Empty));

        let mut pos = (INIT_X, self.height + INIT_Y_BUFF);
        loop {
            // wind
            let wind_pos = match self.wind.get(self.wind_idx).unwrap() {
                Wind::Left => (pos.0 - 1, pos.1),
                Wind::Right => (pos.0 + 1, pos.1),
            };
            self.wind_idx = (self.wind_idx + 1) % self.wind.len();

            if self.can_fit_at(shape, wind_pos) {
                pos = wind_pos
            }
            // fall
            let fall_pos = (pos.0, pos.1 - 1);
            if self.can_fit_at(shape, fall_pos) {
                pos = fall_pos;
            } else {
                break;
            }
        }

        let shape_top = pos.1 + shape.height;
        self.height = self.height.max(shape_top);
        self.place_at(shape, pos);
    }

    fn hash_rock_state(&self) -> u64 {
        let mut hash_state = FxHasher::default();
        let mut visited = FxHashSet::default();

        let mut queue = VecDeque::new();
        queue.push_back((0, self.height));

        while let Some(pos) = queue.pop_front() {
            if !visited.insert(pos) {
                continue;
            }

            (pos.0, self.height - pos.1).hash(&mut hash_state);
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let nb = (pos.0 + dx, pos.1 + dy);
                if self.get(nb) == Block::Empty {
                    queue.push_back(nb);
                }
            }
        }

        hash_state.finish()
    }
}

fn solve(input: &Input, shapes_to_fall: usize) -> usize {
    let mut cave = Cave::new(input);
    let shapes = shapes();

    let mut shape_num_by_state = FxHashMap::default();

    let mut heights = vec![];

    for shape_num in 0..shapes_to_fall {
        let shape_idx = shape_num % shapes.len();

        heights.push(cave.height);

        let current_state = (cave.hash_rock_state(), cave.wind_idx, shape_idx);
        if let Some(last_shape_num) = shape_num_by_state.insert(current_state, shape_num) {
            // Bingo! Now we can just repeat the cycle.
            let remaining_shapes = shapes_to_fall - shape_num;
            let shapes_in_cycle = shape_num - last_shape_num;
            let cycles_to_skip = remaining_shapes / shapes_in_cycle;
            let remainder_cycles = remaining_shapes % shapes_in_cycle;

            let cycle_start_height = heights[last_shape_num];
            let cycle_height = (cave.height - cycle_start_height) as usize;
            let remainder_height =
                (heights[last_shape_num + remainder_cycles] - cycle_start_height) as usize;

            return cave.height as usize + (cycles_to_skip * cycle_height) + remainder_height;
        }

        let shape = &shapes[shape_idx];
        cave.fall(shape);
    }

    cave.height as usize
}

#[aoc(day17, part1)]
pub fn part_1(input: &Input) -> usize {
    solve(input, 2022)
}

#[aoc(day17, part2)]
pub fn part_2(input: &Input) -> usize {
    solve(input, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part_1(&input), 3068);
        assert_eq!(part_2(&input), 1514285714288);

        let my_input = input_generator(include_str!("../../input/2022/day17.txt"));
        assert_eq!(part_1(&my_input), 3227);
        assert_eq!(part_2(&my_input), 1597714285698);
    }
}
