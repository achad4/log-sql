mod apache_log;
mod log_reader;

use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::str::FromStr;
use chrono::{NaiveDateTime};
use crate::apache_log::ApacheLog;

fn create_table(conn: &Connection) -> Result<()> {
    // Use a `CREATE TABLE` statement to create the `apache_logs` table
    conn.execute("DROP TABLE IF EXISTS \"apache_logs\"", params![],)?;
    conn.execute(
        "CREATE TABLE apache_logs (
            ip_address TEXT NOT NULL,
            id TEXT NOT NULL,
            username TEXT NOT NULL,
            time INTEGER NOT NULL,
            request TEXT NOT NULL,
            resource TEXT NOT NULL,
            protocol TEXT NOT NULL,
            statuscode INTEGER NOT NULL,
            size INTEGER NOT NULL,
            referrer TEXT NOT NULL,
            useragent TEXT NOT NULL
        )",
        params![],
    )?;
    Ok(())
}

fn insert_log(conn: &Connection, log: &ApacheLog) -> Result<()> {
    // Use a prepared statement to insert the log entry
    let mut stmt = conn.prepare(
        "INSERT INTO apache_logs (ip_address, id, username, time, request, resource, protocol, statuscode, size, referrer, useragent) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )?;

    // Bind the values from the log struct to the prepared statement
    stmt.execute(params![
        &log.ip_address(), &log.id(), &log.username(), &log.time().timestamp(), &log.request(), &log.resource(), &log.protocol(),
        &log.statuscode(), &log.size(), &log.referrer(), &log.useragent()
    ])?;

    Ok(())
}

fn select_logs(conn: &Connection) -> Result<Vec<ApacheLog>> {
    let query = "SELECT ip_address, id, username, time, request, resource, protocol, statuscode, size, referrer, useragent FROM apache_logs";
    let mut statement = conn.prepare(query)?;
    let apache_log_iter = statement
        .query_map(NO_PARAMS, |row| Ok(ApacheLog {
            ip_address: row.get(0)?,
            id: row.get(1)?,
            username: row.get(2)?,
            time: NaiveDateTime::from_timestamp(row.get(3).unwrap(), 0),
            request: row.get(4)?,
            resource: row.get(5)?,
            protocol: row.get(6)?,
            statuscode: row.get(7)?,
            size: row.get(8)?,
            referrer: row.get(9)?,
            useragent: row.get(10)?,
        }))?;

    let mut apache_logs: Vec<ApacheLog> = Vec::new();
    for log in apache_log_iter {
        let apache_log = log?;
        apache_logs.push(apache_log);
    }
    Ok(apache_logs)
}


fn main() {

    let conn = Connection::open("logs.db").unwrap();
    let mut logs :Vec<ApacheLog> = Vec::new();
    log_reader::update_logs("logs.txt", &mut logs);
    create_table(&conn).expect("Create table failed");
    for log in logs {
        insert_log(&conn, &log).unwrap();
    }
    println!("{:?}", select_logs(&conn));

}