use arrayvec::ArrayVec;

pub enum Wind {
    Left,
    Right,
}

pub type Input = Vec<Wind>;

pub struct Shape {
    height: isize,
    offsets: ArrayVec<(isize, isize), 5>,
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Input {
    input
        .chars()
        .map(|c| match c {
            '<' => Wind::Left,
            '>' => Wind::Right,
            _ => panic!("unexpected char: {c}"),
        })
        .collect()
}

const SHAPES: [(isize, &[(isize, isize)]); 5] = [
    (1, &[(0, 0), (1, 0), (2, 0), (3, 0)]),
    (3, &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
    (3, &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
    (4, &[(0, 0), (0, 1), (0, 2), (0, 3)]),
    (2, &[(0, 0), (1, 0), (0, 1), (1, 1)]),
];

const WIDTH: usize = 7;
const INIT_X: isize = 2;
const INIT_Y_BUFF: isize = 3;

fn shapes() -> [Shape; 5] {
    SHAPES.map(|(height, s)| Shape {
        offsets: ArrayVec::from_iter(s.to_owned()),
        height,
    })
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Block {
    Empty,
    Filled,
}

struct Cave {
    wind: Input,
    wind_idx: usize,
    blocks: Vec<Block>,
    width: isize,
    height: isize,
}

impl Cave {
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

    fn can_fit_at(&self, shape: &Shape, (px, py): (isize, isize)) -> bool {
        shape
            .offsets
            .iter()
            .all(|(ox, oy)| self.get((px + *ox, py + *oy)) == Block::Empty)
    }

    fn fall(&mut self, shape: &Shape) {
        let mut pos = (INIT_X, self.height + INIT_Y_BUFF);
        let mut last_pos = (0, 0);
        while last_pos != pos {
            last_pos = pos;

            // wind
            let wind_pos = match self.wind.get(self.wind_idx).unwrap() {
                Wind::Left => (pos.0 + -1, pos.1),
                Wind::Right => (pos.0 - 1, pos.1),
            };

            if self.can_fit_at(shape, wind_pos) {
                pos = wind_pos
            }
            // fall
            let fall_pos = (pos.0, pos.1 - 1);
            if self.can_fit_at(shape, fall_pos) {
                pos = fall_pos;
            }
        }
    }
}

#[aoc(day17, part1)]
pub fn part_1(input: &Input) -> usize {
    SHAPES.len() as usize
}

#[aoc(day17, part2)]
pub fn part_2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(part_1(&input), 3068);
        // assert_eq!(part_2(&input),);
    }
}
