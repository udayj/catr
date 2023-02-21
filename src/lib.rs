use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {

    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {

    match filename {

        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run(config: Config) -> MyResult<()> {

    for filename in config.files {
        
        match open(&filename) {
            Err(err) => 
                eprintln!("Failed to open {}: {}",&filename, err),
            
            Ok(reader) => {
                let mut count_nonblank_lines = 1;
                for (line_num, line) in reader.lines().enumerate() {
                    let val = line.unwrap();
                    if config.number_lines {
                        println!("{:>6}\t{}",line_num, val);
                    }
                    else if config.number_nonblank_lines {
                        println!("{:>6}\t{}",count_nonblank_lines, val);
                        count_nonblank_lines +=1;
                    }
                    else {
                        println!("{}",val);
                    }
                    
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("catr")
                            .version("0.1.0")
                            .author("udayj")
                            .about("Rust cat")
                            .arg(
                                Arg::with_name("files")
                                    .value_name("FILE")
                                    .help("Input file(s)")
                                    .multiple(true)
                                    .default_value("-")
                            )
                            .arg(

                                Arg::with_name("number")
                                    .short("n")
                                    .long("number")
                                    .help("Number lines")
                                    .takes_value(false)
                                    .conflicts_with("number_nonblank")

                            )
                            .arg(
                                Arg::with_name("number_nonblank")
                                    .short("b")
                                    .long("number-nonblank")
                                    .help("Number non-blank lines")
                                    .takes_value(false)

                            )
                            .get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number");
    let number_nonblank_lines = matches.is_present("number_nonblank");

    Ok(
        Config {
            files: files,
            number_lines: number_lines,
            number_nonblank_lines: number_nonblank_lines
        }
    )
    
}