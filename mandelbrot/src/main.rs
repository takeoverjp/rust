fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
        println!("{}", x);
    }
}

fn _square_loop(mut x: f64) {
    loop {
        x = x * x;
        println!("{}", x);
    }
}

fn main() {
    square_add_loop(0.25);
}
