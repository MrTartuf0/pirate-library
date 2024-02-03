use std::collections::HashMap;



pub struct Response<'a> {
    pub http_version: &'a str,
    pub status_code: u32,
    pub status_text: &'a str,
    pub headers: Vec<&'a str>,

    // Deve essere una stream di byte per gestire cose che non siano testo
    pub body: Vec<u8>,
}

impl Response<'_> {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(self.http_version.as_bytes());
        res.extend_from_slice(format!("{} {}\r\n", self.status_code, self.status_text).as_bytes());

        for header in &self.headers {
            res.extend_from_slice(header.as_bytes());
            res.extend_from_slice(b"\r\n");
        }

        res.extend_from_slice(b"\r\n");
        res.extend_from_slice(&self.body);

        res
    }
}


#[derive(Debug, Clone)]
pub struct HttpReq {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FileData {
    pub path: String,
    pub name: String,
    pub http_subdir: String,
    pub content_type: String,
    pub is_dir: bool,
}


pub struct fileDataTtl {
    pub files: Vec<FileData>,
    pub ttl: u32,
    pub timestamp: u32,
}

impl fileDataTtl {
    pub fn get_file(&self) -> &Vec<FileData> {
        &self.files
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, ttl: u32) {
        self.timestamp = ttl;
    }

}

impl FileData {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_http_subdir(&self) -> String {
        self.http_subdir.clone()
    }



    pub fn get_by_http_subdir(mut subdir: String, dirs: Vec<FileData>) -> Option<FileData> {
            if subdir.len() > 1 {
                if subdir.chars().last().unwrap() == '/' {
                    subdir.pop();
                }
            }

        for dir in dirs {
            if dir.get_http_subdir() == subdir {
                return Some(dir);
            }
        }
        None
    }

    pub fn get_content_type(&self) -> String {
        self.content_type.clone()
    }

    pub fn cloneVec(vec: Vec<FileData>) -> Vec<FileData> {
        let mut newVec = Vec::new();
        for file in vec {
            newVec.push(file.clone());
        }
        newVec
    }
}


