// Advent of Code - Day 11: Plutonian Pebbles

use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Stones(Vec<u64>);

impl Stones {
    fn blink(&self) -> Stones {
        let mut stones = Stones(vec![]);
        let _ = self
            .0
            .iter()
            .map(|stone| match stone {
                0 => stones.0.push(1),
                n if (n.ilog10() + 1) % 2 == 0 => {
                    let engraving = n.to_string();
                    let (left, right) = engraving.split_at(engraving.len() / 2);
                    stones.0.push(left.parse().unwrap());
                    stones.0.push(right.parse().unwrap());
                }
                n => stones.0.push(n * 2024),
            })
            .collect::<Vec<_>>();
        stones
    }

    fn cached_blink(&self, cache: &HashMap<u64, u64>) -> HashMap<u64, u64> {
        let mut local_cache: HashMap<u64, u64> = HashMap::new();

        for (stone, count) in cache.iter() {
            match stone {
                0 => {
                    local_cache
                        .entry(1)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                }
                n if (n.ilog10() + 1) % 2 == 0 => {
                    let engraving = n.to_string();
                    let (left, right) = engraving.split_at(engraving.len() / 2);
                    local_cache
                        .entry(left.parse().unwrap())
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                    local_cache
                        .entry(right.parse().unwrap())
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                }
                n => {
                    local_cache
                        .entry(n * 2024)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                }
            }
        }

        local_cache
    }
}

fn solve_part_one(input: &str) -> usize {
    let mut stones = Stones(
        input
            .split_whitespace()
            .map(|stone| stone.parse::<u64>().unwrap())
            .collect::<Vec<_>>(),
    );
    (0..25).for_each(|_| stones = stones.blink());
    stones.0.len()
}

fn solve_part_two(input: &str, blinks: usize) -> u64 {
    let stones = Stones(
        input
            .split_whitespace()
            .map(|stone| stone.parse::<u64>().unwrap())
            .collect::<Vec<_>>(),
    );
    let mut cache: HashMap<u64, u64> = HashMap::new();

    stones.0.iter().for_each(|stone| {
        cache
            .entry(*stone)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    });

    (0..blinks).for_each(|_| cache = stones.cached_blink(&cache));

    cache.values().sum()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT, 75);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "125 17";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 55312);
    }

    #[test]
    fn part2() {
        let example_input = "125 17";
        let answer = crate::solve_part_two(example_input, 25);
        assert_eq!(answer, 55312);
    }
}
