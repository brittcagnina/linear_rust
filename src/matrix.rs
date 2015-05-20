use traits;
use vector::Vector;
use std::ops::{Add, Mul};

pub struct RowMatrix<T: traits::LinrustTrait<T>> {
    num_rows: usize,
    num_columns: usize,
    rows: Vec<Vector<T>>,
}


impl<T: traits::LinrustTrait<T>> RowMatrix<T> {
    pub fn new(v: Vec<Vector<T>>) -> RowMatrix<T> {
        RowMatrix { num_rows: v.len(), num_columns: v[0].len(), rows: v}
    }
}

pub struct ColumnMatrix<T: traits::LinrustTrait<T>> {
    num_rows: usize,
    num_columns: usize,
    columns: Vec<Vector<T>>,
}

impl<T: traits::LinrustTrait<T>> ColumnMatrix<T> {
    pub fn new(c: Vec<Vector<T>>) -> ColumnMatrix<T> {
        ColumnMatrix { num_rows: c.len(), num_columns: c[0].len(), columns: c}
    }
}


pub trait Matrix<T: traits::LinrustTrait<T>> {
    fn get(&self, r: usize, c: usize) -> T;
    fn get_nth_row(&self, n: usize) -> Vector<T>;
    fn get_nth_col(&self, n: usize) -> Vector<T>;
    fn row_count(&self) -> usize; 
    fn col_count(&self) -> usize; 
}

impl<T: traits::LinrustTrait<T>> Matrix<T> for RowMatrix<T> {
    fn get(&self, r: usize, c: usize) -> T {
        self.rows[r].get(c).clone() 
    }
    fn get_nth_row(&self, n: usize) -> Vector<T> {
        self.rows[n].clone()
    }
    fn get_nth_col(&self, n: usize) -> Vector<T> {
        let mut v = Vec::new();
        for i in 0..self.num_columns {
            v.push(self.get(i, n).clone())
        }
        Vector::new(v)
    }
    fn row_count(&self) -> usize {
        self.num_rows
    }
    fn col_count(&self) -> usize {
        self.num_columns
    }
}

impl<T: traits::LinrustTrait<T>> Matrix<T> for ColumnMatrix<T> {
    fn get(&self, r: usize, c: usize) -> T {
        self.columns[c].get(r).clone() 
    }
    //TODO: fix this impl
    fn get_nth_row(&self, n: usize) -> Vector<T> {
        self.columns[n].clone()
    }
    //TODO: fix this impl
    fn get_nth_col(&self, n: usize) -> Vector<T> {
        let mut v = Vec::new();
        for i in 0..self.num_columns {
            v.push(self.get(n, i).clone())
        }
        Vector::new(v)
    }
    fn row_count(&self) -> usize {
        self.num_rows
    }
    fn col_count(&self) -> usize {
        self.num_columns
    }
}

pub fn print_matrix<T: traits::LinrustTrait<T>, S: Matrix<T>>(matrix: S) {
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

impl<T: traits::LinrustTrait<T>> Add<RowMatrix<T>> for RowMatrix<T> {
    type Output = RowMatrix<T>;
    fn add(self, other: RowMatrix<T>) -> RowMatrix<T> {
        let mut m = Vec::new(); 
        for i in 0..self.row_count() {
            m.push(self.get_nth_row(i) + other.get_nth_row(i))
        }
        RowMatrix::new(m)
    }
}

impl<T: traits::LinrustTrait<T>> Mul<RowMatrix<T>> for RowMatrix<T> {
    type Output = RowMatrix<T>;
    fn mul(self, other: RowMatrix<T>) -> RowMatrix<T> {
        let mut m = Vec::new(); 
        for i in 0..self.row_count() {
            let mut r = Vec::new();
            for j in 0..other.col_count() {
                r.push(self.get_nth_row(i) * other.get_nth_col(j))
            }
            let v = Vector::new(r);
            m.push(v); 
        }
        RowMatrix::new(m)
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
