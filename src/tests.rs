use super::*;

use std::ffi::OsString;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn parse_options_and_format() {
    let args = ["imgsort", "somepic.jpg", "out.png", "--mode", "hue"];
    let opts = options::parse(args.iter().map(OsString::from).collect());

    assert_eq!(opts.mode, options::Mode::Hue);
    assert_eq!(get_format(&opts.outpath), Ok(image::ImageFormat::PNG));
    assert_eq!(get_format(&opts.inpath), Ok(image::ImageFormat::JPEG));
}

#[test]
fn sort_image_red() {
    let img1 = image::open("img/pepsi.png").unwrap();
    let buf1 = sort_pixels(&img1, &options::Mode::Red);
    let data1 = buf1.into_vec();

    let img2 = image::open("img/pepsi_sorted_red.png").unwrap();
    let data2 = img2.to_rgba().into_vec();
    assert_eq!(calculate_hash(&data1), calculate_hash(&data2));
}

#[test]
fn sort_image_green() {
    let img1 = image::open("img/pepsi.png").unwrap();
    let buf1 = sort_pixels(&img1, &options::Mode::Green);
    let data1 = buf1.into_vec();

    let img2 = image::open("img/pepsi_sorted_green.png").unwrap();
    let data2 = img2.to_rgba().into_vec();
    assert_eq!(calculate_hash(&data1), calculate_hash(&data2));
}

#[test]
fn sort_image_blue() {
    let img1 = image::open("img/pepsi.png").unwrap();
    let buf1 = sort_pixels(&img1, &options::Mode::Blue);
    let data1 = buf1.into_vec();

    let img2 = image::open("img/pepsi_sorted_blue.png").unwrap();
    let data2 = img2.to_rgba().into_vec();
    assert_eq!(calculate_hash(&data1), calculate_hash(&data2));
}

#[test]
fn sort_image_alpha() {
    let img1 = image::open("img/pepsi.png").unwrap();
    let buf1 = sort_pixels(&img1, &options::Mode::Alpha);
    let data1 = buf1.into_vec();

    let img2 = image::open("img/pepsi_sorted_alpha.png").unwrap();
    let data2 = img2.to_rgba().into_vec();
    assert_eq!(calculate_hash(&data1), calculate_hash(&data2));
}

#[test]
fn sort_image_hue() {
    let img1 = image::open("img/pepsi.png").unwrap();
    let buf1 = sort_pixels(&img1, &options::Mode::Hue);
    let data1 = buf1.into_vec();

    let img2 = image::open("img/pepsi_sorted_hue.png").unwrap();
    let data2 = img2.to_rgba().into_vec();
    assert_eq!(calculate_hash(&data1), calculate_hash(&data2));
}

#[test]
fn sort_image_sat() {
    let img1 = image::open("img/pepsi.png").unwrap();
    let buf1 = sort_pixels(&img1, &options::Mode::Saturation);
    let data1 = buf1.into_vec();

    let img2 = image::open("img/pepsi_sorted_sat.png").unwrap();
    let data2 = img2.to_rgba().into_vec();
    assert_eq!(calculate_hash(&data1), calculate_hash(&data2));
}

#[test]
fn sort_image_lig() {
    let img1 = image::open("img/pepsi.png").unwrap();
    let buf1 = sort_pixels(&img1, &options::Mode::Lightness);
    let data1 = buf1.into_vec();

    let img2 = image::open("img/pepsi_sorted_lig.png").unwrap();
    let data2 = img2.to_rgba().into_vec();
    assert_eq!(calculate_hash(&data1), calculate_hash(&data2));
}
