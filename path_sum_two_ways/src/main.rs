use std::fs;

// 80 rows of 80 numbers
const INPUT_PATH: &str = "input.txt";

struct Point {
    x: usize,
    y: usize,
}

type Matrix = Vec<Vec<i32>>;

fn main() {
    let matrix = get_data(INPUT_PATH);

    let start = Point { x: 0, y: 0 };

    let accumulated_total = 0;

    let smallest_path_total = walk(&start, &matrix, accumulated_total);

    println!("{smallest_path_total}")
}

fn walk(point: &Point, matrix: &Matrix, accumulated_total: i32) -> i32 {
    let new_total = accumulated_total + get_cell_value(&point, matrix);

    let mut options = Vec::<Point>::new();

    if (point.x + 1) != matrix.first().expect("matrix is empty").len() {
        options.push(Point {
            x: point.x + 1,
            y: point.y,
        })
    }

    if (point.y + 1) != matrix.len() {
        options.push(Point {
            x: point.x,
            y: point.y + 1,
        })
    }

    if options.is_empty() {
        println!("returning total: {new_total}");
        new_total
    } else {
        options
            .iter()
            .map(|p| walk(p, matrix, new_total))
            .min()
            .unwrap()
    }
}

fn get_cell_value(point: &Point, matrix: &Matrix) -> i32 {
    *matrix.get(point.y).unwrap().get(point.x).unwrap()
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