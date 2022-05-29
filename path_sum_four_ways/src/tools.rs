use std::{fmt::Display, str::FromStr, vec};

/// Logic layers require mutability
pub(crate) struct Matrix<T> {
    content: Vec<Vec<T>>,
    logic_layers: Vec<Vec<Vec<T>>>,
}

pub(crate) struct Cell<T> {
    x: usize,
    y: usize,
    val: T,
}

impl<T> Display for Cell<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x index: {}\ny index: {}\npoint value: {}",
            self.x, self.y, self.val
        )
    }
}

pub(crate) trait Table<T> {
    fn from_table_string(unparsed_string: String, delimiter: char) -> Self;
    fn get_start(&self) -> Cell<T>;
    fn get_end(&self) -> Cell<T>;
    fn get_size(&self) -> (usize, usize);

    // fn add_logic_layer(&mut self, logic: Fn(T) -> T);
}

impl<'a, T> Table<T> for Matrix<T>
where
    T: FromStr + Copy,
{
    fn from_table_string(unparsed_string: String, delimiter: char) -> Self {
        Matrix {
            content: unparsed_string
                .split_whitespace()
                .map(|line| {
                    line.split(delimiter)
                        .map(|s| {
                            s.trim()
                                .parse::<T>()
                                .ok()
                                .expect("Failed to parse generic type from &str.")
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
            logic_layers: Vec::new(),
        }
    }

    fn get_start(&self) -> Cell<T> {
        Cell {
            x: 0,
            y: 0,
            val: self.content[0][0],
        }
    }

    fn get_end(&self) -> Cell<T> {
        let x = self.content.len() - 1;
        let y = self.content[x].len() - 1;
        Cell {
            x,
            y,
            val: self.content[x][y],
        }
    }

    /// Returns x and y length of matrix respectively.
    fn get_size(&self) -> (usize, usize) {
        let x = self.content.len();
        let y = self.content[0].len();
        (x, y)
    }
}

impl<T> Matrix<T> {
    pub(crate) fn add_logic_layer(&mut self, _logic: fn(T) -> T) {
        self.logic_layers.push(vec![vec![]]);
    }
}
