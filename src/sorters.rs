extern crate image;
extern crate palette;

use std::f32::consts::PI;
use image::Pixel;
use palette::IntoColor;
use palette::pixel::RgbPixel;

fn pixel_to_hsl(p: &&image::Rgba<u8>) -> palette::Hsl {
    let values = p.channels4().to_rgba();
    palette::Rgb::new(values.0, values.1, values.2).into_hsl()
}

pub fn get_red(p: &&image::Rgba<u8>) -> u8 {
    p.channels4().0
}

pub fn get_green(p: &&image::Rgba<u8>) -> u8 {
    p.channels4().1
}

pub fn get_blue(p: &&image::Rgba<u8>) -> u8 {
    p.channels4().2
}

pub fn get_alpha(p: &&image::Rgba<u8>) -> u8 {
    p.channels4().3
}

pub fn get_hue(p: &&image::Rgba<u8>) -> u8 {
    (pixel_to_hsl(p).hue.to_positive_radians() / (2.0*PI)
        * u8::max_value() as f32) as u8
}

pub fn get_sat(p: &&image::Rgba<u8>) -> u8 {
    (pixel_to_hsl(p).saturation * u8::max_value() as f32) as u8
}

pub fn get_lig(p: &&image::Rgba<u8>) -> u8 {
    (pixel_to_hsl(p).lightness * u8::max_value() as f32) as u8
}
