use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// The input path or custom message
    #[arg(short, long)]
    pub input: String,

    /// The output path
    #[arg(short, long)]
    pub output: String,

    /// Override the generated pixel width of the image
    #[arg(short, long)]
    pub width: Option<u32>,

    /// Override the generated pixel height of the image
    #[arg(short, long)]
    pub height: Option<u32>,

    /// Reverse the process; convert the binary representation PNG back into its original file
    #[arg(short, long, default_value = "false")]
    pub reverse: bool,

    /// Each pixel will store one byte of data instead of one bit.
    /// This means each pixel will be black or white instead of grayscale
    #[arg(short, long, default_value = "false")]
    pub bitmode: bool,
}
