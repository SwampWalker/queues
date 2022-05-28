use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryFrom;
use queues::queues::{EventAnalyser, QueueEvent};
use queues::errors::ApplicationError;

#[derive(StructOpt)]
struct Cli {
    /// The path to the output of a queue counts.
    #[structopt(short, long, parse(from_os_str))]
    path: std::path::PathBuf,
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