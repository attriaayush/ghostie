use anyhow::Result;
use futures::Future;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    Client, Method,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

use crate::github::activity::Activity;

const DEFAULT_HOST: &str = "https://api.github.com/";
const MEDIA_TYPE: &str = "application/vnd.github.v3+json";

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Error fetching notifications from github. Potentially related to connection or GH token.")]
    IOError(#[from] reqwest::Error),
}

#[derive(Debug, Clone)]
pub enum Credentials {
    Token(String),
}

impl Credentials {
    fn bearer_token(&self) -> String {
        match self {
            Credentials::Token(token) => format!("token {}", token),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Github {
    host: String,
    agent: String,
    client: Client,
    credentials: Credentials,
}

impl Github {
    pub fn init_with_token<C>(credentials: C) -> Self
    where
        C: Into<Credentials>,
    {
        let client = Client::builder().build().unwrap();
        Self {
            host: DEFAULT_HOST.to_string(),
            agent: "ghostie".into(),
            client,
            credentials: credentials.into(),
        }
    }

    fn request<P: Serialize + ?Sized>(
        &self,
        method: Method,
        url: String,
        params: Option<&P>,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        let instance = self.clone();
        let mut req = instance.client.request(method, url);

        if let Some(params) = params {
            req = req.query(params);
        }

        req = req.header(USER_AGENT, &instance.agent);
        req = req.header(ACCEPT, MEDIA_TYPE);
        req = req.header(AUTHORIZATION, self.credentials.bearer_token());

        req.send()
    }

    pub async fn patch<P: Serialize + ?Sized>(&self, uri: &str, params: Option<&P>) -> Result<()> {
        self.request(Method::PATCH, self.host.clone() + uri, params).await?;
        Ok(())
    }

    pub async fn get<T: DeserializeOwned, P: Serialize + ?Sized>(
        &self,
        uri: &str,
        params: Option<&P>,
    ) -> Result<T, NotificationError> {
        let response = self.request(Method::GET, self.host.clone() + uri, params).await?;
        let result = response.json::<T>().await?;
        Ok(result)
    }

    pub fn user_activity(&self) -> Activity {
        Activity::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Credentials;

    #[test]
    fn parse_token() {
        let token = String::from("super-secret-stuff");
        let credentials = Credentials::Token(token.clone());
        assert_eq!(credentials.bearer_token(), "token".to_string() + " " + &token)
    }
}
