use std::iter::repeat;

use itertools::Itertools;

#[derive(Clone)]
pub struct Input {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl Input {
    fn get(&self, i: usize, j: usize) -> u8 {
        self.data[j * self.width + i]
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let mut height = 0;
    let width = input.lines().next().map_or(0, str::len);
    let mut data = vec![];
    for line in input.lines() {
        data.extend(line.bytes());
        height += 1;
    }
    Input {
        data,
        height,
        width,
    }
}

fn visit_row_peaks<R, F>(input: &Input, mut visitor: F, range: R)
where
    R: Iterator<Item = (usize, usize)>,
    F: FnMut(usize, usize),
{
    let mut largest = 0;
    for (i, j) in range {
        let new = input.get(i, j);
        if new > largest {
            visitor(i, j);
            largest = new;
        }
    }
}

#[aoc(day8, part1)]
pub fn part_1(input: &Input) -> usize {
    let mut visible_map = vec![false; input.data.len()];
    let mut visit = |i, j| visible_map[j * input.width + i] = true;
    for j in 0..input.height {
        visit_row_peaks(input, &mut visit, (0..input.width).zip(repeat(j)));
        visit_row_peaks(input, &mut visit, (0..input.width).rev().zip(repeat(j)));
    }

    for i in 0..input.width {
        visit_row_peaks(input, &mut visit, repeat(i).zip(0..input.height));
        visit_row_peaks(input, &mut visit, repeat(i).zip((0..input.height).rev()));
    }

    visible_map.into_iter().filter(|b| *b).count()
}

fn visit_viewing_dist<R, F>(input: &Input, mut visitor: F, range: R)
where
    R: Iterator<Item = (usize, usize)>,
    F: FnMut((usize, usize), u8),
{
    let mut dists = [0u8; 10];
    for (i, j) in range {
        let new = input.get(i, j) as usize;
        let dist = dists[new];
        visitor((i, j), dist);
        dists.iter_mut().take(new + 1).for_each(|d| *d = 1);
        dists.iter_mut().skip(new + 1).for_each(|d| *d += 1);
    }
}

#[aoc(day8, part2)]
pub fn part_2(input: &Input) -> u32 {
    let input = &mut input.clone();
    input.data.iter_mut().for_each(|b| *b -= b'0');

    let mut dist_up_map = vec![0; input.data.len()];
    let mut dist_left_map = vec![0; input.data.len()];
    let mut dist_right_map = vec![0; input.data.len()];
    let mut dist_down_map = vec![0; input.data.len()];

    let put = |m: &mut Vec<u8>, (i, j), d| m[j * input.width + i] = d;

    for j in 0..input.height {
        visit_viewing_dist(
            input,
            |c, d| put(&mut dist_left_map, c, d),
            (0..input.width).zip(repeat(j)),
        );
        visit_viewing_dist(
            input,
            |c, d| put(&mut dist_right_map, c, d),
            (0..input.width).rev().zip(repeat(j)),
        );
    }

    for i in 0..input.width {
        visit_viewing_dist(
            input,
            |c, d| put(&mut dist_up_map, c, d),
            repeat(i).zip(0..input.height),
        );
        visit_viewing_dist(
            input,
            |c, d| put(&mut dist_down_map, c, d),
            repeat(i).zip((0..input.height).rev()),
        );
    }

    dist_up_map
        .into_iter()
        .zip_eq(dist_down_map)
        .zip_eq(dist_left_map)
        .zip_eq(dist_right_map)
        .map(|(((u, d), l), r)| u as u32 * d as u32 * l as u32 * r as u32)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
        "
            30373
            25512
            65332
            33549
            35390
            "});
        // assert_eq!(part_1(&input), 21);
        assert_eq!(part_2(&input), 8);
    }
}
