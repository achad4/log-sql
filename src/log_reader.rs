
use crate::apache_log;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;
use chrono::{NaiveDateTime};
use crate::apache_log::ApacheLog;

// This function updates the Vec<ApacheLog> with any new log lines that have
// been added to the file since the last time this function was called.
pub fn update_logs(file_path: &str, logs: &mut Vec<ApacheLog>) {
    let f = File::open(file_path).unwrap();
    let reader = io::BufReader::new(f);
    let mut lines: Lines<BufReader<File>> = reader.lines();
    // Skip over lines that have already been read
    for _ in 0..logs.len() {
        lines.next();
    }
    // Read new lines and update the Vec<ApacheLog>
    for line in lines {
        let line = line.unwrap();
        println!("{}", line);
        if !line.trim().is_empty() {
            let apache_log = ApacheLog::from_str(&line);
            match apache_log {
                Ok(_) => logs.push(apache_log.unwrap()),
                Err(_) => println!("Ignoring line {}", line),
            }
        }
    }
}