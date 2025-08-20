extern crate clap;
extern crate regex;
extern crate serde;

use clap::{Parser, Subcommand};
mod manager;
mod sync;

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
    let config = manager::Config::new()?;
    let sync = sync::Sync::new(&config.project);

    match cli.subcommand {
        SubCommands::Add { url } => {
            // configだけ書いて、自動でSyncさせるようにしたい。
            config.append_plugin(url.clone())?;
        }
        SubCommands::Remove => println!("Removing plugin"),
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
