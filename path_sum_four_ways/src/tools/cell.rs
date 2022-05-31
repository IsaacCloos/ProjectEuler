use std::str::FromStr;

use super::matrix::Matrix;

/// Reflects element of Matrix
#[derive(Clone, Copy)]
pub(crate) struct Cell<T> {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) val: T,
}

/// First element of layer is `Anchor`
pub(crate) type CellLayer<T> = Vec<Cell<T>>;

pub(crate) trait Layer<T>
where
    T: FromStr,
{
    fn get_anchor(&self) -> Cell<T>;

    fn get_exit_cell(&self, cell: &Cell<T>, matrix: &Matrix<T>) -> Option<Cell<T>>;
}

impl<T> Layer<T> for CellLayer<T>
where
    T: FromStr + Copy,
{
    fn get_anchor(&self) -> Cell<T> {
        self[0]
    }

    /// walk to next layer
    fn get_exit_cell(&self, cell: &Cell<T>, matrix: &Matrix<T>) -> Option<Cell<T>> {
        let anchor = self.get_anchor();

        // referencing final layer (single value)
        if cell.x == anchor.x && cell.y == anchor.y {
            return None;
        }

        if cell.x == anchor.x {
            // return Some(matrix.content[cell.y][cell.x + 1]);
            return Some(matrix.get_cell(cell.y, cell.x + 1));
        } else {
            // cell.y == anchor.y
            return Some(matrix.get_cell(cell.y + 1, cell.x));
        };
    }
}
