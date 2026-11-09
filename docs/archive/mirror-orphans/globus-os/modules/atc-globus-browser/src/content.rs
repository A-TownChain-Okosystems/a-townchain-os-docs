use crate::{BrowserError, BrowserResponse};

/// Coarse content classification used to select a safe renderer boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentKind {
    Html,
    Css,
    JavaScript,
    Json,
    Image,
    PlainText,
    Binary,
    Other(String),
}

impl ContentKind {
    pub fn from_content_type(value: Option<&str>) -> Self {
        let media_type = value
            .and_then(|v| v.split(';').next())
            .map(str::trim)
            .unwrap_or_default()
            .to_ascii_lowercase();

        match media_type.as_str() {
            "text/html" | "application/xhtml+xml" => Self::Html,
            "text/css" => Self::Css,
            "application/javascript" | "text/javascript" => Self::JavaScript,
            "application/json" | "application/ld+json" => Self::Json,
            value if value.starts_with("image/") => Self::Image,
            "text/plain" => Self::PlainText,
            "" => Self::Binary,
            value => Self::Other(value.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderInput {
    pub url: String,
    pub status: u16,
    pub kind: ContentKind,
    pub body: Vec<u8>,
}

impl RenderInput {
    pub fn from_response(response: BrowserResponse) -> Result<Self, BrowserError> {
        if response.url.is_empty() {
            return Err(BrowserError::InvalidUrl("empty response URL".into()));
        }
        let kind = ContentKind::from_content_type(response.content_type.as_deref());
        Ok(Self {
            url: response.url,
            status: response.status,
            kind,
            body: response.body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_common_web_content() {
        assert_eq!(
            ContentKind::from_content_type(Some("text/html; charset=utf-8")),
            ContentKind::Html
        );
        assert_eq!(
            ContentKind::from_content_type(Some("text/css")),
            ContentKind::Css
        );
        assert_eq!(
            ContentKind::from_content_type(Some("application/json")),
            ContentKind::Json
        );
        assert_eq!(
            ContentKind::from_content_type(Some("image/png")),
            ContentKind::Image
        );
    }

    #[test]
    fn rejects_empty_response_url() {
        let response = BrowserResponse {
            url: String::new(),
            status: 200,
            content_type: Some("text/plain".into()),
            body: b"test".to_vec(),
        };
        assert!(RenderInput::from_response(response).is_err());
    }
}
