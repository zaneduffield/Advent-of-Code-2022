use rustc_hash::FxHashSet;
use std::str::Lines;

struct Steps<'a> {
    lines: Lines<'a>,
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "L" => Dir::Left,
            "R" => Dir::Right,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!("unexpected dir '{s}'"),
        }
    }
}

struct Step {
    count: i32,
    dir: Dir,
}

impl Step {
    fn visit<F>(self, mut visitor: F, snake: &mut [(i32, i32)])
    where
        F: FnMut((i32, i32)),
    {
        let (dx, dy) = match self.dir {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Down => (0, -1),
            Dir::Up => (0, 1),
        };

        for _ in 0..self.count {
            let mut head = snake.first_mut().unwrap();
            head.0 += dx;
            head.1 += dy;

            // Ideally I would want to use something like Slice::windows_mut but this is apparently
            // not possible with standard iterators.
            for i in 1..snake.len() {
                let head = snake[i - 1];
                let mut tail = &mut snake[i];
                let diff = (head.0 - tail.0, head.1 - tail.1);
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    tail.0 += diff.0.signum();
                    tail.1 += diff.1.signum();
                }
            }
            visitor(*snake.last().unwrap())
        }
    }
}

impl<'a> Iterator for Steps<'a> {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let (dir, count) = line.split_once(' ').expect("line should have a space");
            Step {
                dir: dir.into(),
                count: count.parse().expect("couldn't parse count"),
            }
        })
    }
}

fn solve(input: &str, snake_len: usize) -> usize {
    let mut visited = FxHashSet::default();
    let mut snake = vec![(0i32, 0i32); snake_len];

    let steps = Steps {
        lines: input.lines(),
    };
    for step in steps {
        step.visit(
            |tail| {
                visited.insert(tail);
            },
            &mut snake,
        );
    }

    visited.len()
}

#[aoc(day9, part1)]
pub fn part_1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day9, part2)]
pub fn part_2(input: &str) -> usize {
    solve(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {
        "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
            "
        };
        assert_eq!(part_1(input), 13);
        assert_eq!(part_2(input), 1);

        let input2 = indoc! {
        "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
            "
        };
        assert_eq!(part_2(input2), 36);
    }
}
