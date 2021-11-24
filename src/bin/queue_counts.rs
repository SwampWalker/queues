use queues::queues::{Queue, QueueEvent};

fn main() {
    let minute = 60.;
    let hour = 60. * minute;
    let customer_arrival_lambda = 5.8 / hour;

    let mean_time_to_serve_customer = 10. * minute;
    let customer_service_lambda = 1. / mean_time_to_serve_customer;

    let mut queue = Queue::new_exp_exp(customer_arrival_lambda, customer_service_lambda);

    let mut a: u64 = 0;
    let mut d: u64 = 0;
    println!("# time(s) arrivals departures in_system");
    loop {
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

        println!("{} {} {} {}", time, a, d, a - d);
    }
}