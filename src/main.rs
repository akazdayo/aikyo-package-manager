use clap::{Parser, Subcommand};
mod add;
mod manager;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = manager::Config::new()?;

    match cli.subcommand {
        SubCommands::Add { url } => {
            add::clone_from_git(&url, &config.project.tools_dir)?;
            config.append_plugin(url.clone())?;
        }
        SubCommands::Remove => println!("Removing plugin"),
    }
    Ok(())
}
