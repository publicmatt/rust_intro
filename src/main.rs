mod bits;
mod boxit;
mod compleat;
mod restarant;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Calc,
    Boxed,
    Restaurant,
    Compleat,
}

fn main() {
    let cli = Cli::parse();
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
    match cli.command {
        Some(command) => match command {
            Commands::Calc => bits::cli::main(),
            Commands::Boxed => boxit::cli::main(),
            Commands::Restaurant => restarant::cli::main(),
            Commands::Compleat => compleat::cli::main(),
        },
        None => println!("unrecognized command"),
    }
}
