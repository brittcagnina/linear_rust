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
        v!(4.3, 3.4, 4.5, 5.0),
        v!(4.3, 3.4, 4.5, 5.0),
        v!(4.3, 3.4, 4.5, 5.0),
        v!(4.3, 3.4, 4.5, 5.0)
    );

    lr::matrix::print_matrix(m1);
}
