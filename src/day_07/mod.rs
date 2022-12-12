use itertools::Itertools;
use slab::Slab;

pub type Disk = Slab<Node>;
pub struct File {
    size: u32,
    // not needed in the end
    // name: String,
}
pub struct Node {
    name: String,
    files: Vec<File>,
    children: Vec<usize>,
    parent: Option<usize>,
}
pub struct Input {
    disk: Disk,
    root: usize,
}

impl Node {
    fn new(name: String) -> Node {
        Node {
            name,
            files: vec![],
            children: vec![],
            parent: None,
        }
    }

    fn total_size(disk: &Disk, node_idx: usize) -> u32 {
        let node = disk.get(node_idx).unwrap();
        let dir_size = node
            .children
            .iter()
            .map(|c| Node::total_size(disk, *c))
            .sum::<u32>();
        let file_size = node.files.iter().map(|f| f.size).sum::<u32>();
        dir_size + file_size
    }

    fn resolve(disk: &mut Disk, node_idx: usize, name: String) -> usize {
        let out = match disk
            .get(node_idx)
            .expect("node not found")
            .children
            .iter()
            .find(|c| disk.get(**c).unwrap().name == name)
        {
            Some(n) => *n,
            None => {
                let mut new_child = Node::new(name);
                new_child.parent = Some(node_idx);
                let new_id = disk.insert(new_child);
                disk.get_mut(node_idx).unwrap().children.push(new_id);
                new_id
            }
        };
        out
    }

    fn root(fs: &Disk, mut node_idx: usize) -> usize {
        while let Some(p) = fs.get(node_idx).and_then(|n| n.parent) {
            node_idx = p;
        }
        node_idx
    }
}

struct DiskWalker<'a> {
    disk: &'a Disk,
    cur_idx: usize,
    to_explore: Vec<usize>,
}

impl<'a> From<&'a Input> for DiskWalker<'a> {
    fn from(input: &'a Input) -> Self {
        DiskWalker {
            disk: &input.disk,
            cur_idx: input.root,
            to_explore: vec![],
        }
    }
}

impl<'a> Iterator for DiskWalker<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.disk.get(self.cur_idx).unwrap();
        self.to_explore.extend(cur.children.iter());
        self.cur_idx = self.to_explore.pop()?;
        Some(self.cur_idx)
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    let mut disk = Disk::with_capacity(1024);

    let mut lines = input.lines();
    let root: usize = disk.insert(Node::new("/".to_string()));
    let mut cur_node: usize = root;

    while let Some(line) = lines.next() {
        if let Some(cmd) = line.strip_prefix('$').map(|s| s.trim_start()) {
            match cmd {
                x if x.starts_with("cd") => {
                    let (_, dir) = x.split_once(' ').expect("expected dir after cd");
                    cur_node = match dir {
                        "/" => Node::root(&disk, cur_node),
                        ".." => disk
                            .get(cur_node)
                            .unwrap()
                            .parent
                            .expect("parent does not exist"),
                        x => Node::resolve(&mut disk, cur_node, x.to_string()),
                    };
                }
                "ls" => lines
                    .take_while_ref(|line| !line.starts_with('$'))
                    .for_each(|line| match line.split_once(' ') {
                        Some(("dir", dir)) => {
                            Node::resolve(&mut disk, cur_node, dir.to_string());
                        }
                        Some((size, _name)) => {
                            let file = File {
                                // name: name.to_string(),
                                size: size.parse().expect("couldn't parse size"),
                            };
                            disk.get_mut(cur_node).unwrap().files.push(file);
                        }
                        None => panic!("Couldn't find space in `ls` output"),
                    }),
                _ => panic!("Unexpected command {cmd}"),
            }
        }
    }
    Input { disk, root }
}

#[aoc(day7, part1)]
pub fn part_1(input: &Input) -> u32 {
    DiskWalker::from(input)
        .map(|n| Node::total_size(&input.disk, n))
        .filter(|s| *s <= 100000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part_2(input: &Input) -> u32 {
    let total_size = Node::total_size(&input.disk, input.root);
    let unused_space_required = 30_000_000;
    let remaining_space = 70_000_000 - total_size;
    let min_delete_size = unused_space_required - remaining_space;
    DiskWalker::from(input)
        .map(|n| Node::total_size(&input.disk, n))
        .filter(|s| *s >= min_delete_size)
        .min()
        .expect("no node found that is large enough to save the space")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {"
                $ cd /
                $ ls
                dir a
                14848514 b.txt
                8504156 c.dat
                dir d
                $ cd a
                $ ls
                dir e
                29116 f
                2557 g
                62596 h.lst
                $ cd e
                $ ls
                584 i
                $ cd ..
                $ cd ..
                $ cd d
                $ ls
                4060174 j
                8033020 d.log
                5626152 d.ext
                7214296 k
            "});
        assert_eq!(part_1(&input), 95437);
        assert_eq!(part_2(&input), 24933642);
    }
}
