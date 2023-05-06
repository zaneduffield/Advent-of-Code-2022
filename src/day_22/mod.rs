use nom::character::complete::*;
use nom::combinator::opt;
use nom::multi::many1;

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

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
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

    fn val(self) -> usize {
        match self {
            Direction::North => 3,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
        }
    }
}

struct Pos {
    p: (usize, usize),
    dir: Direction,
}

impl Pos {
    fn password(&self) -> usize {
        1000 * (self.p.1 + 1) + 4 * (self.p.0 + 1) + self.dir.val()
    }
}

pub struct Input {
    width: usize,
    height: usize,
    data: Vec<Tile>,
    instructions: Vec<Instruction>,
}

impl Input {
    fn idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.data.get(self.idx((x, y)))
        }
    }

    fn start_pos(&self) -> Pos {
        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.get((x, y)), Some(&Tile::Floor)) {
                    return Pos {
                        p: (x, y),
                        dir: Direction::East,
                    };
                }
            }
        }

        panic!("no tiles found in grid")
    }

    fn show(&self, pos: &Pos) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(if (x, y) == pos.p {
                    match pos.dir {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'v',
                        Direction::West => '<',
                    }
                } else {
                    match self.get((x, y)) {
                        Some(&Tile::Empty) | None => ' ',
                        Some(&Tile::Floor) => '.',
                        Some(&Tile::Wall) => '#',
                    }
                })
            }
            s.push('\n');
        }
        s
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
    let (width, height): (usize, usize) = {
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

    Ok((
        input,
        Input {
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

fn walk(input: &Input, len: u16, mut pos: Pos) -> Pos {
    let (dx, dy) = match pos.dir {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
    };

    for _ in 0..len {
        let mut new_pos = (
            pos.p.0.wrapping_add_signed(dx),
            pos.p.1.wrapping_add_signed(dy),
        );
        match input.get(new_pos) {
            Some(&Tile::Empty) | None => {
                // search the other direction until you find the end
                let mut search = pos.p;
                loop {
                    search = (
                        search.0.wrapping_add_signed(-dx),
                        search.1.wrapping_add_signed(-dy),
                    );
                    if matches!(input.get(search), None | Some(&Tile::Empty)) {
                        break;
                    }
                    new_pos = search;
                }

                // if we would wrap around to a wall, stop
                if matches!(input.get(new_pos), Some(&Tile::Wall)) {
                    break;
                }
                pos.p = new_pos;
            }
            Some(&Tile::Floor) => {
                pos.p = new_pos;
            }
            Some(&Tile::Wall) => {
                break;
            }
        }
    }

    pos
}

fn step(input: &Input, instruction: &Instruction, pos: Pos) -> Pos {
    // println!("{}", input.show(&pos));
    match instruction {
        Instruction::Len(len) => walk(input, *len, pos),
        Instruction::Turn(r) => Pos {
            dir: pos.dir.turn(r),
            ..pos
        },
    }
}

#[aoc(day22, part1)]
pub fn part_1(input: &Input) -> usize {
    input
        .instructions
        .iter()
        .fold(input.start_pos(), |p, i| step(input, i, p))
        .password()
}

#[aoc(day22, part2)]
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
        // assert_eq!(part_2(&input),);
    }
}
