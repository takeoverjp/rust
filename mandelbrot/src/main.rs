fn square_loop(mut x: f64) {
    loop {
        x = x * x;
        println!("{}", x);
    }
}

fn main() {
    square_loop(0.5);
}
