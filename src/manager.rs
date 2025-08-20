use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub plugins: Vec<String>,
    pub tools_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let path = Path::new("apm.toml");
        let config: Config;

        if path.is_file() {
            config = Self::read_config("apm.toml".to_owned())?;
        } else {
            config = Config {
                project: Project {
                    plugins: Vec::new(),
                    tools_dir: "./tools".to_string(),
                },
            };
            Self::save_file(&"apm.toml".to_string(), &toml::to_string(&config)?)?;
        }
        Ok(config)
    }

    fn read_config(path: String) -> Result<Config, Box<dyn std::error::Error>> {
        let mut content = String::new();

        let mut fr = fs::File::open(path)
            .map(|f| BufReader::new(f))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        fr.read_to_string(&mut content)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(toml::from_str(&content)?)
    }

    fn save_file(path: &String, data: &String) -> Result<(), std::io::Error> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        write!(file, "{}", data)?;
        file.flush()?;
        Ok(())
    }

    pub fn append_plugin(mut self, plugin: String) -> Result<(), Box<dyn std::error::Error>> {
        self.project.plugins.push(plugin);
        Self::save_file(&self.project.tools_dir, &toml::to_string(&self)?)?;
        Ok(())
    }
}
