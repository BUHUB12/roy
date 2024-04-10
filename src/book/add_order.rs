#[repr(C, packed)]
#[derive()]
struct AddOrderMessage {
    message_type: u8,
    market_id: char,
    instrument_id: u32,
    _reserved1: u32,
    _reserved2: u64,
    order_ref_number: u64,
    price: u64, // Assuming Pri_t can be represented as u64
    volume: u64, // Assuming Vol_t can be represented as u64
    side: char,
}


