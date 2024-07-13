mod args;
mod binary;
mod image;

use std::fs;

use args::*;
use binary::bytes_to_bits;
use clap::Parser;
use image::{bits_to_image_buf, bits_to_image_dimensions};

fn main() {
    let args = Args::parse();
    let file = fs::read(args.input.clone()).unwrap_or(args.input.clone().into_bytes());
    let bits = bytes_to_bits(file);
    let (width, height) = bits_to_image_dimensions(bits.len(), args.width, args.height);

    let image_buf = bits_to_image_buf(width, height, bits);

    image_buf
        .save_with_format(args.out, ::image::ImageFormat::Png)
        .unwrap();
}
