extern crate ndarray;

mod parser;
mod pretty_printer;

use ndarray::{ArrayBase, Dim, OwnedRepr};
use std::error::Error;

type Matrix<T> = ArrayBase<OwnedRepr<T>, Dim<[usize; 2]>>;

type Executable = Result<(), Box<dyn Error>>;

fn main() -> Executable {
    let matrix_bytes = include_bytes!("matrix.txt");

    let matrix = parser::parse_matrix(
        std::str::from_utf8(matrix_bytes).expect("Could not convert bytes to string"),
    )?;

    println!("Read matrix:");
    pretty_printer::pretty_print(&matrix);

    let max = max_four_product(&matrix);

    println!("Max four-product is: {}", max);

    Ok(())
}

/// Find the maximum four-product for a given matrix
///
/// The four-product is defined as the product of any four
/// elements in line in a matrix, either horizontally, vertically or diagonally.
///
/// If the matrix is so small that no four-product can be defined
/// (i.e. 1x3, 1x2, 1x1, 3x1, 3x2) the value 0 will be returned.
fn max_four_product(matrix: &Matrix<i32>) -> i32 {
    [diagonal_max, vertial_max, horizontal_max]
        .iter()
        .map(|f| f(&matrix))
        .max()
        .unwrap_or(0)
}

/// Finds the horizontal max four-product of the given matrix
fn horizontal_max(matrix: &Matrix<i32>) -> i32 {
    matrix
        .windows((1, 4))
        .into_iter()
        .map(|w| w[[0, 0]] * w[[0, 1]] * w[[0, 2]] * w[[0, 3]])
        .max()
        .unwrap_or(0)
}

/// Finds the vertical max four-product of the given matrix
fn vertial_max(matrix: &Matrix<i32>) -> i32 {
    matrix
        .windows((4, 1))
        .into_iter()
        .map(|w| w[[0, 0]] * w[[1, 0]] * w[[2, 0]] * w[[3, 0]])
        .max()
        .unwrap_or(0)
}

/// Finds the diagonal max four-product of the given matrix
fn diagonal_max(matrix: &Matrix<i32>) -> i32 {
    matrix
        .windows((4, 4))
        .into_iter()
        .map(|w| {
            let d1 = w[[0, 0]] * w[[1, 1]] * w[[2, 2]] * w[[3, 3]];
            let d2 = w[[0, 3]] * w[[1, 2]] * w[[2, 1]] * w[[3, 0]];
            std::cmp::max(d1, d2)
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal() {
        let matrix = r"2 3 4 5 0
                       0 0 0 0 0
                       0 0 0 0 0
                       0 0 0 0 0";

        let matrix = parser::parse_matrix(matrix).unwrap();

        assert_eq!(max_four_product(&matrix), 120);
        assert_eq!(horizontal_max(&matrix), 120);
    }

    #[test]
    fn vertical() {
        let matrix = r"2 0 0 0 0
                       3 0 0 0 0
                       4 0 0 0 0
                       5 0 0 0 0
                       0 0 0 0 0";

        let matrix = parser::parse_matrix(matrix).unwrap();

        assert_eq!(max_four_product(&matrix), 120);
        assert_eq!(vertial_max(&matrix), 120);
    }

    #[test]
    fn diagonal_1() {
        let matrix = r"2 0 0 0 0
                       0 3 0 0 0
                       0 0 4 0 0
                       0 0 0 5 0
                       0 0 0 0 0";

        let matrix = parser::parse_matrix(matrix).unwrap();

        assert_eq!(max_four_product(&matrix), 120);
        assert_eq!(diagonal_max(&matrix), 120);
    }

    #[test]
    fn diagonal_2() {
        let matrix = r"0 0 0 5 0
                       0 0 4 0 0
                       0 3 0 0 0
                       2 0 0 0 0
                       0 0 0 0 0";

        let matrix = parser::parse_matrix(matrix).unwrap();

        assert_eq!(max_four_product(&matrix), 120);
        assert_eq!(diagonal_max(&matrix), 120);
    }

    #[test]
    fn right_corner() {
        let matrix = r"0 0 0 0 0
                       0 0 0 0 2
                       0 0 0 0 3
                       0 0 0 0 4
                       0 0 0 0 5";

        let matrix = parser::parse_matrix(matrix).unwrap();

        assert_eq!(max_four_product(&matrix), 120);
        assert_eq!(vertial_max(&matrix), 120);
    }

    #[test]
    fn bottom_corner() {
        let matrix = r"0 0 0 0 0
                       0 0 0 0 0
                       0 0 0 0 0
                       0 0 0 0 0
                       0 2 3 4 5";

        let matrix = parser::parse_matrix(matrix).unwrap();

        assert_eq!(max_four_product(&matrix), 120);
        assert_eq!(horizontal_max(&matrix), 120);
    }

    #[test]
    fn large_matrix() {
        let matrix = r" 1  2  1  2 50  2  1  2  1  2  1  2
                        2  1  2 10  2  1  2  1  2  1  2  1
                        1  2 10  1  1 20  1  2  1  2  1  2
                        2 10  2  1  2 10  2  1  2  1  2  1
                        1  2 10  2  1 10  1  2  1  2  1  2
                        2  1  2 10 10 10 30  1  2  1  2  1
                        1  2  1  2 40  2  1  2  1  2  1  2
                        2  1  2  1  2  1  2  1  2  1  2  1
                        1  2  1  2  1  2  1  2  1  2  1  2";

        let matrix = parser::parse_matrix(matrix).unwrap();

        assert_eq!(max_four_product(&matrix), 50_000);
        assert_eq!(horizontal_max(&matrix), 30_000);
        assert_eq!(vertial_max(&matrix), 20_000);
        assert_eq!(diagonal_max(&matrix), 50_000);
    }
}
