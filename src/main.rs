use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Targets {
        #[arg(long, require_equals=true)]
        output: String
    },

    #[command(arg_required_else_help = true)]
    Campaign {
        #[arg(long, require_equals=true)]
        input: String,
        #[arg(long, require_equals=true)]
        output: String
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Targets { output } => {
             println!("Output targets to {}", output);
        },
        Commands::Campaign { input, output } => {
            println!("Importing campaign {} to {}", input, output);
        }
    }

    println!("Hello, world!");
}

// https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html
// https://github.com/Finomnis/rust-tokio-template/blob/main/template/src/main.rs


/*
Next steps
0. Hent launch config fra job
1. Opdel hver kommando i moduler i koden
2. Use tokio - web og async
3. Postgresql - diesel 
 */