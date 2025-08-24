use crate::sync::Sync;
use anyhow::Result;

pub fn blank(path: &String) -> Result<()> {
    let blank_url = "https://github.com/akazdayo/aikyo-blank-template".to_string();
    Sync::clone_from_git(&blank_url, path)?;
    Ok(())
}

pub fn basic(path: &String) -> Result<()> {
    let blank_url = "https://github.com/akazdayo/aikyo-basic-template".to_string();
    Sync::clone_from_git(&blank_url, path)?;
    Ok(())
}
