use crate::{BrowserError, ContentKind, RenderInput};

pub trait Renderer {
    fn render(&mut self, input: RenderInput) -> Result<RenderedDocument, BrowserError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedDocument {
    pub url: String,
    pub status: u16,
    pub kind: ContentKind,
    pub body: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct PassthroughRenderer;

impl Renderer for PassthroughRenderer {
    fn render(&mut self, input: RenderInput) -> Result<RenderedDocument, BrowserError> {
        Ok(RenderedDocument {
            url: input.url,
            status: input.status,
            kind: input.kind,
            body: input.body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough_renderer_preserves_boundary_data() {
        let input = RenderInput::from_response(crate::BrowserResponse {
            url: "https://example.test/".into(),
            status: 200,
            content_type: Some("text/html".into()),
            body: b"<html></html>".to_vec(),
        })
        .expect("render input");
        let mut renderer = PassthroughRenderer;
        let output = renderer.render(input).expect("render");
        assert_eq!(output.status, 200);
        assert_eq!(output.kind, ContentKind::Html);
    }
}
