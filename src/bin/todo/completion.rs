use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

#[derive(Parser)]
pub struct Args {
    #[clap(value_enum, help = "The shell to generate completions script for")]
    pub shell: Shell,
}

pub fn command(args: &Args) {
    let mut cmd = crate::Args::command();
    let bin_name = "todo";
    generate(args.shell, &mut cmd, bin_name, &mut io::stdout());
}
