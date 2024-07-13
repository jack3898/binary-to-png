use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub path: PathBuf,

    #[arg(short, long)]
    pub out: PathBuf,
}
