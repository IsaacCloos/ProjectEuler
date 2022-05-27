use std::fs;

use tools::matrix::{Matrix, Table};
mod tools;

const DATA_PATH: &str = "matrix.txt";

fn main() {
    let unparsed_data = fs::read_to_string(DATA_PATH).expect("inaccurate DATA_PATH");

    let matrix = Matrix::<i32>::from_table_string(unparsed_data, ',');

    println!("{}", matrix.get_start());
    println!("{}", matrix.get_end());
    println!("{:?}", matrix.get_size());
}
