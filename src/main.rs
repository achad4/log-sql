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
        // Print a prompt
        println!("> ");

        // Flush the output so the prompt is displayed
        // io::stdout().flush().unwrap();

        // Read a line of input from the user
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();

        // Print the input
        println!("You entered: {}", query);
        println!("in main thread: {:?}", log_db::select_logs(&outer_conn, &query));
    }

    // loop {
    //     println!("in main thread: {:?}", log_db::select_logs(&outer_conn));
    //     thread::sleep(Duration::from_secs(5));
    // }



}