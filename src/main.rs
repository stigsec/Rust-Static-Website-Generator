use clap::{Arg, Command};
use pulldown_cmark::{Parser, Options, html};
use tera::{Tera, Context};
use std::{fs, io};

fn main() -> io::Result<()> {
    let matches = Command::new("rswg")
        .version("0.1.0")
        .author("stigs")
        .about("Rust Static Website Generator\nTurn your markdown (.md) files into html!")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Input markdown file"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Output HTML file"))
        .arg(Arg::new("background-color")
            .long("bg-c")
            .value_parser(clap::value_parser!(String))
            .default_value("white")
            .help("Background color"))
        .arg(Arg::new("text-color")
            .long("txt-c")
            .value_parser(clap::value_parser!(String))
            .default_value("black")
            .help("Text color"))
        .arg(Arg::new("template")
            .short('t')
            .long("template")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Path to the custom HTML template, eg. /home/user/rswg-templates/basic.html"))
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let background_color = matches.get_one::<String>("background-color").unwrap();
    let text_color = matches.get_one::<String>("text-color").unwrap();
    let template_path = matches.get_one::<String>("template").unwrap();

    let markdown = fs::read_to_string(input_path)?;

    let options = Options::all();
    let parser = Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let mut tera = Tera::default();
    tera.add_template_file(template_path, Some("index.html")).unwrap();

    let mut context = Context::new();
    context.insert("content", &html_output);
    context.insert("background_color", &background_color);
    context.insert("text_color", &text_color);
    context.insert("title", input_path);

    let rendered = tera.render("index.html", &context).unwrap();

    fs::write(output_path, rendered)?;

    println!("Output generated at {}", output_path);

    Ok(())
}
