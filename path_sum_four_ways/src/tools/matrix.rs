// portals system to quickly leap from dissimilar types

#[derive(Debug)]
pub(crate) struct Matrix<T> {
    content: Vec<Vec<T>>,
}

impl<T: std::str::FromStr> From<String> for Matrix<T> {
    fn from(unparsed_string: String) -> Self {
        Matrix {
            content: unparsed_string
                .split_whitespace()
                .map(|line| {
                    line.split(',')
                        .map(|s| s.trim().parse::<T>().ok().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
        }
    }
}
