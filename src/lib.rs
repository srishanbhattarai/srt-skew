use std::error::Error;
use std::fs::File;
use std::io::Read;

// Subtitle durations are separated by this sequence of chars.
const DURATION_SEP: &str = "-->";

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

    let skewed_file_contents = skew_file_contents(contents, config.skew_millis);

    // Rename file to a backup name.
    // Write back to disk with skewed_file_contents

    Ok(())
}

// Skew a file by it's contents. Returns the contents of the file after the skew is complete.
fn skew_file_contents(contents: String, millis: i64) -> String {
    let lines: Vec<&str> = contents.split("\n").collect();
    for line in lines {
        if line.contains(DURATION_SEP) {
            let mut durations: Vec<&str> = line.split(DURATION_SEP).collect();
            let end = durations.pop().unwrap();
            let start = durations.pop().unwrap();

            skew_duration(end.to_string(), millis);
            skew_duration(start.to_string(), millis);
        }
    }

    return "".to_string();
}

// Adjust an SRT duration by millis milliseconds.
// A duration is in the format
//   00:01:19,740 i.e. {hours}:{mins}:{seconds},{milliseconds}
fn skew_duration(duration: String, millis: i64) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use skew_duration;
    use skew_file_contents;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn skew_file_contents_by_10() {
        let input_contents = "
1
00:01:16,820 --> 00:01:19,660
I believe in America.

2
00:01:19,740 --> 00:01:22,700
America has made my fortune.
";

        let expected_contents = "
1
00:01:16,830 --> 00:01:19,670
I believe in America.

2
00:01:19,750 --> 00:01:22,710
America has made my fortune.
";
        let millis = 10;
        let contents = skew_file_contents(input_contents.to_string(), millis);

        assert_eq!(contents, expected_contents)
    }

    #[test]
    fn skew_duration_by_10() {
        let duration = "00:01:16,830";
        let expected = "00:01:16,930";
        let millis = 100;

        assert_eq!(skew_duration(duration.to_string(), millis), expected)
    }

    #[test]
    fn skew_duration_when_overflow_secs() {
        let duration = "00:01:16,830";
        let expected = "00:01:18,330";
        let millis = 1500;

        assert_eq!(skew_duration(duration.to_string(), millis), expected)
    }
}
