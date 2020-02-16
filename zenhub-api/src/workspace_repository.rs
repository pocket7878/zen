extern crate serde_json;

use crate::client::Client;
use serde_json::Value;
use std::error::Error;

pub struct WorkspaceRepository {
    api_token: String,
}

impl WorkspaceRepository {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }

    pub async fn get_workspaces(&self, repo_id: i32) -> Result<Value, Box<dyn Error>> {
        let client = Client::new(self.api_token.to_owned());
        let response = client
            .get(&format!(
                "https://api.zenhub.com/p2/repositories/{}/workspaces",
                repo_id,
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }
}
