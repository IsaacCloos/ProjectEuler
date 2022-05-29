use super::cell::Cell;

/// Logic layers require mutability
pub(crate) struct Matrix<T> {
    pub(crate) content: Vec<Vec<T>>,
    pub(crate) logic_layers: Vec<fn(T) -> T>,
}

impl<T> Matrix<T>
where
    T: Copy,
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


    pub(crate) fn add_logic_layer(&mut self, logic: fn(T) -> T) {
        self.logic_layers.push(logic);
    }
}
