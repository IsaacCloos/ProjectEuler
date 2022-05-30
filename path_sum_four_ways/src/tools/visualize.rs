use std::fmt::{Write as FmtWrite, Display};
use std::str::FromStr;

use super::matrix::Matrix;

pub(crate) trait Visualize {
    fn view_basic(&self) -> Result<String, std::fmt::Error>;
}

impl<T> Visualize for Matrix<T>
where
    T: FromStr + Display
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
