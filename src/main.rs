mod args;
mod binary;

use std::fs;

use args::*;
use binary::bytes_to_bits;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let file = fs::read(args.path).unwrap();
    let bytes = bytes_to_bits(&file);

    println!("{bytes:?}")
}
