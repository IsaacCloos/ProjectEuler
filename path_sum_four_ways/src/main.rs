mod tools;

use std::fs;

use tools::{invocation::Invocation, matrix::Matrix, visualize::Visualize};

const DATA_PATH: &str = "data_easy.txt";

fn main() {
    let unparsed_data = fs::read_to_string(DATA_PATH).expect("inaccurate DATA_PATH");
    let mut matrix = Matrix::<i32>::from_table_string(unparsed_data, ',');

    let layer = matrix.get_layer(1);

    println!("{}", layer.view_basic().unwrap());
    for cell in layer {
        print!("{}", cell.val)
    }

    
}
