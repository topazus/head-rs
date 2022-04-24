use clap::Arg;
use clap::Command;

use std::error::Error;
use std::io::BufRead;
use std::io::Read;

// MyResult, which is defined as either an Ok<T> for any type T
// or something that implements the Error trait and which is stored in a Box:
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: isize,
    bytes: Option<isize>,
}
fn parse_args() {
    let matches = Command::new("headr")
        .version("0.1.0")
        .about("rust version of head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .default_value("-")
                .multiple_values(true)
                .allow_invalid_utf8(true)
                .help("Files to read"),
        )
        .get_matches();
    let files_str = matches.values_of("files").unwrap().collect::<Vec<&str>>();
    let files_string = matches
        .values_of("files")
        .unwrap()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .about("rust version of head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .takes_value(true)
                .default_value("-")
                .multiple_values(true)
                .allow_invalid_utf8(true)
                .help("Files to read"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("[-]NUM")
                .takes_value(true)
                .default_value("10")
                .allow_invalid_utf8(true)
                .allow_hyphen_values(true)
                .help(
                    "print the first NUM lines instead of the first 10;
                with the leading '-', print all but the last
                NUM lines of each file",
                ),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("[-]NUM")
                .takes_value(true)
                .allow_hyphen_values(true)
                .allow_invalid_utf8(true)
                .help(
                    "print the first NUM bytes of each file;
                with the leading '-', print all but the last
                NUM bytes of each file",
                )
                .conflicts_with("lines"),
        )
        .get_matches();

    // attention!: value_of_lossy and values_of_lossy
    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))
        .unwrap()
        .unwrap();
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))
        .unwrap();

    Ok(Config {
        // files is a Vec<String>
        files,
        lines,
        bytes,
    })
}

fn open(file: &str) -> MyResult<Box<dyn std::io::BufRead>> {
    match file {
        "-" => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(std::io::BufReader::new(
            std::fs::File::open(file)
                .map_err(|_| format!("headr: {}: No such file or directory", file))?,
        ))),
    }
}
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_index, file_name) in config.files.iter().enumerate() {
        match open(file_name) {
            Err(e) => eprintln!("{}: {}", file_name, e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{} {}", if file_index > 0 { "\n" } else { "" }, file_name);
                }
                if let Some(num_bytes) = config.bytes {
                    if num_bytes > 0 {
                        // read num_bytes bytes from file
                        let mut file_bytes = file.take(num_bytes as u64);
                        // create a buffer to read the bytes into
                        let mut buf = vec![0; num_bytes.try_into().unwrap()];
                        // read the bytes into the buffer
                        let read_bytes = file_bytes.read(&mut buf).unwrap();
                        println!("{}", String::from_utf8_lossy(&buf[..read_bytes]));
                    } else if num_bytes < 0 {
                        let mut contents = Vec::new();
                        let total_bytes = file.read_to_end(&mut contents).unwrap();
                        let take_bytes = -num_bytes as usize;
                        println!(
                            "{}",
                            String::from_utf8_lossy(&contents[(total_bytes - take_bytes)..])
                        );
                    }
                } else {
                    if config.lines > 0 {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            let bytes = file.read_line(&mut line).unwrap();
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    } else if config.lines < 0 {
                        let lines = file
                            .lines()
                            .map(|line| line.unwrap())
                            .collect::<Vec<String>>();
                        let take_lines = -config.lines as usize;
                        for i in (lines.len() - take_lines)..lines.len() {
                            println!("{}", lines[i]);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn read_lines_file(file_name: &str) -> Vec<String> {
    let mut file = std::io::BufReader::new(std::fs::File::open(file_name).unwrap());
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let lines = text
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    lines
}
fn read_lines_file2(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = std::fs::File::open(file_name)?;
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let lines = text
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    Ok(lines)
}
#[test]
fn test_read_lines_file() {
    assert_eq!(read_lines_file("tests/inputs/empty.txt").is_empty(), true);
    assert_eq!(
        read_lines_file("tests/inputs/three-lines.txt"),
        vec!["line 1", "line 2", "line 3"]
    );
    assert_eq!(
        read_lines_file2("tests/inputs/empty.txt")
            .unwrap()
            .is_empty(),
        true
    );
    assert_eq!(
        read_lines_file2("tests/inputs/non-exist.txt").is_err(),
        true
    );
    assert_eq!(
        read_lines_file2("tests/inputs/three-lines.txt").unwrap(),
        vec!["line 1", "line 2", "line 3"]
    );
}
fn read_bytes_from_file(bytes: usize) {
    let mut file = std::fs::File::open("Cargo.toml").unwrap();
    let mut buf = vec![0; bytes];
    // use file.take() needs import std::io::Read
    let mut take_bytes = file.take(bytes as u64);
    let bytes_read = take_bytes.read(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
}
fn read_bytes_from_file2(bytes: usize) {
    let mut file = std::fs::File::open("Cargo.toml").unwrap();
    let mut contents = String::new();
    let bytes_read = file.read_to_string(&mut contents).unwrap();
    let contents_bytes = contents.as_bytes();
    println!("{}", String::from_utf8_lossy(&contents_bytes[..bytes]));
}
fn read_bytes_from_file3(bytes: usize) {
    let mut file = std::fs::File::open("Cargo.toml").unwrap();
    let mut buf = String::new();
    let bytes_read = file
        .bytes()
        .take(bytes)
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    println!("{}", String::from_utf8_lossy(&bytes_read));
}

fn parse_positive_int(val: &str) -> MyResult<isize> {
    // Rust infers the isize type from the return type.
    match val.parse() {
        Ok(n) => Ok(n),
        // convert &str into an Error
        // From::from() is a conversion from &str to Box<dyn Error>
        // or val.into()
        // or Into::into()
        _ => Err(From::from(val)),
    }
}
#[test]
fn test_parse_positive_int() {
    assert_eq!(parse_positive_int("1").unwrap(), 1);
    assert_eq!(parse_positive_int("0").unwrap(), 0);
    assert_eq!(parse_positive_int("-1").unwrap(), -1);
    assert_eq!(
        parse_positive_int("foo").unwrap_err().to_string(),
        "foo".to_string()
    );
}

fn parse_positive_int2(val: &str) -> Result<isize, &str> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(val)),
    }
}
fn parse_positive_int3(val: &str) -> Result<isize, &str> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(val),
    }
}

#[test]
fn test_parse_positive_int2() {
    assert_eq!(parse_positive_int2("1").unwrap(), 1);
    assert_eq!(parse_positive_int2("0").unwrap(), 0);
    assert_eq!(parse_positive_int2("-1").unwrap(), -1);
    assert_eq!(parse_positive_int2("foo").unwrap_err(), "foo");
}

#[test]
fn test_parse_positive_int3() {
    assert_eq!(parse_positive_int3("1").unwrap(), 1);
    assert_eq!(parse_positive_int3("0").unwrap(), 0);
    assert_eq!(parse_positive_int3("-1").unwrap(), -1);
    assert_eq!(parse_positive_int3("foo").unwrap_err(), "foo");
}
