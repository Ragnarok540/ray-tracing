mod vec3;

use vec3::{Vec3};
use Vec3 as Point3;
use Vec3 as Color;

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in 0..IMAGE_HEIGHT {
        let remaining = IMAGE_HEIGHT - j;
        eprintln!("\rScanlines remaining: {remaining}");

        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color {
                e: [i as f64 / (IMAGE_WIDTH as f64 - 1.0),
                    j as f64 / (IMAGE_HEIGHT as f64 - 1.0),
                    0.0],
            };

            pixel_color.write_color();
        }
    }

    eprintln!("\rDone.");
}

// cargo build
// cargo run > image.ppm
