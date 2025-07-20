// Advent of Code - Day 12: Garden Groups

use std::collections::{HashSet, VecDeque};

use aoc::coord::Coordinate;
use aoc::grid::Grid;

const INPUT: &str = include_str!("./input.txt");

fn find_plant_area(
    seen_points: &mut HashSet<Coordinate<usize>>,
    grid: &Grid<char>,
    coord: Coordinate<usize>,
) -> HashSet<Coordinate<usize>> {
    let mut region: HashSet<Coordinate<usize>> = HashSet::new();
    if seen_points.contains(&coord) {
        return region;
    }
    region.insert(coord);
    let plant = grid.map.get(&coord).unwrap();
    let mut q = VecDeque::from([coord]);
    while let Some(coord) = q.pop_front() {
        let adjacents = grid.adjacent(coord);
        for (adjacent_coord, adjacent_plant) in adjacents.into_iter().flatten() {
            if !region.contains(&adjacent_coord) && adjacent_plant == plant {
                q.push_back(adjacent_coord);
                region.insert(adjacent_coord);
                seen_points.insert(adjacent_coord);
            }
        }
    }
    region
}

fn compute_perimeter(area: &HashSet<Coordinate<usize>>) -> usize {
    let mut perimeter = 0;
    for coord in area {
        let neighbours = coord.adjacent();
        for neighbour in neighbours {
            match neighbour {
                Some(coord) => {
                    if !area.contains(&coord) {
                        perimeter += 1
                    }
                }
                None => perimeter += 1,
            }
        }
    }
    perimeter
}

fn compute_corners(area: &HashSet<Coordinate<usize>>) -> usize {
    let mut corners = 0;

    for &coord in area {
        let surrounding = coord.surrounding();

        // Check all 4 corner configurations using surrounding array indices
        // surrounding: [up, topright, right, bottomright, down, bottomleft, left, topleft]
        // adjacent: [up, right, down, left]
        let corner_configs = [
            (0, 6, 7), // up, left, topleft
            (0, 2, 1), // up, right, topright
            (4, 6, 5), // down, left, bottomleft
            (4, 2, 3), // down, right, bottomright
        ];
        for &(adj1_idx, adj2_idx, diag_idx) in &corner_configs {
            let has_adj1 = surrounding[adj1_idx].is_some_and(|c| area.contains(&c));
            let has_adj2 = surrounding[adj2_idx].is_some_and(|c| area.contains(&c));
            let has_diag = surrounding[diag_idx].is_some_and(|c| area.contains(&c));
            // Outer corner: neither adjacent cell is in the region
            // Inner corner: both adjacent cells are in the region, but diagonal is not
            if (!has_adj1 && !has_adj2) || (has_adj1 && has_adj2 && !has_diag) {
                corners += 1;
            }
        }
    }
    corners
}

fn solve_part_one(input: &str) -> usize {
    let grid = Grid::construct(input, &|c| c);
    let mut seen_points: HashSet<Coordinate<usize>> =
        HashSet::with_capacity(grid.num_rows * grid.num_columns);

    grid.map
        .keys()
        .map(|coord| {
            let area = find_plant_area(&mut seen_points, &grid, *coord);
            let perimeter = compute_perimeter(&area);
            area.len() * perimeter
        })
        .sum()
}

fn solve_part_two(input: &str) -> usize {
    let grid = Grid::construct(input, &|c| c);
    let mut seen_points: HashSet<Coordinate<usize>> =
        HashSet::with_capacity(grid.num_rows * grid.num_columns);
    grid.map
        .keys()
        .map(|coord| {
            let area = find_plant_area(&mut seen_points, &grid, *coord);
            if !area.is_empty() {
                let corners = compute_corners(&area);
                area.len() * corners
            } else {
                0
            }
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
        let example_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 1930);
    }

    #[test]
    fn part2() {
        let example_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 1206);
    }
}
