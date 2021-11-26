use std::io::{Error, Write, stdout};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use structopt::StructOpt;

use queues::queues::{Queue, QueueEvent};
use std::fs::File;

const MINUTE: f64 = 60.;
const HOUR: f64 = 60. * MINUTE;

#[derive(StructOpt)]
struct Cli {
    /// The number of samples to generate.
    #[structopt(short, long, default_value = "1000000")]
    samples: usize,
    /// The path to the file to output to.
    #[structopt(short, long, parse(from_os_str))]
    path: Option<std::path::PathBuf>,
    /// The interarrival rate of customers: lambda
    #[structopt(short, long, default_value = "5.8")]
    customers_per_hour: f64,
    /// The average time it takes to provide service to a customer: mu
    #[structopt(short =  "mu", long, default_value = "10.")]
    customer_service_time_in_minutes: f64,
}

impl Cli {
    pub fn lambda(&self) -> f64 {
        self.customers_per_hour / HOUR
    }

    pub fn mu(&self) -> f64 {
        1. / (self.customer_service_time_in_minutes * MINUTE)
    }

}

fn main() -> Result<(), Error> {
    let terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate))?;

    let cli: Cli = Cli::from_args();

    let mut queue = Queue::new_exp_exp(cli.lambda(), cli.mu());

    if let Some(path) = &cli.path {
        let mut file = File::create(path).unwrap();
        simulate(&mut file, terminate, cli, queue);
    } else {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        simulate(&mut stdout, terminate, cli, queue);
    }

    Ok(())
}

fn simulate<OUT: Write>(out: &mut OUT, terminate: Arc<AtomicBool>, cli: Cli, mut queue: Queue) {
    let mut samples = 0;
    let mut a: u64 = 0;
    let mut d: u64 = 0;
    writeln!(out, "# {{\"lambda\":{}, \"mu\":{}}}", cli.lambda(), cli.mu());
    writeln!(out, "# time(s) arrivals departures in_system");
    while !terminate.load(Ordering::Relaxed) && samples < cli.samples {
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