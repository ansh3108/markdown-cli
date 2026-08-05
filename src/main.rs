use std::env;
use std::println;
use std::process;
use serde::Deserialize;
mod scanner;

struct Frontmatter {
    title: String,
    date: String,
    description: String
}

struct Page {
    content: String,
    generated: String,
    destination: String
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: my_ssg <input_dir> <output_dir>");
        process::exit(1);
    }

    let input_dir = &args[1];
    let output_dir = &args[2];

    println!("Input directory: {}", input_dir);
    println!("Output directory: {}", output_dir);

    let md_files = scanner::get_markdown_files(input_dir);
    println!("Found files: {:?}", md_files);
}