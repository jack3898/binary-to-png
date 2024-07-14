mod args;
mod binary;
mod image;

use std::{fs, io::Read};

use ::image::io::Reader;
use args::*;
use binary::{bits_to_bytes, bytes_to_bits};
use clap::Parser;
use image::{bits_to_image_buf, bits_to_image_dimensions, bytes_to_image_buf, image_to_bits};

fn main() {
    let args = Args::parse();

    match args.reverse {
        true => decode(args.input, args.output, args.bitmode),
        false => encode(
            args.input,
            args.output,
            args.bitmode,
            args.width,
            args.height,
        ),
    }
}

fn decode(input: String, output: String, bitmode: bool) {
    let png = Reader::open(input).unwrap().decode().unwrap().to_luma8();

    let bytes: Vec<u8> = if bitmode {
        let bits = image_to_bits(png);

        bits_to_bytes(bits)
    } else {
        let bytes_result: Result<Vec<_>, _> = png.bytes().collect();

        bytes_result.unwrap()
    };

    fs::write(output, bytes).unwrap();
}

fn encode(input: String, output: String, bitmode: bool, width: Option<u32>, height: Option<u32>) {
    let file_bytes = fs::read(&input).unwrap_or(input.into_bytes());

    let image_buf = if bitmode {
        let file_bits = bytes_to_bits(file_bytes);
        let (width, height) = bits_to_image_dimensions(file_bits.len(), width, height);

        bits_to_image_buf(width, height, file_bits)
    } else {
        let (width, height) = bits_to_image_dimensions(file_bytes.len(), width, height);

        bytes_to_image_buf(width, height, file_bytes)
    };

    image_buf
        .save_with_format(output, ::image::ImageFormat::Png)
        .unwrap();
}
