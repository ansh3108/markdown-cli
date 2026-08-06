use std::env;
use std::println;
use std::process;
mod scanner;
mod parser;

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

    let md_files = scanner::get_markdown_files(input_dir);
    
    for file in md_files {
        let page = parser::parse_file(&file);
        
        let html_document = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    <header>
        <h1>My Rust SSG</h1>
    </header>
    <main>
        {}
    </main>
</body>
</html>"#, 
            page.frontmatter.title, 
            page.generated         
        );

        println!("Generated HTML for: {}", page.frontmatter.title);

        let output_path = file
            .replacen(input_dir, output_dir, 1)
            .replace(".md", ".html");

        if let Some(parent_dir) = std::path::Path::new(&output_path).parent() {
            std::fs::create_dir_all(parent_dir).expect("Failed to create input directories");
        }

        std::fs::write(&output_path, html_document).expect("Failed to write HTML file");
        
        println!("Successfully built: {}", output_path)

    }
    
}