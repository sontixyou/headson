use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub lines: u64,
    pub bytes: Option<u64>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let cli: ArgMatches = Command::new("headson")
        .author("sontixyou")
        .version("0.0.1")
        .about("Print the first part of files")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Files to read")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .value_parser(clap::value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .value_parser(clap::value_parser!(u64).range(1..))
                .help("Number of bytes"),
        )
        .get_matches();

    Ok(Config {
        files: cli
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        lines: cli.get_one("lines").cloned().unwrap(),
        bytes: cli.get_one("bytes").cloned(),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
