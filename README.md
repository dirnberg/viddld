
# viddld

`viddld` is a Rust-based command-line tool for extracting and downloading Vimeo videos embedded in HTML files. Simple, fast, and efficient.

## Features

- Extracts video URLs from HTML files.
- Downloads videos from Vimeo in various resolutions.
- Lightweight and easy to use.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/viddld.git
   cd viddld
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the tool**:
   ```bash
   ./target/release/viddld path/to/your/file.html output_video.mp4
   ```

## Usage

To download a video, provide the HTML file containing the embedded Vimeo video and specify the output filename:

```bash
viddld example.html my_video.mp4
```

## Requirements

- Rust (for building the project)
- Internet connection (for downloading videos)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
