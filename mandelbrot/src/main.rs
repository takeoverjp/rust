extern crate num;
use num::Complex;

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0., im: 0. };
    loop {
        z = z * z + c;
        println!("{:?}", z);
    }
}

#[allow(dead_code)]
fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
        println!("{}", x);
    }
}

#[allow(dead_code)]
fn square_loop(mut x: f64) {
    loop {
        x = x * x;
        println!("{}", x);
    }
}

fn main() {
    complex_square_add_loop(Complex { re: 0.1, im: 0.1});
}
