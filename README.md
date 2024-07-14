# Binary to Image

I got bored, and thought this would be a cool visual project!

This program reads and writes binary data into a PNG file. This allows you to store any file you like as a visual PNG and then convert that PNG back to the file.

The output file size is almost identical to the input binary.

## Example

| Input                                                                                                                             | Output                                                                                                                            |
| --------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| <img src="https://github.com/user-attachments/assets/7703da84-9e80-4fb4-a56c-a1ffadd43385" style="min-width: 300px" height="300"> | <img src="https://github.com/user-attachments/assets/74f4d666-5390-4e09-bb32-5d859e0be5ca" style="min-width: 300px" height="300"> |

## Usage

With a file:
`./binary-to-image.exe --input /path/to/your/binary --output /path/to/output.png`

With a message:
`./binary-to-image.exe --input "a custom message here" --output /path/to/output.png`

## Flags

- `--input` | `-i`: Path to input or a textual message
- `--output` | `-o`: Path to the destination where the generated file will go
- `--width` | `-w`: If you're generating the binary PNG, force a width
- `--height` | `-h`: If you're generating the binary PNG, force a height
- `--reverse` | `-r`: Go into reverse mode, and convert the PNG back to a file
- `--bitmode` | `-b`: Use bitmode, each pixel represents a bit (black/white) instead of a byte (grayscale)
  - This is not recommended because the image crate does not provide an option to store the PNG with a bit depth of 1 and it's slower
  - You can use an online tool like squoosh (https://squoosh.app) to convert the PNG to a bit depth of 1
- `--help`: Pull up the help menu
