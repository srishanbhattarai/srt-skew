use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};

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
    // Read the input file
    let mut file = fs::File::open(config.file_path.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Evaluate new data for the file.
    let skewed_file_contents = skew_file_contents(contents, config.skew_millis);

    // Backup the input file
    let now = timestamp_now()?.as_secs();
    let backup_file_name = format!("{}-{}", now, config.file_path.clone());
    fs::rename(config.file_path.clone(), backup_file_name)?;

    // Re-create the input file with the same name and new contents.
    let mut file = fs::File::create(config.file_path)?;
    file.write_all(skewed_file_contents.as_bytes())?;

    Ok(())
}

fn timestamp_now() -> Result<Duration, SystemTimeError> {
    return SystemTime::now().duration_since(UNIX_EPOCH);
}

// Skew a file by it's contents. Returns the contents of the file after the skew is complete.
fn skew_file_contents(contents: String, millis: i64) -> String {
    let mut adjusted_file: Vec<String> = Vec::new();

    let lines: Vec<&str> = contents.split("\n").collect();
    for line in lines {
        if line.contains(DURATION_SEP) {
            let mut durations: Vec<&str> = line.split(DURATION_SEP).collect();
            let end = durations.pop().unwrap();
            let start = durations.pop().unwrap();

            let new_end = skew_duration(end.trim().to_string(), millis);
            let new_start = skew_duration(start.trim().to_string(), millis);
            let new_duration = format!("{} {} {}", new_start, DURATION_SEP, new_end);

            adjusted_file.push(new_duration)
        } else {
            adjusted_file.push(line.to_string())
        }
    }

    adjusted_file.join("\n")
}

// Adjust an SRT duration by millis milliseconds.
// A duration is in the format
//   00:01:19,740 i.e. {hours}:{mins}:{seconds},{milliseconds}
fn skew_duration(duration: String, skew: i64) -> String {
    let mut duration_vec: Vec<&str> = duration.split(":").collect();
    let mut secs_with_millis: Vec<&str> = duration_vec.pop().unwrap().split(",").collect();

    let mut minutes = duration_vec.pop().unwrap().parse::<i64>().unwrap();
    let mut hours = duration_vec.pop().unwrap().parse::<i64>().unwrap();
    let mut millis = secs_with_millis.pop().unwrap().parse::<i64>().unwrap();
    let mut seconds = secs_with_millis.pop().unwrap().parse::<i64>().unwrap();

    millis += skew;
    seconds += millis / 1000;
    millis = millis % 1000;
    minutes += seconds / 60;
    seconds = seconds % 60;
    hours += minutes / 60;
    minutes = minutes % 60;

    let new_duration = format!(
        "{:0padding$}:{:0padding$}:{:0padding$},{:0padding_millis$}",
        hours,
        minutes,
        seconds,
        millis,
        padding = 2,
        padding_millis = 3
    );

    new_duration.to_string()
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
