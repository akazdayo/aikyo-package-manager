use crate::manager::Project;
use regex::Regex;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

pub struct Sync<'a> {
    project: &'a Project,
}

impl<'a> Sync<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    fn check_exists(&self) -> Result<Vec<String>, std::io::Error> {
        // フォルダが存在しない場合は初期化して返却
        if !Path::new(&self.project.tools_dir).is_dir() {
            fs::create_dir_all(&self.project.tools_dir)?;
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.project.tools_dir)?;
        let mut exist_dirs: Vec<String> = Vec::new();
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                exist_dirs.push(entry.file_name().to_string_lossy().to_string());
            }
        }
        Ok(exist_dirs)
    }

    pub fn parse_repositry_name(urls: &Vec<String>) -> Result<Vec<String>, regex::Error> {
        let mut names: Vec<String> = Vec::new();
        let re = Regex::new(r"/([^/]+)\.git$")?;

        for url in urls {
            if let Some(caps) = re.captures(&url) {
                if let Some(name) = caps.get(1) {
                    names.push(name.as_str().to_string());
                }
            }
        }
        Ok(names)
    }

    fn clone_from_git(url: &String, path: &String) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        let exist_plugins = &self.check_exists()?;
        let all_plugin_names = Self::parse_repositry_name(&self.project.plugins)?;

        let mut queued_plugins: Vec<&String> = Vec::new();
        for i in 0..all_plugin_names.len() {
            if !exist_plugins.contains(&all_plugin_names[i]) {
                queued_plugins.push(&self.project.plugins[i]);
            }
        }

        // Clone all plugins
        for plugin in queued_plugins {
            Self::clone_from_git(plugin, &self.project.tools_dir)?;
        }
        Ok(())
    }
}
