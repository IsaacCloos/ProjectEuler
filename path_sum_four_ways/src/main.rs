use std::{collections::HashSet, fs, hash::Hash, ops::Add, str::FromStr};

const DATA_PATH: &str = "data.txt";

fn main() {
    let unparsed_string = fs::read_to_string(DATA_PATH).expect("Inaccurate data path.");
    let matrix = Matrix::<i32>::from_table_string(unparsed_string, ',');

    let result = matrix.calculate_distance(matrix.get_origin(), matrix.get_end());

    println!("{result}")
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Cell {
    x: usize,
    y: usize,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

type Matrix<T> = Vec<Vec<T>>;

trait Table<T> {
    fn from_table_string(unparsed_string: String, delimiter: char) -> Self;
    /// Return value from the cell coordinates
    fn get_value(&self, cell: &Cell) -> T;
    /// Set value at the cell coordinates
    fn set_value(&mut self, cell: &Cell, value: T);
    /// Get movement options for a given cell
    ///
    /// An off limits list is provided to limit redundancy and infinite loops
    fn get_options(&self, cell: &Cell, off_limits: &HashSet<Cell>) -> Vec<Cell>;
    /// Returns cumulative cost to travel from start to end
    fn calculate_distance(&self, start: Cell, end: Cell) -> T;
    /// Coordinates for first item of first array of matrix
    fn get_origin(&self) -> Cell;
    /// Coordinates for last item of last array of matrix
    fn get_end(&self) -> Cell;
}

impl<T> Table<T> for Matrix<T>
where
    T: FromStr + Copy + Add<Output = T> + Ord,
{
    fn from_table_string(unparsed_string: String, delimiter: char) -> Self {
        unparsed_string
            .split_whitespace()
            .map(|line| {
                line.split(delimiter)
                    .map(|s| {
                        s.trim()
                            .parse::<T>()
                            .ok()
                            .expect("Failed to parse generic type from &str.")
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>()
    }

    fn get_value(&self, cell: &Cell) -> T {
        *self
            .get(cell.y)
            .expect("y value out of range")
            .get(cell.x)
            .expect("x value out of range")
    }

    fn set_value(&mut self, cell: &Cell, value: T) {
        self[cell.y][cell.x] = value
    }

    fn get_options(&self, cell: &Cell, off_limits: &HashSet<Cell>) -> Vec<Cell> {
        let mut result = Vec::new();

        // up is available
        if cell.y > 0 {
            result.push(Cell {
                x: cell.x,
                y: cell.y - 1,
            });
        }

        // down is available
        if cell.y < self.len() - 1 {
            result.push(Cell {
                x: cell.x,
                y: cell.y + 1,
            });
        }

        // left is available
        if cell.x > 0 {
            result.push(Cell {
                x: cell.x - 1,
                y: cell.y,
            });
        }

        // right is available
        if cell.x < self[cell.y].len() - 1 {
            result.push(Cell {
                x: cell.x + 1,
                y: cell.y,
            });
        }

        result
            .into_iter()
            .filter(|c| !off_limits.contains(c))
            .collect::<Vec<Cell>>()
    }

    fn calculate_distance(&self, start: Cell, end: Cell) -> T {
        let mut working_matrix = self.clone();
        let mut shortest_path_set = HashSet::<Cell>::from([start]);

        while !shortest_path_set.contains(&end) {
            let mut options_per_cycle = Vec::<(Cell, T)>::new();

            for cell in shortest_path_set.iter() {
                let cell_val = working_matrix.get_value(cell);

                match working_matrix
                    .get_options(cell, &shortest_path_set)
                    .iter()
                    .map(|c| (*c, working_matrix.get_value(c) + cell_val))
                    .min_by(|a, b| a.1.cmp(&b.1))
                {
                    Some(next_step) => options_per_cycle.push(next_step),
                    None => (), // pass, no next steps available
                }
            }

            let cheapest_next_step = options_per_cycle
                .iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .unwrap();

            working_matrix.set_value(&cheapest_next_step.0, cheapest_next_step.1);
            shortest_path_set.insert(cheapest_next_step.0);
        }

        working_matrix.get_value(&end)
    }

    fn get_origin(&self) -> Cell {
        Default::default()
    }

    fn get_end(&self) -> Cell {
        let y = self.len() - 1;
        let x = self[y].len() - 1;
        Cell { x, y }
    }
}
