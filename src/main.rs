// import the crate
use gbm_rust::geometric_brownian_motion;

fn main () {
    let result = geometric_brownian_motion::geometric_brownian_motion_plot();
    println!("{:?}", result);
}
