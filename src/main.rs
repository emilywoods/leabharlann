extern crate chrono;
extern crate colored;
extern crate regex;

use chrono::{Date, Utc};
use colored::*;
use regex::Regex;
use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::{stdin, stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = parse_config(&args);

    match query {
        "now" | "anois" => now(),
        "finish" | "criochnaigh" => finish(),
        "future" | "sa-todhchai" => future(),
        _ => println!("What are you reading?"),
    }
}

fn parse_config(args: &[String]) -> (&str) {
    let query = &args[1];
    query
}

fn now() {
    let (book_title, author, start_date, end_date) = user_interaction_reading();

    let book_title = book_title.trim();
    let author = author.trim();
    let start_date = start_date.trim();
    let end_date = end_date.trim();

    println!(
        "Reading {}, by {}. Started: {} and ended: {}",
        book_title.bright_green(),
        author.bright_green(),
        start_date.bright_green(),
        end_date.bright_green()
    );

    write_to_file_reading(book_title, author, start_date, end_date).expect("error writing to file");
}

fn finish() {
    let (book_title, thoughts) = user_interaction_finish();

    let contents = read_to_string("reading.txt").expect("error reading");

    let (book_description, book_title) = find_book_in_storage(book_title, &contents);

    let regex_to_match =
        Regex::new(r#"(Reading ")(.*)(", by ")(.*)"(. Started: ")(\w+)"(.*)"#).unwrap();

    let captures = regex_to_match.captures(book_description).unwrap();

    let book_title = book_title.trim();
    let author = captures.get(4).unwrap().as_str();
    let start_date = captures.get(6).unwrap().as_str();
    let end_date = Utc::today();
    let thoughts = thoughts.trim();

    println!(
        "Finished reading {}, by {}. Started: {} and ended: {}.\nConcluding thoughts: {}",
        book_title.bright_magenta(),
        author.bright_magenta(),
        start_date.bright_magenta(),
        end_date.to_string().bright_magenta(),
        thoughts.bright_magenta()
    );

    write_to_file_finished(book_title, author, start_date, end_date, &thoughts)
        .expect("error writing to file");
}

fn future() {
    let (book_title, author, motivation) = user_interaction_future();

    let book_title = book_title.trim();
    let author = author.trim();
    let motivation = motivation.trim();

    println!(
        "Future reading: {}, by {}",
        book_title.bright_red(),
        author.bright_red()
    );

    write_to_file_future(book_title, author, motivation).expect("error writing to file");
}

fn user_interaction_reading() -> (String, String, String, String) {
    let mut book_title = String::new();
    let mut author = String::new();
    let mut start_date = String::new();
    let mut end_date = String::new();

    let _ = stdout().flush(); // what is the purpose of this?

    println!("{}", "What are you reading?".bright_blue());
    stdin()
        .read_line(&mut book_title)
        .expect("Failed to read line");

    println!("{}", "Who wrote it?".bright_blue());
    stdin().read_line(&mut author).expect("Failed to read line");

    println!("{}", "When did you start?".bright_blue());
    stdin()
        .read_line(&mut start_date)
        .expect("Failed to read line");

    println!("{}", "When did you finish?".bright_blue());
    stdin()
        .read_line(&mut end_date)
        .expect("Failed to read line");

    (book_title, author, start_date, end_date)
}

fn user_interaction_future() -> (String, String, String) {
    let mut book_title = String::new();
    let mut author = String::new();
    let mut motivation = String::new();

    let _ = stdout().flush(); // what is the purpose of this?

    println!("{}", "What book do you want to read?".cyan());
    stdin()
        .read_line(&mut book_title)
        .expect("Failed to read line");

    println!("{}", "Who wrote it?".cyan());
    stdin().read_line(&mut author).expect("Failed to read line");

    println!("{}", "Why do you want to read it?".cyan());
    stdin()
        .read_line(&mut motivation)
        .expect("Failed to read line");

    (book_title, author, motivation)
}

fn user_interaction_finish() -> (String, String) {
    let mut book_title = String::new();
    let mut thoughts = String::new();

    let _ = stdout().flush(); // what is the purpose of this?

    println!("{}", "What's the book?".bright_yellow());
    stdin()
        .read_line(&mut book_title)
        .expect("Failed to read line");

    println!(
        "{}",
        "Any concluding thoughts/summary/things to remember it by?".bright_yellow()
    );
    stdin()
        .read_line(&mut thoughts)
        .expect("Failed to read line");

    (book_title, thoughts)
}

fn write_to_file_reading(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: &str,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("reading.txt")?;

    file.write_fmt(format_args!(
        "Reading {:?}, by {:?}. Started: {:?} and ended: {:?}",
        book_title, author, start_date, end_date
    ))?;

    Ok(())
}

fn write_to_file_future(book_title: &str, author: &str, motivation: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("reading.txt")?;

    file.write_fmt(format_args!(
        "Future reading {:?}, by {:?}, because: {:?}\n",
        book_title, author, motivation,
    ))?;

    Ok(())
}

fn write_to_file_finished(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: Date<Utc>,
    thoughts: &str,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("reading.txt")?;

    file.write_fmt(format_args!(
        "Finished reading {:?}, by {:?}. Started: {:?} and ended: {:?}. Concluding thoughts: {:?}\n",
        book_title, author, start_date, end_date, thoughts
    ))?;

    Ok(())
}

fn find_book_in_storage(book_title: String, contents: &str) -> (&str, String) {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&book_title.trim()) {
            results.push(line);
        }
    }

    (results.first().expect("Empty array"), book_title)
}
