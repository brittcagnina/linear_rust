mod define;
mod vector;
mod matrix;

use vector::Vector;
use matrix::RowMatrix;
use matrix::ColumnMatrix;
use matrix::Matrix;

fn main() {
    let v = v!(2.3, 4.3, 4.5);

    let rm = row_matrix!(v); 
    println!{ "{}", rm.get(0, 0) };

    let cm = column_matrix!(
        v!(3.4, 4.5, 45.3),
        v!(2.3, 0.0, 4.3),
        v!(2.3, 0.0, 4.3),
        v!(2.3, 0.0, 4.3)
    );

    let a =  v!(3.4, 4.5);
    let b =  v!(2.3, 0.0);
    let c = column_matrix!(a, b);

    println!{ "{}", cm.rowCount() };
    println!{ "{}", c.rowCount() };
}
