use std::fmt;
use crate::BasicRegulationsbsice::BasicRegulations;
use crate::security::Security;

// Define the action types an order can take
enum ActionType {
    Buy,
    Sell,
}

// Define the types o()f orders based on pricing strategy
#[derive(Debug)]
enum OrderType {
    Limit,
    Market,
    LimitOnOpen,
}

// Define the structure of an Order
pub struct Order {
    pub account_id: String,
    pub security_symbol: Security,
    pub unique_order_id: String,
    pub requested_quantity: u32,
    pub fulfilled_quantity: u32,
    pub quantity: f32,
    pub price: f32,
    pub order_type: OrderType,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::Limit => write!(f, "Limit"),
            OrderType::Market => write!(f, "Market"),
            OrderType::LimitOnOpen => write!(f, "LimitOnOpen"),
        }
    }
}


impl Order {
    // Method to create a new Order
    pub fn new(account_id: String, unique_order_id: String, platform_reference_id: String) -> Self {
        Order {
            account_id,
            security_symbol: Security,
            unique_order_id,
            requested_quantity: 0,
            fulfilled_quantity: 0,
            quantity: 0.0,
            price: 0.0,
            order_type: OrderType::Limit,
        }
    }

    // Method to update an Order's details
    pub fn update_order(&mut self, requested_quantity: u32, quantity: f32, price: f32, order_type: OrderType) {
        self.requested_quantity = requested_quantity;
        self.quantity = quantity;
        self.price = price;
        self.order_type = order_type;
    }
    pub fn place_order(&self, regulations: &BasicRegulations, security: &Security) {
        if regulations.market_open {
            match {
                println!("RequestType = New");
                println!("SecurityId = {}", security.id);
                println!("Name = {}", security.name);
                println!("Qty = {}", self.quantity);
                println!("OrderID = {}", self.unique_order_id);
                println!("Order Type = {:?}", self.order_type); // Assuming OrderType implements Display
            };
        } else {
            println!("Market is not open. Cannot send order for SecurityId = {}", security.id);
            println!("OrderId = {}", self.unique_order_id);
        }
    }
}
