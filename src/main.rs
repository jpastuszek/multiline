use cotton::prelude::*;
use multistream_batch::channel::multi_buf_batch::{MultiBufBatchChannel, Command};
use std::io::{BufReader, BufWriter};
use std::time::Duration;
use regex::Regex;
use Command::*;

/// Join multiple log lines into single line.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    logging: LoggingOpt,

    /// Regex to match stream ID to demultiplex ordered streams of log lines by
    #[structopt(long = "stream-id-pattern", short = "i")]
    stream_id_pattern: Option<String>,

    /// Regex to match first or last line of a message
    #[structopt(long = "message-pattern", short = "p")]
    message_pattern: String,

    /// Negate the pattern
    #[structopt(long = "negate", short = "n")]
    negate: bool,

    /// Match last line of single message instead of first
    #[structopt(long = "match-last", short = "l")]
    match_last: bool,

    /// Strip matched pattern from line
    #[structopt(long = "strip-pattern", short = "s")]
    strip_pattern: bool,

    /// String used to join the lines of a single message with
    #[structopt(long = "join", default_value = "/012", short = "j")]
    join: String,

    /// Maximum number of lines a single message can collect before flushing
    #[structopt(long = "max-size", default_value = "2000", short = "S")]
    max_size: usize,

    /// Maximum time duration in milliseconds a single message will be collecting lines for before flushing
    #[structopt(long = "max-duration", default_value = "200", short = "D")]
    max_duration_ms: u64,
}

fn main() -> Result<(), Problem> {
    let args = Cli::from_args();
    init_logger(&args.logging, vec![module_path!()]);

    let stream_id_regex = args.stream_id_pattern.map(|pattern| Regex::new(&pattern).or_failed_to("compile regex for stream-id-pattern"));
    let pattern = Regex::new(&args.message_pattern).or_failed_to("compile regex for pattern");
    let negate = args.negate;
    let match_last = args.match_last;
    let strip_pattern = args.strip_pattern;

    let mut mbatch = MultiBufBatchChannel::with_producer_thread(args.max_size, Duration::from_millis(args.max_duration_ms), args.max_size * 2, move |sender| {
        for line in BufReader::new(std::io::stdin()).lines().or_failed_to("read lines from STDIN") {
            let (stream_id, line) = if let Some(stream_id_regex) = stream_id_regex.as_ref() {
                let stream_id = stream_id_regex.find(&line).map(|m| m.as_str().to_owned());
                let line = stream_id_regex.replace(&line, "").into_owned();
                (stream_id, line)
            } else {
                (None, line)
            };

            let matched = pattern.is_match(&line);
            let line = if matched && strip_pattern {
                pattern.replace(&line, "").into_owned()
            } else {
                line
            };

            let matched = if negate { !matched } else { matched };

            debug!("[{:?}] ({:?}) {}", stream_id, matched, line);

            if match_last {
                sender.send(Append(stream_id.clone(), line)).unwrap();
                if matched {
                    sender.send(Flush(stream_id)).unwrap();
                }
            } else {
                if matched {
                    sender.send(Flush(stream_id.clone())).unwrap();
                }
                sender.send(Append(stream_id, line)).unwrap();
            }
        }
    });

    let mut stdout = BufWriter::new(std::io::stdout());

    loop {
        match mbatch.next() {
            Ok((key, lines)) => {
                if let Some(key) = key {
                    stdout.write_all(key.as_bytes())?;
                }
                for (i, line) in lines.enumerate() {
                    if i > 0 {
                        stdout.write_all(args.join.as_bytes())?;
                    }
                    stdout.write_all(line.as_bytes())?;
                }
                stdout.write(b"\n")?;
                stdout.flush()?;
            }
            Err(_) => return Ok(()),
        }
    }
}
