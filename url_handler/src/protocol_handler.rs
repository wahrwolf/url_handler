use anyhow::Result;
use url::Url;

mod file;
pub use file::{try_build_url_from_path_buf, FileProtocolHandler};
mod scp;
pub use scp::{try_build_url_from_path_buf_with_hostname, SCPProtocolHandler};
mod http;
pub use http::HttpProtocolHandler;
mod registry;
pub use registry::{ProtocolHandlerConfig, ProtocolHandlerRegistry};

pub trait ProtocolHandler {
    fn fetch_string_from_url(&self, url: &Url) -> Result<Option<String>>;
    fn push_string_to_url(&self, url: &Url, string: &String) -> Result<()>;
}

pub enum KnownProtocolHandler {
    File(FileProtocolHandler),
    Scp(SCPProtocolHandler),
    Http(HttpProtocolHandler),
}

impl KnownProtocolHandler {
    pub fn to_handler(&self) -> &dyn ProtocolHandler {
        match self {
            KnownProtocolHandler::File(handler) => handler as &dyn ProtocolHandler,
            KnownProtocolHandler::Scp(handler) => handler as &dyn ProtocolHandler,
            KnownProtocolHandler::Http(handler) => handler as &dyn ProtocolHandler,
        }
    }
}

impl ProtocolHandler for KnownProtocolHandler {
    fn fetch_string_from_url(&self, url: &Url) -> Result<Option<String>> {
        self.to_handler().fetch_string_from_url(url)
    }

    fn push_string_to_url(&self, url: &Url, string: &String) -> Result<()> {
        self.to_handler().push_string_to_url(url, string)
    }
}

pub fn fetch_string_from_url(
    url: &Url,
    registry: &ProtocolHandlerRegistry,
) -> Result<Option<String>> {
    let protocol = url.scheme();
    let Some(handler) = registry.get_handler_for_protocol(protocol) else {
        anyhow::bail!("Could not find handler for protocol '{protocol}'");
    };
    handler.fetch_string_from_url(url)
}

pub fn push_string_to_url(
    url: &Url,
    string: &String,
    registry: &ProtocolHandlerRegistry,
) -> Result<()> {
    let protocol = url.scheme();
    let Some(handler) = registry.get_handler_for_protocol(protocol) else {
        anyhow::bail!("Could not find handler for protocol '{protocol}'");
    };
    handler.push_string_to_url(url, string)
}
