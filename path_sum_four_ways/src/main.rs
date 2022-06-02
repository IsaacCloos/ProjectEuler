use std::{collections::HashMap, fs, hash::Hash, ops::Add, str::FromStr};

const DATA_PATH: &str = "data_easy.txt";

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
    fn get_value(&self, cell: &Cell) -> T;
    /// Returns cumulative cost to travel from start to end
    fn calculate_distance(&self, start: Cell, end: Cell) -> T;
    /// Coordinates for first item of first array of matrix
    fn get_origin(&self) -> Cell;
    /// Coordinates for last item of last array of matrix
    fn get_end(&self) -> Cell;
}

impl<T> Table<T> for Matrix<T>
where
    T: FromStr + Copy + Add,
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

    fn calculate_distance(&self, start: Cell, end: Cell) -> T {
        let mut shortest_path_map = HashMap::<Cell, T>::from([(start, self.get_value(&start))]);
        

        while !shortest_path_map.contains_key(&end) {
            shortest_path_map.insert(end, self.get_value(&end));
        }

        shortest_path_map[&end]
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
