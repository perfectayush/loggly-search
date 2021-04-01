extern crate reqwest;

use reqwest::Client;
use serde_json::value::Value;


#[derive(Debug)]
pub struct Loggly {
    client: Client,
    account: String,
    api_token: String,
    response: Option<Value>,
}

impl Loggly {
    pub fn init(account: &str, api_token: &str) -> Self {
        Loggly {
            account: String::from(account),
            api_token: String::from(api_token),
            client: Client::new(),
            response: None,
        }
    }

    fn create_search_uri(&self) -> String {
        format!("https://{}.loggly.com/apiv2/events/iterate?q=*&from=-10m&until=now&size=1000", self.account)
    }

    pub async fn fetch_logs(&mut self, uri: &str) {
        let res = self.client
            .get(uri)
            .bearer_auth(&self.api_token)
            .send().await
            .and_then(|r| Ok(r.json()))
            .unwrap().await;
        let json = match res {
            Ok(json) => Some(json),
            _ => None,
        };

        self.response = json;
    }


    fn get_search_uri(&self) -> String {
        let uri = match self.response {
            None => self.create_search_uri(),
            Some(ref json) => String::from(json.get("next").unwrap().as_str().unwrap()),
        };
        uri
    }

    fn get_log_events(&self) -> Option<&Vec<Value>> {
        match &self.response {
            Some(json) => Some(json.get("events").unwrap().as_array().unwrap()),
            None => None,
        }
    }

    pub async fn print_logs(&mut self) {
        let uri = self.get_search_uri();

        self.fetch_logs(&uri).await;
        if let Some(events) = self.get_log_events() {
            events.iter().for_each(|event| {
                    println!("{}", event);
            });
        }
    }
}
