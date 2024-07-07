use clap::{CommandFactory, Parser};
use clap_complete::generate;

mod commands;
use commands::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            eprintln!("Generating completions for {shell}");
            generate(shell, &mut cmd, name, &mut std::io::stdout());
            std::process::exit(0);
        }
        _ => {
            println!("Hello, world!");
        }
    }
}
