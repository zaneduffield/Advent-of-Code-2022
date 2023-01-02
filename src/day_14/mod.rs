use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Air,
    Sand,
    Rock,
}

#[derive(Clone)]
pub struct Cave {
    data: Vec<Block>,
    width: usize,
    height: usize,
    full: bool,
}

#[derive(Clone)]
pub struct Input {
    cave: Cave,
    seed_pos: (usize, usize),
}

impl Cave {
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Block> {
        self.data.get_mut(y * self.width + x)
    }
}

const FLOOR_BUF: usize = 2;
const SEED_X_POS: usize = 500;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let rock_paths: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point
                        .split(',')
                        .map(|i| i.parse::<usize>().expect("could not parse as int"))
                        .collect_tuple()
                        .expect("coordinates much be 2 comma-delimited ints");
                    (x, y)
                })
                .collect()
        })
        .collect();

    let max_y = rock_paths
        .iter()
        .flat_map(|line| line.iter())
        .map(|p| p.1)
        .max()
        .unwrap_or(0)
        + FLOOR_BUF;
    let min_y = 0;

    let min_x = SEED_X_POS - max_y;
    let max_x = SEED_X_POS + max_y;

    let (width, height) = (max_x - min_x + 1, max_y - min_y + 1);
    let size = width * height;
    let mut cave = Cave {
        data: vec![Block::Air; size],
        width,
        height,
        full: false,
    };

    let convert_coords = |(x, y)| (x - min_x, y - min_y);
    let mut make_rock = |p| *cave.get_mut(p).unwrap() = Block::Rock;

    for path in rock_paths {
        for seg in path.windows(2) {
            let from = seg[0];
            let to = seg[1];
            if from.0 == to.0 {
                let (from_y, to_y) = (from.1.min(to.1), from.1.max(to.1));
                (from_y..=to_y)
                    .map(|y| (from.0, y))
                    .map(convert_coords)
                    .for_each(&mut make_rock);
            } else {
                let (from_x, to_x) = (from.0.min(to.0), from.0.max(to.0));
                (from_x..=to_x)
                    .map(|x| (x, from.1))
                    .map(convert_coords)
                    .for_each(&mut make_rock);
            };
        }
    }

    Input {
        cave,
        seed_pos: (SEED_X_POS - min_x, 0),
    }
}

impl Input {
    fn fall_to(&mut self, p: &mut (usize, usize), offset: (isize, isize)) -> bool {
        let old_pos = *p;
        p.0.checked_add_signed(offset.0)
            .zip(p.1.checked_add_signed(offset.1))
            .and_then(|new_pos| match self.cave.get_mut(new_pos)? {
                x if *x == Block::Air => {
                    *x = Block::Sand;
                    *p = new_pos;
                    Some(true)
                }
                _ => Some(false),
            })
            .map(|moved| {
                if moved {
                    if let Some(b) = self.cave.get_mut(old_pos) {
                        *b = Block::Air;
                    }
                }
                moved
            })
            .or_else(|| {
                self.cave.full = true;
                Some(false)
            })
            .unwrap_or(false)
    }

    fn fall(&mut self, p: &mut (usize, usize)) -> bool {
        self.fall_to(p, (0, 1)) || self.fall_to(p, (-1, 1)) || self.fall_to(p, (1, 1))
    }

    fn seed(&mut self) {
        let mut pos = self.seed_pos;
        match self.cave.get_mut(pos) {
            Some(x) if x == &Block::Air => *x = Block::Sand,
            _ => {
                self.cave.full = true;
                return;
            }
        }

        while self.fall(&mut pos) {}
    }

    fn flood(&mut self) -> usize {
        (0..)
            .take_while(|_| {
                self.seed();
                !self.cave.full
            })
            .count()
    }
}

#[aoc(day14, part1)]
pub fn part_1(input: &Input) -> usize {
    input.clone().flood()
}

#[aoc(day14, part2)]
pub fn part_2(input: &Input) -> usize {
    let mut input = input.clone();
    for x in 0..input.cave.width {
        *input.cave.get_mut((x, input.cave.height - 1)).unwrap() = Block::Rock;
    }
    input.clone().flood()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            "
        });
        assert_eq!(part_1(&input), 24);
        assert_eq!(part_2(&input), 93);
    }
}
