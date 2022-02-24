use std::{error::Error, fmt::Debug};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Kevin Phillips <thefriendlycoder@gmail.com>")
        .about("Rust Cat")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("number nonblank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    if matches.is_present("number") && matches.is_present("number_nonblank") {
        panic!("error: The argument '--number-nonblank' cannot be used with '--number'")
    }
    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
