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

pub struct MMC {
    pub lambda: f64,
    pub mu: f64,
    pub servers: i32,
    r: f64,
    rho: f64,
    p0: f64,
}

fn fac(n: u8) -> f64 {
    let mut fac = 1;
    for i in 1..n as u64 {
        fac *= i;
    }
    fac as f64
}

fn faci(n: i32) -> f64 {
    let mut fac = 1.;
    for i in 1..n as u64 {
        fac *= i as f64;
    }
    fac
}

impl MMC {
    pub fn new(lambda: f64, mu: f64, servers: u8) -> MMC {
        let r = lambda / mu;
        let rho = r / servers as f64;
        let mut sum = 0.;
        for i in 0..servers {
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
            self.lambda.powi(n as i32) / faci(n as i32) / self.mu.powi(n as i32) * self.p0
        } else {
            let pow = (self.servers as f64).powi(n as i32 - self.servers);
            self.r.powi(n as i32) * self.p0 / pow / faci(self.servers)
        };
    }
}