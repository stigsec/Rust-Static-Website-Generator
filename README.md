
# Rust Static Website Generator (RSWG)

A simple Rust-based static website generator that converts Markdown files into HTML. This tool allows you to customize the appearance of your HTML output with customizable background and text colors. It also supports the use of custom HTML templates.

## Features

- Converts Markdown (`.md`) files to HTML
- Customizable background and text colors
- Supports custom HTML templates for the output
- Built with Rust using `clap`, `pulldown_cmark`, and `tera` for templating

## Installation

### Build and Run

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/stigsec/Rust-Static-Website-Generator.git
   cd Rust-Static-Website-Generator
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

   This will compile the project and place the binary in the `target/release` directory.

3. You can now run the tool using the following command:

   ```bash
   ./target/release/rswg -i input.md -o output.html -bg-c=black -txt-c=white -t templates/basic.html
   ```

   Replace `input.md` with the path to your Markdown file, `output.html` with the desired output HTML file name, and specify the template you want to use (e.g., `templates/basic.html`).

## Usage

```bash
Usage: rswg [OPTIONS] --input <input> --output <output> --template <template>

Options:
  -i, --input <input>            Input markdown file
  -o, --output <output>          Output HTML file
  --bg-c <background-color>      Background color [default: white]
  --txt-c <text-colorR>           Text color [default: black]
  -t, --template <template>      Path to the custom HTML template, eg. /home/user/rswg-tempaltes/basic.html
  -h, --help                     Print help
  -V, --version                  Print version
```

## Example

To convert a Markdown file `README.md` to `index.html` with a black background and white text, using the `basic.html` template, you would run:

```bash
rswg -i README.md -o index.html -bg-c=black -txt-c=white -t templates/basic.html
```

### Available Templates

- **basic.html**: A simple, minimal template.
- **nav-footer.html**: A template with a navigation bar and footer.

Feel free to add your own templates in the `templates/` directory.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE file](LICENSE) for more details.



---

Developed by [stigsec](https://github.com/stigsec).

