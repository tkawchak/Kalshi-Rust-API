use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ExchangeStatus {
    pub exchange_active: bool,
    pub trading_active: bool,
}
