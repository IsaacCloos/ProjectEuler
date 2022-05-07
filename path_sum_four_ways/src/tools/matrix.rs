// portals system to quickly leap from dissimilar types

#[derive(Debug)]
pub(crate) struct Matrix<T> {
    content: Vec<Vec<T>>,
}

pub(crate) trait Load<T> {
    fn load<const DELIMITER_COUNT: usize>(
        content: T,
        delimiter_set: [char; DELIMITER_COUNT],
    ) -> Self;
}

impl Load<String> for Matrix<i32> {
    fn load<const DELIMITER_COUNT: usize>(
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
