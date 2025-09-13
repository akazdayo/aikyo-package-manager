use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    pub plugins: Vec<String>,
    pub tools_dir: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub project: Project,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let path = Path::new("apm.toml");
        let config: Config;

        if path.is_file() {
            config = Self::read_config("apm.toml")?;
        } else {
            config = Config {
                project: Project {
                    plugins: Vec::new(),
                    tools_dir: "./aikyo_dependencies".to_string(),
                },
            };
            Self::save_file("apm.toml", &toml::to_string(&config)?)?;
        }
        Ok(config)
    }

    fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut content = String::new();

        let mut fr = fs::File::open(path)
            .map(|f| BufReader::new(f))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        fr.read_to_string(&mut content)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(toml::from_str(&content)?)
    }

    fn save_file(path: &str, data: &String) -> Result<(), std::io::Error> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        write!(file, "{}", data)?;
        file.flush()?;

        Ok(())
    }

    // TODO: 重複プラグインのチェック機能を追加する
    pub fn append_plugin(&mut self, plugin: String) -> Result<(), Box<dyn std::error::Error>> {
        self.project.plugins.push(plugin);
        Self::save_file("apm.toml", &toml::to_string(&self)?)?; // BUG: should be "apm.toml"
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() {
        let config_path = Path::new("apm.toml");
        let folder_path = Path::new("./apm_tools");
        if config_path.is_file() {
            fs::remove_file(config_path).unwrap();
        }
        if folder_path.is_dir() {
            fs::remove_dir_all(folder_path).unwrap();
        }
    }

    #[test]
    fn new_config_read_and_write() {
        setup();

        let config = Config::new().unwrap(); // New Config
        let config2 = Config::new().unwrap(); // Read Config

        assert!(Path::new("apm.toml").is_file());
        assert_eq!(config, config2);
    }

    #[test]
    fn append_plugin() {
        let config_path = Path::new("apm.toml");
        if config_path.is_file() {
            fs::remove_file(config_path).unwrap();
        }
        let mut config = Config::new().unwrap();
        //config.append_plugin(PLUGIN_URL.to_string()).unwrap();
        //assert_eq!(
        //    config.project.plugins.contains(&PLUGIN_URL.to_string()),
        //    true
        //);

        let config2 = Config::new().unwrap();
        //assert_eq!(config, config2);

        let download_dir = Path::new("./apm_tools/apm");
        assert_eq!(!download_dir.is_dir(), true);
    }
}
