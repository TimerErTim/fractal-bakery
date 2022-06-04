use std::fs;
use std::str::FromStr;
use std::time::Instant;

use decimal::d128;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::color::color::Color;
use crate::color::color_palette::{ColorPalette, KeyColor, RepeatingColorPalette};
use crate::complex::complex::{Complex, ComplexF64};
use crate::fractal::fractal::Configuration;
use crate::fractal::rendering_settings::{MultiSampling, RenderingSettings, Resolution};
use crate::interpolation::interpolatable::Interpolation;
use crate::interpolation::interpolation_list::InterpolationList;
use crate::mandelbrot::animation::MandelbrotAnimation;
use crate::mandelbrot::mandelbrot::Mandelbrot;

mod interpolation;
mod complex;
mod color;
mod mandelbrot;
mod fractal;

fn main() {
    let mut center_property = InterpolationList::new(
        20,
        ComplexF64::new(
            -0.5,
            0.0,
        ),
    );
    center_property.insert(Interpolation::EASING(1.5f64), 90, Complex::new(
        0.3602404434376143632361252444495453084826078079,
        0.641313061064803174860375015179,
    ));

    let mut zoom_property = InterpolationList::new(20, 0.0);
    zoom_property.insert(Interpolation::LINEAR, 10000, 31.0);

    let mut max_iteration_property = InterpolationList::new(1000, 2000);
    max_iteration_property.insert(Interpolation::EASING(-1f64), 5000, 7500);

    let mut smoothing_property = InterpolationList::new(7500, true);
    smoothing_property.insert(Interpolation::NEAREST, 1, false);

    let mut set_color_property = InterpolationList::new(200, Color::BLACK);
    for _ in 0..100 {
        set_color_property.insert(Interpolation::LINEAR, 250, Color::BLUE);
        set_color_property.insert(Interpolation::LINEAR, 250, Color::MAGENTA);
        set_color_property.insert(Interpolation::LINEAR, 250, Color::RED);
        set_color_property.insert(Interpolation::LINEAR, 250, Color::YELLOW);
        set_color_property.insert(Interpolation::LINEAR, 250, Color::GREEN);
        set_color_property.insert(Interpolation::LINEAR, 250, Color::CYAN);
    }

    let mandelbrot_animation = MandelbrotAnimation::new(
        10000,
        center_property,
        zoom_property,
        max_iteration_property,
        smoothing_property,
        set_color_property,
    );

    let settings = RenderingSettings {
        resolution: Resolution { width: 3840, height: 2160 },
        sampling: MultiSampling::NONE,
    };

    let key_colors = vec![
        KeyColor::new(0f64, Color::new(0.1, 0.05, 0.2)),
        KeyColor::new(10f64, Color::BLUE),
        KeyColor::new(50f64, Color::GREEN),
        KeyColor::new(300f64, Color::YELLOW),
        KeyColor::new(650f64, Color::MAGENTA),
        KeyColor::new(1000f64, Color::CYAN),
    ];

    fs::create_dir_all("out/").unwrap();
    mandelbrot_animation.into_iter().skip(2000).par_bridge().for_each(|frame| {
        let number = frame.number;

        let mut color_palette = RepeatingColorPalette::new(Interpolation::CUBIC, (*(&key_colors).to_vec()).to_owned());

        let current = Instant::now();

        let image = frame.config.to_image(&settings, &mut color_palette);
        image.save(format!("out/Fractal {number}.png")).unwrap();

        println!("{number}: {:?}", current.elapsed());
    })
}
