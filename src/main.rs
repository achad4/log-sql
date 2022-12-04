mod apache_log;
mod log_reader;
mod log_db;

use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::str::FromStr;
use chrono::{NaiveDateTime};
use crate::apache_log::ApacheLog;
use std::thread;
use std::time::Duration;
use std::io;
use polars::prelude::*;
use serde_json;

fn main() {

    let outer_conn =  Connection::open("logs.db").unwrap();
    log_db::create_table(&outer_conn).expect("Create table failed");

    thread::spawn(move || {
        let conn = Connection::open("logs.db").unwrap();
        let mut lines_read = 0;
        loop {
            let l = log_reader::update_logs(&conn, "logs.txt", lines_read);
            lines_read = lines_read + l.unwrap();
            thread::sleep(Duration::from_secs(5));
        }
    });

    loop {
        println!("Log SQL > ");

        // Read a line of input from the user
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();

        // Print the input
        println!("You entered: {}", query);
        let query_result = log_db::select_logs(&outer_conn, &query).expect("Failed to get logs from DB");
        let json_data = serde_json::to_string(&query_result).unwrap();

        let schema = Schema::from(vec![
            Field::new("ip_address", DataType::Utf8),
            Field::new("id", DataType::Utf8),
            Field::new("username", DataType::Utf8),
            Field::new("time", DataType::UInt64),
            Field::new("request", DataType::Utf8),
            Field::new("resource", DataType::Utf8),
            Field::new("protocol", DataType::Utf8),
            Field::new("statuscode", DataType::UInt64),
            Field::new("size", DataType::UInt64),
            Field::new("referrer", DataType::Utf8),
            Field::new("useragent", DataType::Utf8),
        ].into_iter()
        );

        let cursor = std::io::Cursor::new(&json_data);
        let df = JsonReader::new(cursor)
            .with_schema(&schema)
            .finish().unwrap();
        println!("{:?}", df)
    }

    // loop {
    //     println!("in main thread: {:?}", log_db::select_logs(&outer_conn));
    //     thread::sleep(Duration::from_secs(5));
    // }



}