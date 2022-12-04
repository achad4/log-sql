
use crate::apache_log;
use crate::log_db;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;
use chrono::{NaiveDateTime};
use rusqlite::{Connection};
use crate::apache_log::ApacheLog;

// This function updates the Vec<ApacheLog> with any new log lines that have
// been added to the file since the last time this function was called.
pub fn update_logs(conn: &Connection, file_path: &str, lines_read: usize) -> Result<usize, ()> {
    let f = File::open(file_path).unwrap();
    let reader = io::BufReader::new(f);
    let mut lines: Lines<BufReader<File>> = reader.lines();
    // Skip over lines that have already been read
    for _ in 0..lines_read {
        lines.next();
    }
    let mut logs :Vec<ApacheLog> = Vec::new();
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
    let log_len = logs.len();
    for log in logs {
        log_db::insert_log(&conn, &log).unwrap();
    }
    Ok(log_len)
}