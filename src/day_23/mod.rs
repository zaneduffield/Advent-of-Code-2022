use std::fmt::{Debug, Write};

#[derive(Clone, Copy)]
pub enum Tile {
    Elf,
    Empty,
}

#[derive(Clone, Copy)]
pub enum TargetState {
    Available,
    Targeted((usize, usize)),
    Overtargeted,
}

type Step = [(isize, isize); 3];

const INIT_DIFFS: [Step; 4] = [
    [(0, -1), (-1, -1), (1, -1)], // north
    [(0, 1), (-1, 1), (1, 1)],    // south
    [(-1, 0), (-1, -1), (-1, 1)], // west
    [(1, 0), (1, -1), (1, 1)],    // east
];

#[derive(Clone)]
pub struct Input {
    grid: Vec<Tile>,
    target_states: Vec<TargetState>,
    width: usize,
    height: usize,
    diffs: [Step; 4],
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Input {
    let mut grid = vec![];
    let mut height = 0;
    for line in input.lines() {
        height += 1;
        grid.extend(
            line.chars()
                .map(|c| if c == '#' { Tile::Elf } else { Tile::Empty }),
        );
    }

    let width = input.lines().next().unwrap().len();

    Input::new(grid, width, height)
}

impl Input {
    fn new(grid: Vec<Tile>, width: usize, height: usize) -> Input {
        Input {
            grid,
            target_states: vec![],
            width,
            height,
            diffs: INIT_DIFFS,
        }
    }

    fn idx(&self, y: usize, x: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        self.grid.get(self.idx(y, x))
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Tile> {
        let idx = self.idx(y, x);
        self.grid.get_mut(idx)
    }

    fn get_target(&self, (x, y): (usize, usize)) -> Option<&TargetState> {
        self.target_states.get(self.idx(y, x))
    }

    fn get_target_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut TargetState> {
        let idx = self.idx(y, x);
        self.target_states.get_mut(idx)
    }

    fn occupied(&self, (x, y): (usize, usize)) -> bool {
        matches!(self.get((x, y)), Some(&Tile::Elf))
    }

    fn available(&self, (x, y): (usize, usize)) -> bool {
        matches!(self.get((x, y)), Some(&Tile::Empty))
    }

    fn grow(&mut self) {
        let new_width = self.width * 3;
        let new_height = self.height * 3;

        let mut new_grid = Vec::with_capacity(new_height * new_width);
        new_grid.extend(std::iter::repeat(Tile::Empty).take(new_width * self.height));
        for y in 0..self.height {
            new_grid.extend(std::iter::repeat(Tile::Empty).take(self.width));
            new_grid.extend(self.grid.iter().skip(y * self.width).take(self.width));
            new_grid.extend(std::iter::repeat(Tile::Empty).take(self.width));
        }
        new_grid.extend(std::iter::repeat(Tile::Empty).take(new_width * self.height));

        self.grid = new_grid;
        self.width = new_width;
        self.height = new_height;
    }

    fn grow_if_perimeter_occupied(&mut self) {
        if std::iter::empty()
            .chain((0..self.width).map(|x| (x, 0)))
            .chain((0..self.width).map(|x| (x, self.height - 1)))
            .chain((0..self.height).map(|y| (0, y)))
            .chain((0..self.height).map(|y| (self.width - 1, y)))
            .any(|pos| self.occupied(pos))
        {
            self.grow();
        }
    }

    fn candidate_move(&self, pos: (usize, usize)) -> Option<[(isize, isize); 3]> {
        let mut mov = None;
        let mut bad_move_found = false;
        for d in self.diffs {
            if d.iter().all(|(dx, dy)| {
                self.available((
                    pos.0.wrapping_add_signed(*dx),
                    pos.1.wrapping_add_signed(*dy),
                ))
            }) {
                mov = mov.or(Some(d))
            } else {
                bad_move_found = true;
            }
        }

        if bad_move_found {
            mov
        } else {
            None
        }
    }

    fn execute_round(&mut self) -> bool {
        self.grow_if_perimeter_occupied();

        #[cfg(debug_assertions)]
        dbg!(&self);

        self.target_states = vec![TargetState::Available; self.grid.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                if self.occupied((x, y)) {
                    if let Some(diff) = self.candidate_move((x, y)) {
                        let (dx, dy) = diff[0];
                        let target = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                        let target_state = self.get_target_mut(target).unwrap();
                        match target_state {
                            TargetState::Available => *target_state = TargetState::Targeted((x, y)),
                            TargetState::Targeted(_) => *target_state = TargetState::Overtargeted,
                            TargetState::Overtargeted => {}
                        }
                    }
                }
            }
        }

        let mut moved = false;
        for y in 0..self.height {
            for x in 0..self.width {
                let to = (x, y);
                if let Some(TargetState::Targeted(from)) = self.get_target(to).cloned() {
                    *self.get_mut(to).unwrap() = Tile::Elf;
                    *self.get_mut(from).unwrap() = Tile::Empty;
                    moved = true;
                }
            }
        }

        self.diffs.rotate_left(1);

        moved
    }

    fn num_empty_ground_tiles(&self) -> usize {
        let (mut maxx, mut maxy) = (0, 0);
        let (mut minx, mut miny) = (usize::MAX, usize::MAX);
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.occupied((x, y)) {
                    count += 1;

                    maxx = maxx.max(x);
                    maxy = maxy.max(y);
                    minx = minx.min(x);
                    miny = miny.min(y);
                }
            }
        }

        (maxx - minx + 1) * (maxy - miny + 1) - count
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(match self.get((x, y)).unwrap() {
                    Tile::Elf => '#',
                    Tile::Empty => '.',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[aoc(day23, part1)]
pub fn part_1(input: &Input) -> usize {
    let mut input = input.clone();
    for _ in 0..10 {
        input.execute_round();
    }
    input.num_empty_ground_tiles()
}

#[aoc(day23, part2)]
pub fn part_2(input: &Input) -> usize {
    let mut input = input.clone();
    (1..).find(|_| !input.execute_round()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            ....#..
            ..###.#
            #...#.#
            .#...##
            #.###..
            ##.#.##
            .#..#..
            "
        });
        assert_eq!(part_1(&input), 110);
        assert_eq!(part_2(&input), 20);
    }
}
