use clap::{
    command,
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Setup the weather API
    Configure(ConfigureCommand)
}

#[derive(Debug, Args)]
pub struct ConfigureCommand {
    pub name: String
} 