pub trait QueueTheory {
    /// The number of customers in the system at steady state. Also known as L.
    fn number_in_system(&self) -> f64;

    /// The number of customers in the system at steady state: L.
    fn l(&self) -> f64 {
        self.number_in_system()
    }

    /// The average wait in the system (at steady state). Also known as W.
    fn wait_in_system(&self) -> f64;

    /// The average wait in the system (at steady state): W.
    fn w(&self) -> f64 {
        self.wait_in_system()
    }

    /// The number of customer in the queue at steady state. Also known as L_q
    fn number_in_queue(&self) -> f64;

    /// The number of customer in the queue at steady state: L_q
    fn l_q(&self) -> f64 {
        self.number_in_queue()
    }

    /// The average wait in the queue (at steady state). Also known as W_q.
    fn wait_in_queue(&self) -> f64;

    /// The average wait in the queue (at steady state): W_q.
    fn w_q(&self) -> f64 {
        self.wait_in_queue()
    }

    /// The proportion of time spent with n people in the system. Also known as p_n.
    fn proportion(&self, n: u32) -> f64;

    /// The proportion of time spent with n people in the system: p_n.
    fn p(&self, n: u32) -> f64 {
        self.proportion(n)
    }
}

fn fac(n: u8) -> f64 {
    let mut fac = 1;
    for i in 1..(n + 1) as u64 {
        fac *= i;
    }
    fac as f64
}

fn faci(n: i32) -> f64 {
    let mut fac = 1.;
    for i in 1..(n + 1) as u64 {
        fac *= i as f64;
    }
    fac
}

pub struct MMC {
    pub lambda: f64,
    pub mu: f64,
    pub servers: i32,
    r: f64,
    rho: f64,
    p0: f64,
}

impl MMC {
    pub fn new(lambda: f64, mu: f64, servers: u8) -> MMC {
        let r = lambda / mu;
        let rho = r / servers as f64;
        let mut sum = 0.;
        for i in (0..servers).rev() {
            sum += r.powi(i as i32) / fac(i);
        }
        let p0 = 1. / (r.powi(servers as i32) / fac(servers) / (1. - rho) + sum);
        MMC {
            lambda,
            mu,
            servers: servers as i32,
            r,
            rho,
            p0,
        }
    }
}

impl QueueTheory for MMC {
    fn number_in_system(&self) -> f64 {
        self.r + self.number_in_queue()
    }

    fn wait_in_system(&self) -> f64 {
        // Little's law.
        self.number_in_system() / self.lambda
    }

    fn number_in_queue(&self) -> f64 {
        let square = (1. - self.rho) * (1. - self.rho);
        (self.r.powi(self.servers) * self.rho / faci(self.servers) / square) * self.p0
    }

    fn wait_in_queue(&self) -> f64 {
        self.number_in_queue() / self.lambda
    }

    fn proportion(&self, n: u32) -> f64 {
        return if (n as i32) < self.servers {
            self.r.powi(n as i32) / faci(n as i32) * self.p0
        } else {
            let pow = (self.servers as f64).powi(n as i32 - self.servers);
            self.r.powi(n as i32) * self.p0 / pow / faci(self.servers)
        };
    }
}

struct MMCK {
    pub lambda: f64,
    pub mu: f64,
    pub servers: i32,
    pub queue_capacity: u32,
    pub capacity: u32,
    r: f64,
    rho: f64,
    p0: f64,
}

impl MMCK {
    pub fn new(lambda: f64, mu: f64, servers: u8, queue_capacity: u32) -> MMCK {
        let r = lambda / mu;
        let rho = r / servers as f64;
        let mut sum = 0.;
        for i in (0..servers).rev() {
            sum += r.powi(i as i32) / fac(i);
        }
        let p0 = if rho != 1. {
            1. / (r.powi(servers as i32) / fac(servers) * (1. - rho.powi((queue_capacity + 1) as i32)) / (1. - rho) + sum)
        } else {
            // 1. / (r.powi(servers as i32) / fac(servers) * ((queue_capacity + 1) as f64) + sum)
            todo!("rho = 1 currently not fully supported. Terminating early.")
        };
        MMCK {
            lambda,
            mu,
            servers: servers as i32,
            queue_capacity,
            capacity: servers as u32 + queue_capacity,
            r,
            rho,
            p0,
        }
    }
}

impl QueueTheory for MMCK {
    fn number_in_system(&self) -> f64 {
        self.number_in_queue() + self.r * (1. - self.proportion(self.capacity))
    }

    fn wait_in_system(&self) -> f64 {
        let p_k = self.proportion(self.capacity);
        (self.number_in_queue() + self.r * p_k) / (self.lambda * (1. - p_k))
    }

    fn number_in_queue(&self) -> f64 {
        // TODO: when rho = 1, we need to apply L'Hopital's rule twice.
        self.p0 * self.r.powi(self.servers) * self.rho / (faci(self.servers) * (1. - self.rho).powi(2)) *
            (
                1. - self.rho.powi((self.queue_capacity + 1) as i32)
                    - (1. - self.rho) * (self.queue_capacity + 1) as f64 * self.rho.powi(self.queue_capacity as i32)
            )
    }

    fn wait_in_queue(&self) -> f64 {
        self.wait_in_system() - 1. / self.mu
    }

    fn proportion(&self, n: u32) -> f64 {
        return if (n as i32) < self.servers {
            self.r.powi(n as i32) / faci(n as i32) * self.p0
        } else if n <= self.capacity {
            let pow = (self.servers as f64).powi(n as i32 - self.servers);
            self.r.powi(n as i32) * self.p0 / pow / faci(self.servers)
        } else {
            0.
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::theory::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn example__3_4() {
        let mm3 = MMC::new(6., 3., 3);
        assert_approx_eq!(1. / 9., mm3.proportion(0), 1.0e-16);
        assert_approx_eq!(8. / 9., mm3.number_in_queue(), 2.3e-16);
        assert_approx_eq!(28.9 / 60., mm3.wait_in_system(), 5.0e-2 / 60.);

        let fraction_of_time_with_idleness = mm3.proportion(0) + mm3.proportion(1) + mm3.proportion(2);
        assert_approx_eq!(5. / 9., fraction_of_time_with_idleness, 1.0e-16);
    }

    #[test]
    fn example__3_6() {
        let mm37 = MMCK::new(1., 1. / 6., 3, 4);
        assert_approx_eq!(0.00088, mm37.proportion(0), 5.0e-6);
        assert_approx_eq!(3.09, mm37.number_in_queue(), 5.0e-3);
        assert_approx_eq!(6.06, mm37.number_in_system(), 5.0e-3);
        // I think the additional error here comes from using L (number in system), which is already rounded.
        assert_approx_eq!(12.3, mm37.wait_in_system(), 6.2e-2);
    }
}