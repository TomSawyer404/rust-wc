use clap::{App, Arg};
use std::fs::File;
use std::{error::Error, io::BufRead, io::BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool, // `wc -l` or `wc --lines`
    words: bool, // `wc -w` or `wc --words`
    bytes: bool, // `wc -c` or `wc --bytes`
    chars: bool, // `wc -m` or `wc --chars`
}

#[derive(Debug)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn parse_args() -> Result<Config, Box<dyn Error>> {
    let matches = App::new("rust-wc")
        .version("1.0.0")
        .about("words count")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Please input your files")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("show the new line counts")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("show the word counts")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("show the byte counts")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("show the character counts")
                .takes_value(false),
        )
        .get_matches();

    let mut is_lines = matches.is_present("lines");
    let mut is_words = matches.is_present("words");
    let mut is_bytes = matches.is_present("bytes");
    let is_chars = matches.is_present("chars");
    if [is_lines, is_words, is_bytes, is_chars]
        .iter()
        .all(|x| x == &false)
    {
        is_lines = true;
        is_words = true;
        is_bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: is_lines,
        words: is_words,
        chars: matches.is_present("chars"),
        bytes: is_bytes,
    })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut total_lines: usize = 0;
    let mut total_bytes: usize = 0;
    let mut total_chars: usize = 0;
    let mut total_words: usize = 0;

    for filename in &config.files {
        match open_file(filename) {
            Err(e) => eprintln!("{:?}", e),
            Ok(file_handler) => {
                if let Ok(file_info) = count(file_handler) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(file_info.num_lines, config.lines),
                        format_field(file_info.num_words, config.words),
                        format_field(file_info.num_bytes, config.bytes),
                        format_field(file_info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", &filename)
                        }
                    );

                    total_bytes += file_info.num_bytes;
                    total_lines += file_info.num_lines;
                    total_words += file_info.num_words;
                    total_chars += file_info.num_chars;
                }
            }
        }
    }

    // Display total status of this file
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }

    Ok(())
}

/// Given name of a file and we return its file handler
fn open_file(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}

/// Return a formatted string
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!(" {:<2} ", value)
    } else {
        "".to_string()
    }
}

/// Given a file, and we return `FileInfo` struct with every field calculated
fn count(mut file: impl BufRead) -> Result<FileInfo, Box<dyn Error>> {
    let mut num_lines: usize = 0;
    let mut num_words: usize = 0;
    let mut num_bytes: usize = 0;
    let mut num_chars: usize = 0;

    let mut line_buf = String::new();
    loop {
        let lines_bytes = file.read_line(&mut line_buf)?;
        if lines_bytes == 0 {
            break;
        }

        num_bytes += lines_bytes;
        num_lines += 1;
        num_chars += line_buf.chars().count();
        num_words += line_buf.split_whitespace().count();
        line_buf.clear();
    }

    Ok(FileInfo {
        num_words,
        num_lines,
        num_chars,
        num_bytes,
    })
}
