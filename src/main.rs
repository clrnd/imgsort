extern crate image;

use std::fs::File;
use std::process;
use std::ascii::AsciiExt;
use std::path::PathBuf;
mod options;


fn get_format(path: PathBuf) -> Result<image::ImageFormat, String> {
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

fn main() {
    let opts = options::parse();
    println!("{:?}", opts);

    let img = match image::open(opts.inpath) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let mut buf = img.to_rgba();

    let buf2 = buf.clone();
    let mut sorted_pixels: Vec<_> = buf2.pixels().collect();
    sorted_pixels.sort_by_key(|&p| p.data[2]);

    for (i, pixel) in buf.pixels_mut().enumerate() {
        *pixel = *sorted_pixels[i];
    }

    let mut fout = match File::create(&opts.outpath) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let format = match get_format(opts.outpath) {
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
