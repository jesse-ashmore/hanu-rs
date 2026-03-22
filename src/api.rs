use anyhow::Result;
use reqwest::{header::HeaderMap, Url};
use std::sync::Arc;

pub struct Client {
    api_base: Url,
    client: Arc<reqwest::Client>,
}

impl Client {
    pub fn new(domain: &str) -> Result<Client> {
        let api_base = Url::parse(format!("https://{domain}/v0/").as_str())?;

        let headers = HeaderMap::new();

        // set default client operation
        let client = Arc::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        );

        Ok(Client { api_base, client })
    }

    pub fn top_stories(&self) -> TopStoriesEndpoint<'_> {
        TopStoriesEndpoint(self)
    }
}

pub struct TopStoriesEndpoint<'c>(&'c Client);

pub type StoryId = u32;

impl<'c> TopStoriesEndpoint<'c> {
    fn endpoint(&self) -> Result<Url> {
        Ok(self.0.api_base.join("topstories.json")?)
    }

    pub async fn get_all(&self) -> Result<Vec<StoryId>, String> {
        let text = self
            .0
            .client
            .get(self.endpoint().unwrap())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&text).map_err(|e| e.to_string())
    }
}
