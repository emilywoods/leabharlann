extern crate chrono;
extern crate colored;
extern crate regex;

use chrono::Utc;
use cli::{
    output_current_reading_to_user, output_finished_reading_to_user, output_future_reading_to_user,
    reading_finish, reading_future, reading_now,
};
use file_io::{
    find_in_file, initialise_file, write_future_reading, write_past_reading, write_present_reading,
};
use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

#[macro_use]
mod cli;
mod file_io;

static FILENAME: &str = "reading.csv";
static CURRENT_READING_ENTRY: &str = r#"(current,)(.*),(.*),(.*),(.*),(.*)"#;
static ERROR_WRITING: &str = "Error writing to file";

pub struct Config {
    query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();

        Ok(Config { query })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    initialise_file().expect("error creating file.");

    match config.query.as_str() {
        "now" | "anois" => now(),
        "finish" | "criochnaigh" => finish(),
        "future" | "sa-todhchai" => future(),
        _ => panic!("incorrect input"),
    }

    Ok(())
}

fn now() {
    let (book_title, author, start_date, end_date, motivation) = reading_now();
    let book_title = book_title.trim();
    let author = author.trim();
    let start_date = start_date.trim();
    let end_date = end_date.trim();
    let motivation = motivation.trim();

    output_current_reading_to_user(book_title, author, start_date, end_date, motivation);

    write_present_reading(book_title, author, start_date, end_date, motivation)
        .expect(ERROR_WRITING);
}

fn finish() {
    let (book_title, summary) = reading_finish();

    let contents = read_to_string(FILENAME).expect("error reading file");

    let (book_entry, book_title) = find_in_file(book_title, &contents);

    let regex_to_match = Regex::new(CURRENT_READING_ENTRY).unwrap();

    let captures = regex_to_match.captures(book_entry).unwrap();

    let book_title = book_title.trim();
    let author = captures.get(3).unwrap().as_str();
    let start_date = captures.get(4).unwrap().as_str();
    let end_date = Utc::today();
    let summary = summary.trim();

    output_finished_reading_to_user(book_title, author, start_date, end_date, summary);

    write_past_reading(book_title, author, start_date, end_date, &summary).expect(ERROR_WRITING);
}

fn future() {
    let (book_title, author, motivation) = reading_future();

    let book_title = book_title.trim();
    let author = author.trim();
    let motivation = motivation.trim();

    output_future_reading_to_user(book_title, author);

    write_future_reading(book_title, author, motivation).expect(ERROR_WRITING);
}
