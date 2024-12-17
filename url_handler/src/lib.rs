use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;
mod protocol_handler;
pub use protocol_handler::{
    fetch_string_from_url, push_string_to_url, delete_string_from_url, create_empty_string_on_url,
    create_url_container, list_urls_in_url_container,
    try_build_url_from_path_buf, try_build_url_from_path_buf_with_hostname, ProtocolHandlerConfig,
    ProtocolHandlerRegistry,
};
mod external_fascade;
mod format_handler;
pub use format_handler::FormatHandlerRegistry;
use format_handler::{build_record_from_string, build_string_from_record_with_extension};

#[cfg(test)]
mod tests;

pub fn build_record_from_url<T: Serialize + DeserializeOwned>(
    url: &Url,
    protocol_handlers: &ProtocolHandlerRegistry,
    format_handlers: &FormatHandlerRegistry,
) -> Result<T> {
    let Some(string) = fetch_string_from_url(url, protocol_handlers)? else {
        anyhow::bail!("Record at target location empty!");
    };

    let record: T = build_record_from_string(&string, format_handlers)?;

    Ok(record)
}

pub fn push_record_to_url<T: Serialize + DeserializeOwned>(
    url: &Url,
    record: &T,
    protocol_handlers: &ProtocolHandlerRegistry,
    format_handlers: &FormatHandlerRegistry,
) -> Result<()> {
    let path = match url.to_file_path() {
        Ok(path) => path,
        Err(error) => anyhow::bail!("Can not serialize file format due to {:?}!", error),
    };
    let extension = match &path.extension() {
        Some(os_str) => match os_str.to_str() {
            Some(extension) => extension,
            None => anyhow::bail!("Can not deserialize file format because no extension found!"),
        },
        None => anyhow::bail!("Can not deserialize file format because no extension found!"),
    };

    let string = build_string_from_record_with_extension(record, extension, format_handlers)?;
    push_string_to_url(url, &string, protocol_handlers)
}
