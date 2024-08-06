use clap::{Arg, ArgAction, ArgMatches, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
    pub lines: usize,
    pub bytes: Option<usize>,
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
                .value_name("LINES")
                .help("print the first K lines insterd of the first 10")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .value_name("BYTES")
                .help("print the first K bytes of each file"),
        )
        .get_matches();

    Ok(Config {
        files: cli
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        lines: ,
        bytes,
    })
}
