use std::fs;
use std::str::FromStr;
use std::time::Instant;

use decimal::d128;

use crate::color::Color;
use crate::color_palette::{ColorPalette, KeyColor, RepeatingColorPalette};
use crate::complex::{Complex, ComplexF64};
use crate::fractal::Configuration;
use crate::interpolatable::Interpolation;
use crate::mandelbrot::Mandelbrot;
use crate::rendering_settings::{MultiSampling, RenderingSettings, Resolution};

mod interpolatable;
mod complex;
mod fractal;
mod rendering_settings;
mod color;
mod color_palette;
mod mandelbrot;
mod interpolation_list;

fn main() {
    let mandelbrot = Mandelbrot::new(
        ComplexF64::new(-0.5, 0.0),
        0.0,
        2000u128,
        true,
        Color::BLACK,
    );

    let settings = RenderingSettings {
        resolution: Resolution { width: 3840, height: 2160 },
        sampling: MultiSampling::NONE,
    };

    let key_colors = vec![
        KeyColor::new(0f64, Color::CYAN),
        KeyColor::new(4f64, Color::RED),
        KeyColor::new(6f64, Color::BLUE),
        KeyColor::new(20f64, Color::GREEN),
        KeyColor::new(50f64, Color::YELLOW),
        KeyColor::new(200f64, Color::MAGENTA),
        KeyColor::new(1000f64, Color::CYAN),
    ];

    let mut color_palette = RepeatingColorPalette::new(Interpolation::CUBIC, key_colors);

    let current = Instant::now();
    let image = mandelbrot.to_image(&settings, &mut color_palette);
    println!("{:?}", current.elapsed());

    println!("{}, {}", image.width(), image.height());
    fs::create_dir_all("out/").unwrap();
    image.save("out/Fractal.png").unwrap();
}
