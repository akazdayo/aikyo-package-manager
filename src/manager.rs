use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub plugins: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
}

impl Config {
    fn read_config(path: String) -> Result<Config, Box<dyn std::error::Error>> {
        let mut content = String::new();

        let mut fr = fs::File::open(path)
            .map(|f| BufReader::new(f))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        fr.read_to_string(&mut content)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn append_plugin(plugin: String) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new("apm.toml");
        let mut config: Config;

        if path.is_file() {
            config = Self::read_config("apm.toml".to_owned())?;
        } else {
            config = Config {
                project: Project {
                    plugins: Vec::new(),
                },
            };
        }
        config.project.plugins.push(plugin);

        let data = toml::to_string(&config)?;
        let mut file = File::create("apm.toml")?; // 上書き保存/新規作成
        write!(file, "{}", data)?;
        file.flush()?;

        Ok(())
    }
}
