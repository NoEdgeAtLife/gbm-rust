
use std::vec;
use std::time::Instant;
/// Generate geometric brownian motion
/// 

use crate::plot;
use rand_distr::{Distribution, Normal};

pub fn generate_geometric_brownian_motion(
    s_0: f64,
    dt: f64,
    length: usize,
    drift: f64,
    diffusion: f64,
) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0).unwrap();
    let mut v = Vec::<f64>::with_capacity(length);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..length {
        let rv = drift_factor + diffusion_factor * dist.sample(&mut rng);
        let prod: f64 = rv * v[idx - 1];
        v.push(prod);
    }
    v
}

pub fn geometric_brownian_motion_plot() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "geometric_brownian_motion.png";
    let mut val = vec![];
    println!("start");
    let now = Instant::now();
    let n = 100;
    for _ in 0..n{
        let vals = generate_geometric_brownian_motion(0.08, 0.01/365.0, 365, 500.0, 25.0);
        val.push(vals);
    }
    let elapsed_time = now.elapsed();
    println!("end");
    println!("Running it took {:?} seconds.", elapsed_time/n);
    plot::plot_values(val, filename)
}
