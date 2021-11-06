use rand_distr::Exp;

use rand::prelude::*;

fn main() {
    let minute = 60.;
    let mean_time_between_customers = 5.8 * minute;
    let customer_arrival_lambda = 1. / mean_time_between_customers;

    let mean_time_to_serve_customer = 10. * minute;
    let customer_service_lambda = 1. / mean_time_to_serve_customer;

    let customer_arrival_distribution = Exp::new(customer_arrival_lambda).unwrap();
    let customer_service_distribution = Exp::new(customer_service_lambda).unwrap();

    let mut rng = rand::thread_rng();
}