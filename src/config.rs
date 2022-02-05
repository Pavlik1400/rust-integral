use serde::{Deserialize, Serialize};
use serde_json::{self as json};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub abs_error: f64,
    pub rel_error: f64,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,

    pub xsteps: i64,
    pub ysteps: i64,
    pub max_iters: i32,
    pub thread_num: i32,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let content = fs::read_to_string(path).expect("Unable to read file");
        json::from_str(&content).unwrap_or_else(|err| {
            panic!("Json parse error: {:?}", err);
        })
    }
}