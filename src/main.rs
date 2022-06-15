use std::fs;
use std::str::FromStr;
use std::time::Instant;

use decimal::d128;
use iced::{Alignment, Button, button, Column, Container, Element, Image, Length, Sandbox, Settings, Text};
use iced::image::{Handle, Viewer};
use iced::image::viewer::State;
use iced_aw::{number_input, NumberInput};
use image::RgbImage;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::color::color::Color;
use crate::color::color_palette::{ColorPalette, ExponentialColorPalette, HistogramColorPalette, KeyColor, LogarithmicColorPalette, RepeatingColorPalette};
use crate::complex::complex::{Complex, ComplexF64};
use crate::fractal::fractal::Configuration;
use crate::fractal::fractal_animation::FractalAnimation;
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

const THREADS: u8 = 4;

struct FractalBakery {
    skip_frames: u128,
    skip_frames_state: number_input::State,
    generate_button_state: button::State,
    preview_image: Handle,
}

impl Default for FractalBakery {
    fn default() -> Self {
        Self {
            skip_frames: 0,
            skip_frames_state: number_input::State::default(),
            generate_button_state: button::State::default(),
            preview_image: Handle::from_pixels(0, 0, Vec::new()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Generate,
    SkipFramesChanged(u128),
}

impl Sandbox for FractalBakery {
    type Message = Message;

    fn new() -> Self {
        FractalBakery::default()
    }

    fn title(&self) -> String {
        String::from("FractalBakery")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Generate => {
                let sample = sample(self.skip_frames);
                self.preview_image = Handle::from_pixels(sample.width(), sample.height(), sample.into_raw())
            }
            Message::SkipFramesChanged(val) => { self.skip_frames = val }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Text::new("Bezier tool example")
                    .width(Length::Shrink)
                    .size(50),
            )
            .push(
                NumberInput::new(&mut self.skip_frames_state, self.skip_frames, 2500, Message::SkipFramesChanged)
                    .step(1)
                    .min(0)
            )
            .push(
                Button::new(&mut self.generate_button_state, Text::new("Generate"))
                    .padding(5)
                    .on_press(Message::Generate)
            )
            .push(
                Container::new(iced::widget::image::Image::new(self.preview_image.clone()))
            )
            .into()
    }
}

fn main() -> iced::Result {
    rayon::ThreadPoolBuilder::new().num_threads(THREADS as usize).build_global().unwrap();

    example(0);

    FractalBakery::run(Settings {
        antialiasing: true,
        try_opengles_first: true,
        ..Settings::default()
    })
}

fn sample(frame: u128) -> RgbImage {
    let mut center_property = InterpolationList::new(
        20,
        ComplexF64::new(
            -0.5,
            0.0,
        ),
    );
    center_property.insert(Interpolation::EASING(1f64), 90, Complex::new(
        0.3602404434376143632361252444495453084826078079,
        0.641313061064803174860375015179,
    ));

    let mut zoom_property = InterpolationList::new(20, 0.0);
    zoom_property.insert(Interpolation::LINEAR, 2480, 31.0);

    let mut max_iteration_property = InterpolationList::new(500, 2000);
    max_iteration_property.insert(Interpolation::EASING(-0.75f64), 2000, 7500);

    let mut smoothing_property = InterpolationList::new(2000, true);
    smoothing_property.insert(Interpolation::NEAREST, 1, false);

    let mut set_color_property = InterpolationList::new(200, Color::BLACK);

    let mut mandelbrot_animation = MandelbrotAnimation::new(
        2500,
        center_property,
        zoom_property,
        max_iteration_property,
        smoothing_property,
        set_color_property,
    );

    let settings = RenderingSettings {
        resolution: Resolution { width: 1280, height: 720 },
        sampling: MultiSampling::NONE,
    };

    let key_colors = vec![
        KeyColor::new(0f64, Color::new(0.1, 0.05, 0.6)),
        KeyColor::new(10f64, Color::BLUE),
        KeyColor::new(20f64, Color::GREEN),
        KeyColor::new(40f64, Color::RED),
        KeyColor::new(100f64, Color::YELLOW),
        KeyColor::new(250f64, Color::MAGENTA),
        KeyColor::new(400f64, Color::CYAN),
        KeyColor::new(600f64, Color::new(0.1, 0.05, 0.6)),
    ];

    let mut color_palette = LogarithmicColorPalette::new(Interpolation::LINEAR, (*(&key_colors).to_vec()).to_owned(), 1.5f64);

    let image = mandelbrot_animation.get_configuration(frame as u64).unwrap().to_image(&settings, &mut color_palette);

    return image
}

fn example(skip: u128) {
    let mut center_property = InterpolationList::new(
        20,
        ComplexF64::new(
            -0.5,
            0.0,
        ),
    );
    center_property.insert(Interpolation::EASING(1f64), 90, Complex::new(
        0.3602404434376143632361252444495453084826078079,
        0.641313061064803174860375015179,
    ));

    let mut zoom_property = InterpolationList::new(20, 0.0);
    zoom_property.insert(Interpolation::LINEAR, 2480, 31.0);

    let mut max_iteration_property = InterpolationList::new(500, 2000);
    max_iteration_property.insert(Interpolation::EASING(-0.75f64), 2000, 7500);

    let mut smoothing_property = InterpolationList::new(2500, true);
    smoothing_property.insert(Interpolation::NEAREST, 1, false);

    let mut set_color_property = InterpolationList::new(200, Color::BLACK);

    let mandelbrot_animation = MandelbrotAnimation::new(
        2500,
        center_property,
        zoom_property,
        max_iteration_property,
        smoothing_property,
        set_color_property,
    );

    let settings = RenderingSettings {
        resolution: Resolution { width: 3840, height: 2160 },
        sampling: MultiSampling::X4,
    };

    let key_colors = vec![
        KeyColor::new(0f64, Color::new(0.1, 0.05, 0.6)),
        KeyColor::new(10f64, Color::BLUE),
        KeyColor::new(20f64, Color::GREEN),
        KeyColor::new(40f64, Color::RED),
        KeyColor::new(100f64, Color::YELLOW),
        KeyColor::new(250f64, Color::MAGENTA),
        KeyColor::new(400f64, Color::CYAN),
        KeyColor::new(600f64, Color::new(0.1, 0.05, 0.6)),
    ];

    fs::create_dir_all("out/").unwrap();
    mandelbrot_animation.into_iter().skip(skip as usize).par_bridge().for_each(|frame| {
        let number = frame.number;

        let mut color_palette = HistogramColorPalette::new(Interpolation::LINEAR, (*(&key_colors).to_vec()).to_owned());

        let current = Instant::now();

        let image = frame.config.to_image(&settings, &mut color_palette);
        image.save(format!("out/Fractal {number}.png")).unwrap();

        println!("{number}: {:?}", current.elapsed());
    })
}
