use rand::prelude::ThreadRng;
use rand_distr::Exp;

use crate::customer::Customer;
use std::collections::VecDeque;
use crate::queues::QueueEvent::{Departure, Arrival};

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
        let mut customer = Customer::first(&mut rng, &customer_service_distribution);

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
        }
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