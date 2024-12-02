use super::file::FileProtocolHandler;
use super::http::{HttpProtocolHandler, HttpProtocolHandlerConfig};
use super::scp::SCPProtocolHandler;
use super::KnownProtocolHandler;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProtocolHandlerConfig {
    http: HttpProtocolHandlerConfig,
}

pub struct ProtocolHandlerRegistry {
    file_handler: KnownProtocolHandler,
    scp_handler: KnownProtocolHandler,
    http_handler: KnownProtocolHandler,
}

impl ProtocolHandlerRegistry {
    pub fn default() -> Self {
        ProtocolHandlerRegistry::new(&ProtocolHandlerConfig::default())
    }

    pub fn new(config: &ProtocolHandlerConfig) -> Self {
        let registry = ProtocolHandlerRegistry {
            file_handler: KnownProtocolHandler::File(FileProtocolHandler::default()),
            scp_handler: KnownProtocolHandler::Scp(SCPProtocolHandler::default()),
            http_handler: KnownProtocolHandler::Http(HttpProtocolHandler::new(&config.http)),
        };
        registry
    }

    pub fn get_handler_for_protocol(&self, protocol: &str) -> Option<&KnownProtocolHandler> {
        let handler = match protocol {
            "file" => &self.file_handler,
            "scp" => &self.scp_handler,
            "http" | "https" => &self.http_handler,
            _ => return None,
        };
        Some(handler)
    }
}
