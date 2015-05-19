use vector::Vector;
use std::ops::{Add, Mul};
use std::fmt::Display;

pub struct RowMatrix<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> {
    num_rows: usize,
    num_columns: usize,
    rows: Vec<Vector<T>>,
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> RowMatrix<T> {
    pub fn new(v: Vec<Vector<T>>) -> RowMatrix<T> {
        RowMatrix { num_rows: v.len(), num_columns: v[0].len(), rows: v}
    }
}

pub struct ColumnMatrix<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> {
    num_rows: usize,
    num_columns: usize,
    columns: Vec<Vector<T>>,
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> ColumnMatrix<T> {
    pub fn new(c: Vec<Vector<T>>) -> ColumnMatrix<T> {
        ColumnMatrix { num_rows: c.len(), num_columns: c[0].len(), columns: c}
    }
}

pub trait Matrix<T> {
    fn get(&self, r: usize, c: usize) -> T;
    fn row_count(&self) -> usize; 
    fn col_count(&self) -> usize; 
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> Matrix<T> for RowMatrix<T> {
    fn get(&self, r: usize, c: usize) -> T {
        self.rows[r].get(c).clone() 
    }
    fn row_count(&self) -> usize {
        self.num_rows
    }
    fn col_count(&self) -> usize {
        self.num_columns
    }
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> Matrix<T> for ColumnMatrix<T> {
    fn get(&self, r: usize, c: usize) -> T {
        self.columns[c].get(r).clone() 
    }
    fn row_count(&self) -> usize {
        self.num_rows
    }
    fn col_count(&self) -> usize {
        self.num_columns
    }
}

pub fn print_matrix<S: Clone + Display, T: Matrix<S>>(matrix: T) {
    for i in 0..matrix.row_count() {
        print!{ "[ " };
        for j in 0..matrix.col_count() {
            if j == matrix.col_count() - 1 {
                print!{ "{}", matrix.get(i, j) };
            } else {
                print!{ "{}, ", matrix.get(i, j) };
            }
        }
        print!{ " ]\n" };
    }
}

/*
// Addition Operator Overloads 
add_overload!(RowMatrix, RowMatrix, i64, self, object, { 0 });
add_overload!(RowMatrix, ColumnMatrix, i64, self, object, { 0 });

// Multiplication Operator Overloads 
mul_overload!(RowMatrix, ColumnMatrix, i64, self, object, { 0 });
mul_overload!(RowMatrix, RowMatrix, i64, self, object, { 0 });
*/
