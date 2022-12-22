use std::ops::{BitAnd, Not};

use range_union_find::IntRangeUnionFind;
use regex::Regex;

pub struct Sensor {
    pos: (i32, i32),
    closest_beacon: (i32, i32),
}

pub struct Input {
    sensors: Vec<Sensor>,
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let sensors = input
        .lines()
        .map(|line| {
            let captures = re.captures(line).expect("line didn't match re");
            let sx = captures[1].parse().unwrap();
            let sy = captures[2].parse().unwrap();
            let bx = captures[3].parse().unwrap();
            let by = captures[4].parse().unwrap();

            let pos = (sx, sy);
            let closest_beacon = (bx, by);

            Sensor {
                pos,
                closest_beacon,
            }
        })
        .collect();
    Input { sensors }
}

fn hamming_dist(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn non_beacons_for_row(sensors: &[Sensor], row: i32) -> IntRangeUnionFind<i32> {
    sensors
        .iter()
        .filter_map(|s| {
            let dist_to_beacon = hamming_dist(s.closest_beacon, s.pos);
            let dist_to_row = s.pos.1.abs_diff(row);
            if dist_to_row <= dist_to_beacon {
                let excess_dist = (dist_to_beacon - dist_to_row) as i32;
                Some(s.pos.0 - excess_dist..=s.pos.0 + excess_dist)
            } else {
                None
            }
        })
        .collect::<IntRangeUnionFind<i32>>()
}

#[aoc(day15, part1)]
pub fn part_1(input: &Input) -> usize {
    _part_1(input, 2_000_000)
}

pub fn _part_1(input: &Input, row: i32) -> usize {
    non_beacons_for_row(&input.sensors, row)
        .to_collection::<Vec<_>>()
        .iter()
        .map(|r| (r.end() - r.start()) as usize)
        .sum::<usize>()
}

#[aoc(day15, part2)]
pub fn part_2(input: &Input) -> u64 {
    _part_2(input, 4_000_000)
}

pub fn _part_2(input: &Input, max_coord: i32) -> u64 {
    let mut row_range = IntRangeUnionFind::<i32>::new();
    row_range.insert_range(&(0..=max_coord)).unwrap();
    let pos = (0..=max_coord)
        .flat_map(|row| {
            let range_union = row_range.bitand(&non_beacons_for_row(&input.sensors, row).not());
            let ranges: Vec<_> = range_union.to_collection();
            ranges.into_iter().flat_map(|r| r).map(move |x| (x, row))
        })
        .next()
        .unwrap();
    4_000_000 * pos.0 as u64 + pos.1 as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
            "
        });
        assert_eq!(_part_1(&input, 10), 26);
        assert_eq!(_part_2(&input, 20), 56_000_011);
    }
}
