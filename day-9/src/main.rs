// Advent of Code - Day 9: Disk Fragmenter
use std::cmp::Ordering;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
enum Block {
    File { id: usize, size: u32 },
    Empty(u32),
}

#[derive(Debug)]
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
                    blocks.push(Block::Empty(size))
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
                    Block::Empty(size) => (acc, current_pos + (*size as u64)),
                },
            )
            .0
    }

    fn has_gaps(&self) -> bool {
        self.blocks.iter().any(|b| matches!(b, Block::Empty(_)))
    }

    fn trim_from_right(&mut self) {
        if let Some(last_file_pos) = self
            .blocks
            .iter()
            .rposition(|block| matches!(block, Block::File { .. }))
        {
            self.blocks.truncate(last_file_pos + 1);
        }
    }

    fn reduce_segment(&mut self, idx: usize, amount: u32) {
        match &mut self.blocks[idx] {
            Block::Empty(size) | Block::File { size, .. } => match &amount.cmp(size) {
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
        while self.has_gaps() {
            self.trim_from_right();
            if let Some(empty_idx) = self
                .blocks
                .iter()
                .position(|b| matches!(b, Block::Empty(_)))
            {
                if let Block::File { id, .. } = self.blocks[self.blocks.len() - 1] {
                    self.reduce_segment(self.blocks.len() - 1, 1);
                    self.reduce_segment(empty_idx, 1);
                    self.blocks.insert(empty_idx, Block::File { id, size: 1 });
                }
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

fn solve_part_two(_input: &str) -> usize {
    42
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
        // let example_input = "12345";
        let example_input = "2333133121414131402";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 1928);
    }

    #[test]
    fn part2() {
        let example_input = "2333133121414131402";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, usize::MAX);
    }
}
