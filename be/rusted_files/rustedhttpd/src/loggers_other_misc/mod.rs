use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crossbeam_channel::Receiver;

pub fn get_epoch_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}

// Still working on this one
//pub async fn logger_thread(rec: Receiver<String>){
//
//    let file = OpenOptions::new()
//                .read(true)
//                .write(true)
//                .create(true)
//                .open("logs.txt");
//
//    while let Ok(string) = rec.recv() {
//
//        println!("{}", string);
//    }
//}