fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let g: f64 = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let b: f64 = 0.0;

            let ir: u64 = (255.999 * r) as u64;
            let ig: u64 = (255.999 * g) as u64;
            let ib: u64 = (255.999 * b) as u64;

            println!("{ir} {ig} {ib}");
        }
    }
}

// cargo build
// cargo run > image.ppm
