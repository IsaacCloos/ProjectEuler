use std::fmt::{Display, Write as FmtWrite};
use std::str::FromStr;

use super::cell::{Cell, CellLayer, Layer};
use super::matrix::Matrix;

pub(crate) trait Visualize {
    fn view_basic(&self) -> Result<String, std::fmt::Error>;
}

impl<T> Visualize for Matrix<T>
where
    T: FromStr + Display,
{
    fn view_basic(&self) -> Result<String, std::fmt::Error> {
        let mut string_representation = String::new();

        for row in self.content.iter() {
            for col in row {
                write!(&mut string_representation, "{}", col)?;
            }
            write!(&mut string_representation, "\n")?
        }

        Ok(string_representation)
    }
}

// write!(&mut string_representation, "{}", col)?;
// write!(&mut string_representation, "\n")?
impl<T> Visualize for CellLayer<T>
where
    T: FromStr + Display + std::fmt::Debug + Copy
{
    fn view_basic(&self) -> Result<String, std::fmt::Error>
    where
        Self: Layer<T>,
    {
        let mut string_representation = String::new();
        let anchor = self.get_anchor();

        // write layer arm
        for cell in self.iter().filter(|c| c.y == anchor.y) {
            write!(&mut string_representation, "{},", cell.val)?;
        }
        write!(&mut string_representation, "\n")?;

        // write layer stem
        for cell in self.iter().filter(|c| c.x == anchor.x).rev().skip(1) {
            write!(&mut string_representation, "{}", cell.val)?;
            write!(&mut string_representation, "\n")?;
        }

        Ok(string_representation)
    }
}

impl<T> Visualize for Cell<T>
where
    T: FromStr + Display,
{
    fn view_basic(&self) -> Result<String, std::fmt::Error> {
        let mut string_representation = String::new();

        write!(
            string_representation,
            "x index: {}\ny index: {}\npoint value: {}",
            self.x, self.y, self.val
        )?;

        Ok(string_representation)
    }
}
