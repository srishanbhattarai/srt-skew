extern crate clap;
extern crate colored;
extern crate srt_skew;

use clap::{App, Arg};
use colored::*;

fn main() {
    let matches = App::new("srt-skew")
        .version("1.0")
        .author("Srishan Bhattarai <srishanbhattarai@gmail.com>")
        .about("Skew a subtitle file by a number of milliseconds")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Path to the SRT file")
                .takes_value(true)
                .required(true),
        ).arg(
            Arg::with_name("millis")
                .short("ms")
                .long("millis")
                .help("Milliseconds to skew by")
                .takes_value(true)
                .required(true),
        ).get_matches();

    // Since both file and millis are required, they can be safely unwrapped.
    let file = matches.value_of("file").unwrap();
    let millis = matches.value_of("millis").unwrap();
    let millis_i = millis.parse::<i64>();

    match millis_i {
        Ok(millis) => start_skew(file.to_string(), millis),
        Err(_e) => println!("Invalid millis value: {}. Please enter an integer.", millis),
    }
}

// Start the skew process after all validation of input is complete.
fn start_skew(file_path: String, skew_millis: i64) {
    let config = srt_skew::Config::new(file_path, skew_millis);

    match srt_skew::run(config) {
        Ok(_v) => {
            let msg = "Subtitles skewed successfully".green();
            println!("{}", msg);
        }
        Err(e) => eprintln!("Something went wrong: {:?}", e.to_string()),
    }
}
