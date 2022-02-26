use std::fs;

// 80 rows of 80 numbers
const INPUT_PATH: &str = "input.txt";

type Matrix = Vec<Vec<Cell>>;

#[derive(Debug)]
struct Cell {
    value: i32,
    cost_to_finish: i32,
}

fn main() {
    let mut matrix = get_data(INPUT_PATH);
    let matrix_len = matrix.len();

    for i in 0..matrix_len {
        let subject = matrix.get_mut(i).unwrap().get_mut(matrix_len - 1).unwrap();
        subject.cost_to_finish = subject.value;
    }

    for i in (0..(matrix.len() - 1)).rev() {
        // starting one column in
        for k in (0..matrix.len()).rev() {
            // every row
            set_cheapest_path_to_right(&mut matrix, i, k);
        }
    }

    let mut result = 9999999;

    for i in 0..matrix_len {
        let subject = matrix.get(i).unwrap().get(0).unwrap();

        if subject.cost_to_finish < result {
            result = subject.cost_to_finish
        }
    }

    println!("{result}")
}

fn set_cheapest_path_to_right(matrix: &mut Matrix, x: usize, y: usize) {
    let subject = matrix.get(y).unwrap().get(x).unwrap();
    let mut options = Vec::<i32>::new();
    for i in (0..y).rev() {
        // from subject to top
        let mut col_acc = subject.value;
        for k in (i..y).rev() {
            // retrace from subject to pivot point
            col_acc += matrix.get(k).unwrap().get(x).unwrap().value
        }
        let right_value = matrix.get(i).unwrap().get(x + 1).unwrap().cost_to_finish;

        options.push(col_acc + right_value)
    }

    for i in y..matrix.len() {
        let mut col_acc = 0;
        for k in y..=i {
            let rollup_val = matrix.get(k).unwrap().get(x).unwrap().value;


            col_acc += rollup_val
        }
        let right_value = matrix.get(i).unwrap().get(x + 1).unwrap().cost_to_finish;

        options.push(col_acc + right_value)
    }

    matrix
        .get_mut(y)
        .unwrap()
        .get_mut(x)
        .unwrap()
        .cost_to_finish = *options.iter().min().unwrap()
}

fn print_matrix(matrix: &Matrix) {
    for row in matrix {
        for col in row {
            print!("{0}|{1: <10}\t", col.value, col.cost_to_finish)
        }
        print!("\n")
    }
}

fn get_data(input_path: &str) -> Matrix {
    fs::read_to_string(input_path)
        .expect("Failed to read from file")
        .trim()
        .split_whitespace()
        .map(|x| {
            x.split(",")
                .map(|str: &str| -> Cell {
                    let val = str::parse::<i32>(str).expect("Failed to parse integer");
                    Cell {
                        value: val,
                        cost_to_finish: 0,
                    }
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Matrix>()
}