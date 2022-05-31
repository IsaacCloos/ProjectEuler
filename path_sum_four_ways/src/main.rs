#![allow(dead_code, unused)]

mod tools;

use std::{fs, ops::RangeInclusive};

use tools::{cell::Cell, invocation::Invocation, matrix::Matrix, visualize::Visualize};

use crate::tools::cell::{get_exit_cell, Layer};

const DATA_PATH: &str = "data_easy.txt";

fn main() {
    let unparsed_data = fs::read_to_string(DATA_PATH).expect("inaccurate DATA_PATH");
    let mut matrix = Matrix::<i32>::from_table_string(unparsed_data, ',');

    let layer = matrix.get_layer(1);

    println!("{}", layer.view_basic().unwrap());
    print!("set  ");
    for cell in layer.iter() {
        print!("{}", cell.val)
    }
    print!("\n");

    print!("next ");
    let anchor = layer.get_anchor();
    for cell in layer.iter() {
        match get_exit_cell(&cell, &matrix, &anchor) {
            Some(cell) => print!("{}", cell.val),
            None => print!("•"),
        }
    }

    let mut temp_result = Vec::<i32>::new();

    // loop through set
    for (si, subject) in layer.iter().enumerate() {
        let mut new_val = Vec::<i32>::new();

        //loop through other cells
        for (ti, target) in layer.iter().enumerate() {
            match get_exit_cell(target, &matrix, &anchor) {
                Some(target_exit) => {
                    let what = get_range_iter(si, ti);
                    let target_cost =
                        layer[what].to_vec().iter().map(|c| c.val).sum::<i32>() + target_exit.val;

                    new_val.push(target_cost);
                }
                None => (),
            }

            // accumulate steps to target and add targets exit cost
        }

        // print!("\n{}    ", si);
        // for val in new_val.iter() {
        //     print!("{val},")
        // }
        temp_result.push(new_val.into_iter().min().unwrap());
    }

    print!("\ncost ");
    for val in temp_result {
        print!("{val},")
    }
}

fn calc_cost<T>(from: &Cell<T>, to: &Cell<T>) -> usize {
    todo!()
}

fn get_range_iter<T>(start: T, end: T) -> RangeInclusive<T>
where
    T: PartialOrd,
{
    if start < end {
        return start..=end;
    } else {
        return end..=start;
    }
}
