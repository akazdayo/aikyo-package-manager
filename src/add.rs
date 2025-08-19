use std::process::{Command, Stdio};

pub fn clone_from_git(url: String) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("git")
        .arg("clone")
        .arg(&url)
        .stdout(Stdio::piped())
        .spawn()?
        .wait()?;
    Ok(())
}
