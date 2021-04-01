extern crate reqwest;

use std::{io, process::exit};
use std::io::Write;

use reqwest::{Client, StatusCode};
use serde_json::value::Value;


#[derive(Debug)]
pub struct Loggly {
    client: Client,
    account: String,
    api_token: String,
    response: Option<Value>,
    from: String,
}

impl Loggly {
    pub fn init(account: &str, api_token: &str, from: &str) -> Self {
        Loggly {
            account: String::from(account),
            api_token: String::from(api_token),
            client: Client::new(),
            response: None,
            from: String::from(from),
        }
    }

    fn create_search_uri(&self) -> String {
        format!("https://{}.loggly.com/apiv2/events/iterate?q=*&from={}&until=now&size=100", self.account, self.from)
    }

    pub async fn fetch_logs(&mut self) {
        let uri = self.get_search_uri();

        let response = self.client
            .get(uri)
            .bearer_auth(&self.api_token)
            .send().await;

        self.response = match response {
            Err(error) => {
                println!("Error occurred while making request: {:?}", error);
                exit(1)
            }

            Ok(ok_response) => {
                let status_code = ok_response.status();
                let json: Value = ok_response.json().await.unwrap();
                match status_code {
                    StatusCode::UNAUTHORIZED => {
                        println!("Error: {}", json);
                        exit(1)
                    },
                    _ => Some(json)
                }
            }
        };

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
        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();
        if let Some(events) = self.get_log_events() {
            events.iter().for_each(|event| {
                writeln!(stdout_lock, "{}", event).unwrap();
            });
        }
    }

    pub async fn main_loop(&mut self) {
        loop {
            self.fetch_logs().await;
            self.print_logs().await;
        }
    }
}
