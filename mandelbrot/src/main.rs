extern crate num;
use num::Complex;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0., im: 0. };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn main() {
    match escape_time(Complex {re: 3., im: 0.3}, 10000) {
        Some(v) => println!("Not mandelbrot!"),
        None => println!("Mandelbrot!")
    }
}
