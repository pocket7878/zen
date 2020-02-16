extern crate serde_json;

use crate::client::Client;
use serde_json::Value;
use std::error::Error;

pub struct IssueRepository {
    api_token: String,
}

impl IssueRepository {
    pub fn new(api_token: String) -> IssueRepository {
        IssueRepository { api_token }
    }

    pub async fn get_data(&self, repo_id: i32, issue_number: i32) -> Result<Value, Box<dyn Error>> {
        let client = Client::new(self.api_token.to_owned());
        let response = client
            .get(&format!(
                "https://api.zenhub.com/p1/repositories/{}/issues/{}",
                repo_id, issue_number
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }

    pub async fn get_events(
        &self,
        repo_id: i32,
        issue_number: i32,
    ) -> Result<Value, Box<dyn Error>> {
        let client = Client::new(self.api_token.to_owned());
        let response = client
            .get(&format!(
                "https://api.zenhub.com/p1/repositories/{}/issues/{}/events",
                repo_id, issue_number
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }
}
