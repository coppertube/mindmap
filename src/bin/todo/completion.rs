use std::io;

use clap::CommandFactory;
use clap_complete::{generate, Shell};

#[derive(clap::Args)]
pub struct Args {
    #[clap(value_enum, help = "The shell to generate completions script for")]
    pub shell: Shell,
}

pub fn command(args: &Args) {
    let mut cmd = crate::Args::command();
    let bin_name = "todo";
    generate(args.shell, &mut cmd, bin_name, &mut io::stdout());
}
