use std::fs::{self, File};
use std::io::Read;

use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::collections::VecDeque;

use crate::parsing::parse_content_type;
use crate::structs::FileData;

pub fn look_for_files_in_this_dir(path: String) -> Vec<FileData> {
    
    return process_directory(&path,  true);
    
}

pub fn process_if_path_is_dir(path: FileData) -> String {
        let mut buffer = String::from("<h1>Directory</h1>");
        let dirs_and_subdirs = look_for_files_in_this_dir(path.get_path());

        for dir in dirs_and_subdirs {
            buffer.push_str("<ul>");
            buffer.push_str("<li>");
            buffer.push_str("<a href=\"");
            buffer.push_str(&dir.get_http_subdir());
            buffer.push_str("\">");
            buffer.push_str(&dir.get_name());
            buffer.push_str("</a>");
            buffer.push_str("</li>");
            buffer.push_str("</ul>");
        }

        buffer.push_str("</ul>");
        buffer
}


pub fn open_file_by_path(path: FileData, files: Vec<FileData> ) -> Vec<u8> {
    let mut fileVec = vec![];
    fileVec = files.clone();

    if path.is_dir {
        for file in files {
            if file.get_name() == "index.html" {

                return open_file_by_path(file, fileVec);
            }
        }

        return process_if_path_is_dir(path).as_bytes().to_vec();
    }

    let mut file = fs::File::open(&path.path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}

pub fn look_for_dirs_and_subdirs() -> Vec<FileData> {
        let mut files = Vec::new();
    let root_path = "./web";

    files.push(FileData {
        path: root_path.to_string(),
        name: String::from("web"),
        http_subdir: String::from("/"),
        content_type: String::from("text/html"),
        is_dir: true,
        
    });

    for file in process_directory(root_path,  false).iter() {
        files.push(file.clone());
    }

    files
}








fn process_directory(directory_path: &str, no_recursive: bool) -> Vec<FileData> {
    let mut files = Vec::new();
    let mut stack = VecDeque::new();

    stack.push_back(directory_path.to_string());

    while let Some(current_path) = stack.pop_back() {
        if let Ok(paths) = fs::read_dir(&current_path) {
            for path in paths {
                if let Ok(entry) = path {
                    let entry_path = entry.path();
                    let entry_name = entry.file_name().to_string_lossy().replace("'", "\'").to_string();
                    let mut http_subdir = entry_path.to_str().unwrap().to_string();
                    http_subdir.replace_range((0..5), "");
                    let content_type = parse_content_type(&entry_path.to_str().unwrap());

                    if entry_path.is_dir() {
                        files.push(FileData {
                            path: entry_path.to_str().unwrap().to_string(),
                            name: entry_name.clone(),
                            http_subdir: http_subdir.to_string(),
                            content_type: content_type.clone(),
                            is_dir: true,
                        });
                        if !no_recursive {
                            stack.push_back(entry_path.to_str().unwrap().to_string());
                        }
                    } else {
                        files.push(FileData {
                            path: entry_path.to_str().unwrap().to_string(),
                            name: entry_name.clone(),
                            http_subdir: http_subdir.to_string(),
                            content_type: content_type.clone(),
                            is_dir: false,
                        });
                    }
                }
            }
        }
    }

    files
}






