#[cfg(debug_assertions)]
use std::fmt::Debug;

use std::ops::{Index, IndexMut};

use nom::character::complete::*;
use nom::combinator::opt;
use nom::multi::many1;
use num::integer::gcd;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Floor,
    Wall,
}

enum Rotation {
    Left,
    Right,
}

enum Instruction {
    Len(u16),
    Turn(Rotation),
}

#[derive(Copy, Clone, EnumIter, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn step(self, pos: Pos) -> Pos {
        let (dx, dy) = match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy))
    }

    fn turn(self, rotation: &Rotation) -> Self {
        match rotation {
            Rotation::Left => match self {
                Direction::North => Self::West,
                Direction::East => Self::North,
                Direction::South => Self::East,
                Direction::West => Self::South,
            },
            Rotation::Right => match self {
                Direction::North => Self::East,
                Direction::East => Self::South,
                Direction::South => Self::West,
                Direction::West => Self::North,
            },
        }
    }

    fn turn_left(self) -> Self {
        self.turn(&Rotation::Left)
    }

    fn turn_right(self) -> Self {
        self.turn(&Rotation::Right)
    }

    fn val(self) -> usize {
        match self {
            Direction::North => 3,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
        }
    }
}

type Pos = (usize, usize);

#[derive(Clone)]
struct PosDir {
    pos: Pos,
    dir: Direction,
}

impl PosDir {
    fn password(&self) -> usize {
        1000 * (self.pos.1 + 1) + 4 * (self.pos.0 + 1) + self.dir.val()
    }
}

pub struct Input {
    len: usize,
    width: usize,
    height: usize,
    data: Vec<Tile>,
    instructions: Vec<Instruction>,
}

impl Input {
    fn idx(&self, (x, y): Pos) -> usize {
        y * self.width + x
    }

    fn get(&self, (x, y): Pos) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.data.get(self.idx((x, y)))
        }
    }

    fn start_pos(&self) -> PosDir {
        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.get((x, y)), Some(&Tile::Floor)) {
                    return PosDir {
                        pos: (x, y),
                        dir: Direction::East,
                    };
                }
            }
        }

        panic!("no tiles found in grid")
    }
}

#[cfg(debug_assertions)]
struct DebugState<'a> {
    input: &'a Input,
    pos: &'a [PosDir],
}

#[cfg(debug_assertions)]
impl Debug for DebugState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("\n");
        for y in 0..self.input.height {
            for x in 0..self.input.width {
                if let Some(PosDir { dir, .. }) = self.pos.iter().find(|p| p.pos == (x, y)) {
                    s.push(match dir {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'v',
                        Direction::West => '<',
                    });
                } else {
                    s.push(match self.input.get((x, y)) {
                        Some(&Tile::Empty) | None => ' ',
                        Some(&Tile::Floor) => '.',
                        Some(&Tile::Wall) => '#',
                    })
                }
            }
            s.push('\n');
        }

        f.write_str(&s)
    }
}

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

fn parse_instruction(input: &str) -> IResult<Instruction> {
    let (input, maybe_n) = opt(u16)(input)?;
    if let Some(n) = maybe_n {
        return Ok((input, Instruction::Len(n)));
    }

    match one_of("LR")(input)? {
        (input, 'L') => Ok((input, Instruction::Turn(Rotation::Left))),
        (input, 'R') => Ok((input, Instruction::Turn(Rotation::Right))),
        _ => unreachable!(),
    }
}

pub fn parse_input(mut input: &str) -> IResult<Input> {
    let (width, height): Pos = {
        let (mut width, mut height) = (0, 0);
        for len in input
            .lines()
            .map(|line| line.len())
            .take_while(|len| *len > 0)
        {
            height += 1;
            width = width.max(len)
        }
        (width, height)
    };

    let mut data: Vec<Tile> = vec![Tile::Empty; width * height];
    let mut instructions = vec![];

    let mut i = 0;
    let mut empty_found = false;
    for line in input.lines() {
        if line.is_empty() {
            empty_found = true;
            continue;
        }
        if empty_found {
            (input, instructions) = many1(parse_instruction)(line)?;
            break;
        }
        for c in line
            .bytes()
            .chain(std::iter::repeat(b' ').take(width - line.len()))
        {
            data[i] = match c {
                b' ' => Tile::Empty,
                b'.' => Tile::Floor,
                b'#' => Tile::Wall,
                _ => panic!("Unexpected char in grid: {c}"),
            };
            i += 1;
        }
    }

    let len = gcd(width, height);

    Ok((
        input,
        Input {
            len,
            data,
            width,
            height,
            instructions,
        },
    ))
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Input {
    parse_input(input).unwrap().1
}

fn walk<F, T>(input: &Input, wrap_pos_fn: F, len: u16, data: &T, mut pos: PosDir) -> PosDir
where
    F: Fn(&PosDir, &Input, &T) -> PosDir,
{
    for _ in 0..len {
        let new_pos = pos.dir.step(pos.pos);
        match input.get(new_pos) {
            Some(&Tile::Empty) | None => {
                let wrapped_pos = wrap_pos_fn(&pos, input, data);

                // if we would wrap around to a wall, stop
                if matches!(input.get(wrapped_pos.pos), Some(&Tile::Wall)) {
                    break;
                }
                pos = wrapped_pos;
            }
            Some(&Tile::Floor) => {
                pos.pos = new_pos;
            }
            Some(&Tile::Wall) => {
                break;
            }
        }
    }

    pos
}

fn step<F, T>(
    input: &Input,
    wrap_pos_fn: F,
    data: &T,
    instruction: &Instruction,
    pos: PosDir,
) -> PosDir
where
    F: Fn(&PosDir, &Input, &T) -> PosDir,
{
    #[cfg(debug_assertions)]
    dbg!(DebugState {
        input,
        pos: &[pos.clone()]
    });

    match instruction {
        Instruction::Len(len) => walk(input, wrap_pos_fn, *len, data, pos),
        Instruction::Turn(r) => PosDir {
            dir: pos.dir.turn(r),
            ..pos
        },
    }
}

fn wrap_part_1(pos: &PosDir, input: &Input, _: &()) -> PosDir {
    let mut search = pos.pos;
    let opposite_dir = pos.dir.opposite();
    // search the other direction until you find the end
    loop {
        search = opposite_dir.step(search);
        if matches!(input.get(search), None | Some(&Tile::Empty)) {
            return PosDir {
                pos: search,
                dir: pos.dir,
            };
        }
    }
}

#[aoc(day22, part1)]
pub fn part_1(input: &Input) -> usize {
    input
        .instructions
        .iter()
        .fold(input.start_pos(), |p, i| {
            step(input, wrap_part_1, &(), i, p)
        })
        .password()
}

#[derive(Clone)]
struct Face {
    pos: Pos,
    north: Option<PosDir>,
    east: Option<PosDir>,
    south: Option<PosDir>,
    west: Option<PosDir>,
}

impl Index<Direction> for Face {
    type Output = Option<PosDir>;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::South => &self.south,
            Direction::West => &self.west,
        }
    }
}

impl IndexMut<Direction> for Face {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::North => &mut self.north,
            Direction::East => &mut self.east,
            Direction::South => &mut self.south,
            Direction::West => &mut self.west,
        }
    }
}

impl Face {
    fn new(pos: Pos) -> Self {
        Self {
            pos,
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }

    fn filled(&self) -> bool {
        self.north.is_some() & self.east.is_some() & self.south.is_some() & self.west.is_some()
    }
}

struct Net {
    faces: Vec<Face>,
}

fn make_net(input: &Input) -> Net {
    let mut faces = vec![];

    let mut y = 0;
    while y < input.height {
        let mut x = 0;
        while x < input.width {
            if matches!(input.get((x, y)), Some(Tile::Wall) | Some(Tile::Floor)) {
                faces.push(Face::new((x, y)));
            }
            x += input.len;
        }
        y += input.len;
    }

    Net { faces }
}

fn fill_neighbours(input: &Input, net: &mut Net) {
    let last_faces = net.faces.clone();

    for face in &mut net.faces {
        for other in &last_faces {
            let dx = other.pos.0.abs_diff(face.pos.0);
            let dy = other.pos.1.abs_diff(face.pos.1);
            if (dx == input.len) & (dy == 0) {
                let dir = if other.pos.0 > face.pos.0 {
                    Direction::East
                } else {
                    Direction::West
                };
                face[dir] = Some(PosDir {
                    pos: other.pos,
                    dir,
                });
            } else if (dx == 0) & (dy == input.len) {
                let dir = if other.pos.1 > face.pos.1 {
                    Direction::South
                } else {
                    Direction::North
                };
                face[dir] = Some(PosDir {
                    pos: other.pos,
                    dir,
                });
            };
        }
    }

    while net.faces.iter().any(|f| !f.filled()) {
        let last_faces = net.faces.clone();
        for face in &last_faces {
            for dir in Direction::iter() {
                if let Some(nb2) = &face[dir] {
                    let d2 = dir.turn_right();
                    if let Some(nb3) = &face[d2] {
                        // we've found a corner where three faces meet, so we can fuse the sides
                        let face2 = net.faces.iter_mut().find(|f| f.pos == nb2.pos).unwrap();
                        face2[nb2.dir.turn_right()] = Some(PosDir {
                            pos: nb3.pos,
                            dir: nb3.dir.turn_right(),
                        });

                        let face3 = net.faces.iter_mut().find(|f| f.pos == nb3.pos).unwrap();
                        face3[nb3.dir.turn_left()] = Some(PosDir {
                            pos: nb2.pos,
                            dir: nb2.dir.turn_left(),
                        });
                    }
                }
            }
        }
    }
}

fn wrap_part_2(PosDir { pos, dir }: &PosDir, input: &Input, net: &Net) -> PosDir {
    let top_corner_pos = (
        (pos.0 / input.len) * input.len,
        (pos.1 / input.len) * input.len,
    );
    let face = net.faces.iter().find(|f| f.pos == top_corner_pos).unwrap();
    let nb_face = face[*dir].as_ref().unwrap();

    let offset = match dir {
        Direction::North => pos.0 - face.pos.0,
        Direction::East => pos.1 - face.pos.1,
        Direction::South => face.pos.0 + input.len - 1 - pos.0,
        Direction::West => face.pos.1 + input.len - 1 - pos.1,
    };

    let entrance_pos = match nb_face.dir {
        Direction::North => (nb_face.pos.0 + offset, nb_face.pos.1 + input.len - 1),
        Direction::East => (nb_face.pos.0, nb_face.pos.1 + offset),
        Direction::South => (nb_face.pos.0 + input.len - 1 - offset, nb_face.pos.1),
        Direction::West => (
            nb_face.pos.0 + input.len - 1,
            nb_face.pos.1 + input.len - 1 - offset,
        ),
    };

    #[cfg(debug_assertions)]
    dbg!(DebugState {
        input,
        pos: &[
            PosDir {
                pos: *pos,
                dir: *dir,
            },
            PosDir {
                pos: entrance_pos,
                dir: nb_face.dir,
            },
        ]
    });

    PosDir {
        pos: entrance_pos,
        dir: nb_face.dir,
    }
}

#[aoc(day22, part2)]
pub fn part_2(input: &Input) -> usize {
    let mut net = make_net(input);
    fill_neighbours(input, &mut net);
    input
        .instructions
        .iter()
        .fold(input.start_pos(), |p, i| {
            step(input, wrap_part_2, &net, i, p)
        })
        .password()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
                    ...#
                    .#..
                    #...
                    ....
            ...#.......#
            ........#...
            ..#....#....
            ..........#.
                    ...#....
                    .....#..
                    .#......
                    ......#.

            10R5L5R10L4R5L5
            "
        });
        assert_eq!(part_1(&input), 6032);
        assert_eq!(part_2(&input), 5031);
    }
}
