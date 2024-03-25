mod closing_algo;
mod security;

fn main() {
    let mut algorithm_status = false;
    let securities = vec![]; // Populate with your securities
    let current_time = 0; // Define your current time logic
    let cutoff_time = 0; // Define your cutoff time logic

    crate::closing_algo::activate_algorithm(current_time, cutoff_time, &mut algorithm_status);

    if algorithm_status {
        crate::closing_algo::place_orders(&securities, -0.05, 0.05, 10000.0, 1000.0);
    } else {
        println!("Algorithm not activated or it's past the cutoff time.");
    }
}