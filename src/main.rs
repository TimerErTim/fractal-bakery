use std::fs;
use std::str::FromStr;

use decimal::d128;

mod interpolatable;
mod complex;
mod fractal;
mod rendering_settings;
mod color;
mod color_palette;
mod mandelbrot;

fn main() {
    let resolution = Resolution {
        width: 3840,
        height: 2190,
    };
    let max_iterations = BigInt::from_str("2000").unwrap();
    let real_center = d128!(-0.749993);
    let imaginary_center = d128!(0.005);

    let zoom = d128!(1000000);

    let pixel_step = if resolution.width as f32 * 1.5 > resolution.height as f32 {
        d128!(2) / (zoom * d128::from(resolution.height))
    } else {
        d128!(3) / (zoom * d128::from(resolution.width))
    };

    let ln_of_2 = d128!(2.0).ln();
    let mut imgbuf = image::Rgb32FImage::new(resolution.width, resolution.height);

    for x in 0..resolution.width {
        for y in 0..resolution.height {
            let real_c = pixel_step * (d128::from(x) - d128::from(resolution.width) / d128!(2.0)) + real_center;
            let imaginary_c = pixel_step * (d128::from(resolution.height) / d128!(2.0) - d128::from(y)) + imaginary_center;

            let mut iteration = BigInt::zero();
            let mut real_z = d128::zero();
            let mut imaginary_z = d128::zero();
            let big_4 = d128!(4);
            while iteration < max_iterations && real_z * real_z + imaginary_z * imaginary_z < big_4 {
                iteration += 1u8;
                let prev_real_z = real_z;
                real_z = real_z * real_z - imaginary_z * imaginary_z + real_c;
                imaginary_z = prev_real_z * imaginary_z + prev_real_z * imaginary_z + imaginary_c;
            }

            let color = if iteration == max_iterations {
                image::Rgb([0.0, 0.0, 0.0])
            } else {
                let abs = real_z * real_z + imaginary_z * imaginary_z;
                let add = abs.ln().ln() / ln_of_2;
                let final_iteration = iteration.to_f32().unwrap() + 1.0f32 - f32::from_str(add.to_string().as_str()).unwrap();
                let max_iterations_f32 = max_iterations.to_f32().unwrap();
                let gray = ((max_iterations_f32 - final_iteration) / max_iterations_f32);
                image::Rgb([1.0f32, gray, gray])
            };

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = color;
        }
        println!("{}", x);
    }

    fs::create_dir_all("out/").unwrap();
    imgbuf.save("out/Fractal.png").unwrap();
}
