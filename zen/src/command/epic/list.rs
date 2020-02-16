pub struct Command {
    api_token: String,
    repo_id: i32,
}

impl Command {
    pub fn new(api_token: String, repo_id: i32) -> Self {
        Self {
            api_token,
            repo_id,
        }
    }

    pub async fn run(&self) {
        let repo = zenhub_api::EpicRepository::new(self.api_token.to_owned());
        println!(
            "{:?}",
            repo.get_epics(self.repo_id).await
        );
    }
}
