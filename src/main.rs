extern crate clap;
extern crate regex;
extern crate serde;

use anyhow::Result;
use clap::{Parser, Subcommand};
mod manager;
mod sync;
mod template;

// TODO: エラーハンドリングを改善するため専用のエラーモジュールを作成する
// TODO: コマンドハンドラーを別の関数やモジュールに分離する

/// Aikyo Package Manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Add plugin
    #[clap(arg_required_else_help = true)]
    Add {
        /// Plugin name to add
        url: String,
    },
    /// Remove plugin
    #[clap(arg_required_else_help = true)]
    Remove,

    /// Sync from apm.toml
    Sync,

    /// Create a new project or from a template
    Init {
        /// Template to use for initialization
        #[clap(long, short)]
        template: Option<String>,

        #[clap(long, action = clap::ArgAction::SetTrue)]
        url: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = manager::Config::new()?;
    let sync = sync::Sync::new(config.project.clone());

    // TODO: コマンド処理を個別のハンドラー関数に分離する
    // TODO: プラグイン追加後の自動同期機能を実装する
    match cli.subcommand {
        SubCommands::Add { url } => {
            // configだけ書いて、自動でSyncさせるようにしたい。
            config.append_plugin(url.clone())?;
        }
        SubCommands::Remove => {
            // TODO: Remove機能を実装する
            println!("Removing plugin")
        }
        SubCommands::Sync => {
            // TODO: plugin削除
            // TODO: npm workspaceに追加とinstall

            sync.sync()?;
        }
        SubCommands::Init { template, url } => {
            if let Some(template) = template {
                match template.as_str() {
                    "blank" => {
                        template::blank(&"./".to_string())?;
                    }
                    "basic" => {
                        template::basic(&"./".to_string())?;
                    }
                    _ => {
                        match url {
                            true => {
                                template::from_url(&template, &"./".to_string())?;
                                println!("Initialized from {}!", &template);
                                // URLからテンプレートを取得して初期化
                            }
                            _ => {
                                panic!("ERROR: An undefined template has been entered.");
                            }
                        }
                    }
                }
            } else {
                template::basic(&"./".to_string())?;
            }

            println!("Initialized!")
        }
    }
    Ok(())
}
