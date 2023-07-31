/// main.rs
/// A command line program which takes a markdown file as input, converts to HTML, and outputs the HTML file

/// dependencies
use clap::Parser;
use std::fs;
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
    HTMLWriteError(std::io::Error)
}

/// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() -> Result<(), ConversionError> {
    
    // get the file name from the command line input
    let args = Args::parse();

    // read the file contents and save it as a vector of u8
    // convert the file contents into a markdown string
    let file_contents = fs::read(args.filename).map_err(ConversionError::FilereadError)?;
    let markdown_input = String::from_utf8(file_contents).map_err(ConversionError::MarkdownError)?;

    // parse the markdown and convert it to html
    let parser = pulldown_cmark::Parser::new(markdown_input.as_str());
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // write the html output file
    fs::write("output.html", html_output).map_err(ConversionError::HTMLWriteError)?;

    Ok(())
}
