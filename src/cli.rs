use clap::{Parser};

#[derive(Parser)]

pub struct Cli {
    pub message: Option<String>,

    #[arg(long)]
    pub session: Option<usize>,
}
