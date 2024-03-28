use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use crate::security::{Security, TradingStatus};

fn simulation() {
    let mut rng = thread_rng();
    let trading_status_choices = [TradingStatus::Buy, TradingStatus::Sell];
    let trading_status_dist = Uniform::from(0..trading_status_choices.len());

    let mut securities = Vec::with_capacity(105); // Initial 5 + 100 more

    // Programmatically generating 100 more securities
    for i in 1..=100 {
        let price = rng.gen_range(50.0..200.0);
        let threshold_variation = rng.gen_range(5.0..20.0);
        let trading_status_index = trading_status_dist.sample(&mut rng);
        let trading_status = trading_status_choices[trading_status_index];

        securities.push(Security::new(
            &format!("Security {}", i + 5), // Starting naming from Security 6 onwards
            price,
            price - threshold_variation, // buy_threshold
            price + threshold_variation, // sell_threshold
            rng.gen_range(5000.0..20000.0), // max_order_value
            rng.gen_range(2500.0..10000.0), // base_order_value
            rng.gen_bool(0.5), // is_active
            trading_status,
        ));
    }

    // Example: print the first 10 securities to verify
    for security in securities.iter().take(10) {

    }
}
