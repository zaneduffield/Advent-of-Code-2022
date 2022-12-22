use itertools::{Itertools, MinMaxResult};

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
    fn get(&self, (x, y): (usize, usize)) -> Option<&Block> {
        self.data.get(y * self.width + x)
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Block> {
        self.data.get_mut(y * self.width + x)
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let rock_paths: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point
                        .split(",")
                        .map(|i| i.parse::<usize>().expect("could not parse as int"))
                        .collect_tuple()
                        .expect("coordinates much be 2 comma-delimited ints");
                    (x, y)
                })
                .collect()
        })
        .collect();

    let (min_x, max_x) = match rock_paths
        .iter()
        .flat_map(|line| line.iter())
        .map(|p| p.0)
        .minmax()
    {
        MinMaxResult::NoElements => panic!("at least one rock line is expected"),
        MinMaxResult::OneElement(x) => (x, x),
        MinMaxResult::MinMax(min, max) => (min, max),
    };

    let max_y = rock_paths
        .iter()
        .flat_map(|line| line.iter())
        .map(|p| p.1)
        .max()
        .unwrap_or(0);
    let min_y = 0;

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
        seed_pos: (500 - min_x, 0),
    }
}

impl Input {
    fn fall_to(&mut self, p: &mut (usize, usize), offset: (isize, isize)) -> bool {
        let old_pos = *p;
        let new_x = match p.0.checked_add_signed(offset.0) {
            Some(x) => x,
            None => {
                self.cave.full = true;
                return false;
            }
        };
        let new_y = match p.1.checked_add_signed(offset.1) {
            Some(y) => y,
            None => {
                self.cave.full = true;
                return false;
            }
        };
        let new_pos = (new_x, new_y);
        let moved = match self.cave.get_mut(new_pos) {
            Some(x) if *x == Block::Air => {
                *x = Block::Sand;
                *p = new_pos;
                true
            }
            Some(_) => false,
            None => {
                self.cave.full = true;
                false
            }
        };
        if moved {
            if let Some(b) = self.cave.get_mut(old_pos) {
                *b = Block::Air;
            }
        }
        moved
    }

    fn fall(&mut self, p: &mut (usize, usize)) -> bool {
        self.fall_to(p, (0, 1)) || self.fall_to(p, (-1, 1)) || self.fall_to(p, (1, 1))
    }

    fn seed(&mut self) {
        let mut pos = self.seed_pos;
        if self.cave.get(pos) == Some(&Block::Sand) {
            self.cave.full = true;
            return;
        }

        while self.fall(&mut pos) {
            // do nothing
        }
    }
}

#[aoc(day14, part1)]
pub fn part_1(input: &Input) -> u32 {
    let mut input = input.clone();
    let mut count = 0;
    loop {
        input.seed();
        if input.cave.full {
            break;
        }
        count += 1;
    }
    count
}

#[aoc(day14, part2)]
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
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
            "
        });
        assert_eq!(part_1(&input), 24);
        // assert_eq!(part_2(&input), );
    }
}
