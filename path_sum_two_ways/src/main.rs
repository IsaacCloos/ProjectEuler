#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use std::{cmp, fmt::Display, fs};

// 80 rows of 80 numbers
const INPUT_PATH: &str = "input.txt";

type Matrix = Vec<Vec<i32>>;
type Point = (usize, usize);

fn main() {
    let mut matrix = get_data(INPUT_PATH);
    let matrix_index_len = matrix.len() - 1;

    // matrix.show();
    for i in (0..matrix_index_len).rev() {
        matrix.set_val_cmb((matrix_index_len, i + 1), (matrix_index_len, i));
        matrix.set_val_cmb((i + 1, matrix_index_len), (i, matrix_index_len));
    }
    // println!("############");
    // matrix.show();

    for for_row in (0..matrix_index_len).rev() {
        for for_col in (0..matrix_index_len).rev() {
            let point: Point = (for_col, for_row);
            let target_value = matrix.get_value(point);
            let reference_value_right = matrix.get_value((for_col + 1, for_row));
            let reference_value_down = matrix.get_value((for_col, for_row + 1));

            matrix.set_value(point, target_value + cmp::min(reference_value_down, reference_value_right))
        }
    }

    // println!("############");
    // matrix.show();

    println!("{}", matrix.get_value((0, 0)))
}

trait TableFeatures {
    fn get_value(&self, point: Point) -> i32;
    fn set_value(&mut self, point: Point, value: i32);
    fn set_val_cmb(&mut self, point_ref: Point, point_target: Point);
    fn show(&self);
}

impl TableFeatures for Matrix {
    fn get_value(&self, point: Point) -> i32 {
        *self
            .get(point.1)
            .expect("unable to access row")
            .get(point.0)
            .expect("unable to access column")
    }

    fn set_value(&mut self, point: Point, value: i32) {
        *self
            .get_mut(point.1)
            .expect("unable to access row")
            .get_mut(point.0)
            .expect("unable to access column") = value;
    }

    fn set_val_cmb(&mut self, point_ref: Point, point_target: Point) {
        *self
            .get_mut(point_target.1)
            .expect("unable to access row")
            .get_mut(point_target.0)
            .expect("unable to access column") += *self
            .get_mut(point_ref.1)
            .expect("unable to access row")
            .get_mut(point_ref.0)
            .expect("unable to access column")
    }

    fn show(&self) {
        for row in self {
            println!("{row:?}")
        }
    }
}

fn get_data(input_path: &str) -> Matrix {
    fs::read_to_string(input_path)
        .expect("Failed to read from file")
        .trim()
        .split_whitespace()
        .map(|x| {
            x.split(",")
                .map(|str| str::parse::<i32>(str).expect("Failed to parse integer"))
                .collect::<Vec<i32>>()
        })
        .collect::<Matrix>()
}
