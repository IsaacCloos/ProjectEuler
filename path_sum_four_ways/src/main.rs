mod tools;

use std::{fs, vec};

use tools::{invocation::Invocation, matrix::Matrix};

const DATA_PATH: &str = "data.txt";

fn main() {
    let unparsed_data = fs::read_to_string(DATA_PATH).expect("inaccurate DATA_PATH");
    let mut matrix = Matrix::<i32>::from_table_string(unparsed_data, ',');

    matrix.add_logic_layer(|x| {
        x.abs()
    })
}
