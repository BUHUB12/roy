use crate::security::*;

pub(crate) fn activate_algorithm(current_time: u32, cutoff_time: u32, algorithm_status: &mut bool) {
    if current_time < cutoff_time {
        *algorithm_status = true;
    } else {
        // Function to cancel all orders
        cancel_all_orders();
        *algorithm_status = false;
    }
}

pub(crate) fn place_orders(securities: &Vec<Security>, buy_threshold: f32, sell_threshold: f32, max_order_value: f32, base_order_value: f32) {
    for security in securities {
        let price_change_percentage = calculate_price_change_percentage(security);
        let mut total_order_value = 0.0;
        if price_change_percentage <= -buy_threshold {
            while total_order_value < max_order_value && additional_orders_possible() {
                // Function to place a buy order
                place_buy_order(security, base_order_value);
                total_order_value += base_order_value;
            }
        } else if price_change_percentage >= sell_threshold {
            while total_order_value < max_order_value && additional_orders_possible() {
                // Function to place a sell order
                place_sell_order(security, base_order_value);
                total_order_value += base_order_value;
            }
        }
    }
}


