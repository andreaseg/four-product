use ndarray::{Array, Array2};

use err_derive::Error;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum MatrixParseError {
    #[error(display = "could not parse number due to error {}", _0)]
    InvalidNumber(ParseIntError),
    #[error(
        display = "matrix is malformed, expected rows and cols are {} x {}, but actual size was {}",
        rows,
        cols,
        actual_size
    )]
    MalformedMatrix {
        rows: usize,
        cols: usize,
        actual_size: usize,
    },
}

/// Parses the given string into a matrix
/// Expects each row in the matrix to be separated by a newline,
/// and each column to be separated by a space.
///
/// Failure to parse the matrix will result in a MatrixParseError
pub fn parse_matrix(matrix_string: &str) -> Result<Array2<i32>, MatrixParseError> {
    let mut matrix_buffer: Vec<i32> = Vec::new();

    let mut rows: usize = 0;

    for row in matrix_string.split('\n') {
        rows += 1;

        for col in row.split_whitespace() {
            let n: i32 = col.parse().map_err(MatrixParseError::InvalidNumber)?;
            matrix_buffer.push(n)
        }
    }

    let buffer_length = matrix_buffer.len();
    let cols = buffer_length / rows;

    let shape = (rows, cols);

    let matrix = Array::from_shape_vec(shape, matrix_buffer).map_err(|_| {
        MatrixParseError::MalformedMatrix {
            rows,
            cols,
            actual_size: buffer_length,
        }
    })?;

    Ok(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    use ndarray::arr2;

    #[test]
    fn small_matrix() {
        let string = r"0 1 2
                       3 4 5
                       6 7 8";

        let expected = arr2(&[[0, 1, 2], [3, 4, 5], [6, 7, 8]]);

        assert_eq!(parse_matrix(string).unwrap(), expected);
    }

    #[test]
    fn tall_matrix() {
        let string = r"0 1 2
                       3 4 5
                       6 7 8
                       9 10 11";

        let expected = arr2(&[[0, 1, 2], [3, 4, 5], [6, 7, 8], [9, 10, 11]]);

        assert_eq!(parse_matrix(string).unwrap(), expected);
    }

    #[test]
    fn wide_matrix() {
        let string = r"0 1 2 3
                       4 5 6 7
                       8 9 10 11";

        let expected = arr2(&[[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11]]);

        assert_eq!(parse_matrix(string).unwrap(), expected);
    }
}
