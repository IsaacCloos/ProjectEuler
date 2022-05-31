use std::str::FromStr;

use super::{cell::Cell, cell::CellLayer, shape::Shape};

/// Logic layers require mutability
pub(crate) struct Matrix<T>
where
    T: FromStr,
{
    pub(crate) content: Vec<Vec<T>>,
    // no reason to keep track of these in the matrix itself?
    // pub(crate) logic_layers: HashMap<&'static str, fn(T) -> T>,

    // pub(crate) logic_layers: Vec<fn(T) -> T>,
}

impl<T> Matrix<T>
where
    T: Copy + FromStr,
{
    /// First column of first row of matrix
    pub(crate) fn get_origin(&self) -> Cell<T> {
        Cell {
            x: 0,
            y: 0,
            val: self.content[0][0],
        }
    }

    pub(crate) fn get_row_start(&self, row_index: usize) -> Cell<T> {
        Cell {
            x: 0,
            y: row_index,
            val: self.content[row_index][0],
        }
    }

    pub(crate) fn get_row_end(&self, row_index: usize) -> Cell<T> {
        let last_column_index = self.content[row_index].len() - 1;

        Cell {
            x: last_column_index,
            y: row_index,
            val: self.content[row_index][last_column_index],
        }
    }

    pub(crate) fn snapshot<J: FromStr>(&self, lens: fn(T) -> J) -> Matrix<J> {
        Matrix {
            content: vec![Vec::<J>::new()],
        }
    }

    pub(crate) fn get_cell(&self, y: usize, x: usize) -> Cell<T> {
        Cell {
            x,
            y,
            val: self.content[y][x],
        }
    }

    pub(crate) fn get_layer(&self, convergence: usize) -> CellLayer<T> {
        let mut response = CellLayer::<T>::new();

        response.push(Cell {
            x: convergence,
            y: convergence,
            val: self.content[convergence][convergence],
        });

        for point in (convergence + 1)..self.content.len() {
            response.push(Cell {
                x: convergence,
                y: point,
                val: self.content[point][convergence],
            });
            response.push(Cell {
                x: point,
                y: convergence,
                val: self.content[convergence][point],
            });
        }

        response
    }

    // pub(crate) fn add_logic_layer(&mut self, name: &'static str, logic: fn(T) -> T) {
    //     self.logic_layers.insert(name, logic);
    // }

    // to come

    pub(crate) fn get_col_start(&self, col_index: usize) -> Cell<T> {
        todo!()
    }

    pub(crate) fn get_col_end(&self, col_index: usize) -> Cell<T> {
        todo!()
    }

    pub(crate) fn get_shape(&self) -> Shape {
        todo!()
    }

    pub(crate) fn defragment(self) -> Self {
        todo!()
    }

    pub(crate) fn normalize_empty_cells(&mut self) {
        todo!()
    }
}
