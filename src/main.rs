extern crate clap;
extern crate regex;
extern crate serde;

use clap::{Parser, Subcommand};
mod manager;
mod sync;

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

    Sync,

    Init,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = manager::Config::new()?;
    let sync = sync::Sync::new(&config.project);

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
        SubCommands::Init => {
            println!("Initialized!")
        }
    }
    Ok(())
}
