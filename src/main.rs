#[macro_use]
extern crate linrust;
use linrust as lr;

fn main() {

    let z1 = vec!(2, 4);
    let z2 = lr::Vector::new(z1);
    z2.view();

    let x1 = vec!(3, 4);
    let x2 = lr::Vector::new(x1);

    let t = z2 * x2;
    println!{ " 6 + 16 = {}", t };

    let m1 = row_matrix!(
        v!(4, 3, 4, 5),
        v!(4, 3, 4, 5),
        v!(4, 3, 4, 5),
        v!(4, 3, 4, 5)
    );

    let m2 = row_matrix!(
        v!(10, 10, 10, 10),
        v!(10, 10, 10, 10),
        v!(10, 10, 10, 10),
        v!(10, 10, 10, 10)
    );

    let m3 = m1 + m2;

    lr::matrix::print_matrix(m3);

    let m11 = row_matrix!(
        v!(1, 0, 0),
        v!(0, 1, 0),
        v!(0, 0, 1)
    );

    let m22 = row_matrix!(
        v!(1, 1, 1),
        v!(2, 2, 2),
        v!(3, 3, 3)
    );

    let m33 = m11 * m22;

    lr::matrix::print_matrix(m33);
}
