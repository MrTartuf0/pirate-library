use std::{collections::HashMap, io};

use crate::structs::HttpReq;

pub fn parse_req_buffer(Buf: Vec<String>) -> String{
    let mut buffer = String::new();
    for line in Buf {
        buffer.push_str(&line.replace("%20", " "));
    }

    let mut path = match buffer.split("GET").collect::<Vec<&str>>().get(1) {
        Some(p) => p,
        None => "",
    };
    path = match path.split("HTTP/1.1").collect::<Vec<&str>>().get(0) {
        Some(p) => p,
        None => path,
    };
    path = path.trim();
    path.to_string()
}



pub fn parse_request(lines: Vec<String>) -> Result<HttpReq, io::Error> {

    if lines.len() < 1{
        return Err(io::Error::new(io::ErrorKind::InvalidData, "InvalidData"));
   }
    let first_line = lines.first().unwrap().clone();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let path = parts.next().unwrap_or_default().to_string().replace("%20", " ");

    let mut headers = HashMap::new();
    for line in lines.iter().skip(1) {
        let mut parts = line.splitn(2, ':');
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    let http_req = HttpReq {
        method: method,
        path: path,
        headers: headers,
    };

    Ok(http_req)
}

pub fn parse_content_type(path: &str) -> String {
    let mut content_type = String::from("");
    let mut path = path.split(".").collect::<Vec<&str>>();
    let extension = path.pop().unwrap();

    match extension {
        "html" => content_type = String::from("text/html"),
        "css" => content_type = String::from("text/css"),
        "js" => content_type = String::from("text/javascript"),
        "jpg" => content_type = String::from("image/jpeg"),
        "png" => content_type = String::from("image/png"),
        "gif" => content_type = String::from("image/gif"),
        "ico" => content_type = String::from("image/x-icon"),
        "zip" => content_type = String::from("application/zip"),
        "pdf" => content_type = String::from("application/pdf"),
        "svg" => content_type = String::from("image/svg+xml"),
        "json" => content_type = String::from("application/json"),
        "mp4" => content_type = String::from("video/mp4"),
        "mp3" => content_type = String::from("audio/mpeg"),
        _ => content_type = String::from("text/html"),
    }

    content_type
}