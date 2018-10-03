use std::error::Error;
use std::fs::File;
use std::io::Read;

// Config with which the file pointed to by 'file_path' is to be skewed by 'skew_millis' milliseconds.
pub struct Config {
    file_path: String,
    skew_millis: i64,
}

impl Config {
    pub fn new(file_path: String, skew_millis: i64) -> Config {
        Config {
            file_path,
            skew_millis,
        }
    }
}

// Run the skew based on the config.
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("Read contents: {}", contents);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
