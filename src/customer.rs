use rand::Rng;
use rand_distr::Distribution;

#[derive(Clone, Copy, Debug)]
pub struct Customer {
    interarrival_time: f64,
    time_of_arrival: f64,
    service_time: f64,

    time_of_service_start: f64,
    time_of_departure: f64,

    wait_in_queue: f64,
    wait_in_system: f64,
}

impl Customer {
    /// Generate the first customer in a simulation. This is not really dependent on anything
    /// but the service distribution since an empty queue means they will be served right away.
    ///
    /// # Arguments
    /// * `rng` The random number generator to sample with. Obviously mutable.
    /// * `service_time_distribution` The distribution of the service times.
    pub fn first<R: Rng + ?Sized, DS: Distribution<f64>>(
        rng: &mut R,
        service_time_distribution: &DS,
    ) -> Customer {
        let t_0 = 0.;
        let a_0 = 0.;
        let s_0 = service_time_distribution.sample(rng);

        let u_0 = 0.;
        let d_0 = u_0 + s_0;

        let wq_0 = u_0 - a_0;
        let w_0 = wq_0 + s_0;

        Customer {
            interarrival_time: t_0,
            time_of_arrival: a_0,
            service_time: s_0,

            time_of_service_start: u_0,
            time_of_departure: d_0,

            wait_in_queue: wq_0,
            wait_in_system: w_0,
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
        previous: Customer,
    ) -> Customer {
        let t_n = interarrival_time_distribution.sample(rng);
        let a_n = previous.a() + t_n;
        let s_n = service_time_distribution.sample(rng);

        // XXX u is a function of the queue discipline, servers, etc.
        let u_n = f64::max(a_n, previous.d());
        let d_n = u_n + s_n;

        let wq_n = u_n - a_n;
        let w_n = wq_n + s_n;

        Customer {
            interarrival_time: t_n,
            time_of_arrival: a_n,
            service_time: s_n,

            time_of_service_start: u_n,
            time_of_departure: d_n,

            wait_in_queue: wq_n,
            wait_in_system: w_n,
        }
    }

    /// Returns `A_n` the arrival time of this customer.
    pub fn a(&self) -> f64 {
        self.time_of_arrival
    }

    pub fn arrival_time(&self) -> f64 {
        self.time_of_arrival
    }

    /// Returns `D_n` the departure time of this customer.
    pub fn d(&self) -> f64 {
        self.time_of_departure
    }

    pub fn departure_time(&self) -> f64 {
        self.time_of_departure
    }

    /// Returns `W_q_n` the waiting time in the queue of this customer.
    pub fn wq(&self) -> f64 {
        self.wait_in_queue
    }

    /// Returns `W_n` the waiting time in the system of this customer.
    pub fn w(&self) -> f64 {
        self.wait_in_system
    }
}
