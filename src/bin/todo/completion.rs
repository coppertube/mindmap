use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

#[derive(Parser)]
pub struct Args {
    #[clap(value_enum, help = "The shell to generate the completion script for")]
    pub shell: Shell,
}

pub fn command(args: &Args) {
    let mut command = crate::Args::command();
    let binary_name = command.get_name().to_string();
    generate(args.shell, &mut command, binary_name, &mut io::stdout());
}
