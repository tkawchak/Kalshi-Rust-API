pub use crate::portfolio::api_structs::{
    Balance, CancelOrderResponse, CreateOrderResponse, EventPosition, Fill, GetFillsResponse,
    GetOrderResponse, GetOrdersResponse, GetPositionsResponse, GetSettlementsResponse,
    MarketPosition, Order, Settlement,
};

pub mod api_structs;

use std::collections::HashMap;

const GET_BALANCE_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/balance/";
const GET_FILLS_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/fills/";
const GET_ORDERS_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/orders/";
const CREATE_ORDER_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/orders/";
const CANCEL_ORDER_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/orders/";
const GET_ORDER_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/orders/";
const GET_POSITIONS_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/portfolio/positions/";
const GET_SETTLEMENTS_URL: &str =
    "https://trading-api.kalshi.com/trade-api/v2/portfolio/settlements/";

pub async fn get_balance_async(
    client: &reqwest::Client,
    token: &str,
) -> Result<Balance, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "Fetching balance for user at endpoint '{}'",
        GET_BALANCE_URL
    );
    match client
        .get(GET_BALANCE_URL)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<Balance>().await {
                    Ok(body) => {
                        println!("Successfully parsed balance response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse balance response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_balance unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_balance: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn get_fills_async(
    client: &reqwest::Client,
    token: &str,
    ticker: Option<&str>,
    order_id: Option<&str>,
    min_ts: Option<i64>,
    max_ts: Option<i64>,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<GetFillsResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("Fetching fills for user at endpoint '{}'", GET_FILLS_URL);
    let url = reqwest::Url::parse(GET_FILLS_URL)?;
    // let mut query_params: HashMap<Cow<'static, str>, Cow<'static, str>> = HashMap::new();
    let mut query_params: HashMap<String, String> = HashMap::new();
    if let Some(ticker) = ticker {
        query_params.insert("ticker".to_string(), ticker.to_string());
    }
    if let Some(order_id) = order_id {
        query_params.insert("order_id".to_string(), order_id.to_string());
    }
    if let Some(min_ts) = min_ts {
        query_params.insert("min_ts".to_string(), min_ts.to_string());
    }
    if let Some(max_ts) = max_ts {
        query_params.insert("max_ts".to_string(), max_ts.to_string());
    }
    if let Some(limit) = limit {
        query_params.insert("limit".to_string(), limit.to_string());
    }
    if let Some(cursor) = cursor {
        query_params.insert("cursor".to_string(), cursor.to_string());
    }
    match client
        .get(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .query(&query_params)
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<GetFillsResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed fills response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse fills response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_fills unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_fills: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn get_orders_async(
    client: &reqwest::Client,
    token: &str,
    ticker: Option<&str>,
    event_ticker: Option<&str>,
    min_ts: Option<i64>,
    max_ts: Option<i64>,
    status: Option<&str>,
    cursor: Option<&str>,
    limit: Option<i32>,
) -> Result<GetOrderResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("Fetching orders for user at endpoint '{}'", GET_ORDERS_URL);
    let url = reqwest::Url::parse(GET_ORDERS_URL)?;
    let mut query_params: HashMap<String, String> = HashMap::new();
    if let Some(ticker) = ticker {
        query_params.insert("ticker".to_string(), ticker.to_string());
    }
    if let Some(event_ticker) = event_ticker {
        query_params.insert("event_ticker".to_string(), event_ticker.to_string());
    }
    if let Some(min_ts) = min_ts {
        query_params.insert("min_ts".to_string(), min_ts.to_string());
    }
    if let Some(max_ts) = max_ts {
        query_params.insert("max_ts".to_string(), max_ts.to_string());
    }
    if let Some(status) = status {
        query_params.insert("status".to_string(), status.to_string());
    }
    if let Some(cursor) = cursor {
        query_params.insert("cursor".to_string(), cursor.to_string());
    }
    if let Some(limit) = limit {
        query_params.insert("limit".to_string(), limit.to_string());
    }
    match client
        .get(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .query(&query_params)
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<GetOrderResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed orders response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse orders response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_orders unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_orders: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn create_order_async(
    client: &reqwest::Client,
    token: &str,
    action: &str,
    buy_max_cost: Option<i64>,
    client_order_id: &str,
    count: i32,
    expiration_ts: Option<i64>,
    no_price: Option<i64>,
    sell_position_floor: Option<i32>,
    side: &str, // yes or no
    ticker: &str,
    type_: &str, // limit or market
    yes_price: Option<i64>,
) -> Result<CreateOrderResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("Creating order for user at endpoint '{}'", CREATE_ORDER_URL);
    let url = reqwest::Url::parse(CREATE_ORDER_URL)?;
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("action".to_string(), action.to_string());
    body.insert("client_order_id".to_string(), client_order_id.to_string());
    body.insert("count".to_string(), count.to_string());
    body.insert("side".to_string(), side.to_string());
    body.insert("ticker".to_string(), ticker.to_string());
    body.insert("type".to_string(), type_.to_string());
    if let Some(buy_max_cost) = buy_max_cost {
        body.insert("buy_max_cost".to_string(), buy_max_cost.to_string());
    }
    if let Some(expiration_ts) = expiration_ts {
        body.insert("expiration_ts".to_string(), expiration_ts.to_string());
    }
    if let Some(no_price) = no_price {
        body.insert("no_price".to_string(), no_price.to_string());
    }
    if let Some(sell_position_floor) = sell_position_floor {
        body.insert(
            "sell_position_floor".to_string(),
            sell_position_floor.to_string(),
        );
    }
    if let Some(yes_price) = yes_price {
        body.insert("yes_price".to_string(), yes_price.to_string());
    }
    match client
        .post(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .json(&body)
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<CreateOrderResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed create order response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse create order response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "create_order unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from create_order: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn get_order(
    client: &reqwest::Client,
    token: &str,
    order_id: &str,
) -> Result<GetOrderResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("Fetching order for user at endpoint '{}'", GET_ORDER_URL);
    let url = reqwest::Url::parse(GET_ORDER_URL)?;
    let url = url.join(order_id)?;
    match client
        .get(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<GetOrderResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed order response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse order response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_order unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_order: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn cancel_order_async(
    client: &reqwest::Client,
    token: &str,
    order_id: &str,
) -> Result<CancelOrderResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "Cancelling order for user at endpoint '{}'",
        CANCEL_ORDER_URL
    );
    let url = reqwest::Url::parse(CANCEL_ORDER_URL)?;
    let url = url.join(order_id)?;
    match client
        .delete(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<CancelOrderResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed cancel order response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse cancel order response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "cancel_order unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from cancel_order: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn get_positions_async(
    client: &reqwest::Client,
    token: &str,
    cursor: Option<&str>,
    limit: Option<i32>,
    settlement_status: Option<&str>,
    ticker: Option<&str>,
    event_ticker: Option<&str>,
) -> Result<GetPositionsResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "Fetching positions for user at endpoint '{}'",
        GET_POSITIONS_URL
    );
    let url = reqwest::Url::parse(GET_POSITIONS_URL)?;
    let mut query_params: HashMap<String, String> = HashMap::new();
    if let Some(cursor) = cursor {
        query_params.insert("cursor".to_string(), cursor.to_string());
    }
    if let Some(limit) = limit {
        query_params.insert("limit".to_string(), limit.to_string());
    }
    if let Some(settlement_status) = settlement_status {
        query_params.insert(
            "settlement_status".to_string(),
            settlement_status.to_string(),
        );
    }
    if let Some(ticker) = ticker {
        query_params.insert("ticker".to_string(), ticker.to_string());
    }
    if let Some(event_ticker) = event_ticker {
        query_params.insert("event_ticker".to_string(), event_ticker.to_string());
    }
    match client
        .get(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .query(&query_params)
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<GetPositionsResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed positions response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse positions response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_positions unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_positions: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn get_settlements_async(
    client: &reqwest::Client,
    token: &str,
    limit: Option<i64>,
    cursor: Option<String>,
) -> Result<GetSettlementsResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "Fetching portfolio settlements for user at endpoint '{}'",
        GET_SETTLEMENTS_URL
    );
    let url = reqwest::Url::parse(GET_SETTLEMENTS_URL)?;
    let mut query_params: HashMap<String, String> = HashMap::new();
    if let Some(limit) = limit {
        query_params.insert("limit".to_string(), limit.to_string());
    }
    if let Some(cursor) = cursor {
        query_params.insert("cursor".to_string(), cursor.to_string());
    }
    match client
        .get(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .query(&query_params)
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<GetSettlementsResponse>().await {
                    Ok(body) => {
                        println!(
                            "Successfully parsed portfolio settlements response body: {:?}",
                            body
                        );
                        Ok(body)
                    }
                    Err(e) => {
                        println!(
                            "Unable to parse portfolio settlements response body. Error - {}",
                            e
                        );
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_portfolio_settlements unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_portfolio_settlements: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}
