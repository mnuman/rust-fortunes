use lazy_static::lazy_static;
use rand::prelude::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

lazy_static! {
    static ref FORTUNES: Vec<Vec<String>> = init();
}

lazy_static! {
    static ref FORTUNES_SERVED_COUNTER: AtomicU64 = AtomicU64::new(0);
}

pub fn init() -> Vec<Vec<String>> {
    let path = Path::new("fortunes.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_error) => panic!("Failed to open file"),
    };
    let reader = io::BufReader::new(file);

    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim() == "%" {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
        } else {
            current_group.push(line);
        }
    }

    // Push the last group if it's not empty
    if !current_group.is_empty() {
        groups.push(current_group);
    }
    groups
}

pub fn get_random_fortune() -> String {
    increment_fortunes_served();
    FORTUNES.choose(&mut rand::thread_rng()).unwrap().join("\n")
}

pub fn unique_fortunes() -> u64 {
    FORTUNES.len() as u64
}

pub fn increment_fortunes_served() {
    FORTUNES_SERVED_COUNTER.fetch_add(1, Ordering::SeqCst);
}

pub fn get_fortunes_served() -> u64 {
    FORTUNES_SERVED_COUNTER.load(Ordering::SeqCst)
}
