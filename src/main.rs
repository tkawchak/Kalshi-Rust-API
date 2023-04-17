use reqwest::Client;

use kalshi_api::auth;
use kalshi_api::bids;
use kalshi_api::exchange;
use kalshi_api::market;

const TSA_EVENT_TICKER: &str = "TSAW-23APR02";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("Checking if exchange is open");
    match exchange::get_exchange_status_async(&client).await {
        Ok(status) => {
            println!("Exchange status: {}", status.exchange_active);
            println!("Trading status: {}", status.trading_active);
        }
        Err(e) => eprintln!("Error fetching exchange status - {}", e),
    };

    println!("Logging into Kalshi API");
    let token = auth::login(&client).await;

    match token {
        Ok(token) => {
            match market::get_event_async(&client, TSA_EVENT_TICKER, &token).await {
                Ok(event) => {
                    for market in event.markets {
                        let bids: bids::Bids = market.clone().into();

                        println!("Current market '{}' bids: {:?}", market.ticker, bids);

                        match market::get_market_orderbook_async(&client, &market.ticker, 4, &token)
                            .await
                        {
                            Ok(_) => {
                                println!("Successfully fetched market orderbook");
                            }
                            Err(e) => eprintln!("Error fetching market - {}", e),
                        };
                    }
                }
                Err(e) => eprintln!("Error fetching event - {}", e),
            };

            match auth::logout(client, &token).await {
                Ok(()) => {
                    println!("Logged out successfully");
                }
                Err(e) => {
                    eprintln!("Error while logging out - {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Login unsuccessful - {}", e);
        }
    };

    Ok(())
}
