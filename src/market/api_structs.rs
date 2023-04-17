use serde::Deserialize;
use std::vec::Vec;

// TODO: API Discrepancies are commented out. Specifically, the "strike" fields seem to
//       be present on the api documentation, but not on the actual responses, which
//       causes deserialization errors

#[derive(Clone, Debug, Deserialize)]
pub struct Event {
    pub category: String,
    pub event_ticker: String,
    pub mutually_exclusive: bool,
    pub series_ticker: String,
    // strike_date: String,
    pub strike_period: String,
    pub sub_title: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Market {
    pub can_close_early: bool,
    // cap_strike: Strike,
    pub category: String,
    pub close_time: String,
    // custom_strike: Strike,
    pub event_ticker: String,
    pub expiration_time: String,
    pub expiration_value: String,
    // floor_strike: Strike,
    pub floor_strike: i32,
    pub last_price: i32,
    pub liquidity: i32,
    pub no_ask: i32,
    pub no_bid: i32,
    pub open_interest: i32,
    pub open_time: String,
    pub previous_price: i32,
    pub previous_yes_ask: i32,
    pub previous_yes_bid: i32,
    pub result: String,
    pub risk_limit_cents: i32,
    pub strike_type: String,
    pub subtitle: String,
    pub ticker: String,
    pub volume: i32,
    pub volume_24h: i32,
    pub yes_ask: i32,
    pub yes_bid: i32,
}

// TODO: We know the second vector has a size of 2 since it contains a (price, count) pair
//       Can we use this to make the following struct more efficient?
#[derive(Clone, Debug, Deserialize)]
pub struct Orderbook {
    pub no: Vec<Vec<i32>>,
    pub yes: Vec<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct GetEventResponse {
    pub event: Event,
    pub markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
pub struct GetEventsResponse {
    pub cursor: String,
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct GetMarketResponse {
    pub market: Market,
}

#[derive(Debug, Deserialize)]
pub struct GetMarketsResponse {
    pub cursor: String,
    pub markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
pub struct GetMarketOrderbookResponse {
    pub orderbook: Orderbook,
}
