use crate::{BrowserError, BrowserPolicy, History};
use reqwest::blocking::{Client, Response};
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};
use std::io::Read;
use url::Url;

/// Browser configuration and resource limits.
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    pub policy: BrowserPolicy,
    pub user_agent: String,
    pub timeout_seconds: u64,
    pub max_redirects: usize,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            policy: BrowserPolicy::default(),
            user_agent: "GlobusOS-Browser/0.1".into(),
            timeout_seconds: 20,
            max_redirects: 10,
        }
    }
}

/// A bounded HTTP response delivered to the browser content pipeline.
#[derive(Debug, Clone)]
pub struct BrowserResponse {
    pub url: String,
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

/// HTTP(S) navigation and per-instance history.
pub struct Browser {
    client: Client,
    config: BrowserConfig,
    history: History,
}

impl Browser {
    /// Creates a browser with the supplied security and resource policy.
    pub fn new(config: BrowserConfig) -> Result<Self, BrowserError> {
        if config.timeout_seconds == 0 {
            return Err(BrowserError::ResourceLimit(
                "timeout must be greater than zero".into(),
            ));
        }

        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .user_agent(config.user_agent.clone())
            .build()?;

        Ok(Self {
            client,
            config,
            history: History::new(),
        })
    }

    /// Navigates to an HTTP(S) URL, validating every redirect before following it.
    pub fn navigate(&mut self, raw_url: &str) -> Result<BrowserResponse, BrowserError> {
        let mut url = Url::parse(raw_url).map_err(|_| BrowserError::InvalidUrl(raw_url.into()))?;
        self.config.policy.validate(&url)?;

        for redirect_count in 0..=self.config.max_redirects {
            let response = self
                .client
                .get(url.clone())
                .header(USER_AGENT, self.config.user_agent.clone())
                .send()?;

            if response.status().is_redirection() {
                if redirect_count == self.config.max_redirects {
                    return Err(BrowserError::ResourceLimit(
                        "maximum redirect count exceeded".into(),
                    ));
                }

                let location = response
                    .headers()
                    .get(LOCATION)
                    .ok_or_else(|| {
                        BrowserError::InvalidUrl("redirect response has no Location header".into())
                    })?
                    .to_str()
                    .map_err(|_| {
                        BrowserError::InvalidUrl("redirect Location is not valid UTF-8".into())
                    })?;

                let next = url
                    .join(location)
                    .map_err(|_| BrowserError::InvalidUrl(location.into()))?;
                self.config.policy.validate(&next)?;
                url = next;
                continue;
            }

            return self.finish_response(response);
        }

        Err(BrowserError::ResourceLimit(
            "navigation loop exhausted".into(),
        ))
    }

    fn finish_response(&mut self, mut response: Response) -> Result<BrowserResponse, BrowserError> {
        let final_url = response.url().clone();
        self.config.policy.validate(&final_url)?;

        let limit = self.config.policy.max_response_bytes;
        if response.content_length().unwrap_or(0) > limit as u64 {
            return Err(BrowserError::ResourceLimit(format!(
                "response exceeds configured size limit of {limit} bytes"
            )));
        }

        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let status = response.status().as_u16();

        let mut body = Vec::with_capacity(
            response
                .headers()
                .get(CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0)
                .min(limit),
        );
        let mut limited = (&mut response).take((limit as u64).saturating_add(1));
        limited
            .read_to_end(&mut body)
            .map_err(|e| BrowserError::ResourceLimit(format!("response read failed: {e}")))?;
        if body.len() > limit {
            return Err(BrowserError::ResourceLimit(format!(
                "response exceeds configured size limit of {limit} bytes"
            )));
        }

        self.history.push(final_url.to_string());
        Ok(BrowserResponse {
            url: final_url.to_string(),
            status,
            content_type,
            body,
        })
    }

    /// Moves the active history entry backward.
    pub fn back(&mut self) -> Option<&str> {
        self.history.back().map(|e| e.url.as_str())
    }

    /// Moves the active history entry forward.
    pub fn forward(&mut self) -> Option<&str> {
        self.history.forward().map(|e| e.url.as_str())
    }

    /// Returns the active URL.
    pub fn current_url(&self) -> Option<&str> {
        self.history.current().map(|e| e.url.as_str())
    }

    /// Returns the navigation history.
    pub fn history(&self) -> &History {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_web_scheme() {
        let policy = BrowserPolicy::default();
        let url = Url::parse("file:///etc/passwd").expect("parse");
        assert!(policy.validate(&url).is_err());
    }

    #[test]
    fn rejects_invalid_port() {
        let policy = BrowserPolicy::default();
        let url = Url::parse("https://example.com:0/").expect("parse");
        assert!(policy.validate(&url).is_err());
    }

    #[test]
    fn rejects_zero_timeout() {
        assert!(
            Browser::new(BrowserConfig {
                timeout_seconds: 0,
                ..BrowserConfig::default()
            })
            .is_err()
        );
    }
}
