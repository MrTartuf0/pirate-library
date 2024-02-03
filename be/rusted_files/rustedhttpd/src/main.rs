use std::collections::HashMap;
use std::io::{BufRead, Error};
use std::{io, thread};
use std::fs::{self, File};
use std::sync::{Arc, mpsc};
use tokio::sync::{Mutex, Semaphore};
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::Write;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};
mod config;
mod parsing;
mod structs;
mod files;
mod loggers_other_misc;

use structs::*;
use files::*;


pub fn write_data(mut stream: &TcpStream, data: structs::Response) -> io::Result<()> {
    stream.write_all(&data.as_bytes());
    stream.flush();  
    Ok(())
}



async fn handle_client(mut stream: TcpStream, dirs: &mut fileDataTtl, map: &mut HashMap<String, Vec<u8>>) -> Result<String, io::Error> {
    loop {
        let now = loggers_other_misc::get_epoch_now();
        if now > (dirs.timestamp + dirs.ttl) {
            dirs.ttl = dirs.ttl;
            dirs.files = look_for_dirs_and_subdirs();
            dirs.timestamp = now;
            *map = HashMap::new();
        }

        let mut buffer = Vec::new();
        let http_ver = "HTTP/1.1 ";

        let reader = BufReader::new(&mut stream);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            buffer.push(line);
        }

        let mut buffer_page: Vec<u8> = Vec::new();
        let mut http_req_struct: HttpReq;

        match parsing::parse_request(buffer) {
            Ok(buf) => {
                http_req_struct = buf;
            },
            Err(e) => {
                return Err(e);
            },
        }

        let file = FileData::get_by_http_subdir(http_req_struct.path.clone(), dirs.get_file().clone());
        let (status_code, mut buffer_page, file_type) = match file {
            Some(buf) => {
                if map.contains_key(&buf.get_path()) {
                    buffer_page = map.get(&buf.get_path()).unwrap().to_vec();
                } else {
                    buffer_page = open_file_by_path(buf.clone(), dirs.get_file().to_vec());
                    map.insert(buf.get_path(), buffer_page.clone());
                }
                (200, buffer_page, buf.get_content_type())
            }
            None => {
                buffer_page = b"<h1>404 NOT FOUND</h1>".to_vec();
                (404, buffer_page, "text/html".to_string())
            }
        };

        let mut res_headers = format!(
            "Content-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\nServer: RustedHttpd\r\n",
            buffer_page.len(),
            file_type
        );

        for (key, value) in http_req_struct.clone().headers {
            res_headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        let response = format!(
            "{}{} {}\r\n{}\r\n",
            http_ver,
            status_code,
            match status_code {
                200 => "OK",
                404 => "Not Found",
                _ => "Internal Server Error", // Add more cases as needed
            },
            res_headers
        );

        if let Err(e) = stream.write_all(response.as_bytes()) {
//            eprintln!("Error writing response: {}", e);
            return Err(e);
        }

        if let Err(e) = stream.write_all(&buffer_page) {
//            eprintln!("Error writing response body: {}", e);
            return Err(e);
        }

        if let Err(e) = stream.flush() {
//            eprintln!("Error flushing response: {}", e);
            return Err(e);
        }

        if !should_keep_alive(http_req_struct.headers.get("Connection")) {
            break;
        }
    }

    Ok("Ok".to_string())
}

fn should_keep_alive(http_req: std::option::Option<&std::string::String>) -> bool {
    if let Some(connection_header) = http_req {
        connection_header == "Keep-Alive: timeout=5, max=1000"
    } else {
        false
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut senders: Vec<Sender<TcpStream>> = vec![];
    let mut NUM_THREADS: usize;
    let parsed = config::parse_config();
    NUM_THREADS = parsed.server.threads;

    for i in 0..NUM_THREADS {
        let (sender, receiver): (Sender<TcpStream>, _) = unbounded();
        senders.push(sender.clone());
        let receiver = receiver.clone();
        tokio::spawn(process_message(receiver, parsed.clone(), i.try_into().unwrap()));
    }

    let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", parsed.server.port))?;

    for (i, stream) in listener.incoming().enumerate() {
        let stream = stream?;
        senders[i % NUM_THREADS].send(stream).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    Ok(())
}




async fn process_message(receiver: Receiver<TcpStream>, config: config::ServerConfig, idx: i32) {
    let mut dirs: fileDataTtl = fileDataTtl {
        timestamp:  loggers_other_misc::get_epoch_now(),
        files: look_for_dirs_and_subdirs(),
        ttl: config.server.ttl,
    };

    println!("Worker thread {} started and ready to accept connections", idx);
    let mut hashMap: HashMap<String, Vec<u8>> = HashMap::new();
    while let Ok(stream) = receiver.recv() {
        if let Err(e) = handle_client(stream, &mut dirs, &mut hashMap).await {
        }
    }
}


