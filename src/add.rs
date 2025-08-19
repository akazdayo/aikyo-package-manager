use std::process::{Command, Stdio};

pub fn clone_from_git(url: String) {
    Command::new("git")
        .arg("clone")
        .arg(&url)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to clone.");
}
