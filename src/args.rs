use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub out: PathBuf,

    #[arg(short, long)]
    pub width: Option<u32>,

    #[arg(short, long)]
    pub height: Option<u32>,
}
