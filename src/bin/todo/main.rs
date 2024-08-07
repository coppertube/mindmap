use clap::{Parser, Subcommand};

mod completion;
mod create;
mod delete;
mod list;
mod show;
mod update;

#[derive(Parser)]
#[command(name = "todo")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a task
    Create(create::Args),
    /// List tasks (today's due tasks by default)
    List(list::Args),
    /// Show details of a specific task
    Show(show::Args),
    /// Update a task
    Update(update::Args),
    /// Delete a task
    Delete(delete::Args),
    /// Generate shell completion scripts
    Completion(completion::Args),
}

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Create(args) => create::command(args),
        Commands::List(args) => list::command(args),
        Commands::Show(args) => show::command(args),
        Commands::Update(args) => update::command(args),
        Commands::Delete(args) => delete::command(args),
        Commands::Completion(args) => completion::command(args),
    }
}
