use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::vec::Vec;

pub trait FormatHandler<T: Serialize + DeserializeOwned> {
    fn from_str(&self, data: &str) -> Result<T>;
    fn to_string(&self, record: &T) -> Result<String>;
}

#[derive(Clone, Debug)]
pub enum KnownFormatHandler {
    Toml(TomlHandler),
    Json(JsonHandler),
}

impl KnownFormatHandler {
    pub fn to_handler<T: Serialize + DeserializeOwned>(&self) -> &dyn FormatHandler<T> {
        match self {
            KnownFormatHandler::Toml(handler) => handler as &dyn FormatHandler<T>,
            KnownFormatHandler::Json(handler) => handler as &dyn FormatHandler<T>,
        }
    }
}

impl<T: Serialize + DeserializeOwned> FormatHandler<T> for KnownFormatHandler {
    fn from_str(&self, string: &str) -> Result<T> {
        self.to_handler().from_str(string)
    }
    fn to_string(&self, record: &T) -> Result<String> {
        self.to_handler().to_string(record)
    }
}

#[derive(Clone, Debug)]
pub struct FormatHandlerRegistry {
    toml: KnownFormatHandler,
    json: KnownFormatHandler,
}

impl Default for FormatHandlerRegistry {
    fn default() -> Self {
        FormatHandlerRegistry {
            toml: KnownFormatHandler::Toml(TomlHandler::default()),
            json: KnownFormatHandler::Json(JsonHandler::default()),
        }
    }
}

impl FormatHandlerRegistry {
    pub fn new() -> Self {
        FormatHandlerRegistry::default()
    }
    pub fn get_handlers(&self) -> Vec<&KnownFormatHandler> {
        let set = vec![
            &self.toml,
            &self.json
        ];
        set
    }

    pub fn get_handler_for_format(&self, format: &str) -> Option<&KnownFormatHandler> {
        let handler = match format {
            "toml" => &self.toml,
            "json" => &self.toml,
            _ => return None,
        };
        Some(handler)
    }
}

#[derive(Default, Clone, Debug)]
pub struct TomlHandler {}
impl<T: Serialize + DeserializeOwned> FormatHandler<T> for TomlHandler {
    fn from_str(&self, string: &str) -> Result<T> {
        let record = toml::from_str(string)?;
        Ok(record)
    }
    fn to_string(&self, record: &T) -> Result<String> {
        let string = toml::to_string(&record)?;
        Ok(string)
    }
}

#[derive(Default, Clone, Debug)]
pub struct JsonHandler {}
impl<T: Serialize + DeserializeOwned> FormatHandler<T> for JsonHandler {
    fn from_str(&self, string: &str) -> Result<T> {
        let record = serde_json::from_str(string)?;
        Ok(record)
    }
    fn to_string(&self, record: &T) -> Result<String> {
        let string = serde_json::to_string(&record)?;
        Ok(string)
    }
}

pub fn build_string_from_record_with_extension<T: Serialize + DeserializeOwned>(
    record: &T,
    extension: &str,
    registry: &FormatHandlerRegistry,
) -> Result<String> {
    let Some(handler) = registry.get_handler_for_format(extension) else {
        anyhow::bail!("No handler for format '{extension} known!")
    };
    handler.to_string(record)
}

pub fn build_record_from_string_with_extension<T: Serialize + DeserializeOwned>(
    string: &str,
    extension: &str,
    registry: &FormatHandlerRegistry,
) -> Result<T> {
    let Some(handler) = registry.get_handler_for_format(extension) else {
        anyhow::bail!("No handler for format '{extension} known!")
    };
    handler.from_str(string)
}

pub fn build_record_from_string<T: Serialize + DeserializeOwned>(
    string: &str,
    registry: &FormatHandlerRegistry,
) -> Result<T> {
    for handler in registry.get_handlers() {
        let Ok(record) = handler.from_str(string) else {
            continue;
        };
        return Ok(record);
    }
    anyhow::bail!("No handler could parse the string!")
}
