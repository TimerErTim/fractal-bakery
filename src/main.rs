use std::fs;

struct Resolution {
    width: u32,
    height: u32,
}

fn main() {
    let resolution = Resolution {
        width: 3840,
        height: 2190,
    };
    let max_iterations = 2000;
    let center = num_complex::Complex::new(
        -0.749993,
        -0.005,
    );
    let zoom = 100000000.0;

    let pixel_step = if resolution.width as f32 * 1.5 > resolution.height as f32 {
        3.0 / (zoom * resolution.width as f64)
    } else {
        2.0 / (zoom * resolution.height as f64)
    };

    let ln_of_2 = 2.0f64.ln();
    let mut imgbuf = image::RgbImage::new(resolution.width, resolution.height);

    for x in 0..resolution.width {
        for y in 0..resolution.height {
            let c = num_complex::Complex::new(
                pixel_step * (x as f64 - resolution.width as f64 / 2.0),
                pixel_step * (y as f64 - resolution.height as f64 / 2.0),
            ) + center;

            let mut iteration = 0;
            let mut z = num_complex::Complex::new(0.0, 0.0);
            while iteration < max_iterations && z.norm_sqr() < 4.0 {
                iteration += 1;
                z = z * z + c;
            }

            let color = if iteration == max_iterations {
                image::Rgb([0, 0, 0])
            } else {
                let add = z.norm_sqr().ln().ln() / ln_of_2;
                let final_iteration = iteration as f64 + 1.0 - add;
                let gray = (((max_iterations as f64 - final_iteration) / max_iterations as f64) * 255.0) as u8;
                image::Rgb([255, gray, gray])
            };

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = color;
        }
    }

    fs::create_dir_all("out/").unwrap();
    imgbuf.save("out/Fractal.png").unwrap();
}
