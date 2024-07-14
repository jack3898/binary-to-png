mod args;
mod binary;
mod image;

use std::fs;

use ::image::io::Reader;
use args::*;
use binary::{bits_to_bytes, bytes_to_bits};
use clap::Parser;
use image::{bits_to_image_buf, bits_to_image_dimensions, image_to_bits};

fn main() {
    let args = Args::parse();

    match args.reverse {
        true => {
            let png = Reader::open(args.input).unwrap().decode().unwrap();
            let bits = image_to_bits(png);
            let bytes = bits_to_bytes(bits);

            fs::write(args.output, bytes).unwrap();
        }
        false => {
            let file = fs::read(&args.input).unwrap_or(args.input.into_bytes());
            let bits = bytes_to_bits(file);
            let (width, height) = bits_to_image_dimensions(bits.len(), args.width, args.height);
            let image_buf = bits_to_image_buf(width, height, bits);

            image_buf
                .save_with_format(args.output, ::image::ImageFormat::Png)
                .unwrap();
        }
    }
}
