struct Security {
    name: String,
    base_price: f32,
    is_active: bool, // Ensure this field is properly initialized in the constructor
}

impl Security {
    // Constructor method to create a new instance of Security
    fn new(name: &str, base_price: f32) -> Self {
        Self {
            name: name.to_string(),
            base_price,
            is_active: false, // Initialize as false, securities are inactive by default
        }
    }

    // Method to calculate the price change percentage
    fn price_change_percent(&self, new_price: f32) -> f32 {
        let price_change = (new_price - self.base_price) / self.base_price * 100.0;
        price_change.abs() // Return the absolute value of the price change
    }

    // Method to activate the security
    fn activate(&mut self) {
        self.is_active = true;
        println!("{} is now active.", self.name);
    }

    // Method to deactivate the security
    fn deactivate(&mut self) {
        self.is_active = false;
        println!("{} is now inactive.", self.name);
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
}
