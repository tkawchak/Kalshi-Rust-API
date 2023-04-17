use crate::market::Market;

#[derive(Debug)]
pub struct Spread {
    pub ask: i32,
    pub bid: i32,
}

#[derive(Debug)]
pub struct Bids {
    pub no: Spread,
    pub yes: Spread,
}

// TODO: Add some market stats here about what values would be worthwhile to buy, etc.

impl From<Market> for Bids {
    fn from(item: Market) -> Self {
        Bids {
            no: Spread {
                ask: item.no_ask,
                bid: item.no_bid,
            },
            yes: Spread {
                ask: item.yes_ask,
                bid: item.yes_bid,
            },
        }
    }
}
