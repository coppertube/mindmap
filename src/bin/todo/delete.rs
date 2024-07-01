use clap::Parser;
use mindmap::db::get_client;

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    description: String,
}

pub async fn command(args: &Args) {

    let description = args.description.clone();

    let client = get_client().await.expect("Failed to fetch client");
    client.execute("DELETE FROM todo WHERE description = $1", &[&description],).await.expect("Failed to delete task");

    println!("Task \"{}\" deleted successfully!", description);
}
