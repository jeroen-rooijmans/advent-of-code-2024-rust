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
        assert_eq!(answer, usize::MAX);
    }
}
