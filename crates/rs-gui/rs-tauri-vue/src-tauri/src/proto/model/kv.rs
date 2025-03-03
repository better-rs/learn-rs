use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinPriceItem {
    pub coin_type: String,
    pub price: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinPriceEntity {
    pub list: Vec<CoinPriceItem>,
}
