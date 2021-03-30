extern crate reqwest;
use reqwest::{Client,Error};

pub struct Loggly {
    client: Client,
    account: String,
    api_token: String
}

impl Loggly {
    pub fn init(account: &str, api_token: &str) -> Self {
        Loggly {
            account: String::from(account),
            api_token: String::from(api_token),
            client: Client::new(),
        }
    }

    pub async fn fetch_logs(self) -> Result<serde_json::value::Value, Error>  {
        let uri = format!("https://{}.loggly.com/apiv2/events/iterate?q=*&from=-10m&until=now&size=10", &self.account);
        let res = self.client
            .get(uri)
            .bearer_auth(&self.api_token)
            .send().await
            .and_then(|r| Ok(r.json()))
            .unwrap().await;
        res
    }
}
