use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use std::io;

#[derive(Parser)]
pub struct Args {
    #[clap(value_enum, help = "The shell to generate completions script for")]
    pub shell: Shell,
}

pub fn command(args: &Args) {
    let mut app = Args::command();
    generate(args.shell, &mut app, "todo", &mut io::stdout());
}
