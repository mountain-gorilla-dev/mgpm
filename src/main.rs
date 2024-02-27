use clap::{Parser, Subcommand};
mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// パッケージをインストールします
    Install { name: Option<String> },
    /// パッケージを削除します
    Remove { name: Option<String> },
    /// インストールさせたパッケージ一覧を表示します
    List {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Install { name } => commands::install::install(name),
        Commands::Remove { name } => commands::remove::remove(name),
        Commands::List {} => commands::list::list(),
    }
}
