use core::panic;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub description: String
}

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

    Page {
        frontmatter,
        content: markdown_content.to_string(),
        generated: String::new(),
        destination: file_path.to_string(),
    }
}