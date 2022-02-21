#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use std::{cmp, fs};

// 80 rows of 80 numbers
const INPUT_PATH: &str = "input.txt";

type Matrix = Vec<Vec<i32>>;
type Point = (usize, usize);

fn main() {
    let mut matrix = get_data(INPUT_PATH);

    for i in (0..(matrix.len())).rev() {
        println!("{i}")
    }
}

trait TableFeatures {
    fn get_value(&self, point: Point) -> i32;
    fn set_value(&mut self, point: Point, value: i32);
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
