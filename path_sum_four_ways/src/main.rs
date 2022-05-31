#![allow(dead_code, unused)]

mod tools;

use std::{fs, ops::RangeInclusive};

use tools::{cell::Cell, invocation::Invocation, matrix::Matrix, visualize::Visualize};

use crate::tools::cell::{get_exit_cell, Layer};

const DATA_PATH: &str = "data_easy.txt";

fn main() {
    let unparsed_data = fs::read_to_string(DATA_PATH).expect("inaccurate DATA_PATH");
    let mut matrix = Matrix::<i32>::from_table_string(unparsed_data, ',');

    for convergence in (0..matrix.get_layer_count()).rev().skip(1) {
        let mut layer = matrix.get_layer(convergence);
        matrix.solve_layer(&mut layer);
        for cell in layer.iter() {
            matrix.set_cell(cell)
        }
    }

    println!("{}", matrix.get_origin().val);

    // 426975 incorrect
}
