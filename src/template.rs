use crate::sync::Sync;
use anyhow::Result;

pub fn blank(path: &str) -> Result<()> {
    Sync::clone_from_git(
        "https://github.com/akazdayo/aikyo-blank-template",
        &path,
        Some("./blank_tamplate"),
    )?;
    Ok(())
}

pub fn basic(path: &str) -> Result<()> {
    Sync::clone_from_git(
        "https://github.com/akazdayo/aikyo-basic-template",
        path,
        Some("./bacic_template"),
    )?;
    Ok(())
}

pub fn from_url(url: &str, path: &str) -> Result<()> {
    Sync::clone_from_git(url, path, None)?;
    Ok(())
}
