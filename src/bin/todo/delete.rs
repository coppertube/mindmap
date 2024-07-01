use clap::Parser;

#[derive(Parser)]
pub struct Args {}

pub fn command(_args: &Args) {
    println!("Deleted task successfully!")
}
