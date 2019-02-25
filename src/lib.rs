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
    let (book_title, author, start_date, end_date) = reading_now();
    let book_title = book_title.trim();
    let author = author.trim();
    let start_date = start_date.trim();
    let end_date = end_date.trim();

    output_current_reading_to_user(book_title, author, start_date, end_date);

    write_present_reading(book_title, author, start_date, end_date).expect("error writing to file");
}

fn finish() {
    let (book_title, summary) = reading_finish();

    let contents = read_to_string("reading.txt").expect("error reading");

    let (book_description, book_title) = find_in_file(book_title, &contents);

    let regex_to_match =
        Regex::new(r#"(Reading ")(.*)(", by ")(.*)"(. Started: ")(\w+)"(.*)"#).unwrap();

    let captures = regex_to_match.captures(book_description).unwrap();

    let book_title = book_title.trim();
    let author = captures.get(4).unwrap().as_str();
    let start_date = captures.get(6).unwrap().as_str();
    let end_date = Utc::today();
    let summary = summary.trim();

    output_finished_reading_to_user(book_title, author, start_date, end_date, summary);

    write_past_reading(book_title, author, start_date, end_date, &summary)
        .expect("error writing to file");
}

fn future() {
    let (book_title, author, motivation) = reading_future();

    let book_title = book_title.trim();
    let author = author.trim();
    let motivation = motivation.trim();

    output_future_reading_to_user(book_title, author);

    write_future_reading(book_title, author, motivation).expect("error writing to file");
}
