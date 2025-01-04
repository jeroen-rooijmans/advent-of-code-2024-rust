use std::collections::HashMap;

use crate::coord::Coordinate;
use crate::direction::Direction;

#[derive(Debug)]
pub struct Grid<T> {
    pub map: HashMap<Coordinate<usize>, T>,
    pub num_rows: usize,
    pub num_columns: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(map: HashMap<Coordinate<usize>, T>, num_rows: usize, num_columns: usize) -> Grid<T> {
        Grid {
            map,
            num_rows,
            num_columns,
        }
    }

    pub fn construct(input: &str, map_fn: &dyn Fn(char) -> T) -> Grid<T> {
        let lines = input.lines();
        let num_rows = lines.clone().count();
        let num_columns = lines.clone().next().map(|l| l.len()).unwrap_or(0);

        let map = lines
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| (Coordinate { x, y }, map_fn(ch)))
            })
            .collect::<HashMap<Coordinate<usize>, T>>();

        Grid {
            map,
            num_rows,
            num_columns,
        }
    }

    pub fn set(&mut self, coord: Coordinate<usize>, val: T) {
        self.map.insert(coord, val);
    }

    pub fn step(
        &self,
        position: Coordinate<usize>,
        direction: Direction,
        step_size: usize,
    ) -> Option<Coordinate<usize>> {
        match direction {
            Direction::North => (position.x as i32 - step_size as i32 >= 0).then(|| Coordinate {
                x: position.x - step_size,
                y: position.y,
            }),
            Direction::NorthEast => {
                let north_bound = position.x as i32 - step_size as i32 >= 0;
                let east_bound = position.y + step_size <= self.num_columns;
                (north_bound && east_bound).then(|| Coordinate {
                    x: position.x - step_size,
                    y: position.y + step_size,
                })
            }
            Direction::East => (position.y + step_size <= self.num_columns).then(|| Coordinate {
                x: position.x,
                y: position.y + step_size,
            }),
            Direction::SouthEast => {
                let south_bound = position.x + step_size <= self.num_rows;
                let east_bound = position.y + step_size <= self.num_columns;
                (south_bound && east_bound).then(|| Coordinate {
                    x: position.x + step_size,
                    y: position.y + step_size,
                })
            }
            Direction::South => (position.x + step_size <= self.num_rows).then(|| Coordinate {
                x: position.x + step_size,
                y: position.y,
            }),
            Direction::SouthWest => {
                let south_bound = position.x + step_size <= self.num_rows;
                let west_bound = position.y as i32 - step_size as i32 >= 0;
                (south_bound && west_bound).then(|| Coordinate {
                    x: position.x + step_size,
                    y: position.y - step_size,
                })
            }
            Direction::West => (position.y as i32 - step_size as i32 >= 0).then(|| Coordinate {
                x: position.x,
                y: position.y - step_size,
            }),
            Direction::NorthWest => {
                let north_bound = position.x as i32 - step_size as i32 >= 0;
                let west_bound = position.y as i32 - step_size as i32 >= 0;
                (north_bound && west_bound).then(|| Coordinate {
                    x: position.x - step_size,
                    y: position.y - step_size,
                })
            }
        }
    }

    pub fn adjacent(&self, position: Coordinate<usize>) -> [Option<(Coordinate<usize>, &T)>; 4] {
        let north = self
            .step(position, Direction::North, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let east = self
            .step(position, Direction::East, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let south = self
            .step(position, Direction::South, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let west = self
            .step(position, Direction::West, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        [north, east, south, west]
    }

    pub fn surrounding(&self, position: Coordinate<usize>) -> [Option<(Coordinate<usize>, &T)>; 8] {
        let north = self
            .step(position, Direction::North, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let north_east = self
            .step(position, Direction::NorthEast, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let east = self
            .step(position, Direction::East, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let south_east = self
            .step(position, Direction::SouthEast, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let south = self
            .step(position, Direction::South, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let south_west = self
            .step(position, Direction::SouthWest, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let west = self
            .step(position, Direction::West, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        let north_west = self
            .step(position, Direction::NorthWest, 1)
            .and_then(|c| self.map.get(&c).map(|v| (c, v)));
        [
            north, north_east, east, south_east, south, south_west, west, north_west,
        ]
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
