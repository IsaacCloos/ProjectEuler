use std::str::FromStr;
use super::matrix::Matrix;

pub(crate) trait Invocation<T> {
    fn from_table_string(unparsed_string: String, delimiter: char) -> Self;
}

impl<'a, T> Invocation<T> for Matrix<T>
where
    T: FromStr,
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
}