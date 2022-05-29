use super::cell::Cell;

/// Logic layers require mutability
pub(crate) struct Matrix<T> {
    pub(crate) content: Vec<Vec<T>>,
    pub(crate) logic_layers: Vec<Vec<Vec<T>>>,
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub(crate) fn get_start(&self) -> Cell<T> {
        Cell {
            x: 0,
            y: 0,
            val: self.content[0][0],
        }
    }

    pub(crate) fn get_end(&self) -> Cell<T> {
        let x = self.content.len() - 1;
        let y = self.content[x].len() - 1;
        Cell {
            x,
            y,
            val: self.content[x][y],
        }
    }

    /// Returns x and y length of matrix respectively.
    pub(crate) fn get_size(&self) -> (usize, usize) {
        let x = self.content.len();
        let y = self.content[0].len();
        (x, y)
    }

    pub(crate) fn add_logic_layer(&mut self, _logic: fn(T) -> T) {
        self.logic_layers.push(vec![vec![]]);
    }
}