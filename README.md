# srt-skew

> Skew an SRT file by a number of milliseconds.

 This is helpful when the subtitles are slightly out of sync with the video and must be skewed forward or backward.

 ## Build
 You need the Rust toolchain to build the binary.
```
$ cargo build --release
```

Use the generated binary in `target/release/srt-skew` or move it to `$PATH`

## Usage
Run `srt-skew --help` to get all the options.

For example, to skew a file forwards by 100 milliseconds:
```
$ srt-skew --file=/path/to/srt --millis=100
```

To skew a file backwards by 300 milliseconds:
```
$ srt-skew --file=/path/to/srt --millis=-300
```

The old file, before any adjustments, will be backed up with the current timestamp appended to the filename.

## License
MIT