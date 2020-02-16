pub struct Command {
    api_token: String,
    repo_id: i32,
    issue_number: i32,
}

impl Command {
    pub fn new(api_token: String, repo_id: i32, issue_number: i32) -> Self {
        Command {
            api_token,
            repo_id,
            issue_number,
        }
    }

    pub async fn run(&self) {
        let repo = zenhub_api::IssueRepository::new(self.api_token.to_owned());
        for e in repo
            .get_events(self.repo_id, self.issue_number)
            .await
            .unwrap()
        {
            println!("{}", e);
        }
    }
}
