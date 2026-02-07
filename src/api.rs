use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};
use std::sync::Arc;

pub struct Client {
    api_base: Url,
    client: Arc<reqwest::Client>,
}

impl Client {
    pub fn new(domain: &str) -> Result<Client> {
        let api_base = Url::parse(format!("https://{domain}/v0/").as_str())?;

        let mut headers = HeaderMap::new();

        // set default client operation
        let client = Arc::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        );

        Ok(Client { api_base, client })
    }

    pub fn top_stories(&self) -> TopStoriesEndpoint {
        TopStoriesEndpoint(self)
    }
}

pub struct TopStoriesEndpoint<'c>(&'c Client);

impl<'c> TopStoriesEndpoint<'c> {
    fn endpoint(&self) -> Result<Url> {
        Ok(self.0.api_base.join("topstories.json")?)
    }

    pub async fn get_all(&self) -> Result<String> {
        dbg!(Ok(self.0.client.get(self.endpoint().unwrap()).send().await?.text().await?))
    }
}
