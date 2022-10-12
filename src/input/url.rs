use crate::input::Input;
use async_trait::async_trait;
use reqwest::{Url, StatusCode};

/// UrlInput downloads data from an Url
#[derive(Debug)]
pub struct UrlInput {
    url: Url,
    response: Option<reqwest::Response>,
}

impl UrlInput {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            response: None,
        }
    }
}

#[async_trait]
impl Input for UrlInput {
    async fn chunk(&mut self) -> anyhow::Result<Option<Vec<u8>>> {
        if self.response.is_none() {
            self.response = Some(reqwest::get(self.url.clone()).await?);
        }

        let status_code = self.response.as_ref().unwrap().status();
        if status_code != StatusCode::OK {
            return Err(anyhow::anyhow!("status code {}: {}", status_code, self.url,));
        }

        if let Some(len) = self.response.as_ref().unwrap().content_length() && len == 0 {
            return Err(anyhow::anyhow!("empty response body: {}", self.url,));
        }

        match self.response.as_mut().unwrap().chunk().await {
            Ok(Some(r)) => {
                let r = r.to_vec();
                Ok(Some(r))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    async fn reset(&mut self) -> anyhow::Result<()> {
        if self.response.is_none() {
            self.response = Some(reqwest::get(self.url.clone()).await?);
        }
        Ok(())
    }
}
