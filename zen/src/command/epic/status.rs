pub struct Command {
    api_token: String,
    repo_id: i32,
    epic_id: i32,
}

impl Command {
    pub fn new(api_token: String, repo_id: i32, epic_id: i32) -> Self {
        Self {
            api_token,
            repo_id,
            epic_id
        }
    }

    pub async fn run(&self) {
        let repo = zenhub_api::EpicRepository::new(self.api_token.to_owned());
        println!(
            "{:?}",
            repo.get_data(self.repo_id, self.epic_id).await
        );
    }
}
