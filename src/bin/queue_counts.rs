use std::io::{Error, Write, stdout};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;

use queues::queues::{Queue, QueueEvent};
use std::fs::File;

#[derive(StructOpt)]
struct Cli {
    /// The number of samples to generate.
    #[structopt(short, long, default_value = "1000000")]
    samples: usize,
    /// The path to the file to output to.
    #[structopt(short, long, parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Error> {
    let terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate))?;

    let args: Cli = Cli::from_args();

    let minute = 60.;
    let hour = 60. * minute;
    let customer_arrival_lambda = 5.8 / hour;

    let mean_time_to_serve_customer = 10. * minute;
    let customer_service_lambda = 1. / mean_time_to_serve_customer;

    let mut queue = Queue::new_exp_exp(customer_arrival_lambda, customer_service_lambda);

    if let Some(path) = args.path {
        let mut file = File::create(path).unwrap();
        simulate(&mut file, terminate, args.samples, queue);
    } else {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        simulate(&mut stdout, terminate, args.samples, queue);
    }

    Ok(())
}

fn simulate<OUT: Write>(out: &mut OUT, terminate: Arc<AtomicBool>, n_samples: usize, mut queue: Queue) {
    let mut samples = 0;
    let mut a: u64 = 0;
    let mut d: u64 = 0;
    writeln!(out, "# time(s) arrivals departures in_system");
    while !terminate.load(Ordering::Relaxed) && samples < n_samples {
        samples += 1;

        let event = queue.next_event();
        let time = match event {
            QueueEvent::Arrival(t, _) => {
                a += 1;
                t
            }
            QueueEvent::Departure(t, _) => {
                d += 1;
                t
            }
        };

        writeln!(out, "{} {} {} {}", time, a, d, a - d);
    }
}