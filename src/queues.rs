use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Read};
use std::panic::panic_any;
use std::str::SplitAsciiWhitespace;

use rand::prelude::ThreadRng;
use rand_distr::Exp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::customer::Customer;
use crate::queues::QueueEvent::{Arrival, Departure};

// TODO: if I continue down this road, that Exp type will need to be parameterized, probably ThreadRng as well.
/// So, here's the plan:
///
/// We are going to simulate a queue by marching time forward. We will expose multiple abstractions
/// and methods but just use this basic structure.
///
/// 1. If the next customer is not known, generate a customer.
/// 2.a. If we want the next customer, set time to the arrival time of that customer and fix up the queues (serve customers). Return the generated customer (copy).
/// 2.b. If we want the next customer, find the minimum of the arrival time and the departure times of the customers being served. Set time to that, return that customer. Store the arrival if it is not being added to the queue.
///
/// At any time we can switch between checking customer and checking events.
pub struct Queue {
    /// This one is really referred to as lambda in both distribution and markov model.
    customer_arrival_lambda: f64,
    /// This one is referred to as mu in the birth/death markov model but goes in as lambda in the exponential distribution.
    customer_service_lambda: f64,

    customer_arrival_distribution: Exp<f64>,
    customer_service_distribution: Exp<f64>,
    rng: ThreadRng,

    queue: VecDeque<Customer>,
    in_service: Vec<Customer>,
    time: f64,
    next_customer: Customer,
}

pub enum QueueEvent {
    Arrival(f64, Customer),
    Departure(f64, Customer),
}

impl Queue {
    pub fn new_exp_exp(customer_arrival_rate: f64, customer_service_rate: f64) -> Queue {
        let customer_service_distribution = Exp::new(customer_service_rate).unwrap();
        let mut rng = rand::thread_rng();
        let customer = Customer::first(&mut rng, &customer_service_distribution);

        Queue {
            customer_arrival_lambda: customer_arrival_rate,
            customer_service_lambda: customer_service_rate,

            customer_arrival_distribution: Exp::new(customer_arrival_rate).unwrap(),
            customer_service_distribution,
            rng,

            queue: VecDeque::new(),
            in_service: Vec::new(),
            time: 0.,
            next_customer: customer,
        }
    }

    pub fn next_event(&mut self) -> QueueEvent {
        let (index, departure_time) = self.next_departure();

        let next_arrival_time = self.next_customer.arrival_time();

        return if departure_time < next_arrival_time {
            // A customer in service departs before the next customer arrives.
            let served_customer = self.in_service.remove(index);

            // TODO: this is a function of queue discipline, this is first come first served.
            let next_to_be_served = self.queue.pop_front();

            if let Some(customer) = next_to_be_served {
                // TODO: update customer departure here, compute other quantities. The generator below assumes first-come-first-served.
                self.in_service.push(customer);
            }
            self.time = departure_time;

            Departure(self.time, served_customer)
        } else {
            let arriving_customer = self.next_customer;

            // TODO: this is the 1 in M/M/1...
            if self.in_service.len() >= 1 {
                self.queue.push_back(arriving_customer);
            } else {
                self.in_service.push(arriving_customer);
            }
            self.next_customer = Customer::next_1_fcfs(&mut self.rng, &self.customer_arrival_distribution, &self.customer_service_distribution, arriving_customer);
            self.time = next_arrival_time;

            Arrival(self.time, arriving_customer)
        };
    }

    /// Returns the time of the next departure and the index of the customer in the in_service vector.
    ///
    /// When there are no customers in service, time is set to infinity and the index shouldn't be used.
    fn next_departure(&self) -> (usize, f64) {
        let mut departure_time = f64::INFINITY;
        let mut index = 0;
        for (i, customer) in self.in_service.iter().enumerate() {
            if customer.departure_time() < departure_time {
                departure_time = customer.departure_time();
            }
            index = i;
        }

        return (index, departure_time);
    }
}

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Failed to read line.")]
    LineReading(#[from] std::io::Error),
    #[error("Failed to deserialize parameters from file header")]
    ParameterReading(#[from] serde_json::Error),
    #[error("Failed to parse a value from a Counts line")]
    LineParsing(String),
}

const COLUMNS: [&str; 4] = [
    "time(s)",
    "arrivals",
    "departures",
    "in_system"
];

const I_TIME: usize = 0;
const I_ARRIVALS: usize = 1;
const I_DEPARTURES: usize = 2;
const I_IN_SYSTEM: usize = 3;

#[derive(Deserialize, Serialize)]
pub struct Parameters {
    lambda: f64,
    mu: f64,
}

#[derive(Default)]
pub struct CountAnalyser {
    lambda: f64,
    mu: f64,

    last_service_start: f64,
    n_served: u64,
    service_time_sum: f64,
    last_arrival: f64,
    n_arrivals: u64,
    arrival_time_sum: f64,

    last_n: u64,
    time_in_n: HashMap<u64, f64>,
    time_of_last_event: f64,
}

impl CountAnalyser {
    pub fn new<R: Read>(reader: &mut BufReader<R>) -> Result<CountAnalyser, QueueError> {
        let mut line_0 = String::new();
        let _ = reader.read_line(&mut line_0).map_err(|e| QueueError::LineReading(e))?;

        // Sanity check header line.
        let mut line_1 = String::new();
        let _ = reader.read_line(&mut line_1).map_err(|e| QueueError::LineReading(e))?;

        let params: Parameters = serde_json::from_str(&line_0[1..]).map_err(|e| QueueError::ParameterReading(e))?;

        let expected = format!("# {} {} {} {}\n", COLUMNS[I_TIME], COLUMNS[I_ARRIVALS], COLUMNS[I_DEPARTURES], COLUMNS[I_IN_SYSTEM]);
        assert_eq!(expected, line_1);

        let mut analyser = CountAnalyser::default();
        analyser.lambda = params.lambda;
        analyser.mu = params.mu;

        Ok(analyser)
    }

    pub fn add_count(&mut self, count: Count) {
        if !self.time_in_n.contains_key(&self.last_n) {
            self.time_in_n.insert(self.last_n, 0.);
        }
        let delta_t = count.time - self.time_of_last_event;
        let new_time_in_n = self.time_in_n.get(&self.last_n).unwrap() + delta_t;
        self.time_in_n.insert(self.last_n, new_time_in_n);

        if count.in_system < self.last_n {
            // Departure.
            let service_time = count.time - self.last_service_start;
            self.n_served += 1;
            self.service_time_sum += service_time;
            self.last_service_start = count.time;
        } else {
            // Arrival.
            let interarrival_time = count.time - self.last_arrival;
            self.n_arrivals += 1;
            self.arrival_time_sum += interarrival_time;
            self.last_arrival = count.time;

            if self.last_n == 0 {
                self.last_service_start = count.time;
            }
        }

        self.last_n = count.in_system;
        self.time_of_last_event = count.time;
    }

    pub fn analysis(&self) -> CountAnalysis {
        let sample_lambda = self.n_arrivals as f64 / self.arrival_time_sum;
        let sample_mu = self.n_served as f64 / self.service_time_sum;

        let mut proportions = HashMap::new();
        for (n, time_in_n) in &self.time_in_n {
            proportions.insert(*n, time_in_n / self.time_of_last_event);
        }

        CountAnalysis {
            lambda: self.lambda,
            sample_lambda,
            mu: self.mu,
            sample_mu,
            proportions
        }
    }
}

pub struct Count {
    time: f64,
    arrivals: u64,
    departures: u64,
    in_system: u64,
}

impl TryFrom<String> for Count {
    type Error = QueueError;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let mut tokens: Vec<&str> = line.split_ascii_whitespace().into_iter().collect();

        Ok(Count {
            time: parse(&tokens, I_TIME)?,
            arrivals: parse(&tokens, I_ARRIVALS)?,
            departures: parse(&tokens, I_DEPARTURES)?,
            in_system: parse(&tokens, I_IN_SYSTEM)?,
        })
    }
}

fn parse<T: std::str::FromStr>(tokens: &Vec<&str>, index: usize) -> Result<T, QueueError> {
    let maybe_token = tokens.get(index);
    if maybe_token.is_none() {
        let msg = format!("Not enough tokens to read {}.", COLUMNS[index]);
        return Err(QueueError::LineParsing(msg));
    }

    let value_or_parse_error = maybe_token.unwrap().parse::<T>();
    if let Ok(value) = value_or_parse_error {
        return Ok(value);
    }

    let msg = format!("Couldn't parse {} as {}", maybe_token.unwrap(), COLUMNS[index]);
    return Err(QueueError::LineParsing(msg));
}

pub struct CountAnalysis {
    lambda: f64,
    sample_lambda: f64,
    mu: f64,
    sample_mu: f64,
    proportions: HashMap<u64, f64>,
}

impl CountAnalysis {
    /// The expected values are only valid for M/M/1...
    pub fn dump_proportions(&self) {
        let rho = self.lambda / self.mu;
        println!("n measured_p_n p_n");
        for n in 0u64..self.proportions.len() as u64 {
            let expected_p_n = (1. - rho) * rho.powi(n as i32);
            println!("{} {} {}", n, self.proportions.get(&n).unwrap(), expected_p_n);
        }
    }

    pub fn dump_cross_sectional_statistics(&self) {
        println!("lambda: sample = {}, input = {}", self.sample_lambda, self.lambda);
        println!("mu: sample = {}, input = {}", self.sample_mu, self.mu);

        let mut steady_customers_count = 0.;
        for n in 0u64..self.proportions.len() as u64 {
            steady_customers_count += n as f64 * self.proportions.get(&n).unwrap();
        }
        let ell = self.lambda / (self.mu - self.lambda);
        println!("Average number in Queue, L: sample = {}, expected = {}", steady_customers_count, ell);
    }
}