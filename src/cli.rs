extern crate chrono;
extern crate colored;

use chrono::{Date, Utc};
use colored::*;
use std::io::{stdin, stdout, Write};

pub fn reading_now() -> (String, String, String, String) {
    flush();

    let book_title = get_book_title();
    let author = get_author();
    let start_date = get_start_date();
    let end_date = get_end_date();

    (book_title, author, start_date, end_date)
}

pub fn reading_future() -> (String, String, String) {
    flush();

    let book_title = get_book_title();
    let author = get_author();
    let motivation = get_motivation();

    (book_title, author, motivation)
}

pub fn reading_finish() -> (String, String) {
    flush();

    let book_title = get_book_title();
    let summary = get_summary();

    (book_title, summary)
}

pub fn output_current_reading_to_user(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: &str,
) {
    println!(
        "\nReading {}, by {}. Started: {} and ended: {}",
        book_title.bright_green(),
        author.bright_green(),
        start_date.bright_green(),
        end_date.bright_green()
    );
}

pub fn output_future_reading_to_user(book_title: &str, author: &str) {
    println!(
        "\nFuture reading: {}, by {}",
        book_title.bright_red(),
        author.bright_red()
    );
}

pub fn output_finished_reading_to_user(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: Date<Utc>,
    summary: &str,
) {
    println!(
        "\nFinished reading {}, by {}. Started: {} and ended: {}.\nConcluding thoughts: {}",
        book_title.bright_magenta(),
        author.bright_magenta(),
        start_date.bright_magenta(),
        end_date.to_string().bright_magenta(),
        summary.bright_magenta()
    );
}

fn flush() -> () {
    let _ = stdout().flush();
}

fn get_book_title() -> String {
    let mut book_title = String::new();

    println!("{}", "What's the book's title?".cyan());
    stdin()
        .read_line(&mut book_title)
        .expect("Failed to read line");

    book_title
}

fn get_author() -> String {
    let mut author = String::new();
    println!("{}", "Who wrote it?".bright_blue());
    stdin().read_line(&mut author).expect("Failed to read line");

    author
}

fn get_motivation() -> String {
    let mut motivation = String::new();

    println!("{}", "Why do you want to read it?".bright_yellow());
    stdin()
        .read_line(&mut motivation)
        .expect("Failed to read line");

    motivation
}

fn get_start_date() -> String {
    let mut start_date = String::new();

    println!("{}", "When did you start?".bright_yellow());
    stdin()
        .read_line(&mut start_date)
        .expect("Failed to read line");

    start_date
}

fn get_end_date() -> String {
    let mut end_date = String::new();

    println!("{}", "When did you finish?".bright_red());
    stdin()
        .read_line(&mut end_date)
        .expect("Failed to read line");

    end_date
}

fn get_summary() -> String {
    let mut summary = String::new();

    println!(
        "{}",
        "Any concluding thoughts/summary/things to remember it by?".bright_yellow()
    );
    stdin()
        .read_line(&mut summary)
        .expect("Failed to read line");

    summary
}
