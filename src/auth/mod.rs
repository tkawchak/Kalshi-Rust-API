use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
struct LoginResponse {
    member_id: String,
    token: String,
}

const EMAIL_ENV_VAR: &str = "EMAIL";
const PASS_ENV_VAR: &str = "PASS";

const LOGIN_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/login";
const LOGOUT_URL: &str = "https://trading-api.kalshi.com/trade-api/v2/logout";

pub async fn login(
    client: &reqwest::Client,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let email = env::var(EMAIL_ENV_VAR).expect(&format!(
        "Environment variable '{}' must be set to authenticate with kalshi.",
        EMAIL_ENV_VAR
    ));
    let pass = env::var(PASS_ENV_VAR).expect(&format!(
        "Environment variable '{}' must be set to authenticate with kalshi.",
        PASS_ENV_VAR
    ));

    let mut map = HashMap::new();
    map.insert("email", email);
    map.insert("password", pass);

    let url = reqwest::Url::parse(LOGIN_URL)?;
    match client
        .post(url)
        // Send the request with headers. The website guards against scraping, so these
        // help to get around that.
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&map)
        .send()
        .await
    {
        Ok(result) => {
            if result.status().is_success() {
                match result.json::<LoginResponse>().await {
                    Ok(body) => {
                        println!(
                            "Logged in successfully. Member id - {}, token - (hidden)",
                            body.member_id
                        );
                        Ok(body.token)
                    }
                    Err(e) => {
                        eprintln!("Error parsing login response body: {}", e);
                        Err(Box::from(e))
                    }
                }
            } else {
                let status_code = result.status();
                let text = result.text().await?;
                println!(
                    "Login unsuccessful with status {} and message {}",
                    status_code.as_str(),
                    text
                );
                Err(Box::from(format!(
                    "Status: {} - Message: {}",
                    status_code.as_str(),
                    text
                )))
            }
        }
        Err(e) => {
            eprintln!("Received error message: {}", e);
            Err(Box::from(e))
        }
    }
}

pub async fn logout(
    client: reqwest::Client,
    token: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = reqwest::Url::parse(LOGOUT_URL)?;
    match client
        .post(url)
        .bearer_auth(token)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .send()
        .await
    {
        Ok(result) => {
            if !result.status().is_success() {
                let error_message = format!(
                    "Logout unsuccessful with status {} and message {}",
                    result.status().as_str(),
                    result.text().await?
                );
                eprintln!("{}", error_message);
                Err(Box::from(error_message))
            } else {
                println!("Logout successful!");
                Ok(())
            }
        }
        Err(e) => {
            let error_message = format!("Error from logout: {}", e);
            eprintln!("{}", error_message);
            Err(Box::from(error_message))
        }
    }
}
