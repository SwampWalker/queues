use rand::Rng;
use rand_distr::Distribution;

#[derive(Clone, Copy, Debug)]
pub struct ArrivingCustomer {
    interarrival_time: f64,
    time_of_arrival: f64,
    service_time: f64,
}

impl ArrivingCustomer {
    /// Generate the first customer in a simulation. This is not really dependent on anything
    /// but the service distribution since an empty queue means they will be served right away.
    ///
    /// # Arguments
    /// * `rng` The random number generator to sample with. Obviously mutable.
    /// * `service_time_distribution` The distribution of the service times.
    pub fn first<R: Rng + ?Sized, DS: Distribution<f64>>(
        rng: &mut R,
        service_time_distribution: &DS,
    ) -> ArrivingCustomer {
        let t_0 = 0.;
        let a_0 = 0.;
        let s_0 = service_time_distribution.sample(rng);

        ArrivingCustomer {
            interarrival_time: t_0,
            time_of_arrival: a_0,
            service_time: s_0,
        }
    }

    /// Generates a following customer when the queue discipline is first come first served
    /// and there is a single server.
    ///
    /// # Arguments
    /// * `rng` The random number generator to sample with. Obviously mutable.
    /// * `interarrival_time_distribution` The distribution of the interarrival times, the times
    /// between arrivals.
    /// * `service_time_distribution` The distribution of the service times.
    pub fn next_1_fcfs<R: Rng + ?Sized, DA: Distribution<f64>, DS: Distribution<f64>>(
        rng: &mut R,
        interarrival_time_distribution: &DA,
        service_time_distribution: &DS,
        previous: ArrivingCustomer,
    ) -> ArrivingCustomer {
        let t_n = interarrival_time_distribution.sample(rng);
        let a_n = previous.a() + t_n;
        let s_n = service_time_distribution.sample(rng);

        ArrivingCustomer {
            interarrival_time: t_n,
            time_of_arrival: a_n,
            service_time: s_n,
        }
    }

    /// The customer that never arrives, but if they do, they will never finish being served.
    pub fn never() -> ArrivingCustomer {
        ArrivingCustomer {
            interarrival_time: f64::INFINITY,
            time_of_arrival: f64::INFINITY,
            service_time: f64::INFINITY,
        }
    }

    /// Returns `A_n` the arrival time of this customer.
    pub fn a(&self) -> f64 {
        self.time_of_arrival
    }

    pub fn arrival_time(&self) -> f64 {
        self.time_of_arrival
    }
}

#[derive(Copy, Clone)]
pub struct Customer {
    pub(crate) interarrival_time: f64,
    pub(crate) time_of_arrival: f64,
    pub(crate) service_time: f64,

    pub(crate) time_of_service_start: f64,
    pub(crate) time_of_departure: f64,

    pub(crate) wait_in_queue: f64,
    pub(crate) wait_in_system: f64,
}

impl Customer {
    pub fn start_service(arriving_customer: ArrivingCustomer, time_of_service_start: f64) -> Customer {
        Customer {
            interarrival_time: arriving_customer.interarrival_time,
            time_of_arrival: arriving_customer.time_of_arrival,
            service_time: arriving_customer.service_time,
            time_of_service_start,
            time_of_departure: time_of_service_start + arriving_customer.service_time,
            wait_in_queue: time_of_service_start - arriving_customer.time_of_arrival,
            wait_in_system: time_of_service_start - arriving_customer.time_of_arrival + arriving_customer.service_time,
        }
    }

    pub fn time_of_departure(&self) -> f64 {
        self.time_of_departure
    }

    pub fn wait_in_system(&self) -> f64 {
        self.wait_in_system
    }
}