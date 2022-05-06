use std::fs;

use crate::tools::matrix::{Load, Matrix};

mod tools;

const DATA_PATH: &str = "matrix.txt";

fn main() {
    let unparsed_data = get_data_from_file(DATA_PATH);
    let matrix = Matrix::from_string(unparsed_data, ['\n', ',']);

    println!("{matrix:?}");
}

fn get_data_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}