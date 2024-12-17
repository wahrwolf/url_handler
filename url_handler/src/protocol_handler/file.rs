use std::collections::HashSet;
use super::ProtocolHandler;
use anyhow::{Context, Result};
use path_absolutize::*;
use std::fs::{create_dir_all, read_to_string, write, remove_file};
use std::path::Path;
use std::path::PathBuf;
use url::Url;

#[derive(Default)]
pub struct FileProtocolHandler {}

impl ProtocolHandler for FileProtocolHandler {
    fn fetch_string_from_url(&self, url: &Url) -> Result<Option<String>> {
        let Ok(path) = url.to_file_path() else {
            anyhow::bail!("Could not parse URL to path");
        };
        let string = read_to_string(path)?;
        Ok(Some(string))
    }
    fn push_string_to_url(&self, url: &Url, string: &str) -> Result<()> {
        let Ok(path) = url.to_file_path() else {
            anyhow::bail!("Could not parse URL to path");
        };

        let target_dir: &Path = path
            .parent()
            .context("Could not find target directory for {url}")?;
        if !target_dir.exists() {
            create_dir_all(target_dir)?;
        }
        write(path, string)?;
        Ok(())
    }

    fn delete_string_from_url(&self, url: &Url) -> Result<()> {
        let Ok(path) = url.to_file_path() else {
            anyhow::bail!("Could not parse URL to path");
        };
        remove_file(path)?;
        Ok(())
    }
    fn create_empty_string_on_url(&self, url: &Url) -> Result<()> {
        self.push_string_to_url(url, "")
    }

    fn create_url_container(&self, url: &Url) -> Result<()> {
        let Ok(path) = url.to_file_path() else {
            anyhow::bail!("Could not parse URL to path");
        };

        create_dir_all(path)?;
        Ok(())
    }

    fn list_urls_in_url_container(&self, url: &Url) -> Result<HashSet<Url>> {
        let Ok(path) = url.to_file_path() else {
            anyhow::bail!("Could not parse URL to path");
        };
        let mut urls: HashSet<Url> = HashSet::default();

        for dir_entry in path.read_dir()? {
            let Ok(valid_entry) = dir_entry else {
                continue;
            };
            let Ok(entry_url) = try_build_url_from_path_buf(&valid_entry.path()) else {
                continue;
            };
            urls.insert(entry_url);
        }
        Ok(urls)
    }
}

pub fn try_build_url_from_path_buf(path: &PathBuf) -> Result<Url> {
    let absolute_path = path.absolutize()?;
    let Ok(url) = Url::from_file_path(absolute_path) else {
        anyhow::bail!("Could not parse URL to path");
    };
    Ok(url)
}
