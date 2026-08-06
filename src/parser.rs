use core::panic;
use serde::Deserialize;
use pulldown_cmark::{Parser, html};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub description: String
}

#[allow(dead_code)]
pub struct Page {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub generated: String,
    pub destination: String
}

pub fn parse_file(file_path: &str) -> Page {
    let contents = std::fs::read_to_string(file_path)
        .expect("Failed to read file");

    let parts: Vec<&str> = contents.split("---").collect();

    if parts.len() < 3 {
        panic!("Invalid file format in {}. Expected frontmatter.", file_path);
    }

    let yaml_str = parts[1].trim();
    let markdown_content = parts[2].trim();
    
    let frontmatter: Frontmatter = serde_yaml::from_str(yaml_str).expect("Failed to parse YAML");

    let parser = Parser::new(markdown_content);
    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    Page {
        frontmatter,
        content: markdown_content.to_string(),
        generated: html_output,
        destination: file_path.to_string(),
    }
}