use clap::Parser;
use stunnel::arg::{Arg, Commands};

fn main() {
    let args = Arg::parse();

    match args.command {
        Commands::Status => status(),
        Commands::Add { ssh_command } => add(&ssh_command),
        Commands::Stop { name } => stop(&name),
        Commands::Start { name } => start(&name),
    }
}

fn status() {
    println!("status should show here!");
}
fn add(ssh_command: &str) {
    println!("{}", ssh_command);
}
fn stop(name: &str) {
    println!("{}", name);
}
fn start(name: &str) {
    println!("{}", name);
}
