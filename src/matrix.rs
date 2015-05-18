use vector::Vector;
use std::ops::{Add, Mul};
use std::fmt::Display;

pub struct RowMatrix<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> {
    numRows: usize,
    numColumns: usize,
    rows: Vec<Vector<T>>,
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> RowMatrix<T> {
    pub fn new(v: Vec<Vector<T>>) -> RowMatrix<T> {
        RowMatrix { numRows: v.len(), numColumns: v[0].len(), rows: v}
    }
}

pub struct ColumnMatrix<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> {
    numRows: usize,
    numColumns: usize,
    columns: Vec<Vector<T>>,
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> ColumnMatrix<T> {
    pub fn new(c: Vec<Vector<T>>) -> ColumnMatrix<T> {
        ColumnMatrix { numRows: c.len(), numColumns: c[0].len(), columns: c}
    }
}

pub trait Matrix<T> {
    fn get(&self, r: usize, c: usize) -> T;
    fn rowCount(&self) -> usize; 
    fn colCount(&self) -> usize; 
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> Matrix<T> for RowMatrix<T> {
    fn get(&self, r: usize, c: usize) -> T {
        self.rows[r].get(c).clone() 
    }
    fn rowCount(&self) -> usize {
        self.numRows
    }
    fn colCount(&self) -> usize {
        self.numColumns
    }
}

impl<T: Clone + Add<T, Output = T> + Mul<T, Output = T>> Matrix<T> for ColumnMatrix<T> {
    fn get(&self, r: usize, c: usize) -> T {
        self.columns[c].get(r).clone() 
    }
    fn rowCount(&self) -> usize {
        self.numRows
    }
    fn colCount(&self) -> usize {
        self.numColumns
    }
}

pub fn mView<S: Clone + Display, T: Matrix<S>>(matrix: T) {
    for i in 0..matrix.rowCount() {
        print!{ "[" };
        for j in 0..matrix.colCount() {
            print!{ "{}, ", matrix.get(i, j) };
        }
        print!{ "]\n" };
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
