use clap::{
    command,
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[command(author, version)]
#[command(about = "CLI to check the weather", long_about = "CLI to check the weather in certain place")]

pub struct CLI {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Setup the weather cli
    Configure(ConfigureCommand),
    /// Print out current config
    Print,
    /// Get the current weather for configured place
    Run(RunCommand)
}


#[derive(Debug, Args)]
pub struct RunCommand {
    /// Get the current weather for configured place
    #[arg(short = 'a', long = "api", help = "Select whatever API (Open or Aeris) to call.")]
    pub api: Option<String>
}

#[derive(Debug, Args)]
pub struct ConfigureCommand {
    /// [Open, Aeris] - the two possible option.
    /// 
    /// Open - use OpenWeatherAPI (https://openweathermap.org/)
    /// 
    /// Aeris   - use AerisWeatherAPI (https://www.aerisweather.com/)
    pub name: String,

    #[arg(long = "zip", help = "setup ZIP-Code to search the place. (!!ATTENTION!! - USA zip-codes only)")]
    pub zip: Option<String>,
    
    #[arg(long = "city", help = "setup city to search the place. (!!ATTENTION!! - USA cities only)")]
    pub city: Option<String>
} 