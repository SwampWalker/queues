use rand_distr::Exp;

use rand::prelude::*;
use queues::customer::Customer;

fn main() {
    let minute = 60.;
    let hour = 60. * minute;
    let customer_arrival_lambda = 5.8 / hour;

    let mean_time_to_serve_customer = 10. * minute;
    let customer_service_lambda = 1. / mean_time_to_serve_customer;

    let customer_arrival_distribution = Exp::new(customer_arrival_lambda).unwrap();
    let customer_service_distribution = Exp::new(customer_service_lambda).unwrap();

    let mut rng = rand::thread_rng();

    let mut customer = Customer::first(&mut rng, &customer_service_distribution);
    let mut total_wq = customer.wq();
    let mut total_wq_square = customer.wq() * customer.wq();
    let mut total_w = customer.w();
    let mut total_w_square = customer.w() * customer.w();
    let mut n_samples = 1;
    let output_frequency = 50_000;
    loop {
        customer = Customer::next_1_fcfs(&mut rng, &customer_arrival_distribution, &customer_service_distribution, customer);

        total_wq += customer.wq();
        total_w += customer.w();
        total_wq_square += customer.wq() * customer.wq();
        total_w_square += customer.w() * customer.w();
        n_samples += 1;

        let n = n_samples as f64;
        let average_wq = total_wq / n;
        let deviation_wq = ((n * total_wq_square - total_wq * total_wq) / (n * (n - 1.))).sqrt();
        let average_w = total_w / (n_samples as f64);
        let deviation_w = ((n * total_w_square - total_w * total_w) / (n * (n - 1.))).sqrt();

        if n_samples % output_frequency == 0 {
            println!("<W_q> = {} ± {}, <W> = {} ± {} (n = {})",
                     human_readable(average_wq), human_readable(deviation_wq),
                     human_readable(average_w), human_readable(deviation_w),
                     n_samples,
            );
        }
    }
}

fn human_readable(seconds: f64) -> String {
    return if seconds < 2. * 60. {
        // Less than two minutes, use seconds.
        format!("{:.2} sec", seconds)
    } else if seconds < 2. * 60. * 60. {
        // Less than two hours, use minutes.
        format!("{:.2} min", seconds / 60.)
    } else if seconds < 2. * 60. * 60. * 24. {
        format!("{:.2} hr", seconds / (60. * 60.))
    } else {
        // Use days.
        format!("{:.2} hr", seconds / (60. * 60. * 24.))
    }
}