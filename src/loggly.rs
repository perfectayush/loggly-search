extern crate reqwest;

use std::{io, process::exit};
use std::io::Write;

use log::*;
use reqwest::{Client, StatusCode};
use serde_json::value::Value;


#[derive(Debug)]
pub struct Loggly {
    client: Client,
    account: String,
    api_token: String,
    response: Option<Value>,
    from: String,
    query: String,
}

impl Loggly {
    pub fn init(account: &str, api_token: &str, from: &str, query: &str) -> Self {
        Loggly {
            client: Client::new(),
            account: String::from(account),
            api_token: String::from(api_token),
            response: None,
            from: String::from(from),
            query: String::from(query),
        }
    }

    fn create_search_uri(&self) -> String {
        format!("https://{}.loggly.com/apiv2/events/iterate?q={}&from={}&until=now&size=100&order=asc", self.account, self.query, self.from)
    }

    pub async fn fetch_logs(&mut self) {
        let uri = self.get_search_uri();

        let response = self.client
            .get(uri)
            .bearer_auth(&self.api_token)
            .send().await;

        self.response = match response {
            Err(error) => {
                error!("Error occurred while making request: {:?}", error);
                exit(1)
            }

            Ok(ok_response) => {
                let status_code = ok_response.status();
                let json: Value = ok_response.json().await.unwrap();
                match status_code {
                    StatusCode::UNAUTHORIZED => {
                        error!("{}", json);
                        exit(1)
                    },
                    _ => Some(json)
                }
            }
        };

    }


    fn get_search_uri(&mut self) -> String {
        let uri = match self.response {
            None => self.create_search_uri(),
            Some(ref json) => {
                match json.get("next") {
                    Some(next) => String::from(next.as_str().unwrap()),
                    None => {
                        debug!("Updating timestamp!");
                        self.update_last_timestamp();
                        self.create_search_uri()
                    },
                }
            }
        };
        uri
    }

    fn get_log_events(&self) -> Option<&Vec<Value>> {
        match &self.response {
            Some(json) => Some(json.get("events").unwrap().as_array().unwrap()),
            None => None,
        }
    }

    fn update_last_timestamp(&mut self) {
        if let Some(event) = self.get_log_events() {
            match event.last() {
                Some(value) => {
                    self.from = (value.get("timestamp").unwrap().as_u64().unwrap() + 1).to_string();
                },
                None => return,
            }
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
