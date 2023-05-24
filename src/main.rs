
mod cli;

use cli::args::CLI;
use clap::Parser;


fn main() {
    
    let args = CLI::parse();
    println!("ARGS: {:?}", args);
}
