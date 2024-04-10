pub struct Security {
    pub name: String,
    pub symbol: String,
    pub holding_value: f32,
    pub last_price: f32,
    pub units: f32,
    pub is_sell_active: bool,
    pub is_buy_active: bool,
}

impl Security {
    pub fn new(name: &str, symbol: &str, holding_value: f32, last_price: f32, units: f32) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            holding_value,
            last_price,
            units,
            is_sell_active: false,
            is_buy_active: false,
        }
    }

    pub fn update_price(&mut self, new_price: f32) {
        self.last_price = new_price
    }

    pub fn update_holding(&mut self, mut holding: f32) {
        holding = self.last_price * self.units;
        self.holding_value = holding;
    }

    fn activate_sell(&mut self) {
        self.is_sell_active = true;
        println!("{} is now active.", self.name);
    }

    // Method to deactivate the security
    fn deactivate_sell(&mut self) {
        self.is_sell_active = false;
        println!("{} is now inactive.", self.name);
    }
    fn activate_buy(&mut self) {
        self.is_sell_active = true;
        println!("{} is now active.", self.name);
    }

    // Method to deactivate the security
    fn deactivate_buy(&mut self) {
        self.is_sell_active = false;
        println!("{} is now inactive.", self.name);
    }
}