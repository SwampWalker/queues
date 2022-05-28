use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::TryFrom;
use queues::queues::{QueueEvent};
use queues::errors::ApplicationError;

#[derive(StructOpt)]
struct Cli {
    /// The path to the output of a queue counts.
    #[structopt(short, long, parse(from_os_str))]
    path: std::path::PathBuf,
    /// The number of bins to use for analysing the distribution.
    #[structopt(short, long)]
    n_bins: usize,
    /// The width of the window over which to compute the running average.
    #[structopt(short, long)]
    window: f64,
}

fn main() -> Result<(), ApplicationError> {
    let cli: Cli = Cli::from_args();
    assert!(cli.n_bins > 0, "Must have more than one bin to view the distribution. Realistically you want many.");

    let file = File::open(cli.path)?;
    let mut reader = BufReader::new(file);

    let mut header = String::new();
    reader.read_line(&mut header).unwrap();
    header.clear();
    reader.read_line(&mut header).unwrap();

    // These are the ultimate things we will want to plot a distribution of: we map the reader into these.
    let mut average_waits = Vec::new();
    let mut waits = Vec::new();

    // These are intermediate variables used to compute the windowed average as we iterate over the reader.
    let mut sum_of_waits_in_window = 0.;
    let mut n_of_waits_in_window = 0;
    let mut beginning_of_window = 0.;

    // We use the maximum wait to determine the ultimate bin widths.
    let mut max_wait = 0.;

    for line in reader.lines() {
        let line = line?;
        let counts = QueueEvent::try_from(line.clone()).map_err(|e| ApplicationError::LineFormatError(e, line))?;
        if let Some(customer) = counts.served_customer() {
            if customer.time_of_departure() > beginning_of_window + cli.window {
                if n_of_waits_in_window > 0 {
                    let average = sum_of_waits_in_window / n_of_waits_in_window as f64;
                    average_waits.push(Average { wait: average, count: n_of_waits_in_window });
                    sum_of_waits_in_window = 0.;
                    n_of_waits_in_window = 0;
                }

                // Advance beginning of window so this customer will be within the window.
                while customer.time_of_departure() > beginning_of_window + cli.window {
                    beginning_of_window += cli.window;
                }
            }

            if customer.wait_in_system() > max_wait {
                max_wait = customer.wait_in_system();
            }

            waits.push(customer.wait_in_system());

            sum_of_waits_in_window += customer.wait_in_system();
            n_of_waits_in_window += 1;
        }
    }

    // Take the delta a bit larger than necessary to avoid the fencing issue.
    let delta = (max_wait * 1.001) / cli.n_bins as f64;

    let mut wait_counts: Vec<usize> = vec![0; cli.n_bins];
    for wait in waits {
        let i_bin = (wait / delta).floor() as usize;
        wait_counts[i_bin] += 1;
    }

    let mut averaged_wait_counts: Vec<usize> = vec![0; cli.n_bins];
    let mut averaged_wait_totals: Vec<usize> = vec![0; cli.n_bins];
    for average_wait in average_waits {
        let i_bin = (average_wait.wait / delta).floor() as usize;
        averaged_wait_counts[i_bin] += 1;
        averaged_wait_totals[i_bin] += average_wait.count;
    }

    // Print:

    println!("# window_left window_right waits averaged_waits averaged_wait_totals");
    for i in 0..cli.n_bins {
        let window_left = delta * i as f64;
        let window_right = delta * (i + 1) as f64;
        println!("{} {} {} {} {}", window_left, window_right, wait_counts[i], averaged_wait_counts[i], averaged_wait_totals[i]);
    }

    Ok(())
}

struct Average {
    wait: f64,
    count: usize,
}