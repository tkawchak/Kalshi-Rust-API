pub use crate::exchange::api_structs::ExchangeStatus;

pub mod api_structs;

const GET_EXCHANGE_STATUS_URL: &str =
    "https://trading-api.kalshi.com/trade-api/v2/exchange/status/";

pub async fn get_exchange_status_async(
    client: &reqwest::Client,
) -> Result<ExchangeStatus, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "Fetching exchange status at endpoint '{}'",
        GET_EXCHANGE_STATUS_URL
    );
    let url = reqwest::Url::parse(GET_EXCHANGE_STATUS_URL)?;
    match client
        .get(url)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<ExchangeStatus>().await {
                    Ok(body) => {
                        println!(
                            "Successfully parsed exchange status response body: {:?}",
                            body
                        );
                        Ok(body)
                    }
                    Err(e) => {
                        println!(
                            "Unable to parse exchange status response body. Error - {}",
                            e
                        );
                        Err(Box::new(e))
                    }
                }
            } else {
                let error_message = format!(
                    "get_exchange_status unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            }
        }
        Err(e) => {
            let error_message = format!("Error from get_exchange_status: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}
