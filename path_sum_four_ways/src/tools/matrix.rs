#[derive(Debug)]
pub(crate) struct Matrix<T> {
    content: Vec<Vec<T>>,
}

pub(crate) trait Load {
    fn from_string<const DELIMITER_COUNT: usize>(
        content: String,
        delimiter_set: [char; DELIMITER_COUNT],
    ) -> Self;
}

impl Load for Matrix<i32> {
    fn from_string<const DELIMITER_COUNT: usize>(
        content: String,
        delimiter_set: [char; DELIMITER_COUNT],
    ) -> Self {
        Matrix {
            content: content
                .split(delimiter_set[0])
                .map(|line| {
                    line.split(delimiter_set[1])
                        .map(|s| {
                            s.trim()
                                .parse::<i32>()
                                .expect("Failed to parse content to type i32")
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
        }
    }
}
