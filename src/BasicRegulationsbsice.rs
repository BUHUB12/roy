#[derive(Debug)]
pub struct BasicRegulations {
    pub market_open: bool,
    pub max_holding: u32,
    pub max_lost: u32,
    pub block_buy_prec: f32,
    pub block_sell_prec: f32,
    pub max_order_price: u32,
    pub max_cycle_price: u32,
    pub related_index: Option<String>,
}

impl BasicRegulations {
    pub fn new() -> Self {
        BasicRegulations {
            market_open: false,
            max_holding: 100_000,
            max_lost: 1_500,
            block_buy_prec: 8.0,
            block_sell_prec: 8.0,
            max_order_price: 60_000,
            max_cycle_price: 70_000,
            related_index: None,
        }
    }

    pub fn update_market_open(&mut self) { self.market_open = true }
    pub fn update_max_holding(&mut self, max_holding: u32) { self.max_holding = max_holding }
    pub fn update_max_lost(&mut self, max_lost: u32) { self.max_lost = max_lost }
    pub fn update_block_buy_prec(&mut self, buy_prec: f32) { self.block_buy_prec = buy_prec }
    pub fn update_block_sell_prec(&mut self, sell_prec: f32) { self.block_sell_prec = sell_prec }
    pub fn update_max_order_price(&mut self, max_price: u32) { self.max_order_price = max_price }
    pub fn update_max_cycle_price(&mut self, max_cycle_price: u32) { self.max_cycle_price = max_cycle_price }
}
