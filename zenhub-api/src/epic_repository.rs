extern crate serde_json;

use crate::client::Client;
use serde_json::Value;
use std::error::Error;

pub struct EpicRepository {
    api_token: String,
}

impl EpicRepository {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }

    pub async fn get_epics(&self, repo_id: i32) -> Result<Value, Box<dyn Error>> {
        let client = Client::new(self.api_token.to_owned());
        let response = client
            .get(&format!(
                "https://api.zenhub.com/p1/repositories/{}/epics",
                repo_id
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }

    pub async fn get_data(&self, repo_id: i32, epic_id: i32) -> Result<Value, Box<dyn Error>> {
        let client = Client::new(self.api_token.to_owned());
        let response = client
            .get(&format!(
                "https://api.zenhub.com/p1/repositories/{}/epics/{}",
                repo_id, epic_id
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }
}
