# Rust Static Site Generator (SSG)

Lightweight and fast CLI tool written in Rust that recursively scans a directory of Markdown files, extractcs YAML frontmatter, converts the Markdown to HTML, and generate a fully linked static website while preserving your folder structure.

## Features

* **Recursive Directory Traversal**: Scans deeply nested folders to find every `.md` file.
* **Frontmatter Parsing**: Extracts YAML metadata (Title, Date, Description) from the top of Markdown files.
* **Markdown to HTML**: Uses `pulldown-cmark` for highly compliant, fast Markdown parsing.
* **Structure Mirroring**: Automatically replicates your input directory's folder tree inside the output directory.
* **HTML Templating**: Wraps the generated HTML inside a standard web document structure.

## Prerequisites

You will need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine to run this project.


## Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/ansh3108/markdown-cli.git
cd markdown-cli
cargo build --release
```

## Usage

The CLI requires two arguments: the `input` directory containing your Markdown files, and the `output` directory where you want the HTML output to be saved.

```bash
#Run via cargo:
cargo run -- ./content ./public

# Or run the compiled binary directly:
./target/release/my_ssg ./content ./public
```

## Markdown File Format

For the parser to work correctly, every Markdown file must include YAML frontmatter enclosed in `---` at the very top of the file.

The frontmatter expects three fields: `title`,`date`, and `description`.

Example `content/hello-world.md`:

```bash
---
title: My First Rust Post
date: "2026-08-06"
description: Learning how to build a Static Site Generator
---

# Hello from Rust!

This is a paragraph of **Markdown** text. 

* Here is a list item
* Here is another one
```


## Directory Mirrorring

The tool will respect your folder hirarchy. If you structure input like this:

```bash
content/
├── index.md
└── blog/
    └── my-post.md
```

The output will automatically generate the corresponding folders:

```bash
public/
├── index.html
└── blog/
    └── my-post.html
```

## Dependencies

- `serde`&`serde_yaml`: For deserializing YAML frontmatter into Rust structs.
- `pulldown-cmark`: A fast, CommonMark-compliant Markdown parser.


<img width="643" height="193" alt="image" src="https://github.com/user-attachments/assets/0e8e557f-0c83-4fed-86b1-247cd9c6e74e" />
