extern crate image;
extern crate palette;

use std::env;
use std::process;
use std::fs::File;
use std::ascii::AsciiExt;
use std::path::PathBuf;

mod options;
mod sorters;

#[cfg(test)]
mod tests;


fn get_format(path: &PathBuf) -> Result<image::ImageFormat, String> {
    let ext = path.extension().and_then(|s| s.to_str())
                  .map_or("".to_string(), |s| s.to_ascii_lowercase());
    match &ext[..] {
        "jpg" |
        "jpeg" => Ok(image::ImageFormat::JPEG),
        "png"  => Ok(image::ImageFormat::PNG),
        "gif"  => Ok(image::ImageFormat::GIF),
        "webp" => Ok(image::ImageFormat::WEBP),
        "tif" |
        "tiff" => Ok(image::ImageFormat::TIFF),
        "tga" => Ok(image::ImageFormat::TGA),
        "bmp" => Ok(image::ImageFormat::BMP),
        "ico" => Ok(image::ImageFormat::ICO),
        "hdr" => Ok(image::ImageFormat::HDR),
        "ppm" => Ok(image::ImageFormat::PPM),
        format => Err(format!(
            "Image format image/{} is not supported.", format)),
    }
}

fn sort_pixels(img: &image::DynamicImage, mode: &options::Mode)
        -> image::RgbaImage {
    let key_fn = match *mode {
        options::Mode::Red => sorters::get_red,
        options::Mode::Green => sorters::get_green,
        options::Mode::Blue => sorters::get_blue,
        options::Mode::Alpha => sorters::get_alpha,
        options::Mode::Hue => sorters::get_hue,
        options::Mode::Saturation => sorters::get_sat,
        options::Mode::Lightness => sorters::get_lig,
    };

    // Use RBGA for everything so YOLO
    let mut buf = img.to_rgba();

    let buf2 = buf.clone();
    let mut sorted_pixels: Vec<_> = buf2.pixels().collect();
    sorted_pixels.sort_by_key(key_fn);

    for (i, pixel) in buf.pixels_mut().enumerate() {
        *pixel = *sorted_pixels[i];
    }

    buf
}

fn main() {
    let opts = options::parse(env::args_os().collect());

    let img = match image::open(&opts.inpath) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let buf = sort_pixels(&img, &opts.mode);

    let format = match get_format(&opts.outpath) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let mut fout = match File::create(&opts.outpath) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    match image::ImageRgba8(buf).save(&mut fout, format) {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
}
