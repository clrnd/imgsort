extern crate clap;

use std::ffi::OsString;
use self::clap::{Arg, App};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Red,
    Green,
    Blue,
    Alpha,
    Hue,
    Saturation,
    Lightness
}

#[derive(Debug)]
pub struct Options {
    pub inpath: PathBuf,
    pub outpath: PathBuf,
    pub mode: Mode
}

pub fn parse(args: Vec<OsString>) -> Options {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("mode")
            .help("Sort dimension")
            .long("mode")
            .short("m")
            .takes_value(true)
            .default_value("red")
            .possible_values(&["red", "green", "blue", "alpha",
                               "hue", "sat", "lig"]))
        .arg(Arg::with_name("INFILE")
            .help("Input image")
            .required(true))
        .arg(Arg::with_name("OUTFILE")
            .help("Output image")
            .required(true))
        .get_matches_from(args);

    let instr = matches.value_of("INFILE").unwrap();
    let outstr = matches.value_of("OUTFILE").unwrap();
    let mode = match matches.value_of("mode").unwrap() {
        "red" => Mode::Red,
        "green" => Mode::Green,
        "blue" => Mode::Blue,
        "alpha" => Mode::Alpha,
        "hue" => Mode::Hue,
        "sat" => Mode::Saturation,
        "lig" => Mode::Lightness,
        _ => panic!("Shouldn't happen?")
    };

    Options { inpath: PathBuf::from(instr),
              outpath: PathBuf::from(outstr),
              mode: mode }
}
