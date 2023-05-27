use itertools::Itertools;

type Bits = u128;

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Input {
    start: (isize, isize),
    goal: (isize, isize),
    width: isize,
    height: isize,
    wall_mask: Vec<Bits>,
    north: Vec<Bits>,
    south: Vec<Bits>,
    east: Vec<Bits>,
    west: Vec<Bits>,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Input {
    fn gap_pos(s: &str) -> usize {
        s.bytes().position(|c| c == b'.').map(|p| p - 1).unwrap()
    }

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let width = first_line.len() - 2;
    let start_x = gap_pos(first_line);

    let mut north = vec![];
    let mut east = vec![];
    let mut south = vec![];
    let mut west = vec![];

    for line in lines.take_while_ref(|line| !line.contains("##")) {
        let mut n = 0;
        let mut e = 0;
        let mut s = 0;
        let mut w = 0;

        line.bytes()
            .skip(1)
            .take(width)
            .enumerate()
            .map(|(i, b)| (1 << i, b))
            .for_each(|(bit, b)| match b {
                b'^' => n |= bit,
                b'>' => e |= bit,
                b'v' => s |= bit,
                b'<' => w |= bit,
                b'.' => {}
                _ => panic!("unexpected byte {b}"),
            });

        north.push(n);
        east.push(e);
        south.push(s);
        west.push(w);
    }

    let goal_x = gap_pos(lines.next().unwrap());
    let height = north.len() as isize;

    let mut wall_mask = vec![];
    wall_mask.push(1 << start_x);
    wall_mask.extend(std::iter::repeat(0).take(north.len()));
    wall_mask.push(1 << goal_x);

    Input {
        start: (start_x as isize, -1),
        goal: (goal_x as isize, height),
        width: width as isize,
        height,
        wall_mask,
        north,
        south,
        east,
        west,
    }
}

#[derive(Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct State {
    pos: (isize, isize),
    elapsed: usize,
}

fn rotate_east_wind(x: Bits, step: isize, width: isize) -> Bits {
    (x << step) | (x >> (width - step))
}

fn rotate_west_wind(x: Bits, step: isize, width: isize) -> Bits {
    (x >> step) | (x << (width - step))
}

/*
   The idea here is to iterate all the possible positions concurrently, storing them in a bitmap
   with the same shape as the grid. At each step, we can compute the positions of all the winds
   by rotating their bitmaps, and compute the potential positions for our person by taking the
   sum of the rotated previous potential positions in each cardinal direction. We then limit
   the potential positions to those not also occupied by winds. The great thing about this
   algorithm is all the operations are performed rowwise on the bitmaps, and the time complexity
   is O(n) where n is the solution.
*/
fn solve(elapsed: usize, input: &Input) -> usize {
    let valid_pos_mask = Bits::MAX >> (Bits::BITS as isize - input.width);

    let mut possible_positions: Vec<Bits> = vec![0; input.height as usize + 2];
    let mut next_positions: Vec<Bits> = possible_positions.clone();
    possible_positions[(input.start.1 + 1) as usize] |= 1 << input.start.0;

    let mut elapsed: isize = elapsed as isize;
    while possible_positions[(input.goal.1 + 1) as usize] & (1 << input.goal.0) == 0 {
        elapsed += 1;
        let hor_rotation = elapsed % input.width;

        // special case for start and end rows, because they only have rows on one side,
        // they don't have winds, and they have a wall.
        next_positions[0] = possible_positions[0] | (possible_positions[1] & input.wall_mask[0]);
        let last = next_positions.len() - 1;
        next_positions[last] =
            possible_positions[last] | (possible_positions[last - 1] & input.wall_mask[last]);

        // all the middle rows
        for i in 1..(possible_positions.len() - 1) {
            let cur = possible_positions[i];
            // step from any cardinal direction
            next_positions[i] = (cur << 1)
                | (cur >> 1)
                | possible_positions[i - 1]
                | possible_positions[i + 1]
                | cur;
            // be careful not to move outside the grid
            next_positions[i] &= valid_pos_mask;

            // rotate and sum the possible wind positions
            let row = i - 1;
            let blizzard_positions = valid_pos_mask
                & (rotate_east_wind(input.east[row], hor_rotation, input.width)
                    | rotate_west_wind(input.west[row], hor_rotation, input.width)
                    | input.north[(row as isize + elapsed).rem_euclid(input.height) as usize]
                    | input.south[(row as isize - elapsed).rem_euclid(input.height) as usize]);

            next_positions[i] &= !blizzard_positions;
        }

        std::mem::swap(&mut possible_positions, &mut next_positions);
    }

    elapsed as usize
}

#[aoc(day24, part1)]
pub fn part_1(input: &Input) -> usize {
    solve(0, input)
}

#[aoc(day24, part2)]
pub fn part_2(input: &Input) -> usize {
    let cost = solve(0, input);

    let reverse_input = &Input {
        start: input.goal,
        goal: input.start,
        ..input.clone()
    };
    let cost = solve(cost, reverse_input);

    solve(cost, input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            #.######
            #>>.<^<#
            #.<..<<#
            #>v.><>#
            #<^v^^>#
            ######.#
            "
        });
        assert_eq!(part_1(&input), 18);
        // assert_eq!(part_2(&input), 54);
    }
}
