// Advent of Code - Day 10: Hoof It

use std::collections::{HashMap, HashSet, VecDeque};

use aoc::coord::Coordinate;

const INPUT: &str = include_str!("./input.txt");

fn find_coordinates_by_height(
    map: &HashMap<Coordinate<usize>, u32>,
    target_height: u32,
) -> Vec<Coordinate<usize>> {
    map.iter()
        .filter(|(_, &height)| height == target_height)
        .map(|(&coord, _)| coord)
        .collect()
}

fn find_trail(
    map: &HashMap<Coordinate<usize>, u32>,
    start: Coordinate<usize>,
    end: Coordinate<usize>,
) -> Option<Vec<Coordinate<usize>>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut came_from: HashMap<Coordinate<usize>, Coordinate<usize>> = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current_pos) = queue.pop_front() {
        if current_pos == end {
            // reconstruct path
            let mut path = vec![end];
            let mut current = end;
            while current != start {
                current = *came_from.get(&current).unwrap();
                path.push(current);
            }
            path.reverse();
            return Some(path);
        }

        let possible_moves = [
            Coordinate {
                x: current_pos.x + 1,
                y: current_pos.y,
            },
            Coordinate {
                x: current_pos.x.wrapping_sub(1),
                y: current_pos.y,
            },
            Coordinate {
                x: current_pos.x,
                y: current_pos.y + 1,
            },
            Coordinate {
                x: current_pos.x,
                y: current_pos.y.wrapping_sub(1),
            },
        ];

        let current_height = map.get(&current_pos).unwrap();
        for next_pos in possible_moves {
            if let Some(next_height) = map.get(&next_pos) {
                if *next_height == current_height + 1 && !visited.contains(&next_pos) {
                    visited.insert(next_pos);
                    came_from.insert(next_pos, current_pos);
                    queue.push_back(next_pos);
                }
            }
        }
    }
    None // No valid trail found!
}

fn find_all_trails(
    map: &HashMap<Coordinate<usize>, u32>,
    start: Coordinate<usize>,
    end: Coordinate<usize>,
) -> Vec<Vec<Coordinate<usize>>> {
    let mut all_paths = Vec::new();
    let mut current_path = vec![start];
    let mut visited = HashSet::new();
    let mut unique_paths = HashSet::new();
    visited.insert(start);

    find_all_trails_recursive(
        map,
        start,
        end,
        &mut current_path,
        &mut visited,
        &mut all_paths,
        &mut unique_paths,
    );

    all_paths
}

fn find_all_trails_recursive(
    map: &HashMap<Coordinate<usize>, u32>,
    current: Coordinate<usize>,
    end: Coordinate<usize>,
    current_path: &mut Vec<Coordinate<usize>>,
    visited: &mut HashSet<Coordinate<usize>>,
    all_paths: &mut Vec<Vec<Coordinate<usize>>>,
    unique_paths: &mut HashSet<Vec<Coordinate<usize>>>,
) {
    if current == end {
        // only add the path if we haven't seen this exact sequence before
        if unique_paths.insert(current_path.clone()) {
            all_paths.push(current_path.clone());
        }
    }
    let current_height = *map.get(&current).unwrap();

    let possible_moves = [
        Coordinate {
            x: current.x + 1,
            y: current.y,
        },
        Coordinate {
            x: current.x.wrapping_sub(1),
            y: current.y,
        },
        Coordinate {
            x: current.x,
            y: current.y + 1,
        },
        Coordinate {
            x: current.x,
            y: current.y.wrapping_sub(1),
        },
    ];

    for next_pos in possible_moves.iter() {
        if let Some(&next_height) = map.get(next_pos) {
            if next_height == current_height + 1 && !visited.contains(next_pos) {
                // try this path
                visited.insert(*next_pos);
                current_path.push(*next_pos);

                find_all_trails_recursive(
                    map,
                    *next_pos,
                    end,
                    current_path,
                    visited,
                    all_paths,
                    unique_paths,
                );

                // backtrack
                visited.remove(next_pos);
                current_path.pop();
            }
        }
    }
}

fn solve_part_one(input: &str) -> usize {
    let topo_map: HashMap<Coordinate<usize>, u32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (Coordinate { x, y }, ch.to_digit(10).unwrap()))
        })
        .collect();
    let trailheads = find_coordinates_by_height(&topo_map, 0);
    let tops = find_coordinates_by_height(&topo_map, 9);

    let trails = trailheads
        .iter()
        .flat_map(|trailhead| {
            tops.iter()
                .filter_map(|top| find_trail(&topo_map, *trailhead, *top))
        })
        .collect::<Vec<_>>();

    trails.len()
}

fn solve_part_two(input: &str) -> usize {
    let topo_map: HashMap<Coordinate<usize>, u32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (Coordinate { x, y }, ch.to_digit(10).unwrap()))
        })
        .collect();
    let trailheads = find_coordinates_by_height(&topo_map, 0);
    let tops = find_coordinates_by_height(&topo_map, 9);
    trailheads
        .iter()
        .flat_map(|trailhead| {
            tops.iter()
                .map(|top| find_all_trails(&topo_map, *trailhead, *top).len())
        })
        .sum()
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
        let example_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 36);
    }

    #[test]
    fn part2() {
        let example_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 81);
    }
}
