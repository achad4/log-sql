use std::io::ErrorKind;
use std::str::FromStr;
use chrono::{NaiveDateTime};
use recap::lazy_static;
use recap::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApacheLog {
    pub ip_address: String,
    pub id: String,
    pub username: String,
    pub time: NaiveDateTime,
    pub request: String,
    pub resource: String,
    pub protocol: String,
    pub statuscode: i64,
    pub size: i64,
    pub referrer: String,
    pub useragent: String,
}

impl ApacheLog {
    pub fn ip_address(&self) -> &str {
        &self.ip_address
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn time(&self) -> NaiveDateTime {
        self.time
    }
    pub fn request(&self) -> &str {
        &self.request
    }
    pub fn resource(&self) -> &str {
        &self.resource
    }
    pub fn protocol(&self) -> &str {
        &self.protocol
    }
    pub fn statuscode(&self) -> i64 {
        self.statuscode
    }
    pub fn size(&self) -> i64 {
        self.size
    }
    pub fn referrer(&self) -> &str {
        &self.referrer
    }
    pub fn useragent(&self) -> &str {
        &self.useragent
    }
}

const APACHE_LOG_REGEX: &str = r#"^(?P<ip_address>\S+) (?P<id>\S+) (?P<username>\S+) \[(?P<time>[\w:/]+\s[+\-]\d{4})\] "(?P<request>\S+)\s?(?P<resource>\S+)?\s?(?P<protocol>\S+)?" (?P<statuscode>\d{3}|-) (?P<size>\d+|-)\s?"?(?P<referrer>[^"]*)"?\s?"?(?P<useragent>[^"]*)?"?$"#;

impl FromStr for ApacheLog {
    type Err = std::io::Error;

    fn from_str(log: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref regex: Regex = Regex::new(APACHE_LOG_REGEX).unwrap();
        }
        let cap = regex.captures(log).ok_or(std::io::Error::new(ErrorKind::InvalidInput, format!("Log {log} unable to be parsed")))?;
        println!("Parsing log: {}", log);
        Ok(ApacheLog {
            ip_address: cap.name("ip_address").unwrap().as_str().to_string(),
            id: cap.name("ip_address").unwrap().as_str().to_string(),
            username: cap.name("username").unwrap().as_str().to_string(),
            time: NaiveDateTime::parse_from_str(cap.name("time").unwrap().as_str(), "%d/%b/%Y:%H:%M:%S %z").unwrap(),
            request: cap.name("request").unwrap().as_str().to_string(),
            resource: cap.name("resource").unwrap().as_str().to_string(),
            protocol: cap.name("protocol").expect("Couldn't parse protocol").as_str().to_string(),
            statuscode: cap.name("statuscode").expect("Couldn't parse status_code").as_str().parse().unwrap(),
            size: cap.name("size").unwrap().as_str().parse().unwrap(),
            referrer: cap.name("referrer").unwrap().as_str().to_string(),
            useragent: cap.name("useragent").unwrap().as_str().to_string(),
        })
    }
}