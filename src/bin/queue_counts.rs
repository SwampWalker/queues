use queues::queues::Queue;

fn main() {
    let minute = 60.;
    let hour = 60. * minute;
    let customer_arrival_lambda = 5.8 / hour;

    let mean_time_to_serve_customer = 10. * minute;
    let customer_service_lambda = 1. / mean_time_to_serve_customer;

    let mut queue = Queue::new_exp_exp_1(customer_arrival_lambda, customer_service_lambda);


}