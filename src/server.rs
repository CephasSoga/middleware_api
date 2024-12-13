use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use serde::{Deserialize, Serialize};
use serde_json;

const SERVER_MODULES_PATH: &str = "./server.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    name: String,
    path: String,
    modules: Vec<String>,
}

// Custom error type for parsing errors
#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> ParseError {
        ParseError::Json(err)
    }
}

pub struct ServerConfig {
    pub path: String,
}
impl ServerConfig {
    pub fn new(path: Option<&str>) -> Self {
        let path = path.unwrap_or(SERVER_MODULES_PATH);
        Self { path: path.to_string() }
    }

    pub fn parse(&self) -> Result<HashMap<String, Service>, ParseError> { // Updated return type
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let services = serde_json::from_reader(reader)?;
        Ok(services)
    }
}
