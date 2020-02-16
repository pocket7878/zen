extern crate reqwest;

use reqwest::{IntoUrl, RequestBuilder};

pub(crate) struct Client {
    client: reqwest::Client,
}

impl Client {
    pub(crate) fn new(api_token: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-Authentication-Token",
            reqwest::header::HeaderValue::from_str(&api_token).unwrap(),
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        Client { client }
    }

    pub(crate) fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.client.get(url)
    }
}
