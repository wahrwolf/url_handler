use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use url::Url;

pub struct OpenSSHFascade {}

impl OpenSSHFascade {
    pub fn copy(source: &Url, target: &Url) -> Result<()> {
        match source.scheme() {
            "file" => {
                let Ok(source_file) = source.clone().to_file_path() else {
                    anyhow::bail!("Could not transform source into PathBuf");
                };
                match target.scheme() {
                    "file" => {
                        let Ok(target_file) = target.clone().to_file_path() else {
                            anyhow::bail!("Could not transform target into PathBuf");
                        };
                        Self::copy_local_file(&source_file, &target_file)
                    }
                    "scp" => Self::upload_file(&source_file, &target.clone()),
                    _ => anyhow::bail!(
                        "target scheme is not supported! Only scp and file are valid."
                    ),
                }
            }
            "scp" => match target.scheme() {
                "file" => {
                    let Ok(target_file) = target.clone().to_file_path() else {
                        anyhow::bail!("Could not transform target into PathBuf");
                    };
                    Self::download_file(source, &target_file)
                }
                "scp" => Self::copy_remote_file(source.clone(), target.clone()),
                _ => anyhow::bail!("target scheme is not supported! Only scp and file are valid."),
            },
            _ => anyhow::bail!("source scheme is not supported! Only scp and file are valid."),
        }
    }

    pub fn upload_file(source: &Path, target: &Url) -> Result<()> {
        Self::copy_file(source.as_os_str(), target.clone().as_str().as_ref())
    }

    pub fn download_file(source: &Url, target: &Path) -> Result<()> {
        Self::copy_file(
            source.clone().as_str().as_ref(),
            target.as_os_str(),
        )
    }

    pub fn copy_remote_file(source: Url, target: Url) -> Result<()> {
        Self::copy_file(source.as_str().as_ref(), target.as_str().as_ref())
    }

    pub fn copy_local_file(source: &Path, target: &Path) -> Result<()> {
        Self::copy_file(
            source.as_os_str(),
            target.as_os_str(),
        )
    }

    fn copy_file(source: &OsStr, target: &OsStr) -> Result<()> {
        Command::new("scp").args([source, target]).status()?;
        Ok(())
    }
}
