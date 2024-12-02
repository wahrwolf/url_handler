use crate::external_fascade::OpenSSHFascade;
use anyhow::Result;
use path_absolutize::*;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use tempfile::TempDir;
use url::Url;

use super::ProtocolHandler;

#[derive(Default)]
pub struct SCPProtocolHandler {}

impl ProtocolHandler for SCPProtocolHandler {
    fn fetch_string_from_url(&self, url: &Url) -> Result<Option<String>> {
        let tmp_dir: TempDir = TempDir::new()?;
        let target_file = tmp_dir.path().join("string");
        OpenSSHFascade::download_file(&url, &target_file)?;
        let string = read_to_string(target_file)?;
        Ok(Some(string))
    }
    fn push_string_to_url(&self, url: &Url, string: &String) -> Result<()> {
        let tmp_dir: TempDir = TempDir::new()?;
        let source_file = tmp_dir.path().join("string");
        write(&source_file, string)?;
        OpenSSHFascade::upload_file(&source_file, &url)?;
        Ok(())
    }
}

pub fn try_build_url_from_path_buf_with_hostname(path: &PathBuf, hostname: &str) -> Result<Url> {
    let absolute_path = path.absolutize()?;
    let url_string = format!(
        "scp://{}{}",
        hostname,
        absolute_path.as_os_str().to_str().unwrap()
    );
    let url = Url::parse(&url_string)?;
    Ok(url)
}
