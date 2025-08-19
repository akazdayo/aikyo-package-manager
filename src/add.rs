use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

pub fn clone_from_git(url: &String, path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let original_dir = std::env::current_dir()?;
    let dir = Path::new(&path);
    if !dir.is_dir() {
        fs::create_dir(dir)?;
    }
    std::env::set_current_dir(path)?;

    Command::new("git")
        .arg("clone")
        .arg(url)
        .stdout(Stdio::piped())
        .spawn()?
        .wait()?;

    std::env::set_current_dir(original_dir)?;
    Ok(())
}
