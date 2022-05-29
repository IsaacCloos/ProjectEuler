use std::fmt::Display;

/// Reflects element of Matrix
pub(crate) struct Cell<T> {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) val: T,
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
