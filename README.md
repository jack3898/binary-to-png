# Binary to Image

I got bored, and thought this would be a cool visual project!

This program reads and writes binary data into a PNG file. This allows you to store any file you like as a visual PNG and then convert that PNG back to the file.

## Example

| Input                                                                                                                             | Output                                                                                                                            |
| --------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| <img src="https://github.com/user-attachments/assets/7703da84-9e80-4fb4-a56c-a1ffadd43385" style="min-width: 300px" height="300"> | <img src="https://github.com/user-attachments/assets/e6ac528d-059f-4a75-96ad-80bc1b35f1eb" style="min-width: 300px" height="300"> |

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
