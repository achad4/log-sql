mod apache_log;
mod log_reader;
mod log_db;

use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::str::FromStr;
use chrono::{NaiveDateTime};
use crate::apache_log::ApacheLog;
use std::thread;
use std::time::Duration;
use std::sync::Arc;


fn main() {

    let outer_conn =  Connection::open("logs.db").unwrap();
    log_db::create_table(&outer_conn).expect("Create table failed");

    let handle = thread::spawn(move || {
        let conn = Connection::open("logs.db").unwrap();
        let mut lines_read = 0;
        loop {
            let l = log_reader::update_logs(&conn, "logs.txt", lines_read);
            lines_read = lines_read + l.unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // log_reader::update_logs("logs.txt", &mut logs);

    loop {
        println!("in main thread: {:?}", log_db::select_logs(&outer_conn));
        thread::sleep(Duration::from_secs(5));
    }



}