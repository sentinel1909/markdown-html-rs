// main.rs
// A command line program which takes a markdown file as input, converts to HTML, and outputs the HTML file

// dependencies
use color_eyre::Result;
use clap::Parser;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    output: String,
}

// struct to represent the front matter of the markdown document
#[derive(Debug, Deserialize, Serialize)]
struct FrontMatter {
    title: String,
    date: String,
    categories: Vec<String>,
    tags: Vec<String>,
}

impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            title: "".to_string(),
            date: "".to_string(),
            categories: Vec::new(),
            tags: Vec::new(),
        }
    }
}

fn main() -> Result<()> {
    // initalize error handling
    color_eyre::install()?;

    // create an output buffer
    let mut stdout = io::stdout();

    // get the file name from the command line input
    let args = Args::parse();

    // open the file provided by the user and convert the frontmatter and markdown into a string
    let input_file = format!("content/{}", args.file);
    let input_file_path = PathBuf::from(input_file);
    let mut file = File::open(input_file_path)?;
    let mut markdown_input = String::new();
    file.read_to_string(&mut markdown_input)?;
    println!("Markdown file read successfully.");

    // parse the front matter in the input string and deserialize it into a FrontMatter struct
    // remove the front matter, leaving on the body content of the markdown file
    let matter = Matter::<YAML>::new().parse(&markdown_input);
    let front_matter: FrontMatter = matter
        .data
        .as_ref()
        .map(|data| data.deserialize())
        .transpose()?
        .unwrap_or_default();
    println!("Front matter extracted successfully.");

    // write the frontmatter to a file
    let json_output = to_string_pretty(&front_matter)?;
    let front_matter_output_path = "public/frontmatter/front_matter_output.json";
    let mut front_matter_output = File::create(front_matter_output_path)?;
    front_matter_output.write_all(json_output.as_bytes())?;
    writeln!(stdout, "Frontmatter converted to JSON and saved to public/frontmatter/front_matter_output.json")?;

    let frontmatter_regex =
        Regex::new(r"---\s*\n(?s:.+?)\n---\s*\n")?;
    let markdown_body = frontmatter_regex.replace(&markdown_input, "");

    // parse the markdown body and convert it to html, any html tags in the markdown file are passed through
    let parser = pulldown_cmark::Parser::new(&markdown_body);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // write the html output file
    let output_file = format!("public/{}", args.output);
    let output_file_path = PathBuf::from(output_file);
    let mut html = File::create(output_file_path)?;
    html.write_all(html_output.as_bytes())?;
    writeln!(stdout, "Markdown converted and saved to public/output.html")?;

    Ok(())
}
