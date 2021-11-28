use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use serde_json::Value;
use std::convert::TryFrom;
use std::str::SplitAsciiWhitespace;
use queues::queues::{QueueEvent, EventAnalyser, QueueError};
use thiserror::Error;

#[derive(StructOpt)]
struct Cli {
    /// The path to the output of a queue counts.
    #[structopt(short, long, parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug, Error)]
enum ApplicationError {
    #[error("File does not exist.")]
    FileDoesNotExist(#[from] std::io::Error),
    #[error("File is not the expected format")]
    FormatError(#[from] QueueError),
    #[error("File is not the expected format")]
    LineFormatError(QueueError, String)
}

fn main() -> Result<(), ApplicationError>{
    let cli: Cli = Cli::from_args();
    let file = File::open(cli.path)?;
    let mut reader = BufReader::new(file);

    let mut analyser = EventAnalyser::new(&mut reader)?;
    for line in reader.lines() {
        let line = line?;
        let counts = QueueEvent::try_from(line.clone()).map_err(|e| ApplicationError::LineFormatError(e, line))?;
        analyser.add_count(counts);
    }

    let analysis = analyser.analysis();
    analysis.dump_proportions();
    println!();
    analysis.dump_cross_sectional_statistics();

    Ok(())
}