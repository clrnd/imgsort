extern crate clap;

use self::clap::{Arg, App};
use std::path::PathBuf;

#[derive(Debug)]
pub enum Mode {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
pub struct Options {
    pub inpath: PathBuf,
    pub outpath: PathBuf,
    pub mode: Mode
}

pub fn parse() -> Options {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("mode")
            .help("Sort dimension")
            .long("mode")
            .short("m")
            .takes_value(true)
            .default_value("red")
            .possible_values(&["red", "green", "blue"]))
        .arg(Arg::with_name("INFILE")
            .help("Input image")
            .required(true))
        .arg(Arg::with_name("OUTFILE")
            .help("Output image")
            .required(true))
        .get_matches();

    let instr = matches.value_of("INFILE").unwrap();
    let outstr = matches.value_of("OUTFILE").unwrap();
    let mode = match matches.value_of("mode").unwrap() {
        "red" => Mode::Red,
        "green" => Mode::Green,
        "blue" => Mode::Blue,
        _ => panic!("Shouldn't happen?")
    };

    Options { inpath: PathBuf::from(instr),
              outpath: PathBuf::from(outstr),
              mode: mode }
}