#[derive(Debug)]
pub enum TradingStatus {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug)]
pub struct Security {
    pub id: u8,
    pub name: String,
    pub price: f32,
    pub is_active: bool,
    pub trading_status: TradingStatus,
    pub buy_threshold: f32,
    pub sell_threshold: f32,
    pub max_order_value: f32,
    pub base_order_value: f32,
}

impl Security {
    pub fn new(id: u8,name: &str, price: f32, buy_threshold: f32, sell_threshold: f32, max_order_value: f32, base_order_value: f32, is_active: bool, trading_status: TradingStatus) -> Self {
        Self {
            id,
            name: name.to_string(),
            price,
            is_active, // Use the provided argument
            trading_status, // Use the provided argument
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
        price_change
    }

    // Method to simulate placing a buy order
    pub fn place_buy_order(&self) {
        if self.is_active {
            // Only place an order if the security is active
            println!("Buy order has been placed for {}", self.name);
        } else {
            println!("Cannot place buy order. {} is inactive.", self.name);
        }
    }

    // Method to simulate placing a sell order
    pub fn place_sell_order(&self) {
        if self.is_active {
            // Only place an order if the security is active
            println!("Sell order has been placed for {}", self.name);
        } else {
            println!("Cannot place sell order. {} is inactive.", self.name);
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        println!("{} is now active.", self.name);
    }

    // Method to deactivate the security
    pub fn deactivate(&mut self) {
        self.is_active = false;
        println!("{} is now inactive.", self.name);
    }

}