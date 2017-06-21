#[macro_use]
extern crate lazy_static;
extern crate mimir;

mod context;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
lazy_static! {
    pub static ref CREDS: Vec<String> = {
        let file = File::open(".creds/oic-test")
            .expect("bad creds");
        let buf_reader = BufReader::new(file);
        let mut creds = Vec::new();

        for line_res in buf_reader.lines() {
            if let Ok(line) = line_res {
                let parts = line.split(":").map(|x| {
                    x.trim_right().to_string()
                }).collect::<Vec<String>>();
                creds.extend(parts);
            }
        }
        creds
    };
}
