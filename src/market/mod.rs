use std::collections::HashMap;

pub use crate::market::api_structs::{
    GetEventResponse, GetMarketOrderbookResponse, GetMarketResponse, Market,
};

use self::api_structs::{GetEventsResponse, GetMarketsResponse};

pub mod api_structs;

const GET_EVENT_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/events/";
const GET_EVENTS_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/events/";
const GET_MARKET_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/markets/";
const GET_MARKETS_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/markets/";
const MARKET_ORDERBOOK_SUFFIX: &str = "/orderbook";

pub async fn get_event_async(
    client: &reqwest::Client,
    event_ticker: &str,
    token: &str,
) -> Result<GetEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Sanitize event_ticker?
    let url = reqwest::Url::parse(GET_EVENT_URL)?;
    let url = url.join(event_ticker)?;
    println!(
        "Fetching details for event '{}' at endpoint '{}'",
        event_ticker, url
    );
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
                match result.json::<GetEventResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed event response body: {:?}", body);
                        for market in body.markets.clone() {
                            println!("Market: {:?}", market);
                        }
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse event response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_event unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_event: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn get_events_async(
    client: &reqwest::Client,
    token: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    status: Option<&str>, // open, closed, settled
    series_ticker: Option<&str>,
) -> Result<GetEventsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let url = reqwest::Url::parse(GET_EVENTS_URL)?;
    let mut query_params: HashMap<String, String> = HashMap::new();
    if let Some(limit) = limit {
        query_params.insert("limit".to_string(), limit.to_string());
    }
    if let Some(cursor) = cursor {
        query_params.insert("cursor".to_string(), cursor.to_string());
    }
    if let Some(status) = status {
        query_params.insert("status".to_string(), status.to_string());
    }
    if let Some(series_ticker) = series_ticker {
        query_params.insert("series_ticker".to_string(), series_ticker.to_string());
    }
    println!("Fetching events at endpoint '{}'", url);
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
                match result.json::<GetEventsResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed events response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        println!("Unable to parse events response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_events unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_events: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}

pub async fn get_market_async(
    client: &reqwest::Client,
    market_ticker: &str,
    token: &str,
) -> Result<GetMarketResponse, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Sanitize market ticker?
    let url = reqwest::Url::parse(GET_MARKET_URL)?;
    let url = url.join(market_ticker)?;
    println!(
        "Fetching details for market '{}' at endpoint '{}'",
        market_ticker, url
    );
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
                match result.json::<GetMarketResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed market response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        eprintln!("Unable to parse market response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_market unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            eprintln!("Error - {}", e);
            Err(Box::from(e))
        }
    }
}

pub async fn get_markets_async(
    client: &reqwest::Client,
    token: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    event_ticker: &str,
    series_ticker: Option<&str>,
    max_close_ts: Option<i64>,
    min_close_ts: Option<i64>,
    status: Option<&str>,  // open, closed, settled
    tickers: Option<&str>, // comma separated list
) -> Result<GetMarketsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let url = reqwest::Url::parse(GET_MARKETS_URL)?;
    let mut query_params: HashMap<String, String> = HashMap::new();
    if let Some(limit) = limit {
        query_params.insert("limit".to_string(), limit.to_string());
    }
    if let Some(cursor) = cursor {
        query_params.insert("cursor".to_string(), cursor.to_string());
    }
    if let Some(series_ticker) = series_ticker {
        query_params.insert("series_ticker".to_string(), series_ticker.to_string());
    }
    if let Some(max_close_ts) = max_close_ts {
        query_params.insert("max_close_ts".to_string(), max_close_ts.to_string());
    }
    if let Some(min_close_ts) = min_close_ts {
        query_params.insert("min_close_ts".to_string(), min_close_ts.to_string());
    }
    if let Some(status) = status {
        query_params.insert("status".to_string(), status.to_string());
    }
    if let Some(tickers) = tickers {
        query_params.insert("tickers".to_string(), tickers.to_string());
    }
    query_params.insert("event_ticker".to_string(), event_ticker.to_string());
    println!("Fetching markets at endpoint '{}'", url);
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
                match result.json::<GetMarketsResponse>().await {
                    Ok(body) => {
                        println!("Successfully parsed markets response body: {:?}", body);
                        Ok(body)
                    }
                    Err(e) => {
                        eprintln!("Unable to parse markets response body. Error - {}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_markets unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            eprintln!("Error - {}", e);
            Err(Box::from(e))
        }
    }
}

pub async fn get_market_orderbook_async(
    client: &reqwest::Client,
    market_ticker: &str,
    depth: i32,
    token: &str,
) -> Result<GetMarketOrderbookResponse, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Sanitize input?
    let url = reqwest::Url::parse(GET_MARKET_URL)?;
    let url = url.join(market_ticker)?;
    let url = url.join(MARKET_ORDERBOOK_SUFFIX)?;
    println!(
        "Fetching orderbook for market '{}' at endpoint '{}'. Depth='{}'",
        market_ticker, url, depth
    );
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
                match result.json::<GetMarketOrderbookResponse>().await {
                    Ok(body) => {
                        println!(
                            "Successfully parsed market orderbook response body: {:?}",
                            body
                        );
                        Ok(body)
                    }
                    Err(e) => {
                        eprintln!(
                            "Unable to parse market orderbook response body. Error - {}",
                            e
                        );
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_market_orderbook unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            eprintln!("Error - {}", e);
            Err(Box::from(e))
        }
    }
}
