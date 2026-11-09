#![allow(missing_docs)]
//! Internet-browser foundation for GlobusOS.
mod browser;
mod content;
mod error;
mod history;
mod policy;
mod renderer;
#[allow(missing_docs)]
mod tabs;

pub use browser::{Browser, BrowserConfig, BrowserResponse};
pub use content::{ContentKind, RenderInput};
pub use error::BrowserError;
pub use history::{History, HistoryEntry};
pub use policy::BrowserPolicy;
pub use renderer::{PassthroughRenderer, RenderedDocument, Renderer};
pub use tabs::{Tab, TabId, TabManager};
