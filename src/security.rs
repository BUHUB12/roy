enum TradingStatus {
    Buy,
    Sell,
    Hold,
}

struct Security {
    name: String,
    price: f32,
    is_active: bool,
    trading_status: TradingStatus,
    buy_threshold: f32,
    sell_threshold: f32,
    max_order_value: f32,
    base_order_value: f32,
}

impl Security {
    fn new(name: &str, price: f32, buy_threshold: f32, sell_threshold: f32, max_order_value: f32, base_order_value: f32) -> Self {
        Self {
            name: name.to_string(),
            price,
            is_active: false, // Securities are inactive by default.
            trading_status: TradingStatus::Hold, // Default trading status could be 'Hold'.
            buy_threshold,
            sell_threshold,
            max_order_value,
            base_order_value,
        }
    }

    pub fn update_price(&mut self, new_price: f32) {
        self.price = new_price
    }

    pub fn price_change_percentage(&self, new_price: f32) -> f32 {
        let price_change = ((new_price - self.price) / self.price) * 100.0;
        price_change.abs()
    }

    // Method to simulate placing a buy order
    fn place_buy_order(&self) {
        if self.is_active {
            // Only place an order if the security is active
            println!("Buy order has been placed for {}", self.name);
        } else {
            println!("Cannot place buy order. {} is inactive.", self.name);
        }
    }

    // Method to simulate placing a sell order
    fn place_sell_order(&self) {
        if self.is_active {
            // Only place an order if the security is active
            println!("Sell order has been placed for {}", self.name);
        } else {
            println!("Cannot place sell order. {} is inactive.", self.name);
        }
    }

    fn activate(&mut self) {
        self.is_active = true;
        println!("{} is now active.", self.name);
    }

    // Method to deactivate the security
    fn deactivate(&mut self) {
        self.is_active = false;
        println!("{} is now inactive.", self.name);
    }
}
