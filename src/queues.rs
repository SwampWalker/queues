use rand::prelude::ThreadRng;
use rand_distr::Exp;

use crate::customer::Customer;
use std::collections::VecDeque;

// TODO: if I continue down this road, that Exp type will need to be parameterized, probably ThreadRng as well.
pub struct Queue {
    /// This one is really referred to as lambda in both distribution and markov model.
    customer_arrival_lambda: f64,
    /// This one is referred to as mu in the birth/death markov model.
    customer_service_lambda: f64,

    customer_arrival_distribution: Exp<f64>,
    customer_service_distribution: Exp<f64>,
    rng: ThreadRng,

    queue: VecDeque<Customer>,
    in_service: Vec<Customer>,
    time: f64,
}

impl Queue {
    pub fn new_exp_exp(customer_arrival_rate: f64, customer_service_rate: f64) -> Queue {
        Queue {
            customer_arrival_lambda: customer_arrival_rate,
            customer_service_lambda: customer_service_rate,

            customer_arrival_distribution: Exp::new(customer_arrival_lambda).unwrap(),
            customer_service_distribution: Exp::new(customer_service_lambda).unwrap(),
            rng: rand::thread_rng(),

            queue: VecDeque::new(),
            in_service: Vec::new(),
            time: f64,
        }
    }

    // TODO: a method which gets the next event as a tagged union of Arrival/Departure.
}