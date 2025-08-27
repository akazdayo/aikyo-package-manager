use crate::manager::Project;
use anyhow::{Result, anyhow};
use regex::Regex;
use std::{fs, path::Path, process::Command};

pub struct Sync {
    project: Project,
}

impl Sync {
    pub fn new(project: Project) -> Self {
        Self { project }
    }

    fn check_exists(&self) -> Result<Vec<String>> {
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

    pub fn parse_repositry_name(urls: &Vec<String>) -> Result<Vec<String>> {
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

    pub fn clone_from_git(url: &str, tools_dir: &str, target_dir: Option<&str>) -> Result<()> {
        let dir = Path::new(tools_dir);
        if !dir.is_dir() {
            fs::create_dir_all(dir)?;
        }

        let mut command = Command::new("git");
        command.current_dir(tools_dir).arg("clone").arg(url);

        if let Some(dir) = target_dir {
            command.arg(dir);
        }

        let status = command.status()?;
        if !status.success() {
            return Err(anyhow!("git clone failed"));
        }

        Ok(())
    }

    // TODO: イテレータを使用してより関数型プログラミングスタイルにリファクタリング
    // TODO: 並列処理でcloneを高速化する
    // TODO: 既存プラグインの更新機能を追加する
    pub fn sync(&self) -> Result<()> {
        let exist_plugins = &self.check_exists()?;
        let all_plugin_names = Self::parse_repositry_name(&self.project.plugins)?;

        // TODO: zip()とfilter()を使ってより簡潔に書き換える
        let mut queued_plugins: Vec<&String> = Vec::new();
        for i in 0..all_plugin_names.len() {
            if !exist_plugins.contains(&all_plugin_names[i]) {
                queued_plugins.push(&self.project.plugins[i]);
            }
        }
        println!("Clone queued plugins: {:?}", queued_plugins);

        // Clone all plugins
        for plugin in queued_plugins {
            Self::clone_from_git(plugin, &self.project.tools_dir, None)?;
        }
        Ok(())
    }
}
