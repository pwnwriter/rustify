extern crate rand;

use rand::Rng;

fn main() {
    let iterations = 1000000; // number of iterations to use in the calculation

    let mut inside_circle = 0; // number of points that fall within the unit circle

    let mut rng = rand::thread_rng(); // create a random number generator

    // generate random points and check if they fall within the unit circle
    for _ in 0..iterations {
        let x: f64 = rng.gen(); // generate a random x coordinate
        let y: f64 = rng.gen(); // generate a random y coordinate
        if (x*x + y*y).sqrt() <= 1.0 {
            inside_circle += 1;
        }
    }

    // use the ratio of points inside the circle to the total number of points
    // to approximate pi
    let pi = (4.0 * inside_circle as f64) / iterations as f64;

    println!("The value of pi is approximately {:.10}", pi);
}

