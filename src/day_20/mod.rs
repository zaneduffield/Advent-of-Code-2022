// This was more or less ripped from the very clever solution from Crazytieguy
// https://github.com/Crazytieguy/advent-of-code/blob/ae35e9347ee3fd2737f6c1ce291b74186cde11a1/2022/src/bin/day20/main.rs

use itertools::iterate;
use itertools::Itertools;
use nom::character::complete::{i64, line_ending};
use nom::multi::separated_list1;

pub type NumVal = i64;
pub type NumId = usize;

#[derive(Copy, Clone, Debug)]
pub struct Num(NumId, NumVal);

pub type Input = Vec<NumVal>;
pub type Parsed = Vec<NumVal>;

const DECRYPTION_KEY: NumVal = 811589153;

// The tests fail when this is greater or equal to 7
// I think it's because it's the length of the 'file'.
// There must be some bug.
const NEXT_DIST_BASE: usize = 25;

// Storing the indexes in a smaller type makes a significant enough
// to bother with all the casts.
type Idx = u16;

fn solve<F: Fn(NumVal) -> NumVal>(input: &Input, pre_op: F, n_iter: usize) -> NumVal {
    let nums = input.iter().map(|n| pre_op(*n)).collect_vec();

    // There's some bug with this optimisation that I can't quite work out.
    // Capping the 'next_dist' to be less than the length of the file fixes
    // things (and shouldn't make much of a difference in performance for small)
    // files.
    let next_dist: usize = NEXT_DIST_BASE.min(nums.len() - 1);

    let mut prev_idxs = (0..nums.len() as Idx).collect_vec();
    let mut next_idxs = prev_idxs.clone();
    prev_idxs.rotate_right(1);
    next_idxs.rotate_left(next_dist.rem_euclid(nums.len()));

    (0..n_iter).for_each(|_| {
        nums.iter().enumerate().for_each(|(idx, &num)| {
            // move prev idx into the cur idx's position (i.e. 'remove')
            replace_linked_idx(
                prev_idxs[idx],
                next_idxs[idx],
                idx as Idx,
                &mut prev_idxs,
                &mut next_idxs,
            );

            let didx = num.rem_euclid(nums.len() as NumVal - 1) as usize;
            let insert_idx =
                find_insert_idx(next_dist, prev_idxs[idx], didx, &prev_idxs, &next_idxs);

            prev_idxs[idx] = insert_idx;
            replace_linked_idx(
                idx as Idx,
                next_idxs[insert_idx as usize],
                insert_idx,
                &mut prev_idxs,
                &mut next_idxs,
            );
        })
    });

    let zero_idx: Idx = nums
        .iter()
        .position(|n| n == &0)
        .expect("couldn't find zero element") as Idx;
    iterate(zero_idx, |&i| {
        find_insert_idx(next_dist, i, 1000, &prev_idxs, &next_idxs)
    })
    .skip(1)
    .take(3)
    .map(|i| nums[i as usize])
    .sum()
}

fn replace_linked_idx(
    idx: Idx,
    next_idx: Idx,
    stop: Idx,
    prev_idxs: &mut [Idx],
    next_idxs: &mut [Idx],
) {
    let (far_prev_idx, immediate_next_idx) = iterate(idx, |&i| prev_idxs[i as usize])
        .zip(iterate(next_idx, |&i| prev_idxs[i as usize]))
        .inspect(|&(prev, next)| next_idxs[prev as usize] = next)
        .find(|&(_, next)| prev_idxs[next as usize] == stop)
        .unwrap();
    next_idxs[prev_idxs[far_prev_idx as usize] as usize] = idx;
    prev_idxs[immediate_next_idx as usize] = idx;
}

fn find_insert_idx(
    next_dist: usize,
    from: Idx,
    amount_to_move: usize,
    prev: &[Idx],
    next: &[Idx],
) -> Idx {
    let overshot_target = iterate(from, |&cur| next[cur as usize])
        .nth((next_dist + amount_to_move) / next_dist)
        .unwrap();
    iterate(overshot_target, |&cur| prev[cur as usize])
        .nth(next_dist - amount_to_move % next_dist)
        .unwrap()
}

type IResult<'a, T> = nom::IResult<&'a str, T>;

#[aoc_generator(day20)]
pub fn input_generator(data: &str) -> Parsed {
    let r: IResult<Parsed> = separated_list1(line_ending, i64)(data);
    r.unwrap().1
}

#[aoc(day20, part1)]
pub fn part_1(input: &Input) -> NumVal {
    solve(input, |n| n, 1)
}

#[aoc(day20, part2)]
pub fn part_2(input: &Input) -> NumVal {
    solve(input, |n| n * DECRYPTION_KEY, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            1
            2
            -3
            3
            -2
            0
            4
            "
        });
        assert_eq!(part_1(&input), 3);
        assert_eq!(part_2(&input), 1623178306);
    }
}
