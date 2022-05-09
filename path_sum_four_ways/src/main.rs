use std::fs;

use tools::matrix::Matrix;
mod tools;

const DATA_PATH: &str = "matrix.txt";

fn main() {
    let unparsed_data = fs::read_to_string(DATA_PATH).expect("inaccurate DATA_PATH");

    let matrix = Matrix::<i32>::from(unparsed_data);

    println!("{matrix:?}")
}
