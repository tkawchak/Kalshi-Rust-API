use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Fill {
    pub action: String,
    pub count: i32,
    pub created_time: String,
    pub is_taker: bool,
    pub no_price: i32,
    pub order_id: String,
    pub side: String,
    pub ticker: String,
    pub trade_id: String,
    pub yes_price: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Balance {
    pub balance: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Order {
    pub action: String,
    pub client_order_id: String,
    pub close_cancel_count: i32,
    pub created_time: String,
    pub decrease_count: i32,
    pub expiration_time: String,
    pub fcc_cancel_count: i32,
    pub last_update_time: String,
    pub maker_fill_count: i32,
    pub no_price: i64,
    pub order_id: String,
    pub place_count: i32,
    pub queue_position: i32,
    pub remaining_count: i32,
    pub side: String,
    pub status: String,
    pub taker_fees: i32,
    pub taker_fill_cost: i64,
    pub taker_fill_count: i64,
    pub ticker: String,
    pub type_: String,
    pub user_id: String,
    pub yes_price: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EventPosition {
    pub event_exposure: i32,
    pub event_ticker: String,
    pub fees_paid: i32,
    pub realized_pnl: i32,
    pub resting_order_count: i32,
    pub total_cost: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MarketPosition {
    pub fees_paid: i32,
    pub market_exposure: i32,
    pub position: i64,
    pub realized_pnl: i64,
    pub resting_order_count: i64,
    pub ticker: String,
    pub total_cost: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settlement {
    pub market_result: String,
    pub no_count: i64,
    pub no_total_cost: i64,
    pub revenue: i64,
    pub settled_time: String,
    pub ticker: String,
    pub yes_count: i64,
    pub yes_total_cost: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetFillsResponse {
    pub cursor: String,
    pub fills: Vec<Fill>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetOrdersResponse {
    pub cursor: String,
    pub orders: Vec<Order>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateOrderResponse {
    pub order: Order,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetOrderResponse {
    pub order: Order,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CancelOrderResponse {
    pub order: Order,
    pub reduced_by: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetPositionsResponse {
    pub cursor: String,
    pub event_positions: Vec<EventPosition>,
    pub market_positions: Vec<MarketPosition>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetSettlementsResponse {
    pub cursor: String,
    pub settlements: Vec<Settlement>,
}
