use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;
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
                    Self::download_file(&source, &target_file)
                }
                "scp" => Self::copy_remote_file(source.clone(), target.clone()),
                _ => anyhow::bail!("target scheme is not supported! Only scp and file are valid."),
            },
            _ => anyhow::bail!("source scheme is not supported! Only scp and file are valid."),
        }
    }

    pub fn upload_file(source: &PathBuf, target: &Url) -> Result<()> {
        Self::copy_file(&source.clone().into(), &target.clone().to_string().into())
    }

    pub fn download_file(source: &Url, target: &PathBuf) -> Result<()> {
        Self::copy_file(
            &source.clone().to_string().into(),
            &target.clone().into_os_string(),
        )
    }

    pub fn copy_remote_file(source: Url, target: Url) -> Result<()> {
        Self::copy_file(&source.to_string().into(), &target.to_string().into())
    }

    pub fn copy_local_file(source: &PathBuf, target: &PathBuf) -> Result<()> {
        Self::copy_file(
            &source.clone().into_os_string(),
            &target.clone().into_os_string(),
        )
    }

    fn copy_file(source: &OsString, target: &OsString) -> Result<()> {
        Command::new("scp").args(&[source, target]).status()?;
        Ok(())
    }
}
