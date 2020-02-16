extern crate serde_json;

use crate::client::Client;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::fmt;

pub struct IssueRepository {
    api_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    user_id: i32,
    #[serde(rename = "type")]
    event_type: EventType,
    created_at: String,
    from_estimate: Option<EventEstimate>,
    to_estimate: Option<EventEstimate>,
    from_pipeline: Option<EventPipeline>,
    to_pipeline: Option<EventPipeline>,
    workspace_id: Option<String>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.event_type {
            EventType::EstimateIssue => write!(
                f,
                "{} [{}] by {} to {}",
                self.created_at,
                self.event_type,
                self.user_id,
                self.to_estimate.as_ref().unwrap().value
            ),
            EventType::TransferIssue => write!(
                f,
                "{} [{}] by {} {} -> {}",
                self.created_at,
                self.event_type,
                self.user_id,
                self.from_pipeline.as_ref().unwrap().name,
                self.to_pipeline.as_ref().unwrap().name
            ),
            EventType::ConvertIssueToEpic => write!(
                f,
                "{} [{}] by {}",
                self.created_at, self.event_type, self.user_id
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
enum EventType {
    #[serde(rename = "estimateIssue")]
    EstimateIssue,
    #[serde(rename = "transferIssue")]
    TransferIssue,
    #[serde(rename = "convertIssueToEpic")]
    ConvertIssueToEpic,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EventType::EstimateIssue => write!(f, "estimate issue"),
            EventType::TransferIssue => write!(f, "transfer issue"),
            EventType::ConvertIssueToEpic => write!(f, "convert issue to epic"),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct EventEstimate {
    value: i32,
}

#[derive(Debug, Deserialize)]
struct EventPipeline {
    name: String,
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
    ) -> Result<Vec<Event>, Box<dyn Error>> {
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
