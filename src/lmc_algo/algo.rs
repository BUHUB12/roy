use chrono::*;

pub fn is_lock_algo_can_activate() -> bool {
    let now = Local::now();
    let day = now.weekday();
    let current_time = now.time();
    let cutoff_time = Local.ymd(now.year(), now.month(), now.day()).and_hms(17, 22, 0).time();
    let cutoff_time_sunday =Local.ymd(now.year(), now.month(), now.day()).and_hms(15, 45, 0).time();
    println!("{}", cutoff_time);
    println!("{}", cutoff_time_sunday);

    match day {
        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu => current_time <= cutoff_time,
        Weekday::Sun => cutoff_time <= cutoff_time_sunday,
        _ => false, // Deactivate on other days
    }

}

fn meets_buy_threshold(price: f32) -> bool {
    if price_change_percentage(price) <=
}

// Determines if the sell threshold has been met/exceeded
fn meets_sell_threshold(&self) -> bool {
    self.price_change_percentage() >= self.sell_threshold
}