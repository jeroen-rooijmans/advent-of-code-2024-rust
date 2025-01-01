// Advent of Code - Day 9: Disk Fragmenter
use std::cmp::Ordering;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy)]
enum Block {
    File { id: usize, size: u32 },
    Empty { size: u32 },
}

impl Block {
    fn id(&self) -> Option<usize> {
        match self {
            Block::File { id, .. } => Some(*id),
            Block::Empty { .. } => None,
        }
    }

    fn size(&self) -> u32 {
        match self {
            Block::File { size, .. } => *size,
            Block::Empty { size } => *size,
        }
    }
}

#[derive(Debug, Clone)]
struct DiskMap {
    blocks: Vec<Block>,
}

impl FromIterator<u32> for DiskMap {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut id = 0;
        let mut blocks = Vec::new();
        for (i, size) in iter.into_iter().enumerate() {
            if size > 0 {
                if i % 2 == 0 {
                    // files are at even indices
                    blocks.push(Block::File { id, size });
                    id += 1;
                } else {
                    blocks.push(Block::Empty { size })
                }
            }
        }
        Self { blocks }
    }
}

impl DiskMap {
    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .fold(
                (0_u64, 0_u64),
                |(acc, current_pos), (_, segment)| match segment {
                    Block::File { id, size } => {
                        let new_pos = current_pos + (*size as u64);
                        let sum =
                            (current_pos..new_pos).fold(acc, |sum, pos| sum + pos * (*id as u64));
                        (sum, new_pos)
                    }
                    Block::Empty { size } => (acc, current_pos + (*size as u64)),
                },
            )
            .0
    }

    fn max_id(&self) -> Option<usize> {
        self.blocks.iter().filter_map(|b| b.id()).max()
    }

    fn find_idx(&self, target_id: usize) -> Option<usize> {
        self.blocks
            .iter()
            .position(|b| matches!(b, Block::File { id, .. } if *id == target_id))
    }

    fn reduce_segment(&mut self, idx: usize, amount: u32) {
        match &mut self.blocks[idx] {
            Block::Empty { size } | Block::File { size, .. } => match &amount.cmp(size) {
                Ordering::Greater => {
                    panic!("Cannot reduce block {idx} of size {size} by {amount}!")
                }
                Ordering::Equal => {
                    self.blocks.remove(idx);
                }
                Ordering::Less => *size -= amount,
            },
        }
    }

    fn compress(&mut self) {
        let mut block_idx = 0;
        while block_idx < self.blocks.len() {
            if let Block::Empty { size } = self.blocks[block_idx] {
                for _ in 0..size {
                    self.reduce_segment(block_idx, 1);
                    if let Some(last_file_idx) = self
                        .blocks
                        .iter()
                        .rposition(|block| matches!(block, Block::File { .. }))
                    {
                        if last_file_idx < block_idx {
                            break;
                        }
                        let id = self.blocks[last_file_idx].id().unwrap();
                        self.reduce_segment(last_file_idx, 1);
                        self.blocks.insert(block_idx, Block::File { id, size: 1 });
                        block_idx += 1;
                    }
                }
            }
            block_idx += 1;
        }
    }

    fn compressv2(&mut self) {
        for file_id in (0..=self.max_id().unwrap()).rev() {
            let file_idx = self.find_idx(file_id).unwrap();
            if let Some(empty_idx) = self.blocks[..file_idx].iter().position(|b| {
                matches!(b, Block::Empty { .. }) && b.size() >= self.blocks[file_idx].size()
            }) {
                let file = self.blocks.remove(file_idx);
                self.blocks
                    .insert(file_idx, Block::Empty { size: file.size() });
                self.blocks.insert(empty_idx, file);
                self.reduce_segment(empty_idx + 1, file.size());
            }
        }
    }
}

fn solve_part_one(input: &str) -> u64 {
    let input = input.chars().flat_map(|c| c.to_digit(10));
    let mut disk_map = DiskMap::from_iter(input);
    disk_map.compress();
    disk_map.checksum()
}

fn solve_part_two(input: &str) -> u64 {
    let input = input.chars().flat_map(|c| c.to_digit(10));
    let mut disk_map = DiskMap::from_iter(input);
    disk_map.compressv2();
    disk_map.checksum()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "2333133121414131402";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 1928);
    }

    #[test]
    fn part2() {
        let example_input = "2333133121414131402";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 2858);
    }
}
