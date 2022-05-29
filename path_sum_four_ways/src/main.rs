mod tools;

use std::fs;

use tools::{invocation::Invocation, matrix::Matrix};

const DATA_PATH: &str = "data.txt";

fn main() {
    let unparsed_data = fs::read_to_string(DATA_PATH).expect("inaccurate DATA_PATH");
    let mut matrix = Matrix::<i32>::from_table_string(unparsed_data, ',');
}
