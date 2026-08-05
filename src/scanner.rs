use std::fs;

pub fn get_markdown_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                if let Some(dir_str) = path.to_str() {
                    let mut sub_folder_files = get_markdown_files(dir_str);

                    files.append(&mut sub_folder_files);
                }
            } else if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" {
                        if let Some(path_str) = path.to_str() {
                            files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }
    files
}