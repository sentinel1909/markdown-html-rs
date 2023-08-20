/// main.rs
/// A command line program which takes a markdown file as input, converts to HTML, and outputs the HTML file

/// dependencies
use clap::Parser;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::string::FromUtf8Error;
use thiserror::Error;

/// enum to represent error types
#[derive(Error, Debug)]
enum ConversionError {
    #[error("Unable to read the file contents: {0}")]
    FilereadError(std::io::Error),
    #[error("Unable to convert file contents to markdown: {0}")]
    MarkdownError(FromUtf8Error),
    #[error("Unable to write the html file: {0}")]
    HTMLWriteError(std::io::Error),
}

/// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

/// struct to represent the front matter of the markdown document
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    date: String,
    tags: Vec<String>,
}

fn main() -> Result<(), ConversionError> {
    // get the file name from the command line input
    let args = Args::parse();

    // read the file contents and save it as a vector of u8
    // convert the file contents into a markdown string
    let file_contents = fs::read(args.filename).map_err(ConversionError::FilereadError)?;
    let markdown_input =
        String::from_utf8(file_contents).map_err(ConversionError::MarkdownError)?;

    // parse the front matter in the input string and deserialize it into a FrontMatter struct
    // remove the front matter, leaving on the body content of the markdown file
    let matter = Matter::<YAML>::new();
    let result = matter.parse(&markdown_input);
    let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
    let mut stdout = io::stdout();
    writeln!(stdout, "{:?}", front_matter).expect("Could not write to stdout...");
    let frontmatter_regex = Regex::new(r"---\s*\n(?s:.+?)\n---\s*\n").unwrap();
    let markdown_body = frontmatter_regex.replace(&markdown_input, "");

    // parse the markdown body and convert it to html, any html tags in the markdown file are passed through
    let parser = pulldown_cmark::Parser::new(&markdown_body);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // write the html output file
    fs::write("output.html", html_output).map_err(ConversionError::HTMLWriteError)?;
    let mut stdout = io::stdout();
    writeln!(stdout, "Markdown converted and saved to output.html")
        .expect("Could not write to stdout...");

    Ok(())
}
