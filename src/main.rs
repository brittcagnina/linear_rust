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

    let z = v!(2.3, 3.4);
    let z1 = v!(2.3, 3.4);
    println!{ "{}", z * z1 }; 

    let v = v!(1.0, 1.0);
    let v1 = v!(2.0, 3.0);
    println!{ "{}", v.dot(v1) }; 

    let v2 = v!(2.0, 3.0);
    v2.view();

    let s1 = v!(1.0, 1.0);
    let s2 = v!(2.0, 2.0);

    let s3 = s1.clone().dot(s2.clone());


    println!{ "{} {} {}", s1.dim(), s2.dim(), s3 };

}
