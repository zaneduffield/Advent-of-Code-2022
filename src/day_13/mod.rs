use std::{iter::Peekable, str::Chars};

use itertools::{Itertools, PeekingNext};

pub type Val = u8;

pub enum Item {
    Num(Val),
    List(List),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct List {
    items: Vec<Item>,
}
pub struct Pair {
    left: Item,
    right: Item,
}

pub type Input = Vec<Pair>;

impl Item {
    fn parse(chars: &mut Peekable<Chars>) -> Option<Item> {
        loop {
            let c = chars.next()?;
            match c {
                '[' => {
                    let mut list = List { items: vec![] };
                    while let Some(item) = Item::parse(chars) {
                        list.items.push(item);
                    }
                    return Some(Item::List(list));
                }
                '0'..='9' => {
                    return match chars.peeking_next(|c| c.is_ascii_digit()) {
                        None => Some(Item::Num(c as u8 - b'0')),
                        // an ugly but fast way to parse the two digit int
                        Some(c2) => Some(Item::Num((c as u8 - b'0') * 10 + (c2 as u8 - b'0'))),
                    };
                }
                ',' => continue,
                _ => return None,
            }
        }
    }
}

impl From<Val> for List {
    fn from(value: Val) -> Self {
        List {
            items: vec![Item::Num(value)],
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(l0), Self::Num(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::List(l0), Self::Num(r0)) => l0 == &List::from(*r0),
            (Self::Num(l0), Self::List(r0)) => &List::from(*l0) == r0,
        }
    }
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Num(l0), Self::Num(r0)) => l0.cmp(r0),
            (Self::List(l0), Self::List(r0)) => l0.cmp(r0),
            (Self::List(l0), Self::Num(r0)) => l0.cmp(&List::from(*r0)),
            (Self::Num(l0), Self::List(r0)) => List::from(*l0).cmp(r0),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        let mut chars = value.chars().peekable();
        Item::parse(&mut chars).expect("failed to parse line into item")
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair
                .lines()
                .map(Item::from)
                .collect_tuple()
                .expect("two lines must exist in every group");
            Pair { left, right }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part_1(input: &Input) -> usize {
    let idxs = input
        .iter()
        .map(|p| p.left.cmp(&p.right))
        .positions(|o| o.is_le())
        .map(|i| i + 1)
        .collect_vec();
    idxs.iter().sum()
}

#[aoc(day13, part2)]
pub fn part_2(input: &Input) -> usize {
    let div1 = Item::List(List::from(2));
    let div2 = Item::List(List::from(6));

    let divs = [&div1, &div2];
    let sorted = input
        .iter()
        .flat_map(|p| [&p.left, &p.right])
        .chain(divs.into_iter())
        .sorted()
        .collect_vec();

    let idx1 = sorted.binary_search(&&div1).expect("divider 1 not found");
    let idx2 = sorted.binary_search(&&div2).expect("divider 2 not found");

    (idx1 + 1) * (idx2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
            "
        });
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 140);

        let input = input_generator(indoc! {
            "
            [10,1,3,1,1]
            [10,1,5,1,1]
            "
        });
        assert_eq!(part_1(&input), 1);
    }
}
