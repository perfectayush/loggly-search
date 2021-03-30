extern crate reqwest;
use reqwest::{Client,Error};

pub async fn fetch_loggly_logs(account: &str, api_token: &str) -> Result<serde_json::value::Value, Error>  {
    let uri = format!("https://{}.loggly.com/apiv2/events/iterate?q=*&from=-10m&until=now&size=10", account);
    let authorization_header_value = format!("bearer {}", api_token);
    let res = Client::new()
        .get(uri)
        .header("Authorization", authorization_header_value)
        .send().await.
        and_then(|r| Ok(r.json())).
        unwrap().await;
    res
}
